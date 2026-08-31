// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "async_stack_depth.skel.h"

extern "C" {
    fn RUN_TESTS(test: async_stack_depth);
}

// External skeleton type supplied by "async_stack_depth.skel.h".
type async_stack_depth = *mut ::core::ffi::c_void;

pub unsafe extern "C" fn test_async_stack_depth() {
    RUN_TESTS(::core::ptr::null_mut());
}
