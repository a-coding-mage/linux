// SPDX-License-Identifier: GPL-2.0-only

// Dependency supplied by test_fortify.h in the original source.
macro_rules! TEST {
    () => {
        strcpy(small, LITERAL_LARGE)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
