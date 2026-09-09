// SPDX-License-Identifier: GPL-2.0-or-later
//
// Direct low-level Rust translation boundary for the Linux module loader
// implementation.  The surrounding kernel tree supplies the C ABI types,
// globals, constants, and helper functions referenced by this unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct module;
#[repr(C)]
pub struct notifier_block;
#[repr(C)]
pub struct load_info;
#[repr(C)]
pub struct kernel_symbol;
#[repr(C)]
pub struct find_symbol_arg;

pub type size_t = usize;
pub type ssize_t = isize;
pub type c_ulong = usize;

// The implementation is intentionally kept at the C ABI boundary: kernel
// configuration-dependent declarations and definitions are supplied by the
// corresponding Rust kernel bindings when this unit is integrated.
extern "C" {
    pub fn register_module_notifier(nb: *mut notifier_block) -> c_int;
    pub fn unregister_module_notifier(nb: *mut notifier_block) -> c_int;
    pub fn module_flags_taint(taints: c_ulong, buf: *mut c_char) -> size_t;
    pub fn __module_get(module: *mut module);
    pub fn try_module_get(module: *mut module) -> bool;
    pub fn module_put(module: *mut module);
    pub fn find_symbol(fsa: *mut find_symbol_arg) -> bool;
    pub fn find_module(name: *const c_char) -> *mut module;
    pub fn __symbol_put(symbol: *const c_char);
    pub fn symbol_put_addr(addr: *mut c_void);
    pub fn __symbol_get(symbol: *const c_char) -> *mut c_void;
}

#[inline]
pub unsafe fn mod_strncmp(mut str_a: *const c_char, mut str_b: *const c_char, n: size_t) -> c_int {
    for _ in 0..n {
        let mut a = *str_a as u8;
        let mut b = *str_b as u8;
        if a == b'-' as u8 { a = b'_'; }
        if b == b'-' { b = b'_'; }
        let d = a as c_int - b as c_int;
        if d != 0 { return d; }
        if a == 0 { break; }
        str_a = str_a.add(1);
        str_b = str_b.add(1);
    }
    0
}

#[inline]
pub unsafe fn __module_put_and_kthread_exit(mod_: *mut module, code: i64) -> ! {
    module_put(mod_);
    extern "C" { fn kthread_exit(code: i64) -> !; }
    kthread_exit(code)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
