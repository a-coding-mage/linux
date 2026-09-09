/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: REPEAT_BYTE is supplied by the wordpart/bitops implementation.
// The original header is selected by CONFIG_64BIT at build time.

#[repr(C)]
pub struct word_at_a_time {
    pub one_bits: ::core::ffi::c_ulong,
    pub high_bits: ::core::ffi::c_ulong,
}

// #define WORD_AT_A_TIME_CONSTANTS { REPEAT_BYTE(0x01), REPEAT_BYTE(0x80) }
// Requires the externally supplied REPEAT_BYTE constant/function.

#[inline]
pub unsafe fn has_zero(
    a: ::core::ffi::c_ulong,
    bits: *mut ::core::ffi::c_ulong,
    c: *const word_at_a_time,
) -> ::core::ffi::c_ulong {
    let mask = a.wrapping_sub((*c).one_bits) & !a & (*c).high_bits;
    *bits = mask;
    mask
}

#[inline]
pub unsafe fn prep_zero_mask(
    _a: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_ulong,
    _c: *const word_at_a_time,
) -> ::core::ffi::c_ulong {
    bits
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn create_zero_mask(bits: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    bits
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn zero_bytemask(mut bits: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    bits = bits.wrapping_sub(1) & !bits;
    bits >> 7
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn find_zero(bits: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    // __ffs(bits) >> 3; __ffs is supplied by the external bitops implementation.
    unsafe { (__ffs(bits) >> 3) as ::core::ffi::c_ulong }
}

#[cfg(target_pointer_width = "64")]
unsafe extern "C" {
    fn __ffs(x: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub fn create_zero_mask(mut bits: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    bits = bits.wrapping_sub(1) & !bits;
    bits >> 7
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub fn zero_bytemask(mask: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    mask
}

/* Carl Chatfield / Jan Achrenius G+ version for 32-bit */
#[cfg(target_pointer_width = "32")]
#[inline]
pub fn find_zero(mask: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    /* (000000 0000ff 00ffff ffffff) -> ( 1 1 2 3 ) */
    let a = (0x0ff0001u32.wrapping_add(mask as u32) >> 23) as ::core::ffi::c_long;
    /* Fix the 1 for 00 case */
    (a as ::core::ffi::c_ulong) & mask
}

/*
 * Load an unaligned word from kernel space.
 *
 * In the (very unlikely) case of the word being a page-crosser
 * and the next page not being mapped, take the exception and
 * return zeroes in the non-existing part.
 *
 * The original implementation uses architecture-specific exception-table
 * inline assembly (_ASM_EXTABLE_TYPE); that dependency is preserved here.
 */
#[inline]
pub unsafe fn load_unaligned_zeropad(addr: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut ret: ::core::ffi::c_ulong;
    ::core::arch::asm!(
        "mov {ret}, [{mem}]",
        ret = out(reg) ret,
        mem = in(reg) addr,
        options(nostack, preserves_flags)
    );
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
