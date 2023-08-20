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
use crate::core::execution::trace::trace::TraceActionKind;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::message::MessageTypeExpression;


pub fn fold_recursive_alt_frags(frags : &mut Vec<&Interaction>) -> Interaction {
    let frag_num = frags.len();
    if frag_num == 2 {
        let i2 = frags.pop().unwrap();
        let i1 = frags.pop().unwrap();
        Interaction::Alt( Box::new(i1.clone()), Box::new(i2.clone()) )
    } else if frag_num == 1 {
        frags.pop().unwrap().clone()
    } else if frag_num == 0 {
        Interaction::Empty
    } else {
        let i1 = frags.remove(0);
        Interaction::Alt( Box::new(i1.clone()), Box::new( fold_recursive_alt_frags(frags) ) )
    }
}


pub fn fold_recursive_coreg_frags(ref_cr : &Vec<usize>, frags : &mut Vec<&Interaction>) -> Interaction {
    let frag_num = frags.len();
    if frag_num == 2 {
        let i2 = frags.pop().unwrap();
        let i1 = frags.pop().unwrap();
        Interaction::CoReg(ref_cr.clone(), Box::new(i1.clone()), Box::new(i2.clone()) )
    } else if frag_num == 1 {
        frags.pop().unwrap().clone()
    } else if frag_num == 0 {
        Interaction::Empty
    } else {
        let i1 = frags.remove(0);
        Interaction::CoReg( ref_cr.clone(),Box::new(i1.clone()), Box::new( fold_recursive_coreg_frags(ref_cr, frags) ) )
    }
}


pub fn fold_recursive_sync_frags(sync : &BTreeMap<(usize,TraceActionKind),MessageTypeExpression>, frags : &mut Vec<&Interaction>) -> Interaction {
    let frag_num = frags.len();
    if frag_num == 2 {
        let i2 = frags.pop().unwrap();
        let i1 = frags.pop().unwrap();
        Interaction::Sync(sync.clone(),
                          Box::new(i1.clone()),
                          Box::new(i2.clone()) )
    } else if frag_num == 1 {
        frags.pop().unwrap().clone()
    } else if frag_num == 0 {
        Interaction::Empty
    } else {
        let i1 = frags.remove(0);
        Interaction::Sync( sync.clone(),
                           Box::new(i1.clone()),
                           Box::new( fold_recursive_sync_frags(sync, frags) ) )
    }
}
