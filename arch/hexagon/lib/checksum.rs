// SPDX-License-Identifier: GPL-2.0-only
/*
 * Checksum functions for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* This was derived from arch/alpha/lib/checksum.c */

/* The following declarations are supplied by the Hexagon intrinsic dependencies. */
unsafe extern "C" {
    fn HEXAGON_P_vrmpyh_PP(x: u64, y: u64) -> u64;
    fn HEXAGON_P_vrmpyhacc_PP(x: u64, y: u64, z: u64) -> u64;
    fn HEXAGON_R_cl0_R(x: i32) -> i32;
}

/* Vector value operations */
const fn sign(x: u64, y: u32) -> u64 { (0x8000u64 * x) << y }
const fn carry(x: u64, y: u32) -> u64 { (0x0002u64 * x) << y }
const fn select(x: u64, y: u32) -> u64 { (0x0001u64 * x) << y }

const fn vr_negate(a: u64, b: u64, c: u64, d: u64) -> u64 {
    sign(a, 48) + sign(b, 32) + sign(c, 16) + sign(d, 0)
}
const fn vr_carry(a: u64, b: u64, c: u64, d: u64) -> u64 {
    carry(a, 48) + carry(b, 32) + carry(c, 16) + carry(d, 0)
}
const fn vr_select(a: u64, b: u64, c: u64, d: u64) -> u64 {
    select(a, 48) + select(b, 32) + select(c, 16) + select(d, 0)
}

/* optimized HEXAGON V3 intrinsic version */
unsafe fn from64to16(x: u64) -> u16 {
    let mut sum: u64;

    sum = HEXAGON_P_vrmpyh_PP(x ^ vr_negate(1, 1, 1, 1), vr_select(1, 1, 1, 1));
    sum += vr_carry(0, 0, 1, 0);
    sum = HEXAGON_P_vrmpyh_PP(sum, vr_select(0, 0, 1, 1));

    (0xffffu64 & sum) as u16
}

/*
 * computes the checksum of the TCP/UDP pseudo-header
 * returns a 16-bit checksum, already complemented.
 */
pub unsafe fn csum_tcpudp_magic(saddr: __be32, daddr: __be32,
                                len: __u32, proto: __u8, sum: __wsum) -> __sum16 {
    (!from64to16((saddr as u64) + (daddr as u64) + (sum as u64)
        + (((len + proto as __u32) as u64) << 8))) as __sum16
}

pub unsafe fn csum_tcpudp_nofold(saddr: __be32, daddr: __be32,
                                 len: __u32, proto: __u8, sum: __wsum) -> __wsum {
    let mut result: u64;

    result = (saddr as u64) + (daddr as u64) + (sum as u64)
        + (((len + proto as __u32) as u64) << 8);

    /* Fold down to 32-bits so we don't lose in the typedef-less
       network stack. */
    /* 64 to 33 */
    result = (result & 0xffffffffu64) + (result >> 32);
    /* 33 to 32 */
    result = (result & 0xffffffffu64) + (result >> 32);
    result as __wsum
}

/*
 * Do a 64-bit checksum on an arbitrary memory area..
 *
 * This isn't a great routine, but it's not _horrible_ either. The
 * inner loop could be unrolled a bit further, and there are better
 * ways to do the carry, but this is reasonable.
 */

