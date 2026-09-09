// SPDX-License-Identifier: GPL-2.0-only

// Dependency intent: symbols such as `memmove`, `large`, and `instance` are
// supplied by the corresponding test_fortify support source/header.
macro_rules! TEST {
    () => {
        memmove(large, instance.buf, core::mem::size_of_val(&large))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
