// SPDX-License-Identifier: GPL-2.0-only

// Dependency declarations and symbols supplied by test_fortify.h are provided
// by the surrounding translation unit.

macro_rules! TEST {
    () => {{
        unsafe {
            core::ptr::copy(
                large_src.as_ptr(),
                instance.buf.as_mut_ptr(),
                core::mem::size_of_val(&large_src),
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
