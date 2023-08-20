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


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum MessageTypeExpression {
    Singleton(usize), // ms_id
    NamedType(usize), // mt_id
    Union(Box<MessageTypeExpression>,Box<MessageTypeExpression>),
    Intersection(Box<MessageTypeExpression>,Box<MessageTypeExpression>),
    SetMinus(Box<MessageTypeExpression>,Box<MessageTypeExpression>)
}

impl MessageTypeExpression {

    /**
    Current simplification of set-theoretic-like expressions is absolutely not optimized
    It just gets the job more or less done with a lot of superfluous calculations
    TODO: find and implement an efficient algorithm
    **/
    pub fn simplify(self, gen_ctx : &GeneralContext) -> Self {
        match self {
            MessageTypeExpression::Singleton(ms_id) => {
                return MessageTypeExpression::Singleton(ms_id);
            },
            MessageTypeExpression::NamedType(mt_id) => {
                return MessageTypeExpression::NamedType(mt_id);
            },
            _ => {}
        };
        // ***
        let resolved = self.resolve(gen_ctx);
        if resolved.len() == 1 {
            return MessageTypeExpression::Singleton(*resolved.iter().next().unwrap());
        } else {
            for ms_ty_idx in 0..gen_ctx.get_mt_num() {
                let type_messages = gen_ctx.get_mt_messages(ms_ty_idx).unwrap();
                if type_messages == resolved {
                    return MessageTypeExpression::NamedType(ms_ty_idx);
                }
            }
        }
        // ***
        match self {
            MessageTypeExpression::SetMinus(mte1,mte2) => {
                let intersect = MessageTypeExpression::Intersection(
                    Box::new(*mte1.clone()),
                    Box::new(*mte2.clone()));
                if intersect.resolve(gen_ctx).is_empty() {
                    return mte1.simplify(gen_ctx);
                } else {
                    let mut left_unions = mte1.get_recursive_msg_union(gen_ctx);
                    let mut right_unions = mte2.get_recursive_msg_union(gen_ctx);
                    {
                        let l_as_hashset : HashSet<MessageTypeExpression> = left_unions.iter().cloned().collect();
                        let r_as_hashset : HashSet<MessageTypeExpression> = right_unions.iter().cloned().collect();
                        for common in l_as_hashset.intersection(&r_as_hashset) {
                            let index = left_unions.iter().position(|x| x == common).unwrap();
                            left_unions.remove(index);
                            let index = right_unions.iter().position(|x| x == common).unwrap();
                            right_unions.remove(index);
                        }
                    }
                    assert!(!left_unions.is_empty());
                    if right_unions.is_empty() {
                        return Self::fold_recursive_msg_unions(&mut left_unions);
                    } else {
                        return MessageTypeExpression::SetMinus(
                            Box::new(Self::fold_recursive_msg_unions(&mut left_unions)),
                            Box::new(Self::fold_recursive_msg_unions(&mut right_unions)));
                    }
                }
            },
            MessageTypeExpression::Union(mte1,mte2) => {
                let mut unions = mte1.get_recursive_msg_union(gen_ctx);
                for x in mte2.get_recursive_msg_union(gen_ctx) {
                    if !unions.contains(&x) {
                        unions.push(x);
                    }
                }
                return Self::fold_recursive_msg_unions(&mut unions);
            },
            MessageTypeExpression::Intersection(mte1,mte2) => {
                let mut inters = mte1.get_recursive_msg_inter(gen_ctx);
                for x in mte2.get_recursive_msg_inter(gen_ctx) {
                    if !inters.contains(&x) {
                        inters.push(x);
                    }
                }
                return Self::fold_recursive_msg_inters(&mut inters);
            },
            _ => {self}
        }
        // ***

    }

    pub fn resolve(&self, gen_ctx : &GeneralContext) -> HashSet<usize> {
        match self {
            //MessageTypeExpression::Empty => {hashset![]},
            MessageTypeExpression::Singleton(ms_id) => {hashset![*ms_id]},
            MessageTypeExpression::NamedType(mt_id) => {
                gen_ctx.get_mt_messages(*mt_id).unwrap()
            },
            MessageTypeExpression::Union(mte1,mte2) => {
                mte1.resolve(gen_ctx).union(&mte2.resolve(gen_ctx)).into_iter().cloned().collect()
            },
            MessageTypeExpression::Intersection(mte1,mte2) => {
                mte1.resolve(gen_ctx).intersection(&mte2.resolve(gen_ctx)).into_iter().cloned().collect()
            },
            MessageTypeExpression::SetMinus(mte1,mte2) => {
                mte1.resolve(gen_ctx).difference(&mte2.resolve(gen_ctx)).into_iter().cloned().collect()
            }
        }
    }

    fn get_recursive_msg_union(self, gen_ctx : &GeneralContext) -> Vec<MessageTypeExpression> {
        let mut frags = vec![];
        match self {
            MessageTypeExpression::Union(mte1,mte2) => {
                for x in mte1.simplify(gen_ctx).get_recursive_msg_union(gen_ctx) {
                    if !frags.contains(&x) {
                        frags.push(x);
                    }
                }
                for x in mte2.simplify(gen_ctx).get_recursive_msg_union(gen_ctx) {
                    if !frags.contains(&x) {
                        frags.push(x);
                    }
                }
            },
            x => {
                frags.push(x.simplify(gen_ctx));
            }
        }
        frags
    }

    fn get_recursive_msg_inter(self, gen_ctx : &GeneralContext) -> Vec<MessageTypeExpression> {
        let mut frags = vec![];
        match self {
            MessageTypeExpression::Intersection(mte1,mte2) => {
                for x in mte1.simplify(gen_ctx).get_recursive_msg_inter(gen_ctx) {
                    if !frags.contains(&x) {
                        frags.push(x);
                    }
                }
                for x in mte2.simplify(gen_ctx).get_recursive_msg_inter(gen_ctx) {
                    if !frags.contains(&x) {
                        frags.push(x);
                    }
                }
            },
            x => {
                frags.push(x.simplify(gen_ctx));
            }
        }
        frags
    }

    pub fn fold_recursive_msg_unions(frags : &mut Vec<MessageTypeExpression>) -> MessageTypeExpression {
        let frag_num = frags.len();
        if frag_num == 2 {
            let mte2 = frags.pop().unwrap();
            let mte1 = frags.pop().unwrap();
            MessageTypeExpression::Union( Box::new(mte1),
                                          Box::new(mte2) )
        } else if frag_num == 1 {
            frags.pop().unwrap().clone()
        } else if frag_num == 0 {
            panic!()
        } else {
            let mte1 = frags.remove(0);
            MessageTypeExpression::Union( Box::new(mte1),
                                          Box::new( Self::fold_recursive_msg_unions(frags) ) )
        }
    }

    pub fn fold_recursive_msg_inters(frags : &mut Vec<MessageTypeExpression>) -> MessageTypeExpression {
        let frag_num = frags.len();
        if frag_num == 2 {
            let mte2 = frags.pop().unwrap();
            let mte1 = frags.pop().unwrap();
            MessageTypeExpression::Intersection( Box::new(mte1),
                                          Box::new(mte2) )
        } else if frag_num == 1 {
            frags.pop().unwrap().clone()
        } else if frag_num == 0 {
            panic!()
        } else {
            let mte1 = frags.remove(0);
            MessageTypeExpression::Intersection( Box::new(mte1),
                                          Box::new( Self::fold_recursive_msg_inters(frags) ) )
        }
    }
}





