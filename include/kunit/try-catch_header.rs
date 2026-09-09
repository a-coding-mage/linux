/* SPDX-License-Identifier: GPL-2.0 */
/*
 * An API to allow a function, that may fail, to be executed, and recover in a
 * controlled manner.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: Brendan Higgins <brendanhiggins@google.com>
 */

use core::ffi::c_void;

/// Function type used by the try/catch interface.
pub type kunit_try_catch_func_t = unsafe extern "C" fn(*mut c_void);

pub struct kunit;

/**
 * struct kunit_try_catch - provides a generic way to run code which might fail.
 * @test: The test case that is currently being executed.
 * @try_result: Contains any errno obtained while running test case.
 * @try: The function, the test case, to attempt to run.
 * @catch: The function called if @try bails out.
 * @context: used to pass user data to the try and catch functions.
 *
 * kunit_try_catch provides a generic, architecture independent way to execute
 * an arbitrary function of type kunit_try_catch_func_t which may bail out by
 * calling kunit_try_catch_throw(). If kunit_try_catch_throw() is called, @try
 * is stopped at the site of invocation and @catch is called.
 *
 * struct kunit_try_catch provides a generic interface for the functionality
 * needed to implement kunit->abort() which in turn is needed for implementing
 * assertions. Assertions allow stating a precondition for a test simplifying
 * how test cases are written and presented.
 *
 * Assertions are like expectations, except they abort (call
 * kunit_try_catch_throw()) when the specified condition is not met. This is
 * useful when you look at a test case as a logical statement about some piece
 * of code, where assertions are the premises for the test case, and the
 * conclusion is a set of predicates, rather expectations, that must all be
 * true. If your premises are violated, it does not makes sense to continue.
 */
#[repr(C)]
pub struct kunit_try_catch {
    /* private: internal use only. */
    pub test: *mut kunit,
    pub try_result: i32,
    pub r#try: Option<kunit_try_catch_func_t>,
    pub catch: Option<kunit_try_catch_func_t>,
    pub timeout: core::ffi::c_ulong,
    pub context: *mut c_void,
}

unsafe extern "C" {
    pub fn kunit_try_catch_run(
        try_catch: *mut kunit_try_catch,
        context: *mut c_void,
    );

    #[noreturn]
    pub fn kunit_try_catch_throw(try_catch: *mut kunit_try_catch) -> !;
}

#[inline]
pub unsafe fn kunit_try_catch_get_result(try_catch: *mut kunit_try_catch) -> i32 {
    (*try_catch).try_result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
