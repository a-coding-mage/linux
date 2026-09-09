/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Regents of the University of California
 */

//! Rust translation of the RISC-V assembly header.
//!
//! The original header is consumed by the assembler and C preprocessor.  The
//! assembly-only macros are retained below as comments because Rust has no
//! direct item equivalent for assembler `.macro` definitions.

#[macro_export]
macro_rules! __ASM_STR {
    ($x:ident) => { stringify!($x) };
    ($x:literal) => { $x };
}

#[cfg(feature = "config-as-has-insn")]
#[macro_export]
macro_rules! ASM_INSN_I {
    ($x:expr) => { concat!(".insn ", $x) };
}

#[cfg(not(feature = "config-as-has-insn"))]
#[macro_export]
macro_rules! ASM_INSN_I {
    ($x:expr) => { concat!(".4byte ", $x) };
}

#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! __REG_SEL {
    ($a:expr, $b:expr) => { $a };
}

#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! __REG_SEL {
    ($a:expr, $b:expr) => { $b };
}

#[cfg(target_pointer_width = "64")]
pub const REG_L: &str = "ld";
#[cfg(target_pointer_width = "32")]
pub const REG_L: &str = "lw";
#[cfg(target_pointer_width = "64")]
pub const REG_S: &str = "sd";
#[cfg(target_pointer_width = "32")]
pub const REG_S: &str = "sw";
#[cfg(target_pointer_width = "64")]
pub const REG_SC: &str = "sc.d";
#[cfg(target_pointer_width = "32")]
pub const REG_SC: &str = "sc.w";
#[cfg(target_pointer_width = "64")]
pub const REG_AMOSWAP_AQ: &str = "amoswap.d.aq";
#[cfg(target_pointer_width = "32")]
pub const REG_AMOSWAP_AQ: &str = "amoswap.w.aq";
#[cfg(target_pointer_width = "64")]
pub const REG_ASM: &str = ".dword";
#[cfg(target_pointer_width = "32")]
pub const REG_ASM: &str = ".word";
#[cfg(target_pointer_width = "64")]
pub const SZREG: usize = 8;
#[cfg(target_pointer_width = "32")]
pub const SZREG: usize = 4;
#[cfg(target_pointer_width = "64")]
pub const LGREG: usize = 3;
#[cfg(target_pointer_width = "32")]
pub const LGREG: usize = 2;
#[cfg(target_pointer_width = "64")]
pub const SRLI: &str = "srliw";
#[cfg(target_pointer_width = "32")]
pub const SRLI: &str = "srli";
#[cfg(target_pointer_width = "64")]
pub const SLLI: &str = "slliw";
#[cfg(target_pointer_width = "32")]
pub const SLLI: &str = "slli";

#[cfg(target_pointer_width = "64")]
pub const RISCV_PTR: &str = ".dword";
#[cfg(target_pointer_width = "32")]
pub const RISCV_PTR: &str = ".word";
#[cfg(target_pointer_width = "64")]
pub const RISCV_SZPTR: usize = 8;
#[cfg(target_pointer_width = "32")]
pub const RISCV_SZPTR: usize = 4;
#[cfg(target_pointer_width = "64")]
pub const RISCV_LGPTR: usize = 3;
#[cfg(target_pointer_width = "32")]
pub const RISCV_LGPTR: usize = 2;

pub const RISCV_INT: &str = ".word";
pub const RISCV_SZINT: usize = 4;
pub const RISCV_LGINT: usize = 2;
pub const RISCV_SHORT: &str = ".half";
pub const RISCV_SZSHORT: usize = 2;
pub const RISCV_LGSHORT: usize = 1;

/* Common assembly source macros (the original `__ASSEMBLER__` branch). */
/*
.macro nops, num
    .rept \num
    nop
    .endr
.endm

.macro asm_per_cpu dst sym tmp       /* CONFIG_SMP */
    lw    \tmp, TASK_TI_CPU_NUM(tp)
    slli  \tmp, \tmp, RISCV_LGPTR
    la    \dst, __per_cpu_offset
    add   \dst, \dst, \tmp
    REG_L \tmp, 0(\dst)
    la    \dst, \sym
    add   \dst, \dst, \tmp
.endm

.macro asm_per_cpu dst sym tmp       /* !CONFIG_SMP */
    la    \dst, \sym
.endm

.macro load_per_cpu dst ptr tmp
    asm_per_cpu \dst \ptr \tmp
    REG_L \dst, 0(\dst)
.endm

.macro load_global_pointer             /* !CONFIG_SHADOW_CALL_STACK */
.option push
.option norelax
    la gp, __global_pointer$
.option pop
.endm

.macro load_global_pointer             /* CONFIG_SHADOW_CALL_STACK: empty */
.endm

.macro save_from_x6_to_x31
    REG_S x6, PT_T1(sp); REG_S x7, PT_T2(sp); REG_S x8, PT_S0(sp)
    REG_S x9, PT_S1(sp); REG_S x10, PT_A0(sp); REG_S x11, PT_A1(sp)
    REG_S x12, PT_A2(sp); REG_S x13, PT_A3(sp); REG_S x14, PT_A4(sp)
    REG_S x15, PT_A5(sp); REG_S x16, PT_A6(sp); REG_S x17, PT_A7(sp)
    REG_S x18, PT_S2(sp); REG_S x19, PT_S3(sp); REG_S x20, PT_S4(sp)
    REG_S x21, PT_S5(sp); REG_S x22, PT_S6(sp); REG_S x23, PT_S7(sp)
    REG_S x24, PT_S8(sp); REG_S x25, PT_S9(sp); REG_S x26, PT_S10(sp)
    REG_S x27, PT_S11(sp); REG_S x28, PT_T3(sp); REG_S x29, PT_T4(sp)
    REG_S x30, PT_T5(sp); REG_S x31, PT_T6(sp)
.endm

.macro restore_from_x6_to_x31
    REG_L x6, PT_T1(sp); REG_L x7, PT_T2(sp); REG_L x8, PT_S0(sp)
    REG_L x9, PT_S1(sp); REG_L x10, PT_A0(sp); REG_L x11, PT_A1(sp)
    REG_L x12, PT_A2(sp); REG_L x13, PT_A3(sp); REG_L x14, PT_A4(sp)
    REG_L x15, PT_A5(sp); REG_L x16, PT_A6(sp); REG_L x17, PT_A7(sp)
    REG_L x18, PT_S2(sp); REG_L x19, PT_S3(sp); REG_L x20, PT_S4(sp)
    REG_L x21, PT_S5(sp); REG_L x22, PT_S6(sp); REG_L x23, PT_S7(sp)
    REG_L x24, PT_S8(sp); REG_L x25, PT_S9(sp); REG_L x26, PT_S10(sp)
    REG_L x27, PT_S11(sp); REG_L x28, PT_T3(sp); REG_L x29, PT_T4(sp)
    REG_L x30, PT_T5(sp); REG_L x31, PT_T6(sp)
.endm
*/

/* Annotate a function as being unsuitable for kprobes. */
#[cfg(feature = "config-kprobes")]
#[macro_export]
macro_rules! ASM_NOKPROBE {
    ($name:expr) => { $name };
}

#[cfg(not(feature = "config-kprobes"))]
#[macro_export]
macro_rules! ASM_NOKPROBE {
    ($name:expr) => { () };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
