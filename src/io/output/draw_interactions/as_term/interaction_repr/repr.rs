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

use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyleItem, GvArrowHeadStyle};
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::{DotBuildable};

use crate::core::execution::semantics::position::Position;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::output::draw_interactions::as_term::action_repr::model_action::broadcast_prim_as_gv_label;
use crate::io::output::draw_interactions::as_term::action_repr::trace_action::trace_actions_as_gv_label;
use crate::io::textual_convention::{SYNTAX_ALT, SYNTAX_COREG, SYNTAX_LOOP_P, SYNTAX_LOOP_W, SYNTAX_SYNC, SYNTAX_LOOP_C};


pub fn interaction_gv_repr(gen_ctx : &GeneralContext,
                        interaction : &Interaction) -> GraphVizDiGraph {
    let mut digraph = GraphVizDiGraph::new(vec![]);
    interaction_gv_repr_rec(gen_ctx, interaction,Position::Epsilon(None), &mut digraph);
    return digraph;
}


fn interaction_gv_repr_rec(gen_ctx : &GeneralContext,
                        interaction : &Interaction,
                        current_pos : Position,
                           gv_graph : &mut GraphVizDiGraph) -> String {
    let node_name = format!("p{}",current_pos.to_string());
    match interaction {
        &Interaction::Empty => {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
            node_gv_options.push( GraphvizNodeStyleItem::Label( "o".to_string() ) );
            gv_graph.add_node( GraphVizNode::new(node_name.clone(), node_gv_options) );
        },
        &Interaction::Action(ref bp) => {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
            node_gv_options.push( GraphvizNodeStyleItem::Label( broadcast_prim_as_gv_label(gen_ctx,bp) ) );
            gv_graph.add_node( GraphVizNode::new(node_name.clone(), node_gv_options) );
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            let co_localised_lf_names : Vec<String> = cr.iter().map(|lf_id| gen_ctx.get_lf_name(*lf_id).unwrap()).collect();
            let op_label = format!("{}({})", SYNTAX_COREG, co_localised_lf_names.join(","));
            repr_binary_operator(gen_ctx, i1, i2, &op_label, current_pos, gv_graph);
        },
        &Interaction::Sync(ref sync, ref i1, ref i2) => {
            let mut sync_acts = vec![];
            for ((lf_id,act_kind),ms_ty) in sync {
                sync_acts.push(TraceAction::new(*lf_id,*act_kind,ms_ty.clone()));
            }
            let acts_as_str = trace_actions_as_gv_label(gen_ctx,sync_acts.iter());
            let op_label = format!("{}{}", SYNTAX_SYNC,acts_as_str);
            repr_binary_operator(gen_ctx, i1, i2, &op_label, current_pos, gv_graph);
        },
        &Interaction::Alt(ref i1, ref i2) => {
            repr_binary_operator(gen_ctx, i1, i2, SYNTAX_ALT, current_pos, gv_graph);
        },
        &Interaction::Loop(ref cr, ref i1) => {
            // the parent loop node
            {
                let mut node_gv_options : GraphvizNodeStyle = Vec::new();
                node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
                let label = match cr.len() {
                    0 => {
                        SYNTAX_LOOP_W.to_string()
                    },
                    x if x == gen_ctx.get_lf_num() => {
                        SYNTAX_LOOP_P.to_string()
                    },
                    _ => {
                        let co_localised_lf_names : Vec<String> = cr.iter().map(|lf_id| gen_ctx.get_lf_name(*lf_id).unwrap()).collect();
                        format!("{}({})", SYNTAX_LOOP_C, co_localised_lf_names.join(","))
                    }
                };
                node_gv_options.push( GraphvizNodeStyleItem::Label( label ) );
                gv_graph.add_node( GraphVizNode::new(node_name.clone(), node_gv_options) );
            }
            // then the left sub-interaction
            {
                let left_position = Position::Left(Box::new(current_pos.clone()));
                let child_node_name = interaction_gv_repr_rec(gen_ctx,i1,left_position, gv_graph);
                let gv_edge = GraphVizEdge::new(node_name.clone(),
                                                None,
                                                child_node_name,
                                                None,
                                                vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]);
                gv_graph.add_edge( gv_edge );
            }
        }
    }
    return node_name;
}

fn repr_binary_operator(gen_ctx : &GeneralContext,
                        i1 : &Interaction,
                        i2 : &Interaction,
                        operator_label : &str,
                        current_pos : Position,
                        gv_graph : &mut GraphVizDiGraph) {
    let node_name = format!("p{}",current_pos.to_string());
    // the parent node
    {
        let mut parent_node_gv_options : GraphvizNodeStyle = Vec::new();
        parent_node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
        parent_node_gv_options.push( GraphvizNodeStyleItem::Label( operator_label.to_string() ) );
        gv_graph.add_node( GraphVizNode::new(node_name.clone(), parent_node_gv_options) );
    }
    // then the left sub-interaction
    {
        let left_position = Position::Left(Box::new(current_pos.clone()));
        let child_node_name = interaction_gv_repr_rec(gen_ctx,i1,left_position.clone(), gv_graph);
        let gv_edge = GraphVizEdge::new(node_name.clone(),
                                        None,
                                        child_node_name,
                                        None,
                                        vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]);
        gv_graph.add_edge(gv_edge);
    }
    // then the right sub-interaction
    {
        let right_position = Position::Right(Box::new(current_pos.clone()));
        let child_node_name = interaction_gv_repr_rec(gen_ctx,i2,right_position.clone(), gv_graph);
        let gv_edge = GraphVizEdge::new(node_name,
                                        None,
                                        child_node_name,
                                        None,
                                        vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]);
        gv_graph.add_edge(gv_edge);
    }
}


