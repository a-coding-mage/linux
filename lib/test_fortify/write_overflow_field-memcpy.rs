// SPDX-License-Identifier: GPL-2.0-only

// Translated from the C preprocessor macro:
// memcpy(instance.buf, large, sizeof(instance.buf) + 1)
//
// The declarations of `instance` and `large` are supplied by the included
// test support code in the original source.
macro_rules! TEST {
    () => {{
        unsafe {
            ::core::ptr::copy_nonoverlapping(
                large.as_ptr(),
                instance.buf.as_mut_ptr(),
                ::core::mem::size_of_val(&instance.buf) + 1,
            );
        }
    }};
}

// Original dependency: #include "test_fortify.h"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
