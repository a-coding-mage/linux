/* SPDX-License-Identifier: GPL-2.0 */

// When assembling, stringify_in_c expands its arguments without stringizing.
// The assembler-specific branch is preserved here as conditional intent.
#[cfg(__ASSEMBLER__)]
macro_rules! stringify_in_c {
    ($($arg:tt)*) => { $($arg)* };
}

// This version of stringify deals with commas, and appends a trailing space.
#[cfg(not(__ASSEMBLER__))]
macro_rules! __stringify_in_c {
    ($($arg:tt)*) => { stringify!($($arg)*) };
}

#[cfg(not(__ASSEMBLER__))]
macro_rules! stringify_in_c {
    ($($arg:tt)*) => {
        concat!(__stringify_in_c!($($arg)*), " ")
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
