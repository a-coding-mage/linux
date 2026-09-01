/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Source dependencies in the C header:
 * stdarg.h, stddef.h, assert.h, linux/build_bug.h, linux/compiler.h,
 * linux/math.h, linux/panic.h, endian.h, byteswap.h, linux/container_of.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_macros)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type size_t = usize;
pub type va_list = *mut c_void;

#[cfg(not(any()))]
pub const UINT_MAX: u32 = !0u32;

/*
 * C used __builtin_return_address(0). Rust has no direct stable file-local
 * equivalent; this macro preserves the call surface for dependency resolution.
 */
macro_rules! _RET_IP_ {
    () => {
        0 as c_ulong
    };
}

macro_rules! __PERF_ALIGN_MASK {
    ($x:expr, $mask:expr) => {
        (($x).wrapping_add($mask)) & !($mask)
    };
}

macro_rules! PERF_ALIGN {
    ($x:expr, $a:expr) => {
        __PERF_ALIGN_MASK!($x, (($a) as _) - 1)
    };
}

macro_rules! offsetof {
    ($TYPE:ty, $MEMBER:tt) => {
        core::mem::offset_of!($TYPE, $MEMBER)
    };
}

macro_rules! max {
    ($x:expr, $y:expr) => {{
        let _max1 = $x;
        let _max2 = $y;
        if _max1 > _max2 {
            _max1
        } else {
            _max2
        }
    }};
}

macro_rules! min {
    ($x:expr, $y:expr) => {{
        let _min1 = $x;
        let _min2 = $y;
        if _min1 < _min2 {
            _min1
        } else {
            _min2
        }
    }};
}

macro_rules! max_t {
    ($type:ty, $x:expr, $y:expr) => {
        max!(($x) as $type, ($y) as $type)
    };
}

macro_rules! min_t {
    ($type:ty, $x:expr, $y:expr) => {
        min!(($x) as $type, ($y) as $type)
    };
}

macro_rules! clamp {
    ($val:expr, $lo:expr, $hi:expr) => {
        min!(max!($val, $lo) as _, $hi)
    };
}

#[cfg(debug_assertions)]
macro_rules! BUG_ON {
    ($cond:expr) => {
        assert!(!($cond))
    };
}

#[cfg(not(debug_assertions))]
macro_rules! BUG_ON {
    ($cond:expr) => {
        if $cond {}
    };
}

macro_rules! BUG {
    () => {
        BUG_ON!(true)
    };
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_le16(x: u16) -> u16 {
    x.swap_bytes()
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_le32(x: u32) -> u32 {
    x.swap_bytes()
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_le64(x: u64) -> u64 {
    x.swap_bytes()
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn le16_to_cpu(x: u16) -> u16 {
    x.swap_bytes()
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn le32_to_cpu(x: u32) -> u32 {
    x.swap_bytes()
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn le64_to_cpu(x: u64) -> u64 {
    x.swap_bytes()
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_be16(x: u16) -> u16 {
    x
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_be32(x: u32) -> u32 {
    x
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_be64(x: u64) -> u64 {
    x
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn be16_to_cpu(x: u16) -> u16 {
    x
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn be32_to_cpu(x: u32) -> u32 {
    x
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn be64_to_cpu(x: u64) -> u64 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_le16(x: u16) -> u16 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_le32(x: u32) -> u32 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_le64(x: u64) -> u64 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn le16_to_cpu(x: u16) -> u16 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn le32_to_cpu(x: u32) -> u32 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn le64_to_cpu(x: u64) -> u64 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_be16(x: u16) -> u16 {
    x.swap_bytes()
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_be32(x: u32) -> u32 {
    x.swap_bytes()
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_be64(x: u64) -> u64 {
    x.swap_bytes()
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn be16_to_cpu(x: u16) -> u16 {
    x.swap_bytes()
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn be32_to_cpu(x: u32) -> u32 {
    x.swap_bytes()
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn be64_to_cpu(x: u64) -> u64 {
    x.swap_bytes()
}

unsafe extern "C" {
    pub fn vscnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, args: va_list) -> c_int;
    pub fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    pub fn scnprintf_pad(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
}

macro_rules! ARRAY_SIZE {
    ($arr:expr) => {
        core::mem::size_of_val(&$arr) / core::mem::size_of_val(&($arr)[0])
    };
}

macro_rules! current_gfp_context {
    ($k:expr) => {
        0
    };
}

macro_rules! synchronize_rcu {
    () => {};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
