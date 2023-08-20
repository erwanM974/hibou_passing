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
use crate::io::input::hsf::parser::*;


pub fn parse_hsf_string(hsf_string : String) -> Result<GeneralContext,HibouParsingError> {
    match HsfParser::parse(Rule::HSF_PEST_FILE, &hsf_string) {
        Ok( ref mut got_pair ) => {
            let sig_pair = got_pair.next().unwrap();
            match sig_pair.as_rule() {
                Rule::HIBOU_SIGNATURE => {
                    return parse_signature(sig_pair);
                },
                _ => {
                    panic!("what rule then ? : {:?}", sig_pair.as_rule() );
                }
            }
        },
        Err(e) => {
            return Err( HibouParsingError::MatchError(e.to_string()) );
        }
    }
}


fn parse_signature(signature_pair : Pair<Rule>) -> Result<GeneralContext,HibouParsingError> {
    // ***
    let mut got_section_messages  : bool = false;
    let mut got_section_lifelines : bool = false;
    let mut got_section_types : bool = false;
    // ***
    let mut contents = signature_pair.into_inner();
    // ***
    let mut gen_ctx = GeneralContext::new();
    // ***
    while let Some(current_pair) = contents.next() {
        match current_pair.as_rule() {
            Rule::HIBOU_SIG_MS_DECL => {
                if got_section_messages {
                    return Err( HibouParsingError::HsfSetupError("several '@message' sections declared".to_string()));
                }
                got_section_messages = true;
                parse_message_decl(current_pair,&mut gen_ctx);
            },
            Rule::HIBOU_SIG_LF_DECL => {
                if got_section_lifelines {
                    return Err( HibouParsingError::HsfSetupError("several '@lifeline' sections declared".to_string()));
                }
                got_section_lifelines = true;
                parse_lifeline_decl(current_pair,&mut gen_ctx);
            },
            Rule::HIBOU_SIG_MT_DECL => {
                if !got_section_messages {
                    return Err( HibouParsingError::HsfSetupError("'@message' section must be declared before '@type'".to_string()));
                }
                if got_section_types {
                    return Err( HibouParsingError::HsfSetupError("several '@type' sections declared".to_string()));
                }
                got_section_types = true;
                match parse_type_decl(current_pair,&mut gen_ctx) {
                    Ok(_) => {},
                    Err(e) => {return Err(e);}
                }
            },
            _ => {
                panic!("what rule then ? : {:?}", current_pair.as_rule() );
            }
        }
    }
    // ***
    return Ok(gen_ctx);
}


fn parse_message_decl(ms_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext ) {
    for ms_pair in ms_decl_pair.into_inner() {
        let ms_name : String = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        gen_ctx.add_msg(ms_name);
    }
}

fn parse_lifeline_decl(lf_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext ) {
    for lf_pair in lf_decl_pair.into_inner() {
        let lf_name : String = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        gen_ctx.add_lf(lf_name);
    }
}

fn parse_type_decl(ty_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext ) -> Result<(),HibouParsingError>{
    for ty_pair in ty_decl_pair.into_inner() {
        let mut content = ty_pair.into_inner();
        let ty_name_pair = content.next().unwrap();
        let ty_name : String = ty_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        let mss_pair = content.next().unwrap();
        let mut msgs = hashset!{};
        for ms_pair in mss_pair.into_inner() {
            let ms_name : String = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_ms_id(&ms_name) {
                None => {
                    return Err(HibouParsingError::HsfSetupError(format!("unknown message '{}' declared in type '{}'",ms_name,ty_name)));
                },
                Some(ms_id) => {
                    msgs.insert(ms_id);
                }
            }
        }
        gen_ctx.add_mt(ty_name,msgs);
    }
    return Ok(());
}