/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// referenced here rather than reimplemented.

use core::ffi::c_void;

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct task_struct;

extern "C" {
    fn memchr(s: *const c_void, c: i32, n: usize) -> *mut c_void;
    fn strlen(s: *const i8) -> usize;
    fn toupper(c: i32) -> i32;
    fn tolower(c: i32) -> i32;

    pub fn string_get_size(
        size: u64,
        blk_size: u64,
        units: string_size_units,
        buf: *mut i8,
        len: i32,
    ) -> i32;
    pub fn parse_int_array(buf: *const i8, count: usize, array: *mut *mut i32) -> i32;
    pub fn parse_int_array_user(from: *const i8, count: usize, array: *mut *mut i32) -> i32;
    pub fn string_unescape(src: *mut i8, dst: *mut i8, size: usize, flags: u32) -> i32;
    pub fn string_escape_mem(
        src: *const i8,
        isz: usize,
        dst: *mut i8,
        osz: usize,
        flags: u32,
        only: *const i8,
    ) -> i32;
    pub fn kstrdup_quotable(src: *const i8, gfp: gfp_t) -> *mut i8;
    pub fn kstrdup_quotable_cmdline(task: *mut task_struct, gfp: gfp_t) -> *mut i8;
    pub fn kstrdup_quotable_file(file: *mut file, gfp: gfp_t) -> *mut i8;
    pub fn kstrdup_and_replace(src: *const i8, old: i8, new: i8, gfp: gfp_t) -> *mut i8;
    pub fn kasprintf_strarray(gfp: gfp_t, prefix: *const i8, n: usize) -> *mut *mut i8;
    pub fn kfree_strarray(array: *mut *mut i8, n: usize);
    pub fn devm_kasprintf_strarray(
        dev: *mut device,
        prefix: *const i8,
        n: usize,
    ) -> *mut *mut i8;
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum string_size_units {
    STRING_UNITS_10 = 0,
    STRING_UNITS_2 = 1,
}
pub const STRING_UNITS_MASK: u32 = 1 << 0;
pub const STRING_UNITS_NO_SPACE: u32 = 1 << 30;
pub const STRING_UNITS_NO_BYTES: u32 = 1 << 31;

pub unsafe fn string_is_terminated(s: *const i8, len: i32) -> bool {
    memchr(s as *const c_void, 0, len as usize) != core::ptr::null_mut()
}

pub const UNESCAPE_SPACE: u32 = 1 << 0;
pub const UNESCAPE_OCTAL: u32 = 1 << 1;
pub const UNESCAPE_HEX: u32 = 1 << 2;
pub const UNESCAPE_SPECIAL: u32 = 1 << 3;
pub const UNESCAPE_ANY: u32 = UNESCAPE_SPACE | UNESCAPE_OCTAL | UNESCAPE_HEX | UNESCAPE_SPECIAL;
pub const UNESCAPE_ALL_MASK: u32 = 0xF;

pub unsafe fn string_unescape_inplace(buf: *mut i8, flags: u32) -> i32 {
    string_unescape(buf, buf, 0, flags)
}

pub unsafe fn string_unescape_any(src: *mut i8, dst: *mut i8, size: usize) -> i32 {
    string_unescape(src, dst, size, UNESCAPE_ANY)
}

pub unsafe fn string_unescape_any_inplace(buf: *mut i8) -> i32 {
    string_unescape_any(buf, buf, 0)
}

pub const ESCAPE_SPACE: u32 = 1 << 0;
pub const ESCAPE_SPECIAL: u32 = 1 << 1;
pub const ESCAPE_NULL: u32 = 1 << 2;
pub const ESCAPE_OCTAL: u32 = 1 << 3;
pub const ESCAPE_ANY: u32 = ESCAPE_SPACE | ESCAPE_OCTAL | ESCAPE_SPECIAL | ESCAPE_NULL;
pub const ESCAPE_NP: u32 = 1 << 4;
pub const ESCAPE_ANY_NP: u32 = ESCAPE_ANY | ESCAPE_NP;
pub const ESCAPE_HEX: u32 = 1 << 5;
pub const ESCAPE_NA: u32 = 1 << 6;
pub const ESCAPE_NAP: u32 = 1 << 7;
pub const ESCAPE_APPEND: u32 = 1 << 8;
pub const ESCAPE_ALL_MASK: u32 = 0x1FF;

pub unsafe fn string_escape_mem_any_np(
    src: *const i8, isz: usize, dst: *mut i8, osz: usize, only: *const i8,
) -> i32 {
    string_escape_mem(src, isz, dst, osz, ESCAPE_ANY_NP, only)
}

pub unsafe fn string_escape_str(
    src: *const i8, dst: *mut i8, sz: usize, flags: u32, only: *const i8,
) -> i32 {
    string_escape_mem(src, strlen(src), dst, sz, flags, only)
}

pub unsafe fn string_escape_str_any_np(
    src: *const i8, dst: *mut i8, sz: usize, only: *const i8,
) -> i32 {
    string_escape_str(src, dst, sz, ESCAPE_ANY_NP, only)
}

pub unsafe fn string_upper(mut dst: *mut i8, mut src: *const i8) {
    loop {
        *dst = toupper(*src as i32) as i8;
        dst = dst.add(1);
        let value = *src;
        src = src.add(1);
        if value == 0 { break; }
    }
}

pub unsafe fn string_lower(mut dst: *mut i8, mut src: *const i8) {
    loop {
        *dst = tolower(*src as i32) as i8;
        dst = dst.add(1);
        let value = *src;
        src = src.add(1);
        if value == 0 { break; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
