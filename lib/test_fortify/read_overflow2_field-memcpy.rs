// SPDX-License-Identifier: GPL-2.0-only

// Dependency provided by test_fortify.h in the C source.
// C macro:
// #define TEST \
//     memcpy(large, instance.buf, sizeof(instance.buf) + 1)
//
// Rust equivalent, preserving the original raw-memory operation and the
// surrounding names supplied by the including context.
macro_rules! TEST {
    () => {
        unsafe {
            ::core::ptr::copy_nonoverlapping(
                instance.buf.as_ptr(),
                large.as_mut_ptr(),
                ::core::mem::size_of_val(&instance.buf).wrapping_add(1),
            )
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
