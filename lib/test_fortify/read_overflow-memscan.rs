// SPDX-License-Identifier: GPL-2.0-only

// Translated from the C preprocessor macro:
// #define TEST memscan(small, 0x7A, sizeof(small) + 1)
macro_rules! TEST {
    () => {
        memscan(small, 0x7A, core::mem::size_of_val(&small) + 1)
    };
}

// Dependency supplied by test_fortify.h; retained as an external reference.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
