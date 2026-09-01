/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from include/linux/bitops.h. */
/* C includes removed: <asm/types.h>, <limits.h>, <linux/bits.h>,
 * <linux/compiler.h>, <asm-generic/bitops.h>.
 */

pub const __WORDSIZE: usize = core::mem::size_of::<core::ffi::c_ulong>() * 8;
pub const BITS_PER_LONG: usize = __WORDSIZE;

pub const fn BITS_PER_TYPE<T>() -> usize {
    core::mem::size_of::<T>() * BITS_PER_BYTE as usize
}

pub const fn BITS_TO_LONGS(nr: usize) -> usize {
    DIV_ROUND_UP(nr, BITS_PER_TYPE::<core::ffi::c_long>())
}

pub const fn BITS_TO_U64(nr: usize) -> usize {
    DIV_ROUND_UP(nr, BITS_PER_TYPE::<__u64>())
}

pub const fn BITS_TO_U32(nr: usize) -> usize {
    DIV_ROUND_UP(nr, BITS_PER_TYPE::<__u32>())
}

pub const fn BITS_TO_BYTES(nr: usize) -> usize {
    DIV_ROUND_UP(nr, BITS_PER_TYPE::<core::ffi::c_char>())
}

pub const fn BYTES_TO_BITS(nb: usize) -> usize {
    nb * BITS_PER_BYTE as usize
}

unsafe extern "C" {
    pub fn __sw_hweight8(w: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn __sw_hweight16(w: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn __sw_hweight32(w: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn __sw_hweight64(w: __u64) -> core::ffi::c_ulong;
}

/*
 * Defined here because those may be needed by architecture-specific static
 * inlines.
 */

#[macro_export]
macro_rules! bitop {
    ($op:ident, $nr:expr, $addr:expr) => {
        $op($nr, $addr)
    };
}

#[macro_export]
macro_rules! __set_bit {
    ($nr:expr, $addr:expr) => {
        $crate::bitop!(___set_bit, $nr, $addr)
    };
}

#[macro_export]
macro_rules! __clear_bit {
    ($nr:expr, $addr:expr) => {
        $crate::bitop!(___clear_bit, $nr, $addr)
    };
}

#[macro_export]
macro_rules! __change_bit {
    ($nr:expr, $addr:expr) => {
        $crate::bitop!(___change_bit, $nr, $addr)
    };
}

#[macro_export]
macro_rules! __test_and_set_bit {
    ($nr:expr, $addr:expr) => {
        $crate::bitop!(___test_and_set_bit, $nr, $addr)
    };
}

#[macro_export]
macro_rules! __test_and_clear_bit {
    ($nr:expr, $addr:expr) => {
        $crate::bitop!(___test_and_clear_bit, $nr, $addr)
    };
}

#[macro_export]
macro_rules! __test_and_change_bit {
    ($nr:expr, $addr:expr) => {
        $crate::bitop!(___test_and_change_bit, $nr, $addr)
    };
}

#[macro_export]
macro_rules! test_bit {
    ($nr:expr, $addr:expr) => {
        $crate::bitop!(_test_bit, $nr, $addr)
    };
}

/*
 * Include this here because some architectures need generic_ffs/fls in
 * scope
 *
 * XXX: this needs to be asm/bitops.h, when we get to per arch optimizations
 */

#[macro_export]
macro_rules! for_each_set_bit {
    ($bit:ident, $addr:expr, $size:expr, $body:block) => {{
        $bit = find_first_bit($addr, $size);
        while $bit < $size {
            $body
            $bit = find_next_bit($addr, $size, $bit + 1);
        }
    }};
}

#[macro_export]
macro_rules! for_each_clear_bit {
    ($bit:ident, $addr:expr, $size:expr, $body:block) => {{
        $bit = find_first_zero_bit($addr, $size);
        while $bit < $size {
            $body
            $bit = find_next_zero_bit($addr, $size, $bit + 1);
        }
    }};
}

/* same as for_each_set_bit() but use bit as value to start with */
#[macro_export]
macro_rules! for_each_set_bit_from {
    ($bit:ident, $addr:expr, $size:expr, $body:block) => {{
        $bit = find_next_bit($addr, $size, $bit);
        while $bit < $size {
            $body
            $bit = find_next_bit($addr, $size, $bit + 1);
        }
    }};
}

#[inline]
pub unsafe fn hweight_long(w: core::ffi::c_ulong) -> core::ffi::c_ulong {
    if core::mem::size_of_val(&w) == 4 {
        hweight32(w as __u32) as core::ffi::c_ulong
    } else {
        hweight64(w as __u64) as core::ffi::c_ulong
    }
}

#[inline]
pub unsafe fn fls_long(l: core::ffi::c_ulong) -> core::ffi::c_uint {
    if core::mem::size_of_val(&l) == 4 {
        fls(l as core::ffi::c_int) as core::ffi::c_uint
    } else {
        fls64(l as __u64) as core::ffi::c_uint
    }
}

/**
 * rol32 - rotate a 32-bit value left
 * @word: value to rotate
 * @shift: bits to roll
 */
#[inline]
pub fn rol32(word: __u32, shift: core::ffi::c_uint) -> __u32 {
    (word.wrapping_shl(shift) | word.wrapping_shr((0u32.wrapping_sub(shift)) & 31)) as __u32
}

/**
 * sign_extend64 - sign extend a 64-bit value using specified bit as sign-bit
 * @value: value to sign extend
 * @index: 0 based bit index (0<=index<64) to sign bit
 */
#[inline]
pub fn sign_extend64(value: __u64, index: core::ffi::c_int) -> __s64 {
    let shift: __u8 = (63 - index) as __u8;
    ((value.wrapping_shl(shift as u32)) as __s64) >> shift
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
