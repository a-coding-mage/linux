// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <test_progs.h>
// #include "stack_arg_precision.skel.h"

#[no_mangle]
pub extern "C" fn test_stack_arg_precision() {
    RUN_TESTS!(stack_arg_precision);
}
