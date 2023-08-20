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



use std::fmt::Debug;
use crate::core::message::MessageTypeExpression;


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct BroadcastPrimitive {
    pub origin_on_emission : Option<usize>,
    pub message_type : MessageTypeExpression,
    pub targets : Vec<usize>
}



impl BroadcastPrimitive {

    pub fn is_structurally_empty(&self) -> bool {
        if let Some(_) = self.origin_on_emission {
            false
        } else {
            self.targets.is_empty()
        }
    }

    pub fn new(origin_on_emission: Option<usize>, message_type: MessageTypeExpression, targets: Vec<usize>) -> Self {
        Self { origin_on_emission, message_type, targets }
    }
}