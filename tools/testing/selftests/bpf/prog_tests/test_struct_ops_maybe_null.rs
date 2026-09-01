// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_void};

// Dependencies from <test_progs.h> and the generated skeleton headers:
// "struct_ops_maybe_null.skel.h"
// "struct_ops_maybe_null_fail.skel.h"

#[repr(C)]
pub struct struct_ops_maybe_null {
    _private: [u8; 0],
}

#[repr(C)]
pub struct struct_ops_maybe_null_fail {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn struct_ops_maybe_null__open_and_load() -> *mut struct_ops_maybe_null;
    fn struct_ops_maybe_null__destroy(skel: *mut struct_ops_maybe_null);

    fn struct_ops_maybe_null_fail__open_and_load() -> *mut struct_ops_maybe_null_fail;
    fn struct_ops_maybe_null_fail__destroy(skel: *mut struct_ops_maybe_null_fail);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

/* Test that the verifier accepts a program that access a nullable pointer
 * with a proper check.
 */
unsafe fn maybe_null() {
    let skel: *mut struct_ops_maybe_null;

    skel = unsafe { struct_ops_maybe_null__open_and_load() };
    if !unsafe {
        ASSERT_OK_PTR(
            skel as *const c_void,
            b"struct_ops_module_open_and_load\0".as_ptr() as *const c_char,
        )
    } {
        return;
    }

    unsafe { struct_ops_maybe_null__destroy(skel) };
}

/* Test that the verifier rejects a program that access a nullable pointer
 * without a check beforehand.
 */
unsafe fn maybe_null_fail() {
    let skel: *mut struct_ops_maybe_null_fail;

    skel = unsafe { struct_ops_maybe_null_fail__open_and_load() };
    if unsafe {
        ASSERT_ERR_PTR(
            skel as *const c_void,
            b"struct_ops_module_fail__open_and_load\0".as_ptr() as *const c_char,
        )
    } {
        return;
    }

    unsafe { struct_ops_maybe_null_fail__destroy(skel) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_struct_ops_maybe_null() {
    /* The verifier verifies the programs at load time, so testing both
     * programs in the same compile-unit is complicated. We run them in
     * separate objects to simplify the testing.
     */
    if unsafe { test__start_subtest(b"maybe_null\0".as_ptr() as *const c_char) } {
        unsafe { maybe_null() };
    }
    if unsafe { test__start_subtest(b"maybe_null_fail\0".as_ptr() as *const c_char) } {
        unsafe { maybe_null_fail() };
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
