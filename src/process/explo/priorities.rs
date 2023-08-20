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




use std::fmt;
use graph_process_manager_core::delegate::priorities::AbstractPriorities;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::process::explo::step::ExplorationStepKind;


pub struct ExplorationPriorities {
    pub emission : i32,
    pub reception : i32,
    pub in_loop : i32
}

impl ExplorationPriorities {

    pub fn new(emission : i32,
               reception : i32,
               in_loop : i32) -> ExplorationPriorities {
        return ExplorationPriorities{emission,reception,in_loop};
    }

    pub fn default() -> ExplorationPriorities {
        return ExplorationPriorities::new(0,0,0);
    }
}

impl fmt::Display for ExplorationPriorities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "[emission={:},reception={:},loop={:}]",
               self.emission,
               self.reception,
               self.in_loop)
    }
}

impl AbstractPriorities<ExplorationStepKind> for ExplorationPriorities {
    fn get_priority_of_step(&self, step: &ExplorationStepKind) -> i32 {
        match step {
            ExplorationStepKind::Execute( frt_elt ) => {
                let mut priority : i32 = 0;
                // ***
                match frt_elt.target_action.act_kind {
                    TraceActionKind::Emission => {
                        priority += self.emission;
                    },
                    TraceActionKind::Reception => {
                        priority += self.reception;
                    },
                }
                // ***
                priority += self.in_loop * ( frt_elt.max_loop_depth as i32);
                // ***
                priority
            }
        }
    }
}

