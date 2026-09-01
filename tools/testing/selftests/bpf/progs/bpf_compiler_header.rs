// SPDX-License-Identifier: GPL-2.0

// Original C header guard omitted in Rust.

// C macro:
//   #define DO_PRAGMA_(X) _Pragma(#X)
// Rust has no direct source-level equivalent for C's _Pragma operator.
macro_rules! DO_PRAGMA_ {
    ($($x:tt)*) => {};
}

// Original C condition:
//   #if __clang__
//     #define __pragma_loop_unroll DO_PRAGMA_(clang loop unroll(enable))
//   #else
//     /*
//      * In GCC -funroll-loops, which is enabled with -O2, should have the
//      * same impact than the loop-unroll-enable pragma above.
//      */
//     #define __pragma_loop_unroll
//   #endif
// Rust has no direct equivalent for these Clang/GCC loop-unroll pragmas.
macro_rules! __pragma_loop_unroll {
    () => {};
}

// Original C condition:
//   #if __clang__
//     #define __pragma_loop_unroll_count(N) DO_PRAGMA_(clang loop unroll_count(N))
//   #else
//     #define __pragma_loop_unroll_count(N) DO_PRAGMA_(GCC unroll N)
//   #endif
macro_rules! __pragma_loop_unroll_count {
    ($N:expr) => {};
}

// Original C condition:
//   #if __clang__
//     #define __pragma_loop_unroll_full DO_PRAGMA_(clang loop unroll(full))
//   #else
//     #define __pragma_loop_unroll_full DO_PRAGMA_(GCC unroll 65534)
//   #endif
macro_rules! __pragma_loop_unroll_full {
    () => {};
}

// Original C condition:
//   #if __clang__
//     #define __pragma_loop_no_unroll DO_PRAGMA_(clang loop unroll(disable))
//   #else
//     #define __pragma_loop_no_unroll DO_PRAGMA_(GCC unroll 1)
//   #endif
macro_rules! __pragma_loop_no_unroll {
    () => {};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
