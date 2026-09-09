/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// asm/types.h, linux/bits.h, linux/typecheck.h, uapi/linux/kernel.h,
// asm-generic/bitops/generic-non-atomic.h, and asm/bitops.h.

#[allow(non_camel_case_types)]
pub type __u8 = u8;
#[allow(non_camel_case_types)]
pub type __u16 = u16;
#[allow(non_camel_case_types)]
pub type __u32 = u32;
#[allow(non_camel_case_types)]
pub type __u64 = u64;
#[allow(non_camel_case_types)]
pub type __s32 = i32;
#[allow(non_camel_case_types)]
pub type __s64 = i64;

pub const BITS_PER_BYTE: usize = 8;
pub const BITS_PER_LONG: usize = usize::BITS as usize;

pub const fn bits_to_longs(nr: usize) -> usize { (nr + BITS_PER_LONG - 1) / BITS_PER_LONG }
pub const fn bits_to_u64(nr: usize) -> usize { (nr + 63) / 64 }
pub const fn bits_to_u32(nr: usize) -> usize { (nr + 31) / 32 }
pub const fn bits_to_bytes(nr: usize) -> usize { (nr + BITS_PER_BYTE - 1) / BITS_PER_BYTE }
pub const fn bytes_to_bits(nb: usize) -> usize { nb * BITS_PER_BYTE }

extern "C" {
    pub fn __sw_hweight8(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
    pub fn __sw_hweight16(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
    pub fn __sw_hweight32(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
    pub fn __sw_hweight64(w: __u64) -> ::std::os::raw::c_ulong;
}

// Architecture-specific bit operations and generic alternatives are supplied externally.

#[inline]
pub fn get_bitmask_order(count: u32) -> i32 {
    fls(count)
}

#[inline]
pub fn hweight_long(w: usize) -> usize {
    if usize::BITS == 32 { hweight32(w as u32) as usize } else { hweight64(w as u64) as usize }
}

#[inline]
pub fn rol64(word: __u64, shift: u32) -> __u64 {
    (word << (shift & 63)) | (word >> ((shift.wrapping_neg()) & 63))
}
#[inline]
pub fn ror64(word: __u64, shift: u32) -> __u64 {
    (word >> (shift & 63)) | (word << ((shift.wrapping_neg()) & 63))
}
#[inline]
pub fn rol32(word: __u32, shift: u32) -> __u32 {
    (word << (shift & 31)) | (word >> ((shift.wrapping_neg()) & 31))
}
#[inline]
pub fn ror32(word: __u32, shift: u32) -> __u32 {
    (word >> (shift & 31)) | (word << ((shift.wrapping_neg()) & 31))
}
#[inline]
pub fn rol16(word: __u16, shift: u32) -> __u16 {
    ((word as u32).wrapping_shl(shift & 15) | (word as u32).wrapping_shr((shift.wrapping_neg()) & 15)) as __u16
}
#[inline]
pub fn ror16(word: __u16, shift: u32) -> __u16 {
    ((word as u32).wrapping_shr(shift & 15) | (word as u32).wrapping_shl((shift.wrapping_neg()) & 15)) as __u16
}
#[inline]
pub fn rol8(word: __u8, shift: u32) -> __u8 {
    ((word as u32).wrapping_shl(shift & 7) | (word as u32).wrapping_shr((shift.wrapping_neg()) & 7)) as __u8
}
#[inline]
pub fn ror8(word: __u8, shift: u32) -> __u8 {
    ((word as u32).wrapping_shr(shift & 7) | (word as u32).wrapping_shl((shift.wrapping_neg()) & 7)) as __u8
}

#[inline]
pub fn sign_extend32(value: __u32, index: i32) -> __s32 {
    let shift = (31 - index) as u32;
    ((value << shift) as __s32) >> shift
}
#[inline]
pub fn sign_extend64(value: __u64, index: i32) -> __s64 {
    let shift = (63 - index) as u32;
    ((value << shift) as __s64) >> shift
}

#[inline]
pub fn fls_long(l: usize) -> u32 { if usize::BITS == 32 { fls(l as u32) } else { fls64(l as u64) } }
#[inline]
pub fn get_count_order(mut count: u32) -> i32 {
    if count == 0 { return -1; }
    count -= 1;
    fls(count)
}
#[inline]
pub fn get_count_order_long(mut l: usize) -> i32 {
    if l == 0 { return -1; }
    l -= 1;
    fls_long(l) as i32
}

#[inline]
pub fn parity8(mut val: u8) -> i32 {
    val ^= val >> 4;
    ((0x6996u32 >> (val & 0xf)) & 1) as i32
}

#[inline]
pub fn __ffs64(word: u64) -> u32 {
    if usize::BITS == 32 && (word as u32) == 0 { return __ffs((word >> 32) as u32) + 32; }
    __ffs(word as usize as u32)
}

#[inline]
pub fn fns(mut word: usize, mut n: u32) -> u32 {
    while word != 0 && n != 0 { word &= word - 1; n -= 1; }
    if word != 0 { __ffs(word as u32) } else { BITS_PER_LONG as u32 }
}

#[macro_export]
macro_rules! assign_bit { ($nr:expr, $addr:expr, $value:expr) => { if $value { set_bit($nr, $addr) } else { clear_bit($nr, $addr) } }; }
#[macro_export]
macro_rules! __assign_bit { ($nr:expr, $addr:expr, $value:expr) => { if $value { __set_bit($nr, $addr) } else { __clear_bit($nr, $addr) } }; }

// __ptr_set_bit, __ptr_clear_bit, and __ptr_test_bit preserve the C pointer
// type check and reinterpret the pointer variable as an unsigned-long bitmap.
#[macro_export]
macro_rules! __ptr_set_bit { ($nr:expr, $addr:expr) => { __set_bit($nr, $addr as *mut usize) }; }
#[macro_export]
macro_rules! __ptr_clear_bit { ($nr:expr, $addr:expr) => { __clear_bit($nr, $addr as *mut usize) }; }
#[macro_export]
macro_rules! __ptr_test_bit { ($nr:expr, $addr:expr) => { test_bit($nr, $addr as *const usize) }; }

// __KERNEL__-only set_mask_bits and bit_clear_unless retain their original
// atomic read/compare-exchange semantics and are supplied by kernel callers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
