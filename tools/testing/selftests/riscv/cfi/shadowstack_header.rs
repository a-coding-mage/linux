/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/*
 * A CFI test returns true for success or false for fail.
 * Takes a test number to index into array, and a void pointer.
 */
pub type shstk_test_func = Option<unsafe extern "C" fn(test_num: c_ulong, arg1: *mut c_void) -> bool>;

#[repr(C)]
pub struct shadow_stack_tests {
    pub name: *mut c_char,
    pub t_func: shstk_test_func,
}

unsafe extern "C" {
    pub fn shadow_stack_fork_test(test_num: c_ulong, ctx: *mut c_void) -> bool;
    pub fn shadow_stack_map_test(test_num: c_ulong, ctx: *mut c_void) -> bool;
    pub fn shadow_stack_protection_test(test_num: c_ulong, ctx: *mut c_void) -> bool;
    pub fn shadow_stack_gup_tests(test_num: c_ulong, ctx: *mut c_void) -> bool;
    pub fn shadow_stack_signal_test(test_num: c_ulong, ctx: *mut c_void) -> bool;

    pub fn execute_shadow_stack_tests() -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
