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


use std::collections::BTreeMap;
use crate::core::execution::trace::from_model::from_model::InteractionInterpretableAsTraceActions;
use crate::core::execution::trace::trace::TraceActionKind;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::message::MessageTypeExpression;

pub struct InteractionSyntacticSimplifier {}

impl InteractionSyntacticSimplifier {

    fn cr_inclusion(cr1 : &Vec<usize>, cr2 : &Vec<usize>) -> bool {
        for lf in cr1 {
            if !cr2.contains(lf) {
                return false;
            }
        }
        true
    }

    pub fn simplify_loop(cr1 : Vec<usize>, i1 : Interaction) -> Interaction {
        match i1 {
            Interaction::Empty => {
                Interaction::Empty
            },
            Interaction::Loop(cr2,i11) => {
                // if one is included in the other
                if Self::cr_inclusion(&cr1,&cr2) {
                    Interaction::Loop(cr2, i11)
                } else if Self::cr_inclusion(&cr2,&cr1) {
                    Interaction::Loop(cr1, i11)
                } else {
                    // TODO: check what can be done (union ? maybe not)
                    Interaction::Loop(cr1, Box::new(Interaction::Loop(cr2,i11)))
                }
            },
            x => {
                x
            }
        }
    }

    pub fn simplify_alt(i1 : Interaction, i2 : Interaction) -> Interaction {
        match (i1,i2) {
            (Interaction::Empty,Interaction::Empty) => {
                Interaction::Empty
            },
            (Interaction::Empty,Interaction::Loop(cr,i21)) => {
                Interaction::Loop(cr,i21)
            },
            (Interaction::Loop(cr,i11),Interaction::Empty) => {
                Interaction::Loop(cr,i11)
            },
            (pi1,pi2) => {
                Interaction::Alt( Box::new( pi1),
                                  Box::new( pi2) )
            }
        }
    }

    pub fn simplify_coreg(cr : Vec<usize>, i1 : Interaction, i2 : Interaction) -> Interaction {
        if i1 == Interaction::Empty {
            i2
        } else {
            if i2 == Interaction::Empty {
                i1
            } else {
                Interaction::CoReg( cr,
                                           Box::new(i1),
                                           Box::new(i2) )
            }
        }
    }

    pub fn simplify_sync(sync : BTreeMap<(usize,TraceActionKind),MessageTypeExpression>,
                         i1 : Interaction,
                         i2 : Interaction,
                         gen_ctx : &GeneralContext) -> Interaction {
        let acts1 = i1.get_all_trace_actions();
        let acts2 = i2.get_all_trace_actions();
        // ***
        let mut has_synchronization_actions = false;
        for act in acts1.union(&acts2) {
            if let Some(mt) = sync.get(&(act.lf_id,act.act_kind)) {
                let intersect = MessageTypeExpression::Intersection(
                    Box::new(mt.clone()),
                    Box::new(act.message.clone()));
                if !intersect.resolve(gen_ctx).is_empty() {
                    has_synchronization_actions = true;
                    break;
                }
            }
        }
        // ***
        if has_synchronization_actions {
            Interaction::Sync(sync,
                              Box::new(i1) ,
                              Box::new(i2))
        } else {
            if i1 == Interaction::Empty {
                i2
            } else {
                if i2 == Interaction::Empty {
                    i1
                } else {
                    let all_lfs : Vec<usize> = (0..gen_ctx.get_lf_num()).collect();
                    Interaction::CoReg( all_lfs,
                                        Box::new(i1),
                                        Box::new(i2) )
                }
            }
        }
    }

}