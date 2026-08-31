/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of testing/selftests/powerpc/copyloops/asm/ppc_asm.h.
 *
 * The original header includes <ppc-asm.h>.  Symbols such as FUNC_START,
 * r14-r31, and assembler directives/instructions are expected to be supplied
 * by the surrounding assembly environment in the original source.
 */

pub const CONFIG_ALTIVEC: bool = true;

pub const r1: i32 = 1;

macro_rules! R14 {
    () => {
        r14
    };
}
macro_rules! R15 {
    () => {
        r15
    };
}
macro_rules! R16 {
    () => {
        r16
    };
}
macro_rules! R17 {
    () => {
        r17
    };
}
macro_rules! R18 {
    () => {
        r18
    };
}
macro_rules! R19 {
    () => {
        r19
    };
}
macro_rules! R20 {
    () => {
        r20
    };
}
macro_rules! R21 {
    () => {
        r21
    };
}
macro_rules! R22 {
    () => {
        r22
    };
}
macro_rules! R29 {
    () => {
        r29
    };
}
macro_rules! R30 {
    () => {
        r30
    };
}
macro_rules! R31 {
    () => {
        r31
    };
}

pub const STACKFRAMESIZE: i32 = 256;

macro_rules! STK_REG {
    ($i:expr) => {
        (112 + (($i) - 14) * 8)
    };
}

macro_rules! _GLOBAL {
    ($A:ident) => {
        FUNC_START!(test_ ## $A)
    };
}

macro_rules! _GLOBAL_TOC {
    ($A:ident) => {
        _GLOBAL!($A)
    };
}

macro_rules! _GLOBAL_TOC_KASAN {
    ($A:ident) => {
        _GLOBAL!($A)
    };
}

macro_rules! _GLOBAL_KASAN {
    ($A:ident) => {
        _GLOBAL!($A)
    };
}

macro_rules! CFUNC {
    ($name:ident) => {
        $name
    };
}

macro_rules! PPC_MTOCRF {
    ($A:tt, $B:tt) => {
        mtocrf $A, $B
    };
}

macro_rules! EX_TABLE {
    ($x:tt, $y:tt) => {
        .section __ex_table, "a";
        .8byte $x, $y;
        .previous
    };
}

macro_rules! BEGIN_FTR_SECTION {
    () => {
        .if test_feature
    };
}

macro_rules! FTR_SECTION_ELSE {
    () => {
        .else
    };
}

macro_rules! ALT_FTR_SECTION_END_IFCLR {
    ($x:tt) => {
        .endif
    };
}

macro_rules! ALT_FTR_SECTION_END_IFSET {
    ($x:tt) => {
        .endif
    };
}

macro_rules! ALT_FTR_SECTION_END {
    ($x:tt, $y:tt) => {
        .endif
    };
}

macro_rules! END_FTR_SECTION_IFCLR {
    ($x:tt) => {
        .endif
    };
}

macro_rules! END_FTR_SECTION_IFSET {
    ($x:tt) => {
        .endif
    };
}

/* Default to taking the first of any alternative feature sections */
pub const test_feature: i32 = 1;

macro_rules! DCBT_SETUP_STREAMS {
    ($from:tt, $from_parms:tt, $to:tt, $to_parms:tt, $scratch:tt) => {
        lis $scratch, 0x8000;
        /* GO=1 */
        clrldi $scratch, $scratch, 32;
        /* setup read stream 0 */
        dcbt 0, $from, 0b01000;
        /* addr from */
        dcbt 0, $from_parms, 0b01010;
        /* length and depth from */
        /* setup write stream 1 */
        dcbtst 0, $to, 0b01000;
        /* addr to */
        dcbtst 0, $to_parms, 0b01010;
        /* length and depth to */
        eieio;
        dcbt 0, $scratch, 0b01010;
        /* all streams GO */
    };
}

