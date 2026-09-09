/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/asm-arm/unified.h - Unified Assembler Syntax helper macros
 *
 * Copyright (C) 2008 ARM Limited
 */

/* The C header selects these definitions at build time.  Rust feature
 * `cpu_v7m` and `thumb2_kernel` preserve those conditions. */

#[cfg(feature = "cpu_v7m")]
macro_rules! AR_CLASS {
    ($($x:tt)*) => { };
}

#[cfg(not(feature = "cpu_v7m"))]
macro_rules! AR_CLASS {
    ($($x:tt)*) => { $($x)* };
}

#[cfg(feature = "cpu_v7m")]
macro_rules! M_CLASS {
    ($($x:tt)*) => { $($x)* };
}

#[cfg(not(feature = "cpu_v7m"))]
macro_rules! M_CLASS {
    ($($x:tt)*) => { };
}

#[cfg(feature = "thumb2_kernel")]
pub const PSR_ISETSTATE: u32 = PSR_T_BIT;

#[cfg(not(feature = "thumb2_kernel"))]
pub const PSR_ISETSTATE: u32 = 0;

#[cfg(feature = "thumb2_kernel")]
macro_rules! ARM {
    ($($x:tt)*) => { };
}

#[cfg(not(feature = "thumb2_kernel"))]
macro_rules! ARM {
    ($($x:tt)*) => { $($x)* };
}

#[cfg(feature = "thumb2_kernel")]
macro_rules! THUMB {
    ($($x:tt)*) => { $($x)* };
}

#[cfg(not(feature = "thumb2_kernel"))]
macro_rules! THUMB {
    ($($x:tt)*) => { };
}

/* In the assembly build, W(instr) appends the Thumb width suffix. */
#[cfg(feature = "thumb2_kernel")]
macro_rules! W {
    ($instr:ident) => { $instr.w };
}

#[cfg(not(feature = "thumb2_kernel"))]
macro_rules! W {
    ($instr:ident) => { $instr };
}

/* In the non-assembly build, WASM(instr) stringizes the instruction. */
#[cfg(feature = "thumb2_kernel")]
macro_rules! WASM {
    ($instr:ident) => { concat!(stringify!($instr), ".w") };
}

#[cfg(not(feature = "thumb2_kernel"))]
macro_rules! WASM {
    ($instr:ident) => { stringify!($instr) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
