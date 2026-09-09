// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// The C source includes cnum_defs.h for T = 32 and T = 64.  Those
// declarations are represented here directly for this translation unit.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnum32 {
    pub base: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnum64 {
    pub base: u64,
    pub size: u64,
}

pub const U32_MAX: u64 = 0xffff_ffff;
pub const CNUM32_EMPTY: cnum32 = cnum32 { base: 0, size: 0 };
pub const CNUM64_EMPTY: cnum64 = cnum64 { base: 0, size: 0 };

unsafe extern "C" {
    fn cnum64_is_empty(cnum: cnum64) -> bool;
    fn cnum32_is_empty(cnum: cnum32) -> bool;
    fn cnum32_urange_overflow(cnum: cnum32) -> bool;
}

pub unsafe fn cnum32_from_cnum64(cnum: cnum64) -> cnum32 {
    if cnum64_is_empty(cnum) {
        return CNUM32_EMPTY;
    }

    if cnum.size >= U32_MAX {
        cnum32 { base: 0, size: U32_MAX as u32 }
    } else {
        cnum32 { base: cnum.base as u32, size: cnum.size as u32 }
    }
}

/*
 * Suppose 'a' and 'b' are laid out as follows:
 *
 *                                                          64-bit number axis --->
 *
 * N*2^32                   (N+1)*2^32                (N+2)*2^32                (N+3)*2^32
 * ||------|---|=====|-------||----------|=====|-------||----------|=====|----|--||
 *         |   |< b >|                   |< b >|                   |< b >|    |
 *         |   |                                                         |    |
 *         |<--+--------------------------- a ---------------------------+--->|
 *             |                                                         |
 *             |<-------------------------- t -------------------------->|
 *
 * In such a case it is possible to infer a more tight representation t
 * such that forall v in a, (u32)v in b: v in t.
 */
pub unsafe fn cnum64_cnum32_intersect(mut a: cnum64, b: cnum32) -> cnum64 {
    // Rotate the circles so that virtual a1 starts at a u32 boundary.
    let b1 = cnum32 {
        base: b.base.wrapping_sub(a.base as u32),
        size: b.size,
    };
    let mut t = a;
    let mut d: u64;
    let b1_max: u64;

    if cnum64_is_empty(a) || cnum32_is_empty(b) {
        return CNUM64_EMPTY;
    }

    if cnum32_urange_overflow(b1) {
        b1_max = (b1.base as u64).wrapping_add(b1.size as u64);
        if (a.size as u32 as u64) > b1_max && (a.size as u32 as u64) < b1.base as u64 {
            d = (a.size as u32 as u64).wrapping_sub(b1_max);
            t.size = t.size.wrapping_sub(d);
        }
    } else {
        if t.size < b1.base as u64 {
            return CNUM64_EMPTY;
        }
        t.base = t.base.wrapping_add(b1.base as u64);
        t.size = t.size.wrapping_sub(b1.base as u64);
        b1_max = b1.base as u64 + b1.size as u64;
        d = 0;
        if (a.size as u32 as u64) < b1.base as u64 {
            d = (a.size as u32 as u64).wrapping_add((1u64 << 32).wrapping_sub(b1_max));
        } else if (a.size as u32 as u64) >= b1_max {
            d = (a.size as u32 as u64).wrapping_sub(b1_max);
        }
        if t.size < d {
            return CNUM64_EMPTY;
        }
        t.size = t.size.wrapping_sub(d);
    }
    t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
