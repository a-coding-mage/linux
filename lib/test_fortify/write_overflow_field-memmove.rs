// SPDX-License-Identifier: GPL-2.0-only

// The C source includes "test_fortify.h", which supplies the surrounding
// test definitions and the names referenced by this macro.
macro_rules! TEST {
    () => {{
        unsafe {
            ::core::ptr::copy(
                large.as_ptr(),
                instance.buf.as_mut_ptr(),
                ::core::mem::size_of_val(&instance.buf) + 1,
            )
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
