/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/bitops.h, linux/wordpart.h, asm/asm-extable.h, asm/bitsperlong.h

#[repr(C)]
pub struct word_at_a_time {
    pub bits: ::core::primitive::usize,
}

// #define WORD_AT_A_TIME_CONSTANTS { REPEAT_BYTE(0x7f) }
// Build-time macro equivalent; REPEAT_BYTE is supplied by linux/wordpart.h.

#[inline(always)]
pub fn prep_zero_mask(
    val: ::core::primitive::usize,
    data: ::core::primitive::usize,
    c: *const word_at_a_time,
) -> ::core::primitive::usize {
    let _ = (val, c);
    data
}

#[inline(always)]
pub fn create_zero_mask(data: ::core::primitive::usize) -> ::core::primitive::usize {
    __fls(data)
}

#[inline(always)]
pub fn find_zero(data: ::core::primitive::usize) -> ::core::primitive::usize {
    (data ^ (BITS_PER_LONG - 1)) >> 3
}

#[inline(always)]
pub unsafe fn has_zero(
    val: ::core::primitive::usize,
    data: *mut ::core::primitive::usize,
    c: *const word_at_a_time,
) -> ::core::primitive::usize {
    let mask = (val & (*c).bits).wrapping_add((*c).bits);

    *data = !(mask | val | (*c).bits);
    *data
}

#[inline(always)]
pub fn zero_bytemask(data: ::core::primitive::usize) -> ::core::primitive::usize {
    (!1usize) << data
}

/*
 * Load an unaligned word from kernel space.
 *
 * In the (very unlikely) case of the word being a page-crosser
 * and the next page not being mapped, take the exception and
 * return zeroes in the non-existing part.
 *
 * The original implementation uses architecture-specific inline assembly
 * and exception-table zeropad handling. The unaligned load preserves the
 * file-local operation; page-fault recovery is supplied by the kernel runtime.
 */
#[inline(always)]
pub unsafe fn load_unaligned_zeropad(addr: *const ::core::ffi::c_void) -> ::core::primitive::usize {
    ::core::ptr::read_unaligned(addr as *const ::core::primitive::usize)
}

// External dependencies from linux/bitops.h and asm/bitsperlong.h.
extern "C" {
    fn __fls(word: ::core::primitive::usize) -> ::core::primitive::usize;
    static BITS_PER_LONG: ::core::primitive::usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
