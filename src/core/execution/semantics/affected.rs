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
use crate::core::execution::semantics::position::Position;
use crate::core::language::involve::involves::InvolvesLifelines;
use crate::core::language::prune::prunable::LifelinePrunable;

use crate::core::language::syntax::interaction::Interaction;


fn get_affected_on_prune(interaction : &Interaction, lf_id : usize) -> HashSet<usize> {
    match interaction {
        Interaction::Empty => {
            hashset!{}
        },
        Interaction::Action(_) => {
            hashset!{}
        },
        Interaction::CoReg(cr, i1, i2) => {
            get_affected_on_prune(i1,lf_id).union(&get_affected_on_prune(i2,lf_id)).cloned().collect()
        },
        Interaction::Sync(sync, i1, i2) => {
            get_affected_on_prune(i1,lf_id).union(&get_affected_on_prune(i2,lf_id)).cloned().collect()
        },
        Interaction::Alt(i1, i2) => {
            if i1.avoids(lf_id) && i2.avoids(lf_id) {
                get_affected_on_prune(i1,lf_id).union(&get_affected_on_prune(i2,lf_id)).cloned().collect()
            } else {
                i1.involved_lifelines().union(&i2.involved_lifelines()).cloned().collect()
            }
        },
        Interaction::Loop(_, i1) => {
            if i1.avoids(lf_id) {
                get_affected_on_prune(i1,lf_id)
            } else {
                i1.involved_lifelines()
            }
        }
    }
}



pub fn get_affected_on_execute(my_int : &Interaction,
                               my_pos : &Position,
                               targ_act_lf_id : usize) -> HashSet<usize> {
    match my_pos {
        Position::Epsilon(_) => {
            hashset!{}
        },
        Position::Left(ref p1) => {
            match my_int {
                Interaction::Loop(_,i1) => {
                    i1.involved_lifelines()
                },
                Interaction::CoReg(_,i1,_) => {
                    get_affected_on_execute(i1,p1,targ_act_lf_id)
                },
                Interaction::Sync(_,i1,_) => {
                    get_affected_on_execute(i1,p1,targ_act_lf_id)
                },
                Interaction::Alt(i1,i2) => {
                    i1.involved_lifelines().union(&i2.involved_lifelines()).cloned().collect()
                },
                _ => {panic!()}
            }
        },
        Position::Right(ref p2) => {
            match my_int {
                Interaction::CoReg(cr,i1,i2) => {
                    let on_prune = if cr.contains(&targ_act_lf_id) {
                        get_affected_on_prune(i1,targ_act_lf_id)
                    } else {
                        hashset!{}
                    };
                    on_prune.union(&get_affected_on_execute(i2,p2,targ_act_lf_id)).cloned().collect()
                },
                Interaction::Sync(_,_,i2) => {
                    get_affected_on_execute(i2,p2,targ_act_lf_id)
                },
                Interaction::Alt(i1,i2) => {
                    i1.involved_lifelines().union(&i2.involved_lifelines()).cloned().collect()
                },
                _ => {panic!()}
            }
        },
        Position::Both(ref p1,ref p2) => {
            match my_int {
                Interaction::Sync(_,i1,i2) => {
                    get_affected_on_execute(i1,p1,targ_act_lf_id)
                        .union(&get_affected_on_execute(i2,p2,targ_act_lf_id))
                        .cloned().collect()
                },
                Interaction::Alt(i1,i2) => {
                    get_affected_on_execute(i1,p1,targ_act_lf_id)
                        .union(&get_affected_on_execute(i2,p2,targ_act_lf_id))
                        .cloned().collect()
                },
                _ => {panic!()}
            }
        }
    }
}
