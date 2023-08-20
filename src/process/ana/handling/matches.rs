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
use std::iter::FromIterator;

use crate::core::execution::semantics::frontier::{FrontierElement, global_frontier};
use crate::core::execution::trace::flags::{MultiTraceAnalysisFlags, TraceAnalysisFlags};
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::syntax::interaction::Interaction;
use crate::process::ana::context::AnalysisContext;
use crate::process::ana::param::anakind::{SimulationActionCriterion, SimulationConfiguration, SimulationLoopCriterion};
use crate::process::ana::param::param::AnalysisParameterization;
use crate::process::ana::step::{AnalysisStepKind, SimulationStepKind};




pub struct MultiTraceAnalysisMatcher {}

impl MultiTraceAnalysisMatcher {

    pub fn get_matches(context : &AnalysisContext,
                       interaction : &Interaction,
                       flags : &MultiTraceAnalysisFlags) -> Vec<AnalysisStepKind> {
        // ***
        let mut head_actions : HashSet<&TraceAction> = HashSet::new();
        for (canal_id,canal_flags) in flags.canals.iter().enumerate() {
            let trace = context.multi_trace.get(canal_id).unwrap();
            if trace.len() > canal_flags.consumed {
                let trace_head = trace.get(canal_flags.consumed).unwrap();
                head_actions.insert(trace_head);
            }
        }
        // ***
        let mut next_steps = vec![];
        for frt_elt in global_frontier(&interaction,&context.gen_ctx,&Some(head_actions)) {
            next_steps.push( AnalysisStepKind::Execute(frt_elt,None) );
        }
        // ***
        next_steps
    }

    fn is_ok_to_simulate(sim_config : &SimulationConfiguration,
                             frt_elt : &FrontierElement,
                             flags : &MultiTraceAnalysisFlags) -> bool {
        match sim_config.act_crit {
            SimulationActionCriterion::None => {},
            _ => {
                if flags.rem_act_in_sim <= 0 {return false;}
            }
        }
        match sim_config.loop_crit {
            SimulationLoopCriterion::None => {},
            _ => {
                if frt_elt.max_loop_depth > flags.rem_loop_in_sim {return false;}
            }
        }
        return true;
    }

    pub fn get_simulation_steps(sim_config : &SimulationConfiguration,
                                context : &AnalysisContext,
                                interaction : &Interaction,
                                flags : &MultiTraceAnalysisFlags) -> Vec<AnalysisStepKind> {
        let mut next_steps = vec![];
        for frt_elt in global_frontier(&interaction,&context.gen_ctx,&None) {
            // ***
            if !Self::is_ok_to_simulate(sim_config,&frt_elt,flags) {
                break;
            }
            // ***
            let tract_coloc_id = context.co_localizations.get_lf_coloc_id(frt_elt.target_action.lf_id).unwrap();
            let canal_flag : &TraceAnalysisFlags = flags.canals.get(tract_coloc_id).unwrap();
            let canal_trace = context.multi_trace.get(tract_coloc_id).unwrap();
            //
            if canal_flag.consumed == canal_trace.len() {
                next_steps.push( AnalysisStepKind::Execute(frt_elt,
                                                           Some(SimulationStepKind::AfterEnd)) );
            } else {
                if sim_config.sim_before && canal_flag.consumed == 0 {
                    next_steps.push( AnalysisStepKind::Execute(frt_elt,
                                                               Some(SimulationStepKind::BeforeStart)) );
                }
            }
        }
        // ***
        next_steps
    }
}


