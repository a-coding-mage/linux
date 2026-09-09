/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Internal kunit try catch implementation to be shared with tests.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: Brendan Higgins <brendanhiggins@google.com>
 */

// Translated from <kunit/try-catch.h> and <linux/types.h>.

pub struct kunit;

#[inline]
pub unsafe fn kunit_try_catch_init(
    try_catch: *mut kunit_try_catch,
    test: *mut kunit,
    try_fn: kunit_try_catch_func_t,
    catch_fn: kunit_try_catch_func_t,
    timeout: ::core::ffi::c_ulong,
) {
    (*try_catch).test = test;
    (*try_catch).r#try = try_fn;
    (*try_catch).catch = catch_fn;
    (*try_catch).timeout = timeout;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
