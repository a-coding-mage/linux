/* SPDX-License-Identifier: GPL-2.0 */

// The big-endian configuration includes asm-generic/word-at-a-time.h.
// This file contains the little-endian implementation.

/*
 * Little-endian version cribbed from x86.
 */
#[repr(C)]
pub struct word_at_a_time {
    pub one_bits: usize,
    pub high_bits: usize,
}

// WORD_AT_A_TIME_CONSTANTS { REPEAT_BYTE(0x01), REPEAT_BYTE(0x80) }
// REPEAT_BYTE is supplied by the surrounding dependency environment.

/* Carl Chatfield / Jan Achrenius G+ version for 32-bit */
#[inline]
pub fn count_masked_bytes(mask: isize) -> isize {
    /* (000000 0000ff 00ffff ffffff) -> ( 1 1 2 3 ) */
    let a = mask.wrapping_add(0x0ff0001) >> 23;
    /* Fix the 1 for 00 case */
    a & mask
}

/* Return nonzero if it has a zero */
#[inline]
pub unsafe fn has_zero(a: usize, bits: *mut usize, c: *const word_at_a_time) -> usize {
    let mask = ((a.wrapping_sub((*c).one_bits)) & !a) & (*c).high_bits;
    *bits = mask;
    mask
}

#[inline]
pub unsafe fn prep_zero_mask(_a: usize, bits: usize, _c: *const word_at_a_time) -> usize {
    bits
}

#[inline]
pub fn create_zero_mask(mut bits: usize) -> usize {
    bits = (bits.wrapping_sub(1)) & !bits;
    bits >> 7
}

/* The mask we created is directly usable as a bytemask */
#[inline]
pub const fn zero_bytemask(mask: usize) -> usize {
    mask
}

#[inline]
pub fn find_zero(mask: isize) -> isize {
    count_masked_bytes(mask)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
