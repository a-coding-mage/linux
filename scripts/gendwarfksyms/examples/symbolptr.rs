// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Google LLC
 *
 * Example for symbol pointers. When compiled with Clang, gendwarfksyms
 * uses a symbol pointer for `f`.
 *
 * $ clang -g -c examples/symbolptr.c -o examples/symbolptr.o
 * $ echo -e "f\ng\np" | ./gendwarfksyms -d examples/symbolptr.o
 */

use core::ffi::c_int;

// Kernel macros for userspace testing:
// `__used` and `__section` are represented by Rust's `used` and `link_section`
// attributes on the generated symbol-pointer statics below.

unsafe extern "C" {
    fn f(arg: u32);
}

#[repr(C)]
pub struct s {
    _private: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn g(arg: *mut c_int) {
    let _ = arg;
}

unsafe extern "C" {
    static mut p: *mut s;
}

#[used]
#[unsafe(link_section = ".discard.gendwarfksyms")]
static mut __gendwarfksyms_ptr_f: Option<unsafe extern "C" fn(u32)> = Some(f);

#[used]
#[unsafe(link_section = ".discard.gendwarfksyms")]
static mut __gendwarfksyms_ptr_g: Option<unsafe extern "C" fn(*mut c_int)> = Some(g);

#[used]
#[unsafe(link_section = ".discard.gendwarfksyms")]
static mut __gendwarfksyms_ptr_p: *mut *mut s = unsafe { &raw mut p };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
