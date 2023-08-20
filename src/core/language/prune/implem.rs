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
use crate::core::language::prune::prunable::LifelinePrunable;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::simplify::InteractionSyntacticSimplifier;


impl LifelinePrunable for Interaction {

    fn avoids_all_of(&self, lf_ids: &HashSet<usize>) -> bool {
        match self {
            &Interaction::Empty => {
                true
            },
            &Interaction::Action(ref bp) => {
                if let Some(origin) = &bp.origin_on_emission {
                    if lf_ids.contains(origin) {
                        return false;
                    }
                }
                for tar_lf_id in &bp.targets {
                    if lf_ids.contains(tar_lf_id) {
                        return false;
                    }
                }
                true
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids)
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids)
            },
            &Interaction::Alt(ref i1, ref i2) => {
                i1.avoids_all_of(lf_ids) || i2.avoids_all_of(lf_ids)
            },
            &Interaction::Loop(_, _) => {
                true
            }
        }
    }

    fn prune(&self, lf_ids : &HashSet<usize>, gen_ctx : &GeneralContext) -> Interaction {
        match self {
            Interaction::Empty => {
                Interaction::Empty
            },
            Interaction::Action(_) => {
                self.clone()
            },
            Interaction::CoReg(cr, i1, i2) => {
                let pruned_i1 = i1.prune(lf_ids, gen_ctx);
                let pruned_i2 = i2.prune(lf_ids, gen_ctx);
                InteractionSyntacticSimplifier::simplify_coreg(cr.clone(),pruned_i1,pruned_i2)
            },
            Interaction::Sync(sync, i1, i2) => {
                let pruned_i1 = i1.prune(lf_ids, gen_ctx);
                let pruned_i2 = i2.prune(lf_ids, gen_ctx);
                InteractionSyntacticSimplifier::simplify_sync(sync.clone(),pruned_i1,pruned_i2,gen_ctx)
            },
            Interaction::Alt(i1, i2) => {
                if i1.avoids_all_of(lf_ids) {
                    if i2.avoids_all_of(lf_ids) {
                        let pruned_i1 = i1.prune(lf_ids, gen_ctx);
                        let pruned_i2 = i2.prune(lf_ids, gen_ctx);
                        InteractionSyntacticSimplifier::simplify_alt(pruned_i1,pruned_i2)
                    } else {
                        i1.prune(lf_ids, gen_ctx)
                    }
                } else {
                    i2.prune(lf_ids, gen_ctx)
                }
            },
            Interaction::Loop(lkind, i1) => {
                if i1.avoids_all_of(lf_ids) {
                    let pruned_i1 = i1.prune(lf_ids, gen_ctx);
                    if pruned_i1 != Interaction::Empty {
                        return Interaction::Loop(lkind.clone(), Box::new(pruned_i1) );
                    }
                }
                Interaction::Empty
            }
        }
    }

}



