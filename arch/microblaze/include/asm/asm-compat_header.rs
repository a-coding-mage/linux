/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding MicroBlaze environment:
// #include <asm/types.h>

// The C header provides different macro definitions when assembled by the
// assembler. Rust has no direct equivalent of __ASSEMBLER__; preserve that
// build-time condition in the macro definitions below.

#[cfg(not(__ASSEMBLER__))]
#[macro_export]
macro_rules! __stringify_in_c {
    ($($tokens:tt)*) => {
        stringify!($($tokens)*)
    };
}

#[cfg(not(__ASSEMBLER__))]
#[macro_export]
macro_rules! stringify_in_c {
    ($($tokens:tt)*) => {
        concat!(stringify!($($tokens)*), " ")
    };
}

#[cfg(not(__ASSEMBLER__))]
#[macro_export]
macro_rules! __ASM_CONST {
    ($x:expr) => {
        ($x as u64)
    };
}

#[cfg(not(__ASSEMBLER__))]
#[macro_export]
macro_rules! ASM_CONST {
    ($x:expr) => {
        __ASM_CONST!($x)
    };
}

// Under __ASSEMBLER__, the original definitions are:
//   stringify_in_c(...) = __VA_ARGS__
//   ASM_CONST(x) = x
// Keep these definitions available to assembler-oriented consumers as
// documentation of the source-level conditional interface.
#[cfg(__ASSEMBLER__)]
#[macro_export]
macro_rules! stringify_in_c {
    ($($tokens:tt)*) => { $($tokens)* };
}

#[cfg(__ASSEMBLER__)]
#[macro_export]
macro_rules! ASM_CONST {
    ($x:expr) => { $x };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
