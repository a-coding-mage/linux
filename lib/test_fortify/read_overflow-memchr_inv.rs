// SPDX-License-Identifier: GPL-2.0-only

// Equivalent of: #include "test_fortify.h"

macro_rules! TEST {
    () => {
        memchr_inv(small, 0x7A, core::mem::size_of_val(&small) + 1)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
