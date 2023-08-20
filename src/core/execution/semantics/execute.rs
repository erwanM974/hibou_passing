/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/



use std::collections::HashSet;

use crate::core::execution::semantics::position::Position;

use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::core::general_context::GeneralContext;
use crate::core::language::prune::prunable::LifelinePrunable;
use crate::core::language::syntax::action::BroadcastPrimitive;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::simplify::InteractionSyntacticSimplifier;


fn execute_broadcast_primitive(bp : &BroadcastPrimitive,
                               sub_pos : &Option<usize>,
                               target_action : &TraceAction,
                               gen_ctx : &GeneralContext) -> Interaction {
    assert!(target_action.message.resolve(gen_ctx).is_subset(&bp.message_type.resolve(gen_ctx)));
    match (bp.origin_on_emission, sub_pos) {
        (Some(origin),None) => {
            assert_eq!(origin,target_action.lf_id);
            assert_eq!(target_action.act_kind, TraceActionKind::Emission);
            if bp.targets.is_empty() {
                Interaction::Empty
            } else {
                Interaction::Action(BroadcastPrimitive::new(None,
                                                            target_action.message.clone(),
                                                            bp.targets.clone()))
            }
        },
        (None,Some(target_idx)) => {
            assert!(bp.targets.len() > *target_idx);
            assert_eq!(target_action.act_kind, TraceActionKind::Reception);
            let mut new_targets = bp.targets.clone();
            new_targets.remove(*target_idx);
            if new_targets.is_empty() {
                Interaction::Empty
            } else {
                Interaction::Action(BroadcastPrimitive::new(None,
                                                            target_action.message.clone(),
                                                            new_targets))
            }
        },
        _ => {
            panic!()
        }
    }
}




fn make_follow_up_loop(old_i1 : &Interaction,
                       new_i1 : Interaction,
                       cr : &Vec<usize>,
                       ex_act_lf_id : usize,
                       gen_ctx : &GeneralContext) -> Interaction {
    let orig_i = Interaction::Loop(cr.clone(), Box::new(old_i1.clone() ) );
    if new_i1 == Interaction::Empty {
        orig_i
    } else {
        let mut lfs_to_prune : HashSet<usize> = cr.iter().cloned().collect();
        lfs_to_prune.insert(ex_act_lf_id);
        let pruned_loop = orig_i.prune(&lfs_to_prune, gen_ctx);
        let new_right_int = Interaction::CoReg( cr.clone(),
                                                Box::new(new_i1),
                                                Box::new(orig_i) );
        // ***
        if pruned_loop == Interaction::Empty {
            return new_right_int;
        } else {
            return Interaction::CoReg( cr.clone(),
                                       Box::new(pruned_loop),
                                       Box::new(new_right_int) );
        }
    }
}


fn execute_interaction_left(my_int : &Interaction,
                            sub_p1 : &Position,
                            target_action : &TraceAction,
                            gen_ctx : &GeneralContext) -> Interaction {
    match my_int {
        Interaction::Alt(i1, _) => {
            execute_interaction( i1, sub_p1, target_action,gen_ctx)
        },
        Interaction::Loop(cr, i1) => {
            let new_i1 = execute_interaction(i1,sub_p1,target_action,gen_ctx);
            make_follow_up_loop(i1,new_i1,cr,target_action.lf_id, gen_ctx)
        },
        Interaction::CoReg(cr,i1,i2) => {
            let new_i1 = execute_interaction(i1,sub_p1,target_action, gen_ctx);
            InteractionSyntacticSimplifier::simplify_coreg(cr.clone(),new_i1,*i2.clone())
        },
        Interaction::Sync(sync,i1,i2) => {
            let new_i1 = execute_interaction(i1,sub_p1,target_action, gen_ctx);
            InteractionSyntacticSimplifier::simplify_sync(sync.clone(),new_i1,*i2.clone(),gen_ctx)
        },
        _ => {
            panic!("trying to execute left on {:?}", my_int);
        }
    }
}



fn execute_interaction_right(my_int : &Interaction,
                             sub_p2 : &Position,
                             target_action : &TraceAction,
                             gen_ctx : &GeneralContext) -> Interaction {
    match my_int {
        Interaction::Alt(_,i2) => {
            execute_interaction( i2,sub_p2, target_action,gen_ctx)
        },
        Interaction::CoReg(cr,i1,i2) => {
            let new_i1 = if cr.contains(&target_action.lf_id) {
                *i1.clone()
            } else {
                i1.prune(&hashset!{target_action.lf_id}, gen_ctx)
            };
            let new_i2 = execute_interaction(i2,sub_p2,target_action,gen_ctx);
            InteractionSyntacticSimplifier::simplify_coreg(cr.clone(),new_i1,new_i2)
        },
        Interaction::Sync(sync, i1,i2) => {
            let new_i2 = execute_interaction(i2,sub_p2, target_action, gen_ctx);
            InteractionSyntacticSimplifier::simplify_sync(sync.clone(),*i1.clone(),new_i2,gen_ctx)
        },
        _ => {
            panic!("trying to execute right on {:?}", my_int);
        }
    }
}

fn execute_interaction_both(my_int : &Interaction,
                            sub_p1 : &Position,
                            sub_p2 : &Position,
                            target_action : &TraceAction,
                            gen_ctx : &GeneralContext) -> Interaction {
    match my_int {
        Interaction::Alt(i1,i2) => {
            let new_i1 = execute_interaction(i1,sub_p1, target_action,gen_ctx);
            let new_i2 = execute_interaction(i2,sub_p2, target_action,gen_ctx);
            InteractionSyntacticSimplifier::simplify_alt(new_i1,new_i2)
        },
        Interaction::Sync(sync, i1, i2) => {
            let new_i1 = execute_interaction(i1,sub_p1, target_action,gen_ctx);
            let new_i2 = execute_interaction(i2,sub_p2, target_action,gen_ctx);
            InteractionSyntacticSimplifier::simplify_sync(sync.clone(),new_i1,new_i2,gen_ctx)
        },
        _ => {
            panic!("trying to execute both left and right on {:?}", my_int);
        }
    }
}

pub fn execute_interaction(my_int : &Interaction,
                           my_pos : &Position,
                           target_action : &TraceAction,
                           gen_ctx : &GeneralContext) -> Interaction {
    match my_pos {
        Position::Epsilon(sub_pos) => {
            match my_int {
                Interaction::Action(bp) => {
                    execute_broadcast_primitive(bp,sub_pos,target_action,gen_ctx)
                },
                _ => {
                    panic!()
                }
            }
        },
        Position::Left(p1) => {
            execute_interaction_left(my_int,p1,target_action,gen_ctx)
        },
        Position::Right(p2) => {
            execute_interaction_right(my_int,p2,target_action,gen_ctx)
        },
        Position::Both(p1,p2) => {
            execute_interaction_both(my_int,p1,p2,target_action,gen_ctx)
        }
    }
}