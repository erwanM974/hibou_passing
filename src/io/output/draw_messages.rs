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



use image::Rgb;
use image_colored_text::ttp::TextToPrint;
use crate::core::general_context::GeneralContext;
use crate::core::message::MessageTypeExpression;
use crate::io::output::draw_commons::hibou_color_palette::{HC_Grammar_Symbol, HC_Message, HC_MessageKind};






pub fn text_repr_message(message : &MessageTypeExpression, gen_ctx : &GeneralContext) -> String {
    let ttp = diagram_repr_message(message,gen_ctx);
    TextToPrint::flatten(&ttp)
}

pub fn diagram_repr_message(message : &MessageTypeExpression,
                            gen_ctx : &GeneralContext) -> Vec<TextToPrint> {
    let (repr,_) = diagram_repr_message_inner(message,gen_ctx);
    repr
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
enum MessageTypeExpressionRepresentationFlag {
    Atomic,
    Union,
    Intersection,
    Difference
}

fn diagram_repr_message_inner(message : &MessageTypeExpression,
                            gen_ctx : &GeneralContext) -> (Vec<TextToPrint>,MessageTypeExpressionRepresentationFlag) {
    match message {
        MessageTypeExpression::Singleton(ms_id) => {
            let label = gen_ctx.get_ms_name(*ms_id).unwrap();
            (vec![
                TextToPrint::new("{".to_string(),Rgb(HC_Grammar_Symbol)),
                TextToPrint::new(label,Rgb(HC_Message)),
                TextToPrint::new("}".to_string(),Rgb(HC_Grammar_Symbol))
            ],
             MessageTypeExpressionRepresentationFlag::Atomic)
        },
        MessageTypeExpression::NamedType(mt_id) => {
            let label = gen_ctx.get_mt_name(*mt_id).unwrap();
            (vec![TextToPrint::new(label,Rgb(HC_MessageKind))],MessageTypeExpressionRepresentationFlag::Atomic)
        },
        MessageTypeExpression::Union(mte1,mte2) => {
            let (mut v1,t1) = diagram_repr_message_inner(mte1,gen_ctx);
            let (mut v2,t2) = diagram_repr_message_inner(mte2,gen_ctx);
            let mut ret = vec![];
            // ***
            if t1 == MessageTypeExpressionRepresentationFlag::Atomic
                || t1 == MessageTypeExpressionRepresentationFlag::Union {
                ret.append(&mut v1);
            } else {
                ret.push( TextToPrint::new("(".to_string(),Rgb(HC_Grammar_Symbol)) );
                ret.append(&mut v1);
                ret.push( TextToPrint::new(")".to_string(),Rgb(HC_Grammar_Symbol)) );
            }
            // ***
            ret.push( TextToPrint::new("∪".to_string(),Rgb(HC_Grammar_Symbol)) );
            // ***
            if t2 == MessageTypeExpressionRepresentationFlag::Atomic
                || t2 == MessageTypeExpressionRepresentationFlag::Union {
                ret.append(&mut v2);
            } else {
                ret.push( TextToPrint::new("(".to_string(),Rgb(HC_Grammar_Symbol)) );
                ret.append(&mut v2);
                ret.push( TextToPrint::new(")".to_string(),Rgb(HC_Grammar_Symbol)) );
            }
            // ***
            (ret,MessageTypeExpressionRepresentationFlag::Union)
        },
        MessageTypeExpression::Intersection(mte1,mte2) => {
            let (mut v1,t1) = diagram_repr_message_inner(mte1,gen_ctx);
            let (mut v2,t2) = diagram_repr_message_inner(mte2,gen_ctx);
            let mut ret = vec![];
            // ***
            if t1 == MessageTypeExpressionRepresentationFlag::Atomic
                || t1 == MessageTypeExpressionRepresentationFlag::Intersection {
                ret.append(&mut v1);
            } else {
                ret.push( TextToPrint::new("(".to_string(),Rgb(HC_Grammar_Symbol)) );
                ret.append(&mut v1);
                ret.push( TextToPrint::new(")".to_string(),Rgb(HC_Grammar_Symbol)) );
            }
            // ***
            ret.push( TextToPrint::new("∩".to_string(),Rgb(HC_Grammar_Symbol)) );
            // ***
            if t2 == MessageTypeExpressionRepresentationFlag::Atomic
                || t2 == MessageTypeExpressionRepresentationFlag::Intersection {
                ret.append(&mut v2);
            } else {
                ret.push( TextToPrint::new("(".to_string(),Rgb(HC_Grammar_Symbol)) );
                ret.append(&mut v2);
                ret.push( TextToPrint::new(")".to_string(),Rgb(HC_Grammar_Symbol)) );
            }
            // ***
            (ret,MessageTypeExpressionRepresentationFlag::Intersection)
        },
        MessageTypeExpression::SetMinus(mte1,mte2) => {
            let (mut v1,t1) = diagram_repr_message_inner(mte1,gen_ctx);
            let (mut v2,t2) = diagram_repr_message_inner(mte2,gen_ctx);
            let mut ret = vec![];
            // ***
            if t1 == MessageTypeExpressionRepresentationFlag::Atomic {
                ret.append(&mut v1);
            } else {
                ret.push( TextToPrint::new("(".to_string(),Rgb(HC_Grammar_Symbol)) );
                ret.append(&mut v1);
                ret.push( TextToPrint::new(")".to_string(),Rgb(HC_Grammar_Symbol)) );
            }
            // ***
            ret.push( TextToPrint::new("\\".to_string(),Rgb(HC_Grammar_Symbol)) );
            // ***
            if t2 == MessageTypeExpressionRepresentationFlag::Atomic {
                ret.append(&mut v2);
            } else {
                ret.push( TextToPrint::new("(".to_string(),Rgb(HC_Grammar_Symbol)) );
                ret.append(&mut v2);
                ret.push( TextToPrint::new(")".to_string(),Rgb(HC_Grammar_Symbol)) );
            }
            // ***
            (ret,MessageTypeExpressionRepresentationFlag::Difference)
        }
    }
}