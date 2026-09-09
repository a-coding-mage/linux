/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2019 Facebook */

/*
 * This will bring in asm_goto_output and asm_inline macro definitions
 * if enabled by compiler and config options.
 *
 * The Linux types header is an external dependency of the original header.
 */

/*
 * If asm_goto_output was provided by the compiler/configuration, the C
 * header undefines it and replaces it with an invalid asm expression.
 */
#[allow(unused_macros)]
macro_rules! asm_goto_output {
    ($($x:tt)*) => {
        compile_error!("invalid use of asm_goto_output")
    };
}

/*
 * asm_inline is defined as asm __inline in
 * "include/linux/compiler_types.h" if supported by the kernel's CC (i.e.
 * CONFIG_CC_HAS_ASM_INLINE), which is not supported by CLANG.  The C header
 * replaces it with asm when present; Rust's inline assembly is used here as
 * the corresponding construct.
 */
#[allow(unused_macros)]
macro_rules! asm_inline {
    ($($x:tt)*) => {
        core::arch::asm!($($x)*)
    };
}

/* C macro: #define volatile(x...) volatile("") */
#[allow(unused_macros)]
macro_rules! volatile {
    ($($x:tt)*) => {
        core::arch::asm!("")
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
