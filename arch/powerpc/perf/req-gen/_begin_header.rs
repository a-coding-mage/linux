/* SPDX-License-Identifier: GPL-2.0 */
/* Include paths to be used in interface defining headers */

/* The C header includes <linux/stringify.h>; its token-stringification
 * helpers are represented here by the corresponding Rust string values. */

/// Concatenate two path components with a slash, equivalent to CAT2_STR.
macro_rules! CAT2_STR_ {
    ($t:expr, $s:expr) => {
        concat!($t, "/", $s)
    };
}

/// Concatenate two path components with a slash, equivalent to CAT2_STR.
macro_rules! CAT2_STR {
    ($t:expr, $s:expr) => {
        CAT2_STR_!($t, $s)
    };
}

/// Expand to the supplied arguments, equivalent to the C variadic macro I.
macro_rules! I {
    ($($arg:tt)*) => { $($arg)* };
}

pub const REQ_GEN_PREFIX: &str = "req-gen";
pub const REQUEST_BEGIN: &str = "req-gen/_request-begin.h";
pub const REQUEST_END: &str = "req-gen/_request-end.h";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
