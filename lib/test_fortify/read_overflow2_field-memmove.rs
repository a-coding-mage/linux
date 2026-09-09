// SPDX-License-Identifier: GPL-2.0-only

// Dependency declarations from "test_fortify.h" are supplied externally.
macro_rules! TEST {
    () => {
        memmove(large, instance.buf, core::mem::size_of_val(&instance.buf) + 1usize)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
