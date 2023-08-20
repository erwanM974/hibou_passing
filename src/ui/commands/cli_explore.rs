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


use std::time::Instant;



use clap::ArgMatches;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use crate::io::input::hcf::explo::interface::parse_hcf_file_for_explore;
use crate::io::input::hcf::explo::options::HibouExploreOptions;

use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::hif::interface::parse_hif_file;
use crate::process::explo::conf::ExplorationConfig;
use crate::process::explo::context::{ExplorationContext, ExplorationParameterization};
use crate::process::explo::node::ExplorationNodeKind;
use crate::process::explo::priorities::ExplorationPriorities;
use crate::process::explo::step::ExplorationStepKind;


pub fn cli_explore(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],1);
        },
        Ok( gen_ctx ) => {
            let hif_file_path = matches.value_of("hif").unwrap();
            match parse_hif_file(&gen_ctx,hif_file_path) {
                Err(e) => {
                    return (vec![e.to_string()],1);
                },
                Ok( int) => {
                    let explo_opts : HibouExploreOptions;
                    if matches.is_present("hcf") {
                        let hcf_file_path = matches.value_of("hcf").unwrap();
                        match parse_hcf_file_for_explore(&gen_ctx,hcf_file_path) {
                            Err(e) => {
                                return (vec![e.to_string()],1);
                            },
                            Ok( got_explo_opt) => {
                                explo_opts = got_explo_opt;
                            }
                        }
                    } else {
                        explo_opts = HibouExploreOptions::default();
                    }
                    let mut ret_print = vec![];
                    // ***
                    ret_print.push( "".to_string());
                    ret_print.push( "EXPLORING SEMANTICS".to_string());
                    ret_print.push( format!("of interaction from file '{}'",hsf_file_path) );
                    ret_print.push( "".to_string());
                    // ***
                    let explo_ctx = ExplorationContext::new(gen_ctx);
                    let delegate : GenericProcessDelegate<ExplorationStepKind,ExplorationNodeKind,ExplorationPriorities> = GenericProcessDelegate::new(explo_opts.strategy,explo_opts.priorities);

                    let mut exploration_manager : GenericProcessManager<ExplorationConfig> = GenericProcessManager::new(explo_ctx,
                                                                                                                        explo_opts.param,
                                                                                                                  delegate,
                                                                                                                        explo_opts.filters,
                                                                                                                        explo_opts.loggers,
                                                                                                                  None,
                                                                                                                        explo_opts.use_memoization);

                    // ***
                    let init_node = ExplorationNodeKind::new(int,0);
                    // ***
                    let now = Instant::now();
                    let (node_count,_) = exploration_manager.start_process(init_node);
                    let elapsed_time = now.elapsed();
                    ret_print.push( format!("node count : {:?}", node_count ) );
                    ret_print.push( format!("elapsed    : {:?}", elapsed_time.as_secs_f64() ) );
                    return (ret_print,0);
                }
            }
        }
    }
}