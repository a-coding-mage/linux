/* SPDX-License-Identifier: GPL-2.0 */

/*
 * How do these macros work?
 *
 * In __COUNT_ARGS() _0 to _15 are just placeholders from the start
 * in order to make sure _n is positioned over the correct number
 * from 15 to 0 (depending on X, which is a variadic argument list).
 * They serve no purpose other than occupying a position. Since each
 * macro parameter must have a distinct identifier, those identifiers
 * are as good as any.
 *
 * In COUNT_ARGS() we use actual integers, so __COUNT_ARGS() returns
 * that as _n.
 */

/* This counts to 15. Any more, it will return 16th argument. */
#[macro_export]
macro_rules! __COUNT_ARGS {
    ($($arg:tt),* $(,)?) => {
        $crate::__COUNT_ARGS_INNER!($($arg),*)
    };
}

#[macro_export]
macro_rules! __COUNT_ARGS_INNER {
    () => { 0usize };
    ($_0:tt) => { 1usize };
    ($_0:tt, $_1:tt) => { 2usize };
    ($_0:tt, $_1:tt, $_2:tt) => { 3usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt) => { 4usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt) => { 5usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt) => { 6usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt) => { 7usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt) => { 8usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt) => { 9usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt, $_9:tt) => { 10usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt, $_9:tt, $_10:tt) => { 11usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt, $_9:tt, $_10:tt, $_11:tt) => { 12usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt, $_9:tt, $_10:tt, $_11:tt, $_12:tt) => { 13usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt, $_9:tt, $_10:tt, $_11:tt, $_12:tt, $_13:tt) => { 14usize };
    ($_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt, $_9:tt, $_10:tt, $_11:tt, $_12:tt, $_13:tt, $_14:tt) => { 15usize };
}

#[macro_export]
macro_rules! COUNT_ARGS {
    ($($arg:tt)*) => {
        $crate::__COUNT_ARGS_INNER!($($arg)*)
    };
}

/* Rust has no stable token-pasting equivalent to C's ## operator. */
#[macro_export]
macro_rules! __CONCAT {
    ($a:ident, $b:ident) => {
        concat!(stringify!($a), stringify!($b))
    };
}

#[macro_export]
macro_rules! CONCATENATE {
    ($a:ident, $b:ident) => {
        $crate::__CONCAT!($a, $b)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
