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

use std::collections::{BTreeSet, HashMap};
use std::collections::HashSet;
use std::path::Path;

use image::Rgb;
use image_colored_text::draw::multi_line::MultiLineTextAlignment;
use image_colored_text::ttp::TextToPrint;
use itertools::Itertools;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::semantics::position::Position;

use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::trace::TraceAction;
use crate::io::output::draw_commons::hibou_color_palette::*;
use crate::io::output::draw_commons::make_image_of_text::new_image_with_colored_text;
use crate::io::output::draw_traces::implem::trace_action::diagram_repr_trace_action;

// **********



pub fn draw_firing(path : &Path,
                   gen_ctx : &GeneralContext,
                   action_position : &Position,
                   executed_action : &TraceAction,
                          is_simulated : bool) {
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    {
        let mut ttp: Vec<TextToPrint> = Vec::new();
        if is_simulated {
            ttp.push( TextToPrint::new("SIMU ".to_string(),Rgb(HCP_LightGray)) );
        }
        ttp.append( &mut diagram_repr_trace_action(executed_action,gen_ctx) );
        ttp.push( TextToPrint::new(" ".to_string(),Rgb(HCP_Black)) );
        // ***
        ttp.push( TextToPrint::new("@".to_string(),Rgb(HCP_StandardPurple)) );
        ttp.push( TextToPrint::new(action_position.to_string(),Rgb(HCP_Black)) );
        text_lines.push( ttp );
    }
    // ***
    new_image_with_colored_text(path,
                                &MultiLineTextAlignment::Center,
                                &text_lines);
}



