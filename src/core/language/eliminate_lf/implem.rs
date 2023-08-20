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
use crate::core::general_context::GeneralContext;
use crate::core::language::eliminate_lf::eliminable::LifelineEliminable;
use crate::core::language::syntax::action::BroadcastPrimitive;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::simplify::InteractionSyntacticSimplifier;



impl LifelineEliminable for BroadcastPrimitive {

    fn eliminate_lifelines(self,
                           lfs_to_eliminate: &HashSet<usize>,
                           gen_ctx : &GeneralContext) -> Interaction {
        let origin = match self.origin_on_emission {
            None => {None},
            Some(lf) => {
                if lfs_to_eliminate.contains(&lf) {
                    None
                } else {
                    Some(lf)
                }
            }
        };
        let mut new_targs = vec![];
        for lf in self.targets {
            if !lfs_to_eliminate.contains(&lf) {
                new_targs.push(lf)
            }
        }
        if origin == None && new_targs.is_empty() {
            Interaction::Empty
        } else {
            Interaction::Action(BroadcastPrimitive::new(origin,self.message_type.clone(),new_targs))
        }
    }

}


impl LifelineEliminable for Interaction {

    fn eliminate_lifelines(self,
                           lfs_to_eliminate: &HashSet<usize>,
                           gen_ctx : &GeneralContext) -> Interaction {
        match self {
            Interaction::Empty => {
                Interaction::Empty
            },
            Interaction::Action( bp ) => {
                bp.eliminate_lifelines(lfs_to_eliminate,gen_ctx)
            },
            Interaction::Sync(sync,i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate,gen_ctx);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate,gen_ctx);
                InteractionSyntacticSimplifier::simplify_sync(sync,new_i1,new_i2,gen_ctx)
            },
            Interaction::CoReg(cr,i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate,gen_ctx);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate,gen_ctx);
                InteractionSyntacticSimplifier::simplify_coreg(cr,new_i1,new_i2)
            },
            Interaction::Alt(i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate,gen_ctx);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate,gen_ctx);
                InteractionSyntacticSimplifier::simplify_alt(new_i1,new_i2)
            },
            Interaction::Loop(cr,i1) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate,gen_ctx);
                InteractionSyntacticSimplifier::simplify_loop(cr,new_i1)
            }
        }
    }

}

