/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 ARM Ltd.
 */

// Dependency: linux/uaccess.h

// The __AARCH64EB__ build-time condition is preserved from the C header.
#[cfg(not(target_endian = "big"))]
pub struct word_at_a_time {
    pub one_bits: ::core::ffi::c_ulong,
    pub high_bits: ::core::ffi::c_ulong,
}

#[cfg(not(target_endian = "big"))]
#[macro_export]
macro_rules! WORD_AT_A_TIME_CONSTANTS {
    () => {
        { REPEAT_BYTE!(0x01), REPEAT_BYTE!(0x80) }
    };
}

#[cfg(not(target_endian = "big"))]
#[inline]
pub unsafe fn has_zero(
    a: ::core::ffi::c_ulong,
    bits: *mut ::core::ffi::c_ulong,
    c: *const word_at_a_time,
) -> ::core::ffi::c_ulong {
    let mask = ((a.wrapping_sub((*c).one_bits)) & !a) & (*c).high_bits;
    *bits = mask;
    mask
}

#[cfg(not(target_endian = "big"))]
#[macro_export]
macro_rules! prep_zero_mask {
    ($a:expr, $bits:expr, $c:expr) => { $bits };
}

#[cfg(not(target_endian = "big"))]
#[macro_export]
macro_rules! create_zero_mask {
    ($bits:expr) => { $bits };
}

#[cfg(not(target_endian = "big"))]
#[macro_export]
macro_rules! find_zero {
    ($bits:expr) => { (__ffs!($bits) >> 3) };
}

#[cfg(not(target_endian = "big"))]
#[inline]
pub fn zero_bytemask(mut bits: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    bits = (bits.wrapping_sub(1)) & !bits;
    bits >> 7
}

// __AARCH64EB__: use the declarations supplied by asm-generic/word-at-a-time.h.

/*
 * Load an unaligned word from kernel space.
 *
 * In the (very unlikely) case of the word being a page-crosser
 * and the next page not being mapped, take the exception and
 * return zeroes in the non-existing part.
 */
#[inline]
pub unsafe fn load_unaligned_zeropad(addr: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut ret: ::core::ffi::c_ulong;

    __mte_enable_tco_async();

    /* Load word from unaligned pointer addr */
    ::core::arch::asm!(
        "1: ldr {ret}, [{addr}]",
        "2:",
        // _ASM_EXTABLE_LOAD_UNALIGNED_ZEROPAD(1b, 2b, %0, %1)
        ret = lateout(reg) ret,
        addr = in(reg) addr,
    );

    __mte_disable_tco_async();

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
