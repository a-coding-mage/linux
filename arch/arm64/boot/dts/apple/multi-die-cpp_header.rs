/* SPDX-License-Identifier: GPL-2.0+ OR MIT
 *
 * Rust equivalents of the C preprocessor macros for t600x multi die support.
 */

/* C preprocessor token stringification. */
macro_rules! __stringify_1 {
    ($($x:tt)*) => { stringify!($($x)*) };
}

macro_rules! __stringify {
    ($($x:tt)*) => { __stringify_1!($($x)*) };
}

/* C token pasting has no stable, general Rust equivalent. */
macro_rules! __concat_1 {
    ($x:ident, $($y:tt)*) => {
        compile_error!("C token pasting requires a generated Rust identifier")
    };
}

macro_rules! __concat {
    ($x:ident, $($y:tt)*) => { __concat_1!($x, $($y)*) };
}

macro_rules! DIE_NODE {
    ($a:ident) => { __concat!($a, DIE) };
}

macro_rules! DIE_LABEL {
    ($a:ident) => { concat!(stringify!($a), "DIE") };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
