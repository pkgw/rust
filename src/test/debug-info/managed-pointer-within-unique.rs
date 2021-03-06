// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-win32 Broken because of LLVM bug: http://llvm.org/bugs/show_bug.cgi?id=16249

// compile-flags:-Z extra-debug-info
// debugger:set print pretty off
// debugger:break zzz
// debugger:run
// debugger:finish

// debugger:print *ordinary_unique
// check:$1 = {-1, -2}

// debugger:print managed_within_unique.val->x
// check:$2 = -3

// debugger:print managed_within_unique.val->y->val
// check:$3 = -4

struct ContainsManaged
{
	x: int,
	y: @int
}

fn main() {

	let ordinary_unique = ~(-1, -2);


	// This is a special case: Normally values allocated in the exchange heap are not boxed, unless,
	// however, if they contain managed pointers.
	// This test case verifies that both cases are handled correctly.
    let managed_within_unique = ~ContainsManaged { x: -3, y: @-4 };

    zzz();
}

fn zzz() {()}