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



use crate::core::execution::trace::multitrace::MultiTrace;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TraceAnalysisFlags {
    pub consumed : usize,
    pub no_longer_observed : bool,
    pub dirty4local : bool,
    pub simulated_before : u32,
    pub simulated_after : u32
}

impl TraceAnalysisFlags {
    pub fn new(
        consumed : usize,
        no_longer_observed : bool,
        dirty4local : bool,
        simulated_before : u32,
        simulated_after : u32) -> TraceAnalysisFlags {
        return TraceAnalysisFlags{consumed,no_longer_observed,dirty4local,simulated_before,simulated_after};
    }

    pub fn new_init() -> TraceAnalysisFlags {
        return TraceAnalysisFlags::new(0,false,true,0,0);
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct MultiTraceAnalysisFlags {
    pub canals : Vec<TraceAnalysisFlags>,
    pub rem_loop_in_sim : u32,
    pub rem_act_in_sim : u32
}

pub enum WasMultiTraceConsumedWithSimulation {
    No,
    OnlyAfterEnd,
    AsSlice
}

impl MultiTraceAnalysisFlags {

    pub fn new_init(canals_num : usize,
                    rem_loop_in_sim : u32,
                    rem_act_in_sim : u32) -> MultiTraceAnalysisFlags {
        let mut canals : Vec<TraceAnalysisFlags> = vec![];
        for i in 0..canals_num {
            canals.push(TraceAnalysisFlags::new_init());
        }
        return MultiTraceAnalysisFlags::new(canals,rem_loop_in_sim,rem_act_in_sim);
    }

    pub fn new(canals:Vec<TraceAnalysisFlags>,
               rem_loop_in_sim : u32,
               rem_act_in_sim : u32) -> MultiTraceAnalysisFlags {
        return MultiTraceAnalysisFlags{canals,rem_loop_in_sim,rem_act_in_sim};
    }
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub(crate) fn get_number_of_consumed_actions(&self) -> usize {
        return self.canals.iter().fold(0,|sum,trace_flag| sum + trace_flag.consumed);
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn is_any_component_empty(&self, multi_trace : &MultiTrace) -> bool {
        for (canal_id,canal_flags) in self.canals.iter().enumerate() {
            let trace= multi_trace.get(canal_id).unwrap();
            if trace.len() == canal_flags.consumed {
                return true;
            }
        }
        return false;
    }

    pub fn is_multi_trace_empty(&self, multi_trace : &MultiTrace) -> bool {
        for (canal_id,canal_flags) in self.canals.iter().enumerate() {
            let trace = multi_trace.get(canal_id).unwrap();
            if trace.len() > canal_flags.consumed {
                return false;
            }
        }
        return true;
    }

    pub fn is_any_component_hidden(&self) -> bool {
        for canal in &self.canals {
            if canal.no_longer_observed {
                return true;
            }
        }
        return false;
    }

    pub fn is_simulated(&self) -> WasMultiTraceConsumedWithSimulation {
        let mut got_sim_after = false;
        for canal in &self.canals {
            if canal.simulated_before > 0 {
                return WasMultiTraceConsumedWithSimulation::AsSlice;
            }
            if canal.simulated_after > 0 {
                got_sim_after = true;
            }
        }
        if got_sim_after {
            return WasMultiTraceConsumedWithSimulation::OnlyAfterEnd;
        } else {
            return WasMultiTraceConsumedWithSimulation::No;
        }
    }

}