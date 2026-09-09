// SPDX-License-Identifier: GPL-2.0-only

// Dependency declarations from "test_fortify.h" are supplied by the
// surrounding translation unit.

macro_rules! TEST {
    () => {
        memcmp(large, small, ::core::mem::size_of_val(&small) + 1)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
