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



use std::fs;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};
use crate::core::general_context::GeneralContext;
use crate::trace_manip::slice::conf::*;
use crate::trace_manip::slice::exhaustive::{get_all_prefixes_rec, get_all_slices_rec, get_all_suffixes_rec};
use crate::trace_manip::slice::random::{get_random_slicing};




fn get_exhaustive_slicing<'a>(gen_ctx : &GeneralContext,
                              co_localizations : &CoLocalizations,
                          kind : &SliceKind,
                          dir_name : &String,
                              file_name_prefix : &String,
                          rem_canals : &mut (impl Iterator<Item = &'a Trace> + Clone)) {
    match kind {
        &SliceKind::Prefix => {
            get_all_prefixes_rec(gen_ctx,
                                 co_localizations,
                                 dir_name,file_name_prefix,
                                 &mut 1,
                                 &vec![],
                                 rem_canals);
        },
        &SliceKind::Suffix => {
            get_all_suffixes_rec(gen_ctx,
                                 co_localizations,
                                 dir_name,file_name_prefix,
                                 &mut 1,
                                 &vec![],
                                 rem_canals);
        },
        &SliceKind::Slice => {
            get_all_slices_rec(gen_ctx,
                               co_localizations,
                                 dir_name,file_name_prefix,
                                 &mut 1,
                                 &vec![],
                               rem_canals);
        }
    }
}

pub fn generate_slices(gen_ctx : &GeneralContext,
                       co_localizations : &CoLocalizations,
                       mu_name : &str,
                       multi_trace : &MultiTrace,
                       parent_folder : Option<&str>,
                       file_name_prefix_opt : Option<&str>,
                       select : &SliceGenerationSelection,
                       kind : &SliceKind) {
    let dir_name : String;
    match parent_folder {
        None => {
            dir_name = format!("./{:}_slices", mu_name);
        },
        Some( parent ) => {
            dir_name = parent.to_string();
        }
    }
    let file_name_prefix : String;
    match file_name_prefix_opt {
        None => {
            file_name_prefix = "".to_string();
        },
        Some( got_fnp ) => {
            file_name_prefix = got_fnp.to_string();
        }
    }
    /*
    // empties directory if exists
    match fs::remove_dir_all(&dir_name) {
        Ok(_) => {
            // do nothing
        },
        Err(e) => {
            // do nothing
        }
    }
    */
    // creates directory
    fs::create_dir_all(&dir_name).unwrap();
    // ***
    match select {
        &SliceGenerationSelection::Exhaustive => {
            get_exhaustive_slicing(gen_ctx,
                                   co_localizations,
                                   kind,
                                   &dir_name,&file_name_prefix,
                                   &mut multi_trace.iter());
        },
        &SliceGenerationSelection::Random( mut num_slices, wide ) => {
            get_random_slicing(gen_ctx,
                               co_localizations,
                               &dir_name,&file_name_prefix,
                               &mut num_slices,
                               &multi_trace,
                               kind,
                               wide);
        }
    }
}

