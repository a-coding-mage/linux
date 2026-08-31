// SPDX-License-Identifier: GPL-2.0

/* Copyright (c) 2024 Meta Platforms, Inc */

// C dependencies:
// #include <test_progs.h>
// #include "test_kfunc_param_nullable.skel.h"

extern "C" {
    fn RUN_TESTS(test_name: *const ::core::ffi::c_void);
    static test_kfunc_param_nullable: ::core::ffi::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn test_kfunc_param_nullable() {
    unsafe {
        RUN_TESTS(&test_kfunc_param_nullable as *const ::core::ffi::c_void);
    }
}
