// SPDX-License-Identifier: GPL-2.0-only

// Dependency intent: declarations supplied by test_fortify.h are provided by
// the surrounding translation unit.

macro_rules! TEST {
    () => {{
        unsafe {
            std::ptr::copy_nonoverlapping(
                instance.buf.as_ptr(),
                large.as_mut_ptr(),
                std::mem::size_of_val(&large),
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
