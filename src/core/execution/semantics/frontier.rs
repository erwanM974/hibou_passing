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
use crate::core::language::syntax::interaction::Interaction;
use crate::core::message::MessageTypeExpression;


#[derive(Clone, PartialEq, Debug)]
pub struct FrontierElement {
    pub position : Position,
    pub target_action : TraceAction,
    pub max_loop_depth : u32
}

impl FrontierElement {
    pub fn new(position: Position, target_action: TraceAction, max_loop_depth: u32) -> Self {
        Self { position, target_action, max_loop_depth }
    }
}


pub fn global_frontier(interaction : &Interaction,
                       gen_ctx : &GeneralContext,
                   to_match : &Option<HashSet<&TraceAction>>) -> Vec<FrontierElement> {
    match to_match {
        None => {
            return abstract_frontier_rec(interaction, 0,gen_ctx);
        },
        Some( to_match_set ) => {
            let mut frt = vec![];
            for frt_elt in abstract_frontier_rec(interaction, 0,gen_ctx) {
                for to_match in to_match_set {
                    if to_match.lf_id == frt_elt.target_action.lf_id
                        && to_match.act_kind == frt_elt.target_action.act_kind {
                        let intersect = MessageTypeExpression::Intersection(
                            Box::new(to_match.message.clone()),
                            Box::new(frt_elt.target_action.message.clone()));
                        if !intersect.resolve(gen_ctx).is_empty() {
                            frt.push(FrontierElement::new(frt_elt.position.clone(),
                                                          TraceAction::new(frt_elt.target_action.lf_id,
                                                                           frt_elt.target_action.act_kind,
                                                                           intersect.simplify(gen_ctx)),
                                                          frt_elt.max_loop_depth));
                        }
                    }
                }
            }
            return frt;
        }
    }
}



