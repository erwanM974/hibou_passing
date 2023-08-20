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



use std::collections::{HashMap, HashSet};
use crate::core::execution::trace::flags::{MultiTraceAnalysisFlags, TraceAnalysisFlags};
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::process::ana::param::anakind::SimulationConfiguration;
use crate::process::ana::step::SimulationStepKind;

impl MultiTraceAnalysisFlags {

    pub fn update_on_execution(&self,
                               sim_config : &Option<SimulationConfiguration>,
                               executed_action_coloc_id : usize,
                                is_simu : &Option<SimulationStepKind>,
                                loop_depth : u32, // loop depth of action that is executed
                                init_multitrace_length : usize,
                                new_interaction : &Interaction) -> MultiTraceAnalysisFlags {
        // ***
        let mut new_canal_flags : Vec<TraceAnalysisFlags> = Vec::new();
        // ***
        for (flag_id,old_flag) in self.canals.iter().enumerate() {
            if flag_id == executed_action_coloc_id {
                let mut new_flag : TraceAnalysisFlags = old_flag.clone();
                // ***
                match is_simu {
                    None => {
                        new_flag.consumed += 1;
                    },
                    Some(kind) => {
                        match kind {
                            SimulationStepKind::BeforeStart => {
                                new_flag.simulated_before += 1;
                            },
                            SimulationStepKind::AfterEnd => {
                                new_flag.simulated_after += 1;
                            }
                        }
                    }
                }
                new_canal_flags.push( new_flag );
            } else {
                new_canal_flags.push( old_flag.clone() );
            }
        }
        let (rem_loop_in_sim,rem_act_in_sim) : (u32,u32);
        match sim_config {
            None => {
                rem_loop_in_sim = 0;
                rem_act_in_sim = 0;
            },
            Some( got_sim_config ) => {
                match is_simu {
                    None => {
                        if got_sim_config.reset_crit_after_exec {
                            let rem_multitrace_length = init_multitrace_length - (self.get_number_of_consumed_actions() + 1);
                            rem_loop_in_sim = got_sim_config.get_reset_rem_loop(rem_multitrace_length,new_interaction);
                            rem_act_in_sim = got_sim_config.get_reset_rem_act(rem_multitrace_length,new_interaction);
                        } else {
                            rem_loop_in_sim = self.rem_loop_in_sim;
                            rem_act_in_sim = self.rem_act_in_sim;
                        }
                    },
                    Some(_) => {
                        let rem_multitrace_length = init_multitrace_length - self.get_number_of_consumed_actions();
                        let (rem_loop,rem_act) = self.update_criterion_on_simulation(rem_multitrace_length,got_sim_config,new_interaction,loop_depth);
                        rem_loop_in_sim = rem_loop;
                        rem_act_in_sim = rem_act;
                    }
                }
            }
        }
        // ***
        return MultiTraceAnalysisFlags::new(new_canal_flags,rem_loop_in_sim,rem_act_in_sim);
    }

    fn update_criterion_on_simulation(&self,rem_multitrace_length : usize,
                               sim_config : &SimulationConfiguration,
                               new_interaction : &Interaction,
                               loop_depth : u32) -> (u32,u32) {
        // ***
        let rem_loop_in_sim : u32;
        {
            let removed = self.rem_loop_in_sim - loop_depth;
            let reset = sim_config.get_reset_rem_loop(rem_multitrace_length,new_interaction);
            rem_loop_in_sim = reset.min(removed);
        }
        // ***
        let rem_act_in_sim : u32;
        {
            let removed = self.rem_act_in_sim - 1;
            let reset = sim_config.get_reset_rem_act(rem_multitrace_length,new_interaction);
            rem_act_in_sim = reset.min(removed);
        }
        // ***
        return (rem_loop_in_sim,rem_act_in_sim)
    }
}


