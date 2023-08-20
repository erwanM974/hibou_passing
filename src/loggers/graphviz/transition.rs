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





use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;


use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};


use crate::core::execution::semantics::position::Position;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::io::output::draw_transitions::draw_firing::draw_firing;
use crate::io::output::draw_transitions::draw_string_label::draw_string_label;
use crate::loggers::graphviz::drawer::InteractionProcessDrawer;



impl InteractionProcessDrawer {

    pub(crate) fn make_graphic_logger_string_label(&self,
                                            string_label : String,
                                            name : String) -> GraphVizNode {
        let image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        // ***
        draw_string_label(image_file_path.as_path(),string_label);
        // ***
        let mut gv_node_options : GraphvizNodeStyle = Vec::new();
        gv_node_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name,gv_node_options)
    }


    pub(crate) fn make_graphic_logger_firing(&self,
                                      gen_ctx : &GeneralContext,
                                      action_position : &Position,
                                      executed_action : &TraceAction,
                                             is_simulated : bool,
                                      name : String) -> GraphVizNode {
        let image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        // ***
        draw_firing(image_file_path.as_path(),
                    gen_ctx,
                    action_position,executed_action,is_simulated);
        // ***
        let mut gv_node_options : GraphvizNodeStyle = Vec::new();
        gv_node_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name,gv_node_options)
    }
}



