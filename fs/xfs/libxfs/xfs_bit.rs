// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS translation.

/*
 * XFS bit manipulation routines, used in non-realtime code.
 */

/*
 * Return whether bitmap is empty.
 * Size is number of words in the bitmap, which is padded to word boundary
 * Returns 1 for empty, 0 for non-empty.
 */
pub unsafe fn xfs_bitmap_empty(map: *mut u32, size: u32) -> i32 {
    let mut i: u32;

    i = 0;
    while i < size {
        if *map.add(i as usize) != 0 {
            return 0;
        }
        i += 1;
    }

    1
}

/*
 * Count the number of contiguous bits set in the bitmap starting with bit
 * start_bit.  Size is the size of the bitmap in words.
 */
pub unsafe fn xfs_contig_bits(map: *mut u32, mut size: u32, mut start_bit: u32) -> i32 {
    let mut p = map.add((start_bit >> BIT_TO_WORD_SHIFT) as usize);
    let mut result: u32 = 0;
    let mut tmp: u32;

    size <<= BIT_TO_WORD_SHIFT;

    debug_assert!(start_bit < size);
    size -= start_bit & !(NBWORD - 1);
    start_bit &= NBWORD - 1;
    if start_bit != 0 {
        tmp = *p;
        p = p.add(1);
        /* set to one first offset bits prior to start */
        tmp |= !0u32 >> (NBWORD - start_bit);
        if tmp != !0u32 {
            return (result + ffz(tmp) - start_bit) as i32;
        }
        result += NBWORD;
        size -= NBWORD;
    }
    while size != 0 {
        tmp = *p;
        p = p.add(1);
        if tmp != !0u32 {
            return (result + ffz(tmp) - start_bit) as i32;
        }
        result += NBWORD;
        size -= NBWORD;
    }
    (result - start_bit) as i32
}

/*
 * This takes the bit number to start looking from and
 * returns the next set bit from there.  It returns -1
 * if there are no more bits set or the start bit is
 * beyond the end of the bitmap.
 *
 * Size is the number of words, not bytes, in the bitmap.
 */
pub unsafe fn xfs_next_bit(map: *mut u32, mut size: u32, start_bit: u32) -> i32 {
    let mut p = map.add((start_bit >> BIT_TO_WORD_SHIFT) as usize);
    let mut result = start_bit & !(NBWORD - 1);
    let mut tmp: u32;

    size <<= BIT_TO_WORD_SHIFT;

    if start_bit >= size {
        return -1;
    }
    size -= result;
    let start_bit = start_bit & (NBWORD - 1);
    if start_bit != 0 {
        tmp = *p;
        p = p.add(1);
        /* set to zero first offset bits prior to start */
        tmp &= !0u32 << start_bit;
        if tmp != 0u32 {
            return (result + ffs(tmp) - 1) as i32;
        }
        result += NBWORD;
        size -= NBWORD;
    }
    while size != 0 {
        tmp = *p;
        p = p.add(1);
        if tmp != 0u32 {
            return (result + ffs(tmp) - 1) as i32;
        }
        result += NBWORD;
        size -= NBWORD;
    }
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