fn abstract_frontier_rec(interaction : &Interaction,
                         loop_depth : u32,
                         gen_ctx : &GeneralContext) -> Vec<FrontierElement> {
    match interaction {
        Interaction::Empty => {
            vec![]
        },
        Interaction::Action( bp ) => {
            if let Some(origin) = bp.origin_on_emission {
                vec![FrontierElement::new(Position::Epsilon(None),
                                          TraceAction::new(origin,
                                                           TraceActionKind::Emission,
                                                           bp.message_type.clone()),
                                          loop_depth)]
            } else {
                let mut frt = vec![];
                for (rcp_idx,rcp_lf_id) in bp.targets.iter().enumerate() {
                    let reception_tract = TraceAction::new(*rcp_lf_id,
                                                           TraceActionKind::Reception,
                                                           bp.message_type.clone());
                    frt.push( FrontierElement::new(Position::Epsilon(Some(rcp_idx)),
                                                   reception_tract,
                                                   loop_depth) );
                }
                frt
            }
        },
        Interaction::CoReg(ref cr, ref i1, ref i2) => {
            let mut front = push_frontier_left( &mut abstract_frontier_rec(i1,loop_depth,gen_ctx) );
            // ***
            let avoidance_map = i1.get_avoidance_map(gen_ctx);
            for frt_elt2 in push_frontier_right( &mut abstract_frontier_rec(i2,loop_depth,gen_ctx)) {
                if cr.contains(&frt_elt2.target_action.lf_id) || *avoidance_map.get(frt_elt2.target_action.lf_id).unwrap() {
                    front.push(frt_elt2)
                }
            }
            front
        },
        Interaction::Alt(ref i1, ref i2) => {
            let mut match_indices : HashSet<(usize,usize)> = hashset! {};
            let mut frt1_matched : HashSet<usize> = hashset![];
            let mut frt2_matched : HashSet<usize> = hashset![];
            // ***
            let frt1 = abstract_frontier_rec(i1,loop_depth,gen_ctx);
            let frt2 = abstract_frontier_rec(i2,loop_depth,gen_ctx);
            // ***
            for (frt1_idx,frt1_elt) in frt1.iter().enumerate() {
                for (frt2_idx,frt2_elt) in frt2.iter().enumerate() {
                    if frt1_elt.target_action == frt2_elt.target_action {
                        frt1_matched.insert(frt1_idx);
                        frt2_matched.insert(frt2_idx);
                        match_indices.insert( (frt1_idx,frt2_idx) );
                    }
                }
            }
            // ***
            let mut new_front = vec![];
            // ***
            for (frt1_idx,frt2_idx) in match_indices {
                let frt1_elt : &FrontierElement = frt1.get(frt1_idx).unwrap();
                let frt2_elt: &FrontierElement = frt2.get(frt2_idx).unwrap();
                let new_pos = Position::Both( Box::new(frt1_elt.position.clone()), Box::new(frt2_elt.position.clone()));
                let new_target_action = frt1_elt.target_action.clone();
                let new_max_loop_depth = frt1_elt.max_loop_depth.max(frt2_elt.max_loop_depth);
                // ***
                new_front.push( FrontierElement::new(new_pos,
                                                     new_target_action,
                                                     new_max_loop_depth ));
            }
            // ***
            for (frt1_idx,frt1_elt) in frt1.into_iter().enumerate() {
                if !frt1_matched.contains(&frt1_idx) {
                    let shifted_pos = Position::Left(Box::new(frt1_elt.position));
                    new_front.push( FrontierElement::new(shifted_pos,
                                                         frt1_elt.target_action,
                                                         frt1_elt.max_loop_depth ));
                }
            }
            // ***
            for (frt2_idx,frt2_elt) in frt2.into_iter().enumerate() {
                if !frt2_matched.contains(&frt2_idx) {
                    let shifted_pos = Position::Right(Box::new(frt2_elt.position));
                    new_front.push( FrontierElement::new(shifted_pos,
                                                         frt2_elt.target_action,
                                                         frt2_elt.max_loop_depth ));
                }
            }
            // ***
            new_front
        },
        Interaction::Sync(ref sync_acts,ref i1, ref i2) => {
            // ***
            let mut new_front : Vec<FrontierElement> = vec![];
            let mut rem_frt1 : Vec<FrontierElement> = vec![];
            let mut rem_frt2 : Vec<FrontierElement> = vec![];
            // ***
            for frt1_elt in abstract_frontier_rec(i1,loop_depth,gen_ctx) {
                match sync_acts.get(&(frt1_elt.target_action.lf_id,frt1_elt.target_action.act_kind)) {
                    None => {
                        let shifted_pos = Position::Left(Box::new(frt1_elt.position));
                        new_front.push( FrontierElement::new(shifted_pos,
                                                             frt1_elt.target_action,
                                                             frt1_elt.max_loop_depth ));
                    },
                    Some(ms_type) => {
                        let intersect = MessageTypeExpression::Intersection(
                            Box::new(frt1_elt.target_action.message.clone()),
                            Box::new(ms_type.clone())
                            );
                        if !intersect.resolve(gen_ctx).is_empty() {
                            rem_frt1.push(FrontierElement::new(frt1_elt.position.clone(),
                                                               TraceAction::new(frt1_elt.target_action.lf_id,
                                                                                frt1_elt.target_action.act_kind,
                                                                                intersect.simplify(gen_ctx)),
                                                               frt1_elt.max_loop_depth ) );
                        }
                        let setminus = MessageTypeExpression::SetMinus(
                            Box::new(frt1_elt.target_action.message.clone()),
                            Box::new(ms_type.clone())
                        );
                        if !setminus.resolve(gen_ctx).is_empty() {
                            let shifted_pos = Position::Left(Box::new(frt1_elt.position));
                            new_front.push(FrontierElement::new(shifted_pos,
                                                               TraceAction::new(frt1_elt.target_action.lf_id,
                                                                                frt1_elt.target_action.act_kind,
                                                                                setminus.simplify(gen_ctx)),
                                                               frt1_elt.max_loop_depth ) );
                        }
                    }
                }
            }
            // ***
            for frt2_elt in abstract_frontier_rec(i2,loop_depth,gen_ctx) {
                match sync_acts.get(&(frt2_elt.target_action.lf_id,frt2_elt.target_action.act_kind)) {
                    None => {
                        let shifted_pos = Position::Right(Box::new(frt2_elt.position));
                        new_front.push( FrontierElement::new(shifted_pos,
                                                             frt2_elt.target_action,
                                                             frt2_elt.max_loop_depth ));
                    },
                    Some(ms_type) => {
                        let intersect = MessageTypeExpression::Intersection(
                            Box::new(frt2_elt.target_action.message.clone()),
                            Box::new(ms_type.clone())
                        );
                        if !intersect.resolve(gen_ctx).is_empty() {
                            rem_frt2.push(FrontierElement::new(frt2_elt.position.clone(),
                                                               TraceAction::new(frt2_elt.target_action.lf_id,
                                                                                frt2_elt.target_action.act_kind,
                                                                                intersect.simplify(gen_ctx)),
                                                               frt2_elt.max_loop_depth ) );
                        }
                        let setminus = MessageTypeExpression::SetMinus(
                            Box::new(frt2_elt.target_action.message.clone()),
                            Box::new(ms_type.clone())
                        );
                        if !setminus.resolve(gen_ctx).is_empty() {
                            let shifted_pos = Position::Right(Box::new(frt2_elt.position));
                            new_front.push(FrontierElement::new(shifted_pos,
                                                                TraceAction::new(frt2_elt.target_action.lf_id,
                                                                                 frt2_elt.target_action.act_kind,
                                                                                 setminus.simplify(gen_ctx)),
                                                                frt2_elt.max_loop_depth ) );
                        }
                    }
                }
            }
            // ***
            for frt1_elt in &rem_frt1 {
                for frt2_elt in &rem_frt2 {
                    if frt1_elt.target_action.lf_id == frt2_elt.target_action.lf_id
                        && frt1_elt.target_action.act_kind == frt2_elt.target_action.act_kind {
                        let intersect = MessageTypeExpression::Intersection(
                            Box::new(frt1_elt.target_action.message.clone()),
                            Box::new(frt2_elt.target_action.message.clone()));
                        if !intersect.resolve(gen_ctx).is_empty() {
                            let new_pos = Position::Both(Box::new(frt1_elt.position.clone()),
                                                         Box::new(frt2_elt.position.clone()));
                            let new_target_action = TraceAction::new(frt1_elt.target_action.lf_id,
                                                                     frt1_elt.target_action.act_kind,
                                                                     intersect.simplify(gen_ctx));
                            let new_max_loop_depth = frt1_elt.max_loop_depth.max(frt2_elt.max_loop_depth);
                            // ***
                            new_front.push( FrontierElement::new(new_pos,
                                                                 new_target_action,
                                                                 new_max_loop_depth ));
                        }
                    }
                }
            }
            // ***
            new_front
        },
        Interaction::Loop(_, ref i1) => {
            push_frontier_left( &mut abstract_frontier_rec(i1,loop_depth+1,gen_ctx) )
        }
    }
}



fn push_frontier_left(frontier : &mut Vec<FrontierElement>) -> Vec<FrontierElement> {
    frontier.drain(..)
        .map(|frt_elt|
            FrontierElement::new(Position::Left( Box::new(frt_elt.position ) ),
                                 frt_elt.target_action,
                                 frt_elt.max_loop_depth ) ).collect()
}

fn push_frontier_right(frontier : &mut Vec<FrontierElement>) -> Vec<FrontierElement> {
    frontier.drain(..)
        .map(|frt_elt|
            FrontierElement::new(Position::Right( Box::new(frt_elt.position ) ),
                                 frt_elt.target_action,
                                 frt_elt.max_loop_depth) ).collect()
}