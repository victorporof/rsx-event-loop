/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![cfg_attr(feature = "cargo-clippy", allow(match_ref_pats))]
#![feature(box_syntax)]
#![feature(proc_macro)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate enum_str_derive;
extern crate num_traits;
extern crate rsx_shared;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod longhands;
pub mod manager;
pub mod types;
