/* SPDX-License-Identifier: GPL-2.0 */

// The original header includes Linux compiler and type definitions.
use core::ffi::{c_char, c_long, c_ulong, c_ulonglong};

/* Internal, do not use. */
unsafe extern "C" {
    pub fn _kstrtoul(s: *const c_char, base: u32, res: *mut c_ulong) -> i32;
    pub fn _kstrtol(s: *const c_char, base: u32, res: *mut c_long) -> i32;

    pub fn kstrtoull(s: *const c_char, base: u32, res: *mut c_ulonglong) -> i32;
    pub fn kstrtoll(s: *const c_char, base: u32, res: *mut i64) -> i32;

    pub fn kstrtouint(s: *const c_char, base: u32, res: *mut u32) -> i32;
    pub fn kstrtoint(s: *const c_char, base: u32, res: *mut i32) -> i32;

    pub fn kstrtou16(s: *const c_char, base: u32, res: *mut u16) -> i32;
    pub fn kstrtos16(s: *const c_char, base: u32, res: *mut i16) -> i32;
    pub fn kstrtou8(s: *const c_char, base: u32, res: *mut u8) -> i32;
    pub fn kstrtos8(s: *const c_char, base: u32, res: *mut i8) -> i32;
    pub fn kstrtobool(s: *const c_char, res: *mut bool) -> i32;

    pub fn kstrtoudec64(s: *const c_char, scale: u32, res: *mut u64) -> i32;
    pub fn kstrtodec64(s: *const c_char, scale: u32, res: *mut i64) -> i32;

    pub fn kstrtoull_from_user(s: *const c_char, count: usize, base: u32, res: *mut c_ulonglong) -> i32;
    pub fn kstrtoll_from_user(s: *const c_char, count: usize, base: u32, res: *mut i64) -> i32;
    pub fn kstrtoul_from_user(s: *const c_char, count: usize, base: u32, res: *mut c_ulong) -> i32;
    pub fn kstrtol_from_user(s: *const c_char, count: usize, base: u32, res: *mut c_long) -> i32;
    pub fn kstrtouint_from_user(s: *const c_char, count: usize, base: u32, res: *mut u32) -> i32;
    pub fn kstrtoint_from_user(s: *const c_char, count: usize, base: u32, res: *mut i32) -> i32;
    pub fn kstrtou16_from_user(s: *const c_char, count: usize, base: u32, res: *mut u16) -> i32;
    pub fn kstrtos16_from_user(s: *const c_char, count: usize, base: u32, res: *mut i16) -> i32;
    pub fn kstrtou8_from_user(s: *const c_char, count: usize, base: u32, res: *mut u8) -> i32;
    pub fn kstrtos8_from_user(s: *const c_char, count: usize, base: u32, res: *mut i8) -> i32;
    pub fn kstrtobool_from_user(s: *const c_char, count: usize, res: *mut bool) -> i32;

    pub fn simple_strtoul(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> c_ulong;
    pub fn simple_strtol(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> c_long;
    pub fn simple_strtoull(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> c_ulonglong;
    pub fn simple_strtoll(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> i64;
}

pub unsafe fn kstrtoul(s: *const c_char, base: u32, res: *mut c_ulong) -> i32 {
    if core::mem::size_of::<c_ulong>() == core::mem::size_of::<c_ulonglong>()
        && core::mem::align_of::<c_ulong>() == core::mem::align_of::<c_ulonglong>()
    {
        kstrtoull(s, base, res.cast::<c_ulonglong>())
    } else {
        _kstrtoul(s, base, res)
    }
}

pub unsafe fn kstrtol(s: *const c_char, base: u32, res: *mut c_long) -> i32 {
    if core::mem::size_of::<c_long>() == core::mem::size_of::<i64>()
        && core::mem::align_of::<c_long>() == core::mem::align_of::<i64>()
    {
        kstrtoll(s, base, res.cast::<i64>())
    } else {
        _kstrtol(s, base, res)
    }
}

pub unsafe fn kstrtou64(s: *const c_char, base: u32, res: *mut u64) -> i32 { kstrtoull(s, base, res.cast()) }
pub unsafe fn kstrtos64(s: *const c_char, base: u32, res: *mut i64) -> i32 { kstrtoll(s, base, res) }
pub unsafe fn kstrtou32(s: *const c_char, base: u32, res: *mut u32) -> i32 { kstrtouint(s, base, res) }
pub unsafe fn kstrtos32(s: *const c_char, base: u32, res: *mut i32) -> i32 { kstrtoint(s, base, res) }

pub unsafe fn kstrtou64_from_user(s: *const c_char, count: usize, base: u32, res: *mut u64) -> i32 { kstrtoull_from_user(s, count, base, res.cast()) }
pub unsafe fn kstrtos64_from_user(s: *const c_char, count: usize, base: u32, res: *mut i64) -> i32 { kstrtoll_from_user(s, count, base, res) }
pub unsafe fn kstrtou32_from_user(s: *const c_char, count: usize, base: u32, res: *mut u32) -> i32 { kstrtouint_from_user(s, count, base, res) }
pub unsafe fn kstrtos32_from_user(s: *const c_char, count: usize, base: u32, res: *mut i32) -> i32 { kstrtoint_from_user(s, count, base, res) }

/*
 * Use kstrto<foo> instead.
 *
 * NOTE: simple_strto<foo> does not check for the range overflow and,
 *      depending on the input, may give interesting results.
 *
 * Use these functions if and only if you cannot use kstrto<foo>, because
 * the conversion ends on the first non-digit character, which may be far
 * beyond the supported range. It might be useful to parse the strings like
 * 10x50 or 12:21 without altering original string or temporary buffer in use.
 * Keep in mind above caveat.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
