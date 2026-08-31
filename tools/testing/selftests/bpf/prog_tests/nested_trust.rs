// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <test_progs.h>
// #include "nested_trust_failure.skel.h"
// #include "nested_trust_success.skel.h"
// #include "nested_acquire.skel.h"

#[repr(C)]
pub struct Env {
    pub has_testmod: bool,
}

unsafe extern "C" {
    static env: Env;

    fn RUN_TESTS(test_name: *const ::std::os::raw::c_char);
}

const NESTED_TRUST_SUCCESS: &[u8] = b"nested_trust_success\0";
const NESTED_TRUST_FAILURE: &[u8] = b"nested_trust_failure\0";
const NESTED_ACQUIRE: &[u8] = b"nested_acquire\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_nested_trust() {
    unsafe {
        RUN_TESTS(NESTED_TRUST_SUCCESS.as_ptr() as *const ::std::os::raw::c_char);
        RUN_TESTS(NESTED_TRUST_FAILURE.as_ptr() as *const ::std::os::raw::c_char);

        if env.has_testmod {
            RUN_TESTS(NESTED_ACQUIRE.as_ptr() as *const ::std::os::raw::c_char);
        }
    }
}