/* optimized HEXAGON intrinsic version, with over read fixed */
pub unsafe fn do_csum(voidptr: *const core::ffi::c_void, len: i32) -> u32 {
    let mut sum0: u64;
    let mut sum1: u64;
    let mut x0: u64;
    let mut x1: u64;
    let mut ptr8_o: *const u64;
    let mut ptr8_e: *const u64;
    let mut ptr8: *const u64;
    let mut i: i32;
    let mut start: i32;
    let mut mid: i32;
    let mut end: i32;
    let mut mask: i32;
    let ptr = voidptr as *const i8;
    let mut ptr2: *const u16;
    let mut ptr4: *const u32;

    if len <= 0 { return 0; }

    start = 0xf & (16 - ((ptr as usize as i32) & 0xf));
    mask = 0x7fffffff_i32 >> HEXAGON_R_cl0_R(len);
    start &= mask;

    mid = len - start;
    end = mid & 0xf;
    mid >>= 4;
    sum0 = (mid as u64) << 18;
    sum1 = 0;

    if start & 1 != 0 { sum0 += ((*ptr.add(0) as i32) << 8) as u64; }
    ptr2 = ptr.add((start & 1) as usize) as *const u16;
    if start & 2 != 0 { sum1 += *ptr2 as u64; }
    ptr4 = ptr.add((start & 3) as usize) as *const u32;
    if start & 4 != 0 {
        sum0 = HEXAGON_P_vrmpyhacc_PP(sum0, vr_negate(0, 0, 1, 1) ^ *ptr4 as u64,
                                       vr_select(0, 0, 1, 1));
        sum0 += vr_select(0, 0, 1, 0);
    }
    ptr8 = ptr.add((start & 7) as usize) as *const u64;
    if start & 8 != 0 {
        sum1 = HEXAGON_P_vrmpyhacc_PP(sum1, vr_negate(1, 1, 1, 1) ^ *ptr8,
                                       vr_select(1, 1, 1, 1));
        sum1 += vr_carry(0, 0, 1, 0);
    }
    ptr8_o = ptr.add(start as usize) as *const u64;
    ptr8_e = ptr.add((start + 8) as usize) as *const u64;

    if mid != 0 {
        x0 = *ptr8_e; ptr8_e = ptr8_e.add(2);
        x1 = *ptr8_o; ptr8_o = ptr8_o.add(2);
        if mid > 1 {
            i = 0;
            while i < mid - 1 {
                sum0 = HEXAGON_P_vrmpyhacc_PP(sum0, x0 ^ vr_negate(1, 1, 1, 1), vr_select(1, 1, 1, 1));
                sum1 = HEXAGON_P_vrmpyhacc_PP(sum1, x1 ^ vr_negate(1, 1, 1, 1), vr_select(1, 1, 1, 1));
                x0 = *ptr8_e; ptr8_e = ptr8_e.add(2);
                x1 = *ptr8_o; ptr8_o = ptr8_o.add(2);
                i += 1;
            }
        }
        sum0 = HEXAGON_P_vrmpyhacc_PP(sum0, x0 ^ vr_negate(1, 1, 1, 1), vr_select(1, 1, 1, 1));
        sum1 = HEXAGON_P_vrmpyhacc_PP(sum1, x1 ^ vr_negate(1, 1, 1, 1), vr_select(1, 1, 1, 1));
    }

    ptr4 = ptr.add((start + mid * 16 + (end & 8)) as usize) as *const u32;
    if end & 4 != 0 {
        sum1 = HEXAGON_P_vrmpyhacc_PP(sum1, vr_negate(0, 0, 1, 1) ^ *ptr4 as u64, vr_select(0, 0, 1, 1));
        sum1 += vr_select(0, 0, 1, 0);
    }
    ptr2 = ptr.add((start + mid * 16 + (end & 12)) as usize) as *const u16;
    if end & 2 != 0 { sum0 += *ptr2 as u64; }
    if end & 1 != 0 { sum1 += *ptr.add((start + mid * 16 + (end & 14)) as usize) as u8 as u64; }
    ptr8 = ptr.add((start + mid * 16) as usize) as *const u64;
    if end & 8 != 0 {
        sum0 = HEXAGON_P_vrmpyhacc_PP(sum0, vr_negate(1, 1, 1, 1) ^ *ptr8, vr_select(1, 1, 1, 1));
        sum0 += vr_carry(0, 0, 1, 0);
    }
    sum0 = HEXAGON_P_vrmpyh_PP((sum0 + sum1) ^ vr_negate(0, 0, 0, 1), vr_select(0, 0, 1, 1));
    sum0 += vr_negate(0, 0, 0, 1);
    sum0 = HEXAGON_P_vrmpyh_PP(sum0, vr_select(0, 0, 1, 1));

    if start & 1 != 0 { sum0 = (sum0 << 8) | (0xff & (sum0 >> 8)); }
    (0xffff & sum0) as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
