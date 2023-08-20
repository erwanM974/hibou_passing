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



use std::collections::HashSet;

use crate::core::language::involve::involves::InvolvesLifelines;
use crate::core::language::syntax::action::BroadcastPrimitive;
use crate::core::language::syntax::interaction::Interaction;


impl InvolvesLifelines for BroadcastPrimitive {

    fn involved_lifelines(&self) -> HashSet<usize> {
        let mut involved = match self.origin_on_emission {
            None => {
                hashset!{}
            },
            Some(source) => {
                hashset!{source}
            }
        };
        for lf_id in &self.targets {
            involved.insert( *lf_id );
        }
        involved
    }

    fn involves_any_of(&self, lf_ids : &HashSet<usize>) -> bool {
        !self.involved_lifelines().is_disjoint(lf_ids)
    }
}


impl InvolvesLifelines for Interaction {
    fn involved_lifelines(&self) -> HashSet<usize> {
        match &self {
            &Interaction::Empty => {
                hashset!{}
            },
            &Interaction::Action(ref bp) => {
                bp.involved_lifelines()
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                content
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                content
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                content
            },
            &Interaction::Loop(_, i1) => {
                i1.involved_lifelines()
            }
        }
    }

    fn involves_any_of(&self, lf_ids : &HashSet<usize>) -> bool {
        match self {
            &Interaction::Empty => {
                false
            },
            &Interaction::Action(ref bp) => {
                bp.involves_any_of(lf_ids)
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids)
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids)
            },
            &Interaction::Alt(ref i1, ref i2) => {
                i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids)
            },
            &Interaction::Loop(_, ref i1) => {
                i1.involves_any_of(lf_ids)
            }
        }
    }
}