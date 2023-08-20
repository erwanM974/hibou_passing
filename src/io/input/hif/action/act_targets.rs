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


use pest::iterators::Pair;

use crate::core::general_context::GeneralContext;
use crate::io::input::error::HibouParsingError;



#[allow(unused_imports)]
use pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hif::parser::{HifParser,Rule};


pub fn parse_comm_act_targets_as_lifelines(gen_ctx : &GeneralContext, target_pair : Pair<Rule>) -> Result<Vec<usize>,HibouParsingError> {
    let inner_pair = target_pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::HIBOU_LABEL => {
            let lf_name : String = inner_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_lf_id( &lf_name ) {
                None => {
                    return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
                },
                Some( lf_id ) => {
                    return Ok( vec![lf_id] );
                }
            }
        },
        Rule::HIBOU_LABEL_LIST_paren => {
            let mut target_lf_ids : Vec<usize> = vec![];
            for label_pair in inner_pair.into_inner() {
                let lf_name : String = label_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                match gen_ctx.get_lf_id( &lf_name ) {
                    None => {
                        return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
                    },
                    Some( lf_id ) => {
                        if target_lf_ids.contains(&lf_id) {
                            return Err( HibouParsingError::EmissionDefinitionError( format!("duplicate target lifeline {:}",lf_name) ) );
                        } else {
                            target_lf_ids.push( lf_id );
                        }
                    }
                }
            }
            return Ok( target_lf_ids );
        },
        Rule::ENVIRONMENT_TARGET => {
            return Ok( vec![] );
        },
        _ => {
            panic!("what rule then ? : {:?}", inner_pair.as_rule() );
        }
    }
}




