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
use std::hash::Hash;


use crate::core::execution::trace::trace::TraceActionKind;
use crate::core::language::syntax::action::BroadcastPrimitive;
use crate::core::message::MessageTypeExpression;




#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Interaction {
    Empty,
    Action(BroadcastPrimitive),
    CoReg(Vec<usize>,Box<Interaction>,Box<Interaction>),
    Loop(Vec<usize>,Box<Interaction>),
    Alt(Box<Interaction>,Box<Interaction>),
    Sync(BTreeMap<(usize,TraceActionKind),MessageTypeExpression>,Box<Interaction>,Box<Interaction>)
}


impl Interaction {

    pub fn express_empty(&self) -> bool {
        match self {
            &Interaction::Empty => {
                true
            },
            &Interaction::Action(ref bp) => {
                bp.is_structurally_empty()
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            &Interaction::Loop(_, _) => {
                true
            },
            &Interaction::Alt(ref i1, ref i2) => {
                i1.express_empty() || i2.express_empty()
            },
            &Interaction::Sync(_,ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            }
        }
    }


    pub fn max_nested_loop_depth(&self) -> u32 {
        match self {
            &Interaction::Empty => {
                0
            }, &Interaction::Action(_) => {
                0
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                i1.max_nested_loop_depth().max(i2.max_nested_loop_depth())
            }, &Interaction::Alt(ref i1, ref i2) => {
                i1.max_nested_loop_depth().max(i2.max_nested_loop_depth())
            }, &Interaction::Loop(_, ref i1) => {
                1 + i1.max_nested_loop_depth()
            }, &Interaction::Sync(_, ref i1, ref i2) => {
                i1.max_nested_loop_depth().max(i2.max_nested_loop_depth())
            }
        }
    }

    pub fn total_loop_num(&self) -> u32 {
        match self {
            &Interaction::Empty => {
                0
            }, &Interaction::Action(_) => {
                0
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                i1.total_loop_num() + i2.total_loop_num()
            }, &Interaction::Alt(ref i1, ref i2) => {
                i1.total_loop_num() + i2.total_loop_num()
            }, &Interaction::Loop(_, ref i1) => {
                1 + i1.total_loop_num()
            }, &Interaction::Sync(_, ref i1, ref i2) => {
                i1.total_loop_num() + i2.total_loop_num()
            }
        }
    }

}


