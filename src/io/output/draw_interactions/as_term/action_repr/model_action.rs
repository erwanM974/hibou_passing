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


use itertools::Itertools;

use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::action::BroadcastPrimitive;
use crate::io::output::draw_messages::text_repr_message;
use crate::io::textual_convention::{SYNTAX_EMISSION, SYNTAX_RECEPTION};


pub fn broadcast_prim_as_gv_label(gen_ctx : &GeneralContext,
                                  bp : &BroadcastPrimitive) -> String {
    let msg_label = text_repr_message(&bp.message_type,gen_ctx);
    let targs_label = match bp.targets.len() {
        0 => {
            "".to_string()
        },
        _ => {
            format!("({})",bp.targets.iter().map(|lf_id| gen_ctx.get_lf_name(*lf_id).unwrap()).join(","))
        }
    };
    match bp.origin_on_emission {
        None => {
            format!("{}{}{}",targs_label,SYNTAX_RECEPTION,msg_label)
        },
        Some(source) => {
            let lf_name = gen_ctx.get_lf_name(source).unwrap();
            format!("{}{}{}{}",lf_name,SYNTAX_EMISSION,targs_label,msg_label)
        }
    }
}
