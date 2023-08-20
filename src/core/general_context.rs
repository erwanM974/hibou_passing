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
use std::iter::FromIterator;


use crate::core::error::HibouCoreError;

#[derive(Clone, PartialEq, Debug)]
pub struct GeneralContext {
    lf_names : Vec<String>,
    ms_names : Vec<String>,
    mt_names : Vec<(String,HashSet<usize>)> // named message types (hashset identifies which messages are of this type)
}



impl GeneralContext {

    pub fn new() -> GeneralContext {
        return GeneralContext {
            lf_names: Vec::new(),
            ms_names: Vec::new(),
            mt_names: Vec::new()
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn add_lf(&mut self, lf_name : String) -> usize {
        match self.get_lf_id(&lf_name) {
            None => {
                self.lf_names.push(lf_name);
                return self.lf_names.len() - 1;
            },
            Some(lf_id) => {
                return lf_id;
            }
        }
    }

    pub fn add_msg(&mut self, ms_name : String) -> usize {
        match self.get_ms_id(&ms_name) {
            None => {
                self.ms_names.push(ms_name);
                return self.ms_names.len() - 1;
            },
            Some(ms_id) => {
                return ms_id;
            }
        }
    }

    pub fn add_mt(&mut self, mt_name : String, messages : HashSet<usize>) -> usize {
        match self.get_mt_id(&mt_name) {
            None => {
                self.mt_names.push((mt_name,messages));
                return self.mt_names.len() - 1;
            },
            Some(mt_id) => {
                return mt_id;
            }
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_id(&self, lf_name : &str) -> Option<usize> {
        self.lf_names.iter().position(|r| r == lf_name)
    }

    pub fn get_ms_id(&self, ms_name : &str) -> Option<usize> {
        self.ms_names.iter().position(|n| n == ms_name)
    }

    pub fn get_mt_id(&self, mt_name : &str) -> Option<usize> {
        self.mt_names.iter().position(|(n,_)| n == mt_name)
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_num(&self) -> usize {
        self.lf_names.len()
    }

    pub fn get_ms_num(&self) -> usize {
        self.ms_names.len()
    }

    pub fn get_mt_num(&self) -> usize {
        self.mt_names.len()
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_all_lfs_ids(&self) -> HashSet<usize> {
        return HashSet::from_iter(0..self.get_lf_num() );
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_name(&self, lf_id : usize) -> Result<String,HibouCoreError> {
        match self.lf_names.get(lf_id) {
            None => {
                return Err( HibouCoreError::UnknownLifeline(lf_id) );
            },
            Some( got_str ) => {
                return Ok( got_str.to_string() );
            }
        }
    }

    pub fn get_ms_name(&self, ms_id : usize) -> Result<String,HibouCoreError> {
        match self.ms_names.get(ms_id) {
            None => {
                return Err( HibouCoreError::UnknownMessage(ms_id) );
            },
            Some( ms_name ) => {
                return Ok( ms_name.to_string() );
            }
        }
    }

    pub fn get_mt_name(&self, mt_id : usize) -> Result<String,HibouCoreError> {
        match self.mt_names.get(mt_id) {
            None => {
                return Err( HibouCoreError::UnknownMessage(mt_id) );
            },
            Some( (mt_name,_) ) => {
                return Ok( mt_name.to_string() );
            }
        }
    }

    pub fn get_mt_messages(&self, mt_id : usize) -> Result<HashSet<usize>,HibouCoreError> {
        match self.mt_names.get(mt_id) {
            None => {
                return Err( HibouCoreError::UnknownMessage(mt_id) );
            },
            Some( (_,messages) ) => {
                return Ok( messages.clone() );
            }
        }
    }

}
