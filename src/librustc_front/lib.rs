// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Rust compiler.
//!
//! # Note
//!
//! This API is completely unstable and subject to change.

#![crate_name = "rustc_front"]
#![unstable(feature = "rustc_private", issue = "27812")]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "http://doc.rust-lang.org/nightly/")]
#![cfg_attr(not(stage0), deny(warnings))]

#![feature(associated_consts)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(const_fn)]
#![feature(quote)]
#![feature(rustc_diagnostic_macros)]
#![feature(rustc_private)]
#![feature(slice_patterns)]
#![feature(staged_api)]
#![feature(str_char)]
#![feature(question_mark)]

extern crate serialize;
#[macro_use]
extern crate log;
#[macro_use]
extern crate syntax;
#[macro_use]
#[no_link]
extern crate rustc_bitflags;

extern crate serialize as rustc_serialize; // used by deriving

#[macro_use]
pub mod hir;
pub mod lowering;
pub mod fold;
pub mod intravisit;
pub mod util;

pub mod print {
    pub mod pprust;
}
