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
use graph_process_manager_core::handler::filter::AbstractFilter;
use crate::process::ana::filter::elim::AnalysisFilterEliminationKind;


pub struct AnalysisFilterCriterion {
    pub loop_depth : u32
}

impl fmt::Display for AnalysisFilterCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"max loop depth : {:}", self.loop_depth)
    }
}

pub enum AnalysisFilter {
    MaxLoopInstanciation(u32),
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl fmt::Display for AnalysisFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisFilter::MaxLoopInstanciation(num) => {
                write!(f,"MaxLoop={}",num)
            },
            AnalysisFilter::MaxProcessDepth(num) => {
                write!(f,"MaxDepth={}",num)
            },
            AnalysisFilter::MaxNodeNumber(num) => {
                write!(f,"MaxNum={}",num)
            }
        }
    }
}

impl AbstractFilter<AnalysisFilterCriterion,AnalysisFilterEliminationKind>  for AnalysisFilter {

    fn apply_filter(&self,
                    depth: u32,
                    node_counter: u32,
                    criterion: &AnalysisFilterCriterion) -> Option<AnalysisFilterEliminationKind> {
        match self {
            AnalysisFilter::MaxProcessDepth( max_depth ) => {
                if depth > *max_depth {
                    return Some( AnalysisFilterEliminationKind::MaxProcessDepth );
                }
            },
            AnalysisFilter::MaxLoopInstanciation( loop_num ) => {
                if criterion.loop_depth > *loop_num {
                    return Some( AnalysisFilterEliminationKind::MaxLoopInstanciation );
                }
            },
            AnalysisFilter::MaxNodeNumber( max_node_number ) => {
                if node_counter >= *max_node_number {
                    return Some( AnalysisFilterEliminationKind::MaxNodeNumber );
                }
            }
        }
        return None;
    }

}

