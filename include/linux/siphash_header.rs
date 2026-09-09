/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* Copyright (C) 2016-2022 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * SipHash: a fast short-input PRF
 * https://131002.net/siphash/
 *
 * This implementation is specifically for SipHash2-4 for a secure PRF
 * and HalfSipHash1-3/SipHash1-3 for an insecure PRF only suitable for
 * hashtables.
 */

// C dependencies supplied by the surrounding translation unit: u32, u64,
// size_t, __le32, __le64, endian helpers, alignment helpers, and rol32/rol64.

pub const SIPHASH_ALIGNMENT: usize = core::mem::align_of::<u64>();

#[repr(C)]
pub struct siphash_key_t {
    pub key: [u64; 2],
}

// C: #define siphash_aligned_key_t siphash_key_t __aligned(16)
#[repr(C, align(16))]
pub struct siphash_aligned_key_t {
    pub key: [u64; 2],
}

#[inline]
pub unsafe fn siphash_key_is_zero(key: *const siphash_key_t) -> bool {
    ((*key).key[0] | (*key).key[1]) == 0
}

extern "C" {
    pub fn __siphash_aligned(data: *const core::ffi::c_void, len: usize, key: *const siphash_key_t) -> u64;
    pub fn __siphash_unaligned(data: *const core::ffi::c_void, len: usize, key: *const siphash_key_t) -> u64;
    pub fn siphash_1u64(a: u64, key: *const siphash_key_t) -> u64;
    pub fn siphash_2u64(a: u64, b: u64, key: *const siphash_key_t) -> u64;
    pub fn siphash_3u64(a: u64, b: u64, c: u64, key: *const siphash_key_t) -> u64;
    pub fn siphash_4u64(a: u64, b: u64, c: u64, d: u64, key: *const siphash_key_t) -> u64;
    pub fn siphash_1u32(a: u32, key: *const siphash_key_t) -> u64;
    pub fn siphash_3u32(a: u32, b: u32, c: u32, key: *const siphash_key_t) -> u64;
    pub fn __hsiphash_aligned(data: *const core::ffi::c_void, len: usize, key: *const hsiphash_key_t) -> u32;
    pub fn __hsiphash_unaligned(data: *const core::ffi::c_void, len: usize, key: *const hsiphash_key_t) -> u32;
    pub fn hsiphash_1u32(a: u32, key: *const hsiphash_key_t) -> u32;
    pub fn hsiphash_2u32(a: u32, b: u32, key: *const hsiphash_key_t) -> u32;
    pub fn hsiphash_3u32(a: u32, b: u32, c: u32, key: *const hsiphash_key_t) -> u32;
    pub fn hsiphash_4u32(a: u32, b: u32, c: u32, d: u32, key: *const hsiphash_key_t) -> u32;
}

#[inline]
pub unsafe fn siphash_2u32(a: u32, b: u32, key: *const siphash_key_t) -> u64 {
    siphash_1u64((b as u64).wrapping_shl(32) | a as u64, key)
}

#[inline]
pub unsafe fn siphash_4u32(a: u32, b: u32, c: u32, d: u32, key: *const siphash_key_t) -> u64 {
    siphash_2u64((b as u64).wrapping_shl(32) | a as u64,
                 (d as u64).wrapping_shl(32) | c as u64, key)
}

// __builtin_constant_p(len) branches are preserved through the explicit
// constant-length entry points; endian helpers are supplied externally.
#[inline]
pub unsafe fn ___siphash_aligned(data: *const u64, len: usize, key: *const siphash_key_t) -> u64 {
    match len {
        4 => siphash_1u32((*(data as *const u32)).to_le(), key),
        8 => siphash_1u64(u64::from_le(*data), key),
        16 => siphash_2u64(u64::from_le(*data), u64::from_le(*data.add(1)), key),
        24 => siphash_3u64(u64::from_le(*data), u64::from_le(*data.add(1)), u64::from_le(*data.add(2)), key),
        32 => siphash_4u64(u64::from_le(*data), u64::from_le(*data.add(1)), u64::from_le(*data.add(2)), u64::from_le(*data.add(3)), key),
        _ => __siphash_aligned(data as *const core::ffi::c_void, len, key),
    }
}

#[inline]
pub unsafe fn siphash(data: *const core::ffi::c_void, len: usize, key: *const siphash_key_t) -> u64 {
    // IS_ENABLED(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS) is supplied by the build configuration.
    __siphash_unaligned(data, len, key)
}

pub const HSIPHASH_ALIGNMENT: usize = core::mem::align_of::<usize>();

#[repr(C)]
pub struct hsiphash_key_t {
    pub key: [usize; 2],
}

#[inline]
pub unsafe fn ___hsiphash_aligned(data: *const u32, len: usize, key: *const hsiphash_key_t) -> u32 {
    match len {
        4 => hsiphash_1u32(u32::from_le(*data), key),
        8 => hsiphash_2u32(u32::from_le(*data), u32::from_le(*data.add(1)), key),
        12 => hsiphash_3u32(u32::from_le(*data), u32::from_le(*data.add(1)), u32::from_le(*data.add(2)), key),
        16 => hsiphash_4u32(u32::from_le(*data), u32::from_le(*data.add(1)), u32::from_le(*data.add(2)), u32::from_le(*data.add(3)), key),
        _ => __hsiphash_aligned(data as *const core::ffi::c_void, len, key),
    }
}

#[inline]
pub unsafe fn hsiphash(data: *const core::ffi::c_void, len: usize, key: *const hsiphash_key_t) -> u32 {
    // IS_ENABLED(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS) is supplied by the build configuration.
    __hsiphash_unaligned(data, len, key)
}

// These macros expose the raw SipHash and HalfSipHash permutations.
// Do not use them directly! If you think you have a use for them,
// be sure to CC the maintainer of this file explaining why.

#[macro_export]
macro_rules! SIPHASH_PERMUTATION {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {{
        $a = $a.wrapping_add($b); $b = $b.rotate_left(13); $b ^= $a; $a = $a.rotate_left(32);
        $c = $c.wrapping_add($d); $d = $d.rotate_left(16); $d ^= $c;
        $a = $a.wrapping_add($d); $d = $d.rotate_left(21); $d ^= $a;
        $c = $c.wrapping_add($b); $b = $b.rotate_left(17); $b ^= $c; $c = $c.rotate_left(32);
    }};
}

pub const SIPHASH_CONST_0: u64 = 0x736f6d6570736575;
pub const SIPHASH_CONST_1: u64 = 0x646f72616e646f6d;
pub const SIPHASH_CONST_2: u64 = 0x6c7967656e657261;
pub const SIPHASH_CONST_3: u64 = 0x7465646279746573;

#[macro_export]
macro_rules! HSIPHASH_PERMUTATION {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {{
        $a = $a.wrapping_add($b); $b = $b.rotate_left(5); $b ^= $a; $a = $a.rotate_left(16);
        $c = $c.wrapping_add($d); $d = $d.rotate_left(8); $d ^= $c;
        $a = $a.wrapping_add($d); $d = $d.rotate_left(7); $d ^= $a;
        $c = $c.wrapping_add($b); $b = $b.rotate_left(13); $b ^= $c; $c = $c.rotate_left(16);
    }};
}

pub const HSIPHASH_CONST_0: u32 = 0;
pub const HSIPHASH_CONST_1: u32 = 0;
pub const HSIPHASH_CONST_2: u32 = 0x6c796765;
pub const HSIPHASH_CONST_3: u32 = 0x74656462;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
