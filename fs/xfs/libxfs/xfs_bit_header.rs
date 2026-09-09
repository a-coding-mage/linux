/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000,2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/*
 * XFS bit manipulation routines.
 */

/* External bit-scan routines supplied by the surrounding environment. */
unsafe extern "C" {
    fn fls(v: u32) -> i32;
    fn fls64(v: u64) -> i32;
    fn ffs(v: u32) -> i32;
}

/*
 * masks with n high/low bits set, 64-bit values
 */
#[inline]
pub unsafe fn xfs_mask64hi(n: i32) -> u64 {
    (!0u64).wrapping_shl((64i32.wrapping_sub(n)) as u32)
}

#[inline]
pub unsafe fn xfs_mask32lo(n: i32) -> u32 {
    (1u32.wrapping_shl(n as u32)).wrapping_sub(1)
}

#[inline]
pub unsafe fn xfs_mask64lo(n: i32) -> u64 {
    (1u64.wrapping_shl(n as u32)).wrapping_sub(1)
}

/* Get high bit set out of 32-bit argument, -1 if none set */
#[inline]
pub unsafe fn xfs_highbit32(v: u32) -> i32 {
    fls(v) - 1
}

/* Get high bit set out of 64-bit argument, -1 if none set */
#[inline]
pub unsafe fn xfs_highbit64(v: u64) -> i32 {
    fls64(v) - 1
}

/* Get low bit set out of 32-bit argument, -1 if none set */
#[inline]
pub unsafe fn xfs_lowbit32(v: u32) -> i32 {
    ffs(v) - 1
}

/* Get low bit set out of 64-bit argument, -1 if none set */
#[inline]
pub unsafe fn xfs_lowbit64(v: u64) -> i32 {
    let mut w = v as u32;
    let mut n: i32 = 0;

    if w != 0 {
        /* lower bits */
        n = ffs(w);
    } else {
        /* upper bits */
        w = (v >> 32) as u32;
        if w != 0 {
            n = ffs(w);
            if n != 0 {
                n += 32;
            }
        }
    }
    n - 1
}

/* Return whether bitmap is empty (1 == empty) */
unsafe extern "C" {
    pub fn xfs_bitmap_empty(map: *mut usize, size: usize) -> i32;

    /* Count continuous one bits in map starting with start_bit */
    pub fn xfs_contig_bits(map: *mut usize, size: usize, start_bit: usize) -> i32;

    /* Find next set bit in map */
    pub fn xfs_next_bit(map: *mut usize, size: usize, start_bit: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
