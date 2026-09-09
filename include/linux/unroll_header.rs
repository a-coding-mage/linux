/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2023 Google LLC.
 */

// Dependency intent: the original header includes <linux/args.h>.

// The original compiler-specific _Pragma selection is preserved as intent.
// Rust has no direct equivalent for compiler loop-unroll pragmas.

/**
 * unrolled - loop attributes to ask the compiler to unroll it
 *
 * Usage:
 *
 * #define BATCH 8
 *
 *  unrolled_count(BATCH)
 *  for (u32 i = 0; i < BATCH; i++)
 *      // loop body without cross-iteration dependencies
 *
 * This is only a hint and the compiler is free to disable unrolling if it
 * thinks the count is suboptimal and may hurt performance and/or hugely
 * increase object code size.
 * Not having any cross-iteration dependencies (i.e. when iter x + 1 depends
 * on what iter x will do with variables) is not a strict requirement, but
 * provides best performance and object code size.
 * Available only on Clang and GCC 8.x onwards.
 */

/// Ask the compiler to pick an optimal unroll count, Clang only.
#[macro_export]
macro_rules! unrolled {
    () => {};
}

/// Unroll each `n` iterations of the loop.
#[macro_export]
macro_rules! unrolled_count {
    ($n:expr) => {};
}

/// Unroll the whole loop.
#[macro_export]
macro_rules! unrolled_full {
    () => {};
}

/// Never unroll the loop.
#[macro_export]
macro_rules! unrolled_none {
    () => {};
}

// The following macro_rules! definitions preserve the recursive expansion
// order and callback arguments of the original C preprocessor macros.
#[macro_export]
macro_rules! __UNROLL_0 {
    ($macro:ident $(, $args:tt)*) => {};
}

#[macro_export]
macro_rules! __UNROLL_1 {
    ($macro:ident $(, $args:tt)*) => {
        $crate::__UNROLL_0!($macro $(, $args)*);
        $macro!(0 $(, $args)*);
    };
}

#[macro_export]
macro_rules! __UNROLL_2 {
    ($macro:ident $(, $args:tt)*) => {
        $crate::__UNROLL_1!($macro $(, $args)*); $macro!(1 $(, $args)*);
    };
}

#[macro_export]
macro_rules! __UNROLL_3 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_2!($macro $(, $args)*); $macro!(2 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_4 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_3!($macro $(, $args)*); $macro!(3 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_5 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_4!($macro $(, $args)*); $macro!(4 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_6 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_5!($macro $(, $args)*); $macro!(5 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_7 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_6!($macro $(, $args)*); $macro!(6 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_8 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_7!($macro $(, $args)*); $macro!(7 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_9 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_8!($macro $(, $args)*); $macro!(8 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_10 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_9!($macro $(, $args)*); $macro!(9 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_11 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_10!($macro $(, $args)*); $macro!(10 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_12 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_11!($macro $(, $args)*); $macro!(11 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_13 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_12!($macro $(, $args)*); $macro!(12 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_14 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_13!($macro $(, $args)*); $macro!(13 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_15 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_14!($macro $(, $args)*); $macro!(14 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_16 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_15!($macro $(, $args)*); $macro!(15 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_17 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_16!($macro $(, $args)*); $macro!(16 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_18 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_17!($macro $(, $args)*); $macro!(17 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_19 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_18!($macro $(, $args)*); $macro!(18 $(, $args)*); }; }
#[macro_export]
macro_rules! __UNROLL_20 { ($macro:ident $(, $args:tt)*) => { $crate::__UNROLL_19!($macro $(, $args)*); $macro!(19 $(, $args)*); }; }

#[macro_export]
macro_rules! UNROLL {
    ($n:tt, $macro:ident $(, $args:tt)*) => {
        $crate::paste_unroll_dispatch!($n, $macro $(, $args)*);
    };
}

// Token concatenation used by the C macro is not available in stable
// macro_rules! without an external helper; retain the dispatch intent here.
#[macro_export]
macro_rules! paste_unroll_dispatch {
    ($n:tt, $macro:ident $(, $args:tt)*) => {
        compile_error!("UNROLL dispatch requires token concatenation support");
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
