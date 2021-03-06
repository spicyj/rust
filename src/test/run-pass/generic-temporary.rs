// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast
#[legacy_modes];

fn mk() -> int { return 1; }

fn chk(&&a: int) { log(debug, a); assert (a == 1); }

fn apply<T>(produce: extern fn() -> T,
            consume: extern fn(T)) {
    consume(produce());
}

fn main() {
    let produce: extern fn() -> int = mk;
    let consume: extern fn(&&v: int) = chk;
    apply::<int>(produce, consume);
}
