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


use graph_process_manager_core::delegate::node::GenericNode;
use graph_process_manager_core::handler::handler::AbstractProcessHandler;
use graph_process_manager_core::queued_steps::step::GenericStep;

use crate::core::execution::semantics::execute::execute_interaction;
use crate::core::execution::semantics::frontier::{FrontierElement, global_frontier};
use crate::core::execution::trace::trace::TraceAction;
use crate::core::message::MessageTypeExpression;
use crate::process::explo::conf::{ExplorationConfig, ExplorationStaticLocalVerdictAnalysisProof};
use crate::process::explo::context::{ExplorationContext, ExplorationParameterization};
use crate::process::explo::filter::filter::ExplorationFilterCriterion;
use crate::process::explo::node::ExplorationNodeKind;
use crate::process::explo::step::ExplorationStepKind;
use crate::process::explo::verdict::local::ExplorationLocalVerdict;


pub struct ExplorationProcessHandler {}

impl AbstractProcessHandler<ExplorationConfig> for ExplorationProcessHandler {

    fn process_new_step(context: &ExplorationContext,
                        _param : &ExplorationParameterization,
                        parent_state: &GenericNode<ExplorationNodeKind>,
                        step_to_process: &GenericStep<ExplorationStepKind>,
                        _new_state_id: u32,
                        _node_counter: u32) -> ExplorationNodeKind {
        match step_to_process.kind {
            ExplorationStepKind::Execute( ref frt_elt ) => {
                let new_loop_depth = parent_state.kind.loop_depth + frt_elt.max_loop_depth;
                let new_int = execute_interaction(&parent_state.kind.interaction,
                                                     &frt_elt.position,
                                                     &frt_elt.target_action,
                                                     &context.gen_ctx);
                ExplorationNodeKind::new(new_int,new_loop_depth)
            }
        }
    }

    fn get_criterion(_context: &ExplorationContext,
                     _param : &ExplorationParameterization,
                     parent_state: &GenericNode<ExplorationNodeKind>,
                     step_to_process: &GenericStep<ExplorationStepKind>,
                     _new_state_id: u32,
                     _node_counter: u32) -> ExplorationFilterCriterion {
        match step_to_process.kind {
            ExplorationStepKind::Execute( ref frt_elt ) => {
                let loop_depth = parent_state.kind.loop_depth + frt_elt.max_loop_depth;
                ExplorationFilterCriterion{loop_depth}
            }
        }
    }

    fn collect_next_steps(context: &ExplorationContext,
                          param : &ExplorationParameterization,
                          parent_node_kind: &ExplorationNodeKind)
                -> Vec<ExplorationStepKind> {
        
        let mut glob_front = global_frontier(&parent_node_kind.interaction,&context.gen_ctx,&None);
        // reverse so that when one pops from right to left the actions appear from the top to the bottom
        glob_front.reverse();
        // ***
        let new_front = if param.concretize {
            let mut got = vec![];
            for frt_elt in glob_front {
                let resolved = frt_elt.target_action.message.resolve(&context.gen_ctx);
                assert!(!resolved.is_empty());
                if resolved.len() == 1 {
                    got.push(frt_elt)
                } else {
                    for ms_id in resolved {
                        let new_frt_elt = FrontierElement::new(frt_elt.position.clone(),
                        TraceAction::new(frt_elt.target_action.lf_id,
                                         frt_elt.target_action.act_kind,
                                         MessageTypeExpression::Singleton(ms_id)),
                                                               frt_elt.max_loop_depth);
                        got.push(new_frt_elt)
                    }
                }
            }
            got
        } else {
            glob_front
        };
        new_front.into_iter().map(|x| ExplorationStepKind::Execute(x)).collect()
    }

    fn get_local_verdict_when_no_child(_context: &ExplorationContext,
                                       _param : &ExplorationParameterization,
                                       node_kind: &ExplorationNodeKind) -> ExplorationLocalVerdict {
        if node_kind.interaction.express_empty() {
            ExplorationLocalVerdict::Accepting
        } else {
            ExplorationLocalVerdict::DeadLocked
        }
    }

    fn get_local_verdict_from_static_analysis(_context: &ExplorationContext,
                                              _param : &ExplorationParameterization,
                                              node_kind: &mut ExplorationNodeKind)
                -> Option<(ExplorationLocalVerdict,ExplorationStaticLocalVerdictAnalysisProof)> {
        if node_kind.interaction.express_empty() {
            Some((ExplorationLocalVerdict::Accepting,ExplorationStaticLocalVerdictAnalysisProof{}))
        } else {
            None
        }
    }

    fn pursue_process_after_static_verdict(_context: &ExplorationContext,
                                           _param : &ExplorationParameterization,
                                           loc_verd: &ExplorationLocalVerdict) -> bool {
        match loc_verd {
            ExplorationLocalVerdict::Accepting => {
                true
            },
            ExplorationLocalVerdict::DeadLocked => {
                false
            }
        }
    }
}