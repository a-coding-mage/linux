/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 *
 * Derived from arch/x86/include/asm/word-at-a-time.h
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/asm-extable.h, linux/bitops.h, and linux/wordpart.h.

#[repr(C)]
pub struct word_at_a_time {
    pub one_bits: usize,
    pub high_bits: usize,
}

// #define WORD_AT_A_TIME_CONSTANTS { REPEAT_BYTE(0x01), REPEAT_BYTE(0x80) }
#[macro_export]
macro_rules! WORD_AT_A_TIME_CONSTANTS {
    () => {
        word_at_a_time {
            one_bits: REPEAT_BYTE!(0x01),
            high_bits: REPEAT_BYTE!(0x80),
        }
    };
}

#[inline]
pub unsafe fn has_zero(
    val: usize,
    bits: *mut usize,
    c: *const word_at_a_time,
) -> usize {
    let mask = val.wrapping_sub((*c).one_bits) & !val & (*c).high_bits;
    *bits = mask;
    mask
}

#[inline]
pub unsafe fn prep_zero_mask(
    _val: usize,
    bits: usize,
    _c: *const word_at_a_time,
) -> usize {
    bits
}

#[inline]
pub fn create_zero_mask(mut bits: usize) -> usize {
    bits = bits.wrapping_sub(1) & !bits;
    bits >> 7
}

#[inline]
pub fn find_zero(mask: usize) -> usize {
    // C fls64(mask) returns the one-based position of the highest set bit;
    // zero is not a valid input in the original call sites.
    (usize::BITS as usize - mask.leading_zeros() as usize) >> 3
}

// The mask we created is directly usable as a bytemask.
#[inline]
pub const fn zero_bytemask(mask: usize) -> usize {
    mask
}

// CONFIG_DCACHE_WORD_ACCESS is a build-time condition from the original
// header.  The function is retained here under the corresponding cfg.
#[cfg(feature = "CONFIG_DCACHE_WORD_ACCESS")]
#[inline]
pub unsafe fn load_unaligned_zeropad(_addr: *const core::ffi::c_void) -> usize {
    /*
     * Load an unaligned word from kernel space.  The original implementation
     * uses RISC-V assembler and _ASM_EXTABLE_LOAD_UNALIGNED_ZEROPAD to catch a
     * page-crossing fault and zero the non-existing portion.  Those assembler
     * and exception-table dependencies are supplied by the surrounding kernel
     * translation and cannot be represented file-locally here.
     */
    // TODO: translate the architecture exception-table inline assembly.
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
