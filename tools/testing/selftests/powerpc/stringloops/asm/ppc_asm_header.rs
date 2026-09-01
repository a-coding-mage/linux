/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the C header ppc_asm.h. */
/* C include dependency preserved for build integration: <ppc-asm.h>. */

#[macro_export]
macro_rules! r1 {
    () => {
        sp
    };
}

#[macro_export]
macro_rules! _GLOBAL {
    ($A:ident) => {
        FUNC_START!(test_ $A)
    };
}

#[macro_export]
macro_rules! _GLOBAL_TOC {
    ($A:ident) => {
        FUNC_START!(test_ $A)
    };
}

#[macro_export]
macro_rules! CFUNC {
    ($name:ident) => {
        $name
    };
}

pub const CONFIG_ALTIVEC: bool = true;

#[macro_export]
macro_rules! R14 {
    () => {
        r14
    };
}

#[macro_export]
macro_rules! R15 {
    () => {
        r15
    };
}

#[macro_export]
macro_rules! R16 {
    () => {
        r16
    };
}

#[macro_export]
macro_rules! R17 {
    () => {
        r17
    };
}

#[macro_export]
macro_rules! R18 {
    () => {
        r18
    };
}

#[macro_export]
macro_rules! R19 {
    () => {
        r19
    };
}

#[macro_export]
macro_rules! R20 {
    () => {
        r20
    };
}

#[macro_export]
macro_rules! R21 {
    () => {
        r21
    };
}

#[macro_export]
macro_rules! R22 {
    () => {
        r22
    };
}

#[macro_export]
macro_rules! R29 {
    () => {
        r29
    };
}

#[macro_export]
macro_rules! R30 {
    () => {
        r30
    };
}

#[macro_export]
macro_rules! R31 {
    () => {
        r31
    };
}

pub const STACKFRAMESIZE: usize = 256;

#[macro_export]
macro_rules! STK_REG {
    ($i:expr) => {
        (112 + (($i) - 14) * 8)
    };
}

#[macro_export]
macro_rules! BEGIN_FTR_SECTION {
    () => {};
}

#[macro_export]
macro_rules! END_FTR_SECTION_IFSET {
    ($val:expr) => {};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
