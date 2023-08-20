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
use crate::core::execution::trace::from_model::from_model::InteractionInterpretableAsTraceActions;

use crate::core::language::syntax::interaction::Interaction;

#[derive(Clone, PartialEq, Debug)]
pub enum SimulationLoopCriterion {
    MaxNum,
    MaxDepth,
    SpecificNum(u32),
    None
}

impl fmt::Display for SimulationLoopCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationLoopCriterion::MaxNum => {
                write!(f,"total number of loops in interaction")
            },
            SimulationLoopCriterion::MaxDepth => {
                write!(f,"maximum depth of nested loops in interaction")
            },
            SimulationLoopCriterion::SpecificNum(sn) => {
                write!(f,"specific number of loops : {:}", sn)
            },
            SimulationLoopCriterion::None => {
                write!(f,"no limit on loops")
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SimulationActionCriterion {
    MaxNumOutsideLoops,
    SpecificNum(u32),
    None
}

impl fmt::Display for SimulationActionCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationActionCriterion::MaxNumOutsideLoops => {
                write!(f,"number of actions outside loops")
            },
            SimulationActionCriterion::SpecificNum(sn) => {
                write!(f,"specific number of actions : {:}", sn)
            },
            SimulationActionCriterion::None => {
                write!(f,"no limit on actions")
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SimulationConfiguration {
    pub sim_before : bool,
    pub reset_crit_after_exec : bool,
    pub multiply_by_multitrace_length : bool,
    pub loop_crit : SimulationLoopCriterion,
    pub act_crit : SimulationActionCriterion
}

impl fmt::Display for SimulationConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "sim before/slice : {:} | reset after exec : {:} | multiply by mu length : {:} | {:} | {:}",
                       self.sim_before,
                       self.reset_crit_after_exec,
                       self.multiply_by_multitrace_length,
                       self.loop_crit,
                       self.act_crit)
    }
}

impl SimulationConfiguration {

    pub fn new(sim_before : bool,
               reset_crit_after_exec : bool,
               multiply_by_multitrace_length : bool,
               loop_crit : SimulationLoopCriterion,
               act_crit : SimulationActionCriterion) -> SimulationConfiguration {
        return SimulationConfiguration{sim_before,reset_crit_after_exec,multiply_by_multitrace_length,loop_crit,act_crit};
    }

    pub fn get_reset_rem_loop(&self,
                              multi_trace_len : usize,
                              interaction : &Interaction) -> u32 {
        let num : u32;
        match self.loop_crit {
            SimulationLoopCriterion::MaxDepth => {
                num = interaction.max_nested_loop_depth();
            },
            SimulationLoopCriterion::MaxNum => {
                num = interaction.total_loop_num();
            },
            SimulationLoopCriterion::SpecificNum( sn ) => {
                num = sn;
            },
            SimulationLoopCriterion::None => {
                num = 0;
            }
        }
        if self.multiply_by_multitrace_length {
            return num * (multi_trace_len as u32);
        } else {
            return num;
        }
    }
    pub fn get_reset_rem_act(&self,
                             multi_trace_len : usize,
                              interaction : &Interaction) -> u32 {
        let num : u32;
        match self.act_crit {
            SimulationActionCriterion::MaxNumOutsideLoops => {
                num = interaction.get_atomic_actions_number(true,false) as u32;
            },
            SimulationActionCriterion::SpecificNum( sn ) => {
                num = sn;
            },
            SimulationActionCriterion::None => {
                num = 0;
            }
        }
        if self.multiply_by_multitrace_length {
            return num * (multi_trace_len as u32);
        } else {
            return num;
        }
    }
}





