// SPDX-License-Identifier: GPL-2.0-only

// Translated from the C preprocessor macro:
// #define TEST memset(instance.buf, 0x5A, sizeof(large_src))
//
// The C macro operates on the caller's `instance` and `large_src` objects.
// Pass those objects explicitly in Rust to preserve the same operation.
macro_rules! TEST {
    ($instance:expr, $large_src:expr) => {{
        unsafe {
            core::ptr::write_bytes(
                ($instance).buf.as_mut_ptr(),
                0x5Au8,
                core::mem::size_of_val(&$large_src),
            );
        }
    }};
}

// Symbols and declarations supplied by "test_fortify.h" are external to this
// isolated translation unit and are intentionally not reimplemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
