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




use crate::core::general_context::GeneralContext;
use crate::core::message::MessageTypeExpression;

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Copy)]
pub enum TraceActionKind {
    Reception,
    Emission
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TraceAction {
    pub lf_id : usize,
    pub act_kind : TraceActionKind,
    pub message : MessageTypeExpression
}

impl TraceAction {

    pub fn new(lf_id : usize,
               act_kind : TraceActionKind,
               message : MessageTypeExpression) -> TraceAction {
        return TraceAction{lf_id,act_kind,message};
    }

    pub fn is_type_included(&self,
                            other : &Self,
                            gen_ctx : &GeneralContext) -> bool {
        if self.lf_id != other.lf_id {
            return false;
        }
        if self.act_kind != other.act_kind {
            return false;
        }
        let intersect = MessageTypeExpression::Intersection(
            Box::new(self.message.clone()),
            Box::new(other.message.clone()));
        !intersect.resolve(gen_ctx).is_empty()
    }

}


