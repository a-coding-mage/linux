/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm-generic/word-at-a-time.h. */

#[cfg(target_endian = "big")]
#[repr(C)]
pub struct word_at_a_time {
    pub high_bits: ::core::ffi::c_ulong,
    pub low_bits: ::core::ffi::c_ulong,
}

#[cfg(target_endian = "big")]
pub const WORD_AT_A_TIME_CONSTANTS: [::core::ffi::c_ulong; 2] = [
    REPEAT_BYTE(0xfe) + 1,
    REPEAT_BYTE(0x7f),
];

#[cfg(target_endian = "big")]
#[inline]
pub unsafe fn prep_zero_mask(
    val: ::core::ffi::c_ulong,
    rhs: ::core::ffi::c_ulong,
    c: *const word_at_a_time,
) -> ::core::ffi::c_long {
    let mask = (val & (*c).low_bits).wrapping_add((*c).low_bits);
    (!(mask | rhs)) as ::core::ffi::c_long
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! create_zero_mask {
    ($mask:expr) => { $mask };
}

#[cfg(all(target_endian = "big", target_pointer_width = "64"))]
#[inline]
pub fn find_zero(mut mask: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    let mut byte: ::core::ffi::c_long = 0;
    if mask >> 32 != 0 {
        mask >>= 32;
    } else {
        byte = 4;
    }
    if mask >> 16 != 0 {
        mask >>= 16;
    } else {
        byte += 2;
    }
    if mask >> 8 != 0 { byte } else { byte + 1 }
}

#[cfg(all(target_endian = "big", target_pointer_width = "32"))]
#[inline]
pub fn find_zero(mut mask: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    let mut byte: ::core::ffi::c_long = 0;
    if mask >> 16 != 0 { mask >>= 16; } else { byte += 2; }
    if mask >> 8 != 0 { byte } else { byte + 1 }
}

#[cfg(target_endian = "big")]
#[inline]
pub unsafe fn has_zero(
    val: ::core::ffi::c_ulong,
    data: *mut ::core::ffi::c_ulong,
    c: *const word_at_a_time,
) -> ::core::ffi::c_ulong {
    let rhs = val | (*c).low_bits;
    *data = rhs;
    (val.wrapping_add((*c).high_bits)) & !rhs
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! zero_bytemask {
    ($mask:expr) => { (!1usize.wrapping_shl($mask.trailing_zeros())) };
}

#[cfg(target_endian = "little")]
#[repr(C)]
pub struct word_at_a_time {
    pub one_bits: ::core::ffi::c_ulong,
    pub high_bits: ::core::ffi::c_ulong,
}

#[cfg(target_endian = "little")]
pub const WORD_AT_A_TIME_CONSTANTS: [::core::ffi::c_ulong; 2] = [
    REPEAT_BYTE(0x01),
    REPEAT_BYTE(0x80),
];

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[inline]
pub fn count_masked_bytes(mask: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    ((mask.wrapping_mul(0x0001020304050608u64 as ::core::ffi::c_ulong)) >> 56)
        as ::core::ffi::c_long
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub fn count_masked_bytes(mask: ::core::ffi::c_long) -> ::core::ffi::c_long {
    let a = (0x0ff0001i64.wrapping_add(mask as i64) >> 23) as ::core::ffi::c_long;
    a & mask
}

#[cfg(target_endian = "little")]
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

#[cfg(target_endian = "little")]
#[inline]
pub unsafe fn prep_zero_mask(
    _a: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_ulong,
    _c: *const word_at_a_time,
) -> ::core::ffi::c_ulong {
    bits
}

#[cfg(target_endian = "little")]
#[inline]
pub fn create_zero_mask(mut bits: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    bits = (bits.wrapping_sub(1)) & !bits;
    bits >> 7
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! zero_bytemask {
    ($mask:expr) => { $mask };
}

#[cfg(target_endian = "little")]
#[inline]
pub fn find_zero(mask: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    count_masked_bytes(mask)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
