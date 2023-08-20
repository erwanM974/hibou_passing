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


use pest::iterators::{Pair, Pairs};




#[allow(unused_imports)]
use pest::Parser;

#[allow(unused_imports)]
use crate::io::input::hif::parser::{HifParser,Rule};



use crate::core::language::syntax::action::BroadcastPrimitive;
use crate::core::message::MessageTypeExpression;

use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::input::hif::action::act_targets::*;
use crate::io::input::error::HibouParsingError;


fn parse_message_pair(gen_ctx : &GeneralContext, pair : &Pair<Rule>) -> Result<MessageTypeExpression,HibouParsingError> {
    let ms_name : String = pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    match gen_ctx.get_ms_id(&ms_name) {
        None => {
            match gen_ctx.get_mt_id(&ms_name) {
                None => {
                    Err( HibouParsingError::MissingMessageDeclarationError(ms_name) )
                },
                Some(mt_id) => {
                    Ok(MessageTypeExpression::NamedType(mt_id))
                }
            }
        },
        Some(ms_id) => {
            Ok(MessageTypeExpression::Singleton(ms_id))
        }
    }
}

pub fn parse_communication_action(gen_ctx : &GeneralContext, contents : &mut Pairs<Rule>) -> Result<Interaction,HibouParsingError> {

    let message : MessageTypeExpression;
    let first_pair = contents.next().unwrap();
    let origin = match first_pair.as_rule() {
        Rule::SD_COMMUNICATION_ORIGIN => {
            let origin_name_pair = first_pair.into_inner().next().unwrap();
            let origin_name : String = origin_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_lf_id(&origin_name) {
                None => {
                    return Err( HibouParsingError::MissingLifelineDeclarationError(origin_name) );
                },
                Some(lf_id) => {
                    let second_pair = contents.next().unwrap();
                    match parse_message_pair(gen_ctx,&second_pair) {
                        Err(e) => {return Err(e);},
                        Ok(m) => {message = m;}
                    }
                    Some(lf_id)
                }
            }
        },
        Rule::HIBOU_LABEL => {
            match parse_message_pair(gen_ctx,&first_pair) {
                Err(e) => {return Err(e);},
                Ok(m) => {message = m;}
            }
            None
        },
        _ => {
            panic!("what rule then ? : {:?}", first_pair.as_rule() );
        }
    };
    // ***
    let targets_pair = contents.next().unwrap();
    let targets = match parse_comm_act_targets_as_lifelines(gen_ctx,targets_pair) {
        Err(e) => { return Err(e); },
        Ok( tar_lf_ids) => {
            tar_lf_ids
        }
    };
    // ***
    Ok( Interaction::Action(BroadcastPrimitive::new(origin,message,targets)) )
}