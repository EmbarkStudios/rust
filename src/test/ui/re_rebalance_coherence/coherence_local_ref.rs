// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that we are able to introduce a negative constraint that
// `MyType: !MyTrait` along with other "fundamental" wrappers.

// aux-build:coherence_copy_like_lib.rs
// compile-pass
// skip-codegen
#![allow(dead_code)]
#![feature(re_rebalance_coherence)]

extern crate coherence_copy_like_lib as lib;

struct MyType { x: i32 }

// naturally, legal
impl lib::MyCopy for MyType { }


fn main() { }
