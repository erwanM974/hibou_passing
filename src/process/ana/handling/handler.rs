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
use graph_process_manager_core::delegate::node::GenericNode;
use graph_process_manager_core::handler::handler::AbstractProcessHandler;
use graph_process_manager_core::queued_steps::step::GenericStep;
use crate::core::execution::semantics::affected::get_affected_on_execute;
use crate::core::execution::semantics::execute::execute_interaction;
use crate::core::execution::trace::multitrace::Trace;
use crate::process::ana::conf::{AnalysisConfig, AnalysisStaticLocalVerdictAnalysisProof};
use crate::process::ana::context::AnalysisContext;
use crate::process::ana::filter::filter::AnalysisFilterCriterion;
use crate::process::ana::handling::local_analysis::{get_local_analysis_starting_data, is_dead_local_analysis};
use crate::process::ana::handling::matches::MultiTraceAnalysisMatcher;
use crate::core::execution::trace::flags::WasMultiTraceConsumedWithSimulation;
use crate::process::ana::node::node::AnalysisNodeKind;
use crate::process::ana::param::param::AnalysisParameterization;
use crate::process::ana::step::AnalysisStepKind;
use crate::process::ana::verdict::inconc::InconcReason;
use crate::process::ana::verdict::local::AnalysisLocalVerdict;


pub struct AnalysisProcessHandler {}

impl AbstractProcessHandler<AnalysisConfig> for AnalysisProcessHandler {

    fn process_new_step(context: &AnalysisContext,
                        param : &AnalysisParameterization,
                        parent_state: &GenericNode<AnalysisNodeKind>,
                        step_to_process: &GenericStep<AnalysisStepKind>,
                        _new_state_id: u32,
                        _node_counter: u32) -> AnalysisNodeKind {
        match step_to_process.kind {
            AnalysisStepKind::Execute( ref frt_elt, ref simu) => {
                let new_int = execute_interaction(&parent_state.kind.interaction,
                                                     &frt_elt.position,
                                                     &frt_elt.target_action,
                                                     &context.gen_ctx);
                let mut new_flags = parent_state.kind.flags.update_on_execution(&param.use_simulation,
                                                                            context.co_localizations.get_lf_coloc_id(frt_elt.target_action.lf_id).unwrap(),
                                                                            simu,
                                                                            frt_elt.max_loop_depth,
                                                                            context.init_multitrace_length,
                                                                            &new_int);
                if param.use_locana {
                    let affected = get_affected_on_execute(&parent_state.kind.interaction,
                                                           &frt_elt.position,
                                                           frt_elt.target_action.lf_id);
                    let aff_colocs = context.co_localizations.get_coloc_ids_from_lf_ids(&affected);
                    for (idx,flag) in new_flags.canals.iter_mut().enumerate() {
                        if aff_colocs.contains(&idx) {
                            flag.dirty4local = true
                        }
                    }
                }
                // ***
                let new_ana_loop_depth = parent_state.kind.ana_loop_depth + frt_elt.max_loop_depth;
                AnalysisNodeKind::new(new_int,new_flags,new_ana_loop_depth)
            }
        }
    }

    fn get_criterion(_context: &AnalysisContext,
                     _param : &AnalysisParameterization,
                     parent_state: &GenericNode<AnalysisNodeKind>,
                     step_to_process: &GenericStep<AnalysisStepKind>,
                     _new_state_id: u32,
                     _node_counter: u32) -> AnalysisFilterCriterion {
        match step_to_process.kind {
            AnalysisStepKind::Execute( ref frt_elt, _ ) => {
                let loop_depth = parent_state.kind.ana_loop_depth + frt_elt.max_loop_depth;
                AnalysisFilterCriterion{loop_depth}
            }
        }
    }

    fn collect_next_steps(context: &AnalysisContext,
                          param : &AnalysisParameterization,
                          parent_node_kind: &AnalysisNodeKind)
                -> Vec<AnalysisStepKind> {

        if !parent_node_kind.flags.is_multi_trace_empty(&context.multi_trace) {
            match &param.use_simulation {
                None => {
                    MultiTraceAnalysisMatcher::get_matches(context,
                                                           &parent_node_kind.interaction,
                                                           &parent_node_kind.flags)
                },
                Some(sim_config) => {
                    let mut steps = MultiTraceAnalysisMatcher::get_matches(context,
                                                                           &parent_node_kind.interaction,
                                                                           &parent_node_kind.flags);
                    steps.extend(MultiTraceAnalysisMatcher::get_simulation_steps(sim_config,
                                                                                 context,
                                                             &parent_node_kind.interaction,
                                                             &parent_node_kind.flags));
                    steps
                }
            }
        } else {
            vec![]
        }
    }

    fn get_local_verdict_when_no_child(context: &AnalysisContext,
                                       _param : &AnalysisParameterization,
                                       node_kind: &AnalysisNodeKind) -> AnalysisLocalVerdict {
        if node_kind.flags.is_multi_trace_empty(&context.multi_trace) {
            match node_kind.flags.is_simulated() {
                WasMultiTraceConsumedWithSimulation::No => {
                    if node_kind.interaction.express_empty() {
                        AnalysisLocalVerdict::Cov
                    } else {
                        AnalysisLocalVerdict::GloPref
                    }
                },
                WasMultiTraceConsumedWithSimulation::OnlyAfterEnd => {
                    AnalysisLocalVerdict::MultiPref
                },
                WasMultiTraceConsumedWithSimulation::AsSlice => {
                    AnalysisLocalVerdict::Slice
                }
            }
        } else { /* multi-trace not emptied */
            AnalysisLocalVerdict::Out(false)
        }
    }

    fn get_local_verdict_from_static_analysis(context: &AnalysisContext,
                                              param : &AnalysisParameterization,
                                              node_kind: &mut AnalysisNodeKind)
            -> Option<(AnalysisLocalVerdict, AnalysisStaticLocalVerdictAnalysisProof)> {

        match is_dead_local_analysis(&context.gen_ctx,
                                     &context.co_localizations,
                                     &param.use_simulation,
                                     param.use_locana,
                                     &node_kind.interaction,
                                     &context.multi_trace,
                                     &mut node_kind.flags) {
            None => {
                None
            },
            Some( fail_on_canal_id ) => {
                let (local_coloc,local_interaction,local_multi_trace,local_flags) =
                    get_local_analysis_starting_data(&context.gen_ctx,
                                                     fail_on_canal_id,
                                                     &context.co_localizations,
                                                     &node_kind.interaction,
                                                     &context.multi_trace,
                                                     &node_kind.flags);
                let data = AnalysisStaticLocalVerdictAnalysisProof::new(local_coloc,
                                                                        local_interaction,
                                                                        local_multi_trace,
                                                                        local_flags);
                Some( (AnalysisLocalVerdict::Out(true),data) )
            }
        }
    }

    fn pursue_process_after_static_verdict(_context: &AnalysisContext,
                                           _param : &AnalysisParameterization,
                                           loc_verd: &AnalysisLocalVerdict) -> bool {
        match loc_verd {
            AnalysisLocalVerdict::Out(_) => {
                false
            },
            _ => {true}
        }
    }
}

