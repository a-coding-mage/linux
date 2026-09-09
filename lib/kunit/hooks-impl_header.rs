/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Declarations for hook implementations.
 *
 * These will be set as the function pointers in struct kunit_hook_table,
 * found in include/kunit/test-bug.h.
 *
 * Copyright (C) 2023, Google LLC.
 * Author: David Gow <davidgow@google.com>
 */

/* Declarations supplied by kunit/test-bug.h are external Rust dependencies. */

extern "C" {
    pub fn __kunit_fail_current_test_impl(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...,
    );
    pub fn __kunit_get_static_stub_address_impl(
        test: *mut ::core::ffi::c_void,
        real_fn_addr: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    pub fn __kunit_is_suppressed_warning_impl(count: bool) -> bool;
}

/* Code to set all of the function pointers. */
#[inline]
pub unsafe fn kunit_install_hooks() {
    /* Install the KUnit hook functions. */
    kunit_hooks.fail_current_test = Some(__kunit_fail_current_test_impl);
    kunit_hooks.get_static_stub_address = Some(__kunit_get_static_stub_address_impl);
    kunit_hooks.is_suppressed_warning = Some(__kunit_is_suppressed_warning_impl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
