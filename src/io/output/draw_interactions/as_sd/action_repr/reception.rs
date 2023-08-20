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



use std::collections::HashMap;

use image::{Rgb, RgbImage};
use image_colored_text::draw::single_line::{draw_line_of_colored_text, DrawCoord};
use image_colored_text::ttp::TextToPrint;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use crate::core::general_context::GeneralContext;
use crate::core::message::MessageTypeExpression;
use crate::io::output::draw_commons::font::{get_hibou_font, HIBOU_FONT_SCALE};
use crate::io::output::draw_commons::hibou_color_palette::{HC_Message, HCP_Black};
use crate::io::output::draw_commons::sd_drawing_conf::*;
use crate::io::output::draw_interactions::as_sd::action_repr::common::draw_line_for_message_exchange;
use crate::io::output::draw_interactions::as_sd::util::arrow_heads::draw_arrowhead_rightward;
use crate::io::output::draw_interactions::as_sd::util::dimensions_tools::get_y_pos_from_yshift;
use crate::io::output::draw_interactions::as_sd::util::lf_coords::DrawingLifelineCoords;
use crate::io::output::draw_messages::diagram_repr_message;

// **********

pub fn draw_reception( image : &mut RgbImage,
                    gen_ctx: &GeneralContext,
                    message : &MessageTypeExpression,
                       targets : &Vec<usize>,
                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                    yshift : u32) -> [usize;2] {
    // ***
    let mut min_lf_id : usize = gen_ctx.get_lf_num();
    let mut max_lf_id : usize = 0;
    // ***
    let msg_to_print = diagram_repr_message(message,gen_ctx);
    // ***
    let text_y_pos = get_y_pos_from_yshift(yshift) + VERTICAL_SIZE/2.0;
    let arrow_y_pos = get_y_pos_from_yshift(yshift+2);
    // ***
    for rcv_lf_id in targets {
        {
            min_lf_id = min_lf_id.min(*rcv_lf_id);
            max_lf_id = max_lf_id.max(*rcv_lf_id);
        }
        let tar_lf_coords = lf_x_widths.get(rcv_lf_id).unwrap();
        // ***
        let tar_x_right = tar_lf_coords.x_middle;
        let tar_x_left= tar_x_right - (tar_lf_coords.x_span_inner/2.0);
        draw_arrowhead_rightward(image, tar_x_right, arrow_y_pos,Rgb(HCP_Black));
        draw_line_for_message_exchange(image,tar_x_left,tar_x_right,arrow_y_pos);
        let msg_x_middle = (tar_x_left + tar_x_right)/2.0;
        draw_line_of_colored_text(image,
                                  &DrawCoord::CenteredAround(msg_x_middle),
                                  &DrawCoord::CenteredAround(text_y_pos),
                                  &msg_to_print,
                                  &get_hibou_font(),
                                  &HIBOU_FONT_SCALE);
    }
    // ***
    return [min_lf_id,max_lf_id];
}


