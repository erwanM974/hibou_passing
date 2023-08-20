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

use maplit::hashset;

use crate::core::general_context::GeneralContext;

pub trait LifelinePrunable : Sized {

    fn avoids_all_of(&self, lf_ids : &HashSet<usize>) -> bool;

    fn avoids(&self, lf_id : usize) -> bool {
        self.avoids_all_of(&hashset!{lf_id})
    }

    fn get_avoidance_map(&self, gen_ctx : &GeneralContext) -> Vec<bool> {
        (0..gen_ctx.get_lf_num()).map(|lf_id| self.avoids(lf_id)).collect()
    }

    fn prune(&self, lf_ids : &HashSet<usize>, gen_ctx : &GeneralContext) -> Self;

}

