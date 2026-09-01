/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from include/linux/args.h.
 *
 * The original C header guard and include-only preprocessor structure are not
 * executable Rust. The macro definitions below preserve the local macro intent.
 */

/*
 * How do these macros work?
 *
 * In __COUNT_ARGS() _0 to _12 are just placeholders from the start
 * in order to make sure _n is positioned over the correct number
 * from 12 to 0 (depending on X, which is a variadic argument list).
 * They serve no purpose other than occupying a position. Since each
 * macro parameter must have a distinct identifier, those identifiers
 * are as good as any.
 *
 * In COUNT_ARGS() we use actual integers, so __COUNT_ARGS() returns
 * that as _n.
 */

/* This counts to 15. Any more, it will return 16th argument. */
macro_rules! __COUNT_ARGS {
    (
        $_0:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt,
        $_8:tt, $_9:tt, $_10:tt, $_11:tt, $_12:tt, $_13:tt, $_14:tt, $_15:tt,
        $_n:tt $(, $X:tt)*
    ) => {
        $_n
    };
}

macro_rules! COUNT_ARGS {
    () => {
        __COUNT_ARGS!(, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0)
    };
    ($($X:tt),+) => {
        __COUNT_ARGS!(, $($X,)* 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0)
    };
}

/* Concatenate two parameters, but allow them to be expanded beforehand. */
/*
 * C conditional macro definitions:
 * #ifndef __CONCAT
 * #ifndef CONCATENATE
 *
 * Rust macro_rules! has no direct file-local equivalent for testing whether a
 * macro name is already defined, so the guarded definitions are translated as
 * the macro definitions supplied by this header.
 */
macro_rules! __CONCAT {
    ($a:ident, $b:ident) => {
        concat_idents!($a, $b)
    };
}

macro_rules! CONCATENATE {
    ($a:ident, $b:ident) => {
        __CONCAT!($a, $b)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
