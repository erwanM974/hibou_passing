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
use maplit::hashset;

use crate::core::execution::trace::from_model::from_model::InteractionInterpretableAsTraceActions;

use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::core::language::syntax::interaction::Interaction;


impl InteractionInterpretableAsTraceActions for Interaction {

    fn get_all_trace_actions(&self) -> HashSet<TraceAction> {
        match &self {
            &Interaction::Empty => {
                hashset!{}
            },
            &Interaction::Action(ref bp) => {
                let mut acts = hashset!{};
                if let Some(origin) = &bp.origin_on_emission {
                    acts.insert(TraceAction::new(*origin,
                                                 TraceActionKind::Emission,
                                                 bp.message_type.clone()));
                }
                for tar_lf_id in &bp.targets {
                    acts.insert(TraceAction::new(*tar_lf_id,
                                                 TraceActionKind::Reception,
                                                 bp.message_type.clone()));
                }
                acts
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                acts1
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                acts1
            },
            &Interaction::Loop(_, i1) => {
                i1.get_all_trace_actions()
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                acts1
            }
        }
    }

    fn get_atomic_actions_number(&self,
                                 get_max_instead_of_sum : bool,
                                 count_in_loops : bool) -> usize {
        match &self {
            &Interaction::Empty => {
                0
            },
            &Interaction::Action(ref bp) => {
                if let Some(_) = bp.origin_on_emission {
                    bp.targets.len() + 1
                } else {
                    bp.targets.len()
                }
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                i1.get_atomic_actions_number(get_max_instead_of_sum,count_in_loops)
                    + i2.get_atomic_actions_number(get_max_instead_of_sum,count_in_loops)
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let i1_num = i1.get_atomic_actions_number(get_max_instead_of_sum, count_in_loops);
                let i2_num = i2.get_atomic_actions_number(get_max_instead_of_sum, count_in_loops);
                if get_max_instead_of_sum {
                    i1_num.max(i2_num)
                } else {
                    i1_num + i2_num
                }
            },
            &Interaction::Loop(_, i1) => {
                if count_in_loops {
                    i1.get_atomic_actions_number(get_max_instead_of_sum, count_in_loops)
                } else {
                    0
                }
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                i1.get_atomic_actions_number(get_max_instead_of_sum,count_in_loops)
                    + i2.get_atomic_actions_number(get_max_instead_of_sum,count_in_loops)
            }
        }
    }

}