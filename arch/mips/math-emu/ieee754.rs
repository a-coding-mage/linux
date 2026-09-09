// SPDX-License-Identifier: GPL-2.0-only
/* ieee754 floating point arithmetic
 * single and double precision
 *
 * BUGS
 * not much dp done
 * doesn't generate IEEE754_INEXACT
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependencies supplied by the corresponding IEEE754 headers and Linux build.

/*
 * Special constants
 */

#[inline]
const fn dp_cnst(sign: u64, bexp: u64, mant: u64) -> ieee754dp {
    // The C initializer addresses the IEEE754 bit-fields; preserve the exact
    // resulting representation in the union's integer view.
    unsafe { core::mem::transmute::<u64, ieee754dp>((sign << 63) | (bexp << 52) | mant) }
}

#[inline]
const fn sp_cnst(sign: u32, bexp: u32, mant: u32) -> ieee754sp {
    unsafe { core::mem::transmute::<u32, ieee754sp>((sign << 31) | (bexp << 23) | mant) }
}

pub static __ieee754dp_spcvals: [ieee754dp; 18] = [
    dp_cnst(0, (DP_EMIN - 1) as u64 + DP_EBIAS as u64, 0x0000000000000), // + zero
    dp_cnst(1, (DP_EMIN - 1) as u64 + DP_EBIAS as u64, 0x0000000000000), // - zero
    dp_cnst(0, DP_EBIAS as u64, 0x0000000000000), // + 1.0
    dp_cnst(1, DP_EBIAS as u64, 0x0000000000000), // - 1.0
    dp_cnst(0, (3 + DP_EBIAS) as u64, 0x4000000000000), // + 10.0
    dp_cnst(1, (3 + DP_EBIAS) as u64, 0x4000000000000), // - 10.0
    dp_cnst(0, (DP_EMAX + 1 + DP_EBIAS) as u64, 0), // + infinity
    dp_cnst(1, (DP_EMAX + 1 + DP_EBIAS) as u64, 0), // - infinity
    dp_cnst(0, (DP_EMAX + 1 + DP_EBIAS) as u64, 0x7FFFFFFFFFFFF), // + ind legacy qNaN
    dp_cnst(0, (DP_EMAX + 1 + DP_EBIAS) as u64, 0x8000000000000), // + indef 2008 qNaN
    dp_cnst(0, (DP_EMAX + DP_EBIAS) as u64, 0xFFFFFFFFFFFFF), // + max
    dp_cnst(1, (DP_EMAX + DP_EBIAS) as u64, 0xFFFFFFFFFFFFF), // - max
    dp_cnst(0, (DP_EMIN + DP_EBIAS) as u64, 0), // + min normal
    dp_cnst(1, (DP_EMIN + DP_EBIAS) as u64, 0), // - min normal
    dp_cnst(0, (DP_EMIN - 1 + DP_EBIAS) as u64, 1), // + min denormal
    dp_cnst(1, (DP_EMIN - 1 + DP_EBIAS) as u64, 1), // - min denormal
    dp_cnst(0, (31 + DP_EBIAS) as u64, 0), // + 1.0e31
    dp_cnst(0, (63 + DP_EBIAS) as u64, 0), // + 1.0e63
];

pub static __ieee754sp_spcvals: [ieee754sp; 18] = [
    sp_cnst(0, (SP_EMIN - 1 + SP_EBIAS) as u32, 0x000000), // + zero
    sp_cnst(1, (SP_EMIN - 1 + SP_EBIAS) as u32, 0x000000), // - zero
    sp_cnst(0, SP_EBIAS as u32, 0x000000), // + 1.0
    sp_cnst(1, SP_EBIAS as u32, 0x000000), // - 1.0
    sp_cnst(0, (3 + SP_EBIAS) as u32, 0x200000), // + 10.0
    sp_cnst(1, (3 + SP_EBIAS) as u32, 0x200000), // - 10.0
    sp_cnst(0, (SP_EMAX + 1 + SP_EBIAS) as u32, 0), // + infinity
    sp_cnst(1, (SP_EMAX + 1 + SP_EBIAS) as u32, 0), // - infinity
    sp_cnst(0, (SP_EMAX + 1 + SP_EBIAS) as u32, 0x3FFFFF), // + indef legacy quiet NaN
    sp_cnst(0, (SP_EMAX + 1 + SP_EBIAS) as u32, 0x400000), // + indef 2008 quiet NaN
    sp_cnst(0, (SP_EMAX + SP_EBIAS) as u32, 0x7FFFFF), // + max normal
    sp_cnst(1, (SP_EMAX + SP_EBIAS) as u32, 0x7FFFFF), // - max normal
    sp_cnst(0, (SP_EMIN + SP_EBIAS) as u32, 0), // + min normal
    sp_cnst(1, (SP_EMIN + SP_EBIAS) as u32, 0), // - min normal
    sp_cnst(0, (SP_EMIN - 1 + SP_EBIAS) as u32, 1), // + min denormal
    sp_cnst(1, (SP_EMIN - 1 + SP_EBIAS) as u32, 1), // - min denormal
    sp_cnst(0, (31 + SP_EBIAS) as u32, 0), // + 1.0e31
    sp_cnst(0, (63 + SP_EBIAS) as u32, 0), // + 1.0e63
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
