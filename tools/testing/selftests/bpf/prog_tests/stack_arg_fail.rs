// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <test_progs.h>
// #include "stack_arg_fail.skel.h"

extern "C" {
    fn RUN_TESTS(test: stack_arg_fail);
}

// External type supplied by "stack_arg_fail.skel.h".
type stack_arg_fail = *mut ::core::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_fail() {
    RUN_TESTS(::core::ptr::null_mut());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
