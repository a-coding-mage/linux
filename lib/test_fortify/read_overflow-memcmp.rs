// SPDX-License-Identifier: GPL-2.0-only

// Translated from the C preprocessor macro:
// #define TEST \
//     memcmp(small, large, sizeof(small) + 1)
//
// `small`, `large`, and `memcmp` are supplied by the translated
// `test_fortify.h` dependency.
macro_rules! TEST {
    () => {{
        memcmp(
            small as *const _,
            large as *const _,
            ::core::mem::size_of_val(&small) + 1,
        )
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
