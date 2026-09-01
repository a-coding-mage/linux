/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2009 Freescale Semiconductor, Inc.
 *
 * provides masks and opcode images for use by code generation, emulation
 * and for instructions that older assemblers might not know about
 */

// C header guard omitted.

// C preprocessor helpers:
// #define stringify_in_c(...) __VA_ARGS__
// #define ASM_CONST(x) x

pub const PPC_INST_VCMPEQUD_RC: u32 = 0x100000c7;
pub const PPC_INST_VCMPEQUB_RC: u32 = 0x10000006;

pub const __PPC_RC21: u32 = 0x1 << 10;

/* macros to insert fields into opcodes */
#[inline]
pub const fn ___PPC_RA(a: u32) -> u32 {
    (a & 0x1f) << 16
}

#[inline]
pub const fn ___PPC_RB(b: u32) -> u32 {
    (b & 0x1f) << 11
}

#[inline]
pub const fn ___PPC_RS(s: u32) -> u32 {
    (s & 0x1f) << 21
}

#[inline]
pub const fn ___PPC_RT(t: u32) -> u32 {
    ___PPC_RS(t)
}

// C macro emitted an assembler directive through stringify_in_c:
// .long PPC_INST_VCMPEQUD_RC | ___PPC_RT(vrt) | ___PPC_RA(vra) |
//       ___PPC_RB(vrb) | __PPC_RC21
#[inline]
pub const fn VCMPEQUD_RC(vrt: u32, vra: u32, vrb: u32) -> u32 {
    PPC_INST_VCMPEQUD_RC | ___PPC_RT(vrt) | ___PPC_RA(vra) | ___PPC_RB(vrb) | __PPC_RC21
}

// C macro emitted an assembler directive through stringify_in_c:
// .long PPC_INST_VCMPEQUB_RC | ___PPC_RT(vrt) | ___PPC_RA(vra) |
//       ___PPC_RB(vrb) | __PPC_RC21
#[inline]
pub const fn VCMPEQUB_RC(vrt: u32, vra: u32, vrb: u32) -> u32 {
    PPC_INST_VCMPEQUB_RC | ___PPC_RT(vrt) | ___PPC_RA(vra) | ___PPC_RB(vrb) | __PPC_RC21
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
