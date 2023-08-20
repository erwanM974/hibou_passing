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



use std::collections::BTreeMap;
use pest::iterators::{Pair, Pairs};
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};

use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::input::error::HibouParsingError;
use crate::io::input::hif::action::action::parse_communication_action;


#[allow(unused_imports)]
use pest::Parser;
use crate::core::message::MessageTypeExpression;
#[allow(unused_imports)]
use crate::io::input::hif::parser::{HifParser,Rule};
use crate::io::input::hif::trace::sync_acts_from_pair;


pub fn parse_hif_string(gen_ctx : &GeneralContext, hif_string : String) -> Result<Interaction,HibouParsingError> {
    match HifParser::parse(Rule::HIF_PEST_FILE, &hif_string) {
        Ok( ref mut got_pair ) => {
            let int_pair = got_pair.next().unwrap();
            match int_pair.as_rule() {
                Rule::SD_INTERACTION => {
                    return parse_interaction(gen_ctx,int_pair);
                },
                _ => {
                    panic!("what rule then ? : {:?}", int_pair.as_rule() );
                }
            }
        },
        Err(e) => {
            return Err( HibouParsingError::MatchError(e.to_string()) );
        }
    }
}

fn parse_coreg(gen_ctx : &GeneralContext, cr_pair : Pair<Rule>) -> Result<Vec<usize>,HibouParsingError> {
    let mut cr = vec![];
    for tar_lf_pair in cr_pair.into_inner() {
        let target_lf_name : String = tar_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        // ***
        match gen_ctx.get_lf_id( &target_lf_name ) {
            None => {
                return Err( HibouParsingError::MissingLifelineDeclarationError( target_lf_name ) );
            },
            Some( tar_lf_id ) => {
                if cr.contains(&tar_lf_id) {
                    return Err( HibouParsingError::OtherDefinitionError( format!("duplicate lifeline in co-region : {:}", target_lf_name) ) );
                } else {
                    cr.push(tar_lf_id);
                }
            }
        }
    }
    Ok(cr)
}

fn parse_interaction(gen_ctx : &GeneralContext, interaction_pair : Pair<Rule>) -> Result<Interaction,HibouParsingError> {
    let content_pair = interaction_pair.into_inner().next().unwrap();
    match content_pair.as_rule() {
        Rule::SD_EMPTY_INTERACTION => {
            return Ok( Interaction::Empty );
        },
        Rule::SD_COMMUNICATION_ACTION => {
            return parse_communication_action(gen_ctx,&mut content_pair.into_inner());
        },
        Rule::SD_SEQ_INT => {
            match get_nary_sub_interactions_from_pair(gen_ctx, content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::CoReg(vec![]),&mut sub_ints) );
                }
            }
        },
        Rule::SD_SYNC_INT => {
            let mut content = content_pair.into_inner();
            content.next(); // get rid of the operator name
            let sync_acts_pair = content.next().unwrap();
            match sync_acts_from_pair(gen_ctx,
                                          sync_acts_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok(sync_acts) => {
                    match get_nary_sub_interactions(gen_ctx, content) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( mut sub_ints ) => {
                            let mut sync = btreemap!{};
                            for act in sync_acts {
                                sync.insert((act.lf_id,act.act_kind), act.message);
                            }
                            return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::Sync(sync),&mut sub_ints) );
                        }
                    }
                }
            }
        }
        Rule::SD_COREG_INT => {
            let mut content = content_pair.into_inner();
            content.next(); // get rid of the operator name
            let cr_pair = content.next().unwrap();
            match parse_coreg(gen_ctx,cr_pair) {
                Err(e) => {return Err(e);},
                Ok(coreg) => {
                    match get_nary_sub_interactions(gen_ctx, content) {
                        Err(e) => { return Err(e); },
                        Ok( mut sub_ints ) => {
                            return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::CoReg(coreg),&mut sub_ints) );
                        }
                    }
                }
            }
        },
        Rule::SD_ALT_INT => {
            match get_nary_sub_interactions_from_pair(gen_ctx, content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::Alt,&mut sub_ints) );
                }
            }
        },
        Rule::SD_PAR_INT => {
            match get_nary_sub_interactions_from_pair(gen_ctx, content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    let coreg : Vec<usize> = (0..gen_ctx.get_lf_num()).collect();
                    return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::CoReg(coreg),&mut sub_ints) );
                }
            }
        },
        Rule::SD_LOOP_INT => {
            let mut loop_content = content_pair.into_inner();
            let loop_kind_pair = loop_content.next().unwrap().into_inner().next().unwrap();
            match parse_interaction(gen_ctx,loop_content.next().unwrap()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( sub_int ) => {
                    match loop_kind_pair.as_rule() {
                        Rule::SD_LOOP_KIND_W => {
                            return Ok( Interaction::Loop(vec![],Box::new(sub_int)) );
                        },
                        Rule::SD_LOOP_KIND_P => {
                            let coreg : Vec<usize> = (0..gen_ctx.get_lf_num()).collect();
                            return Ok( Interaction::Loop(coreg,Box::new(sub_int)) );
                        },
                        Rule::SD_LOOP_KIND_C => {
                            let cr_pair = loop_kind_pair.into_inner().next().unwrap();
                            match parse_coreg(gen_ctx,cr_pair) {
                                Err(e) => {return Err(e);},
                                Ok(coreg) => {
                                    return Ok( Interaction::Loop(coreg,Box::new(sub_int)) );
                                }
                            }
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", content_pair.as_rule());
        }
    }
}

fn get_nary_sub_interactions_from_pair(gen_ctx : &GeneralContext, sd_content_pair : Pair<Rule>) -> Result<Vec<Interaction>,HibouParsingError> {
    let mut content = sd_content_pair.into_inner();
    content.next(); // get rid of the operator name
    return get_nary_sub_interactions(gen_ctx, content);
}

fn get_nary_sub_interactions(gen_ctx : &GeneralContext, content : Pairs<Rule>) -> Result<Vec<Interaction>,HibouParsingError> {
    let mut sub_ints : Vec<Interaction> = Vec::new();
    for sub_interaction in content {
        match parse_interaction(gen_ctx,sub_interaction) {
            Err(e) => {
                return Err(e);
            },
            Ok( parsed_sub_int ) => {
                sub_ints.push( parsed_sub_int );
            }
        }
    }
    return Ok( sub_ints );
}

enum BinaryOperatorKind {
    CoReg(Vec<usize>),
    Alt,
    Sync(BTreeMap<(usize,TraceActionKind),MessageTypeExpression>)
}

fn fold_interactions_in_binary_operator(op_kind : &BinaryOperatorKind, sub_ints : &mut Vec<Interaction>) -> Interaction {
    assert!(sub_ints.len() > 0);
    if sub_ints.len() == 1 {
        return sub_ints.remove(0);
    } else {
        let first_int = sub_ints.remove(0);
        match op_kind {
            BinaryOperatorKind::CoReg(ref cr) => {
                return Interaction::CoReg( cr.clone(),Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Sync(ref sync_acts) => {
                return Interaction::Sync( sync_acts.clone(),Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Alt => {
                return Interaction::Alt( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            }
        }
    }
}





