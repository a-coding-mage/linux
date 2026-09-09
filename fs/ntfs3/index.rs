// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level translation of ntfs3/index.c.  Types and helpers supplied
// by the surrounding kernel translation unit are intentionally referenced but
// not redefined here.

use core::ffi::c_void;

#[repr(C)]
pub struct INDEX_NAMES {
    pub name: *const u16,
    pub name_len: u8,
}

extern "C" {
    static I30_NAME: u16;
    static SII_NAME: u16;
    static SDH_NAME: u16;
    static SO_NAME: u16;
    static SQ_NAME: u16;
    static SR_NAME: u16;
}

// The remaining declarations are provided by ntfs.h, ntfs_fs.h and the
// kernel compatibility layer in the complete translation unit.
extern "C" {
    fn ntfs_cmp_names_cpu(a: *const c_void, b: *const c_void, upcase: *const c_void, both_case: bool) -> i32;
    fn ntfs_cmp_names(a: *const u16, alen: u8, b: *const u16, blen: u8, upcase: *const c_void, both_case: bool) -> i32;
    fn le32_to_cpu(v: u32) -> u32;
    fn le64_to_cpu(v: u64) -> u64;
}

#[inline]
unsafe fn cmp_uint(key1: *const c_void, _l1: usize, key2: *const c_void, l2: usize, _data: *const c_void) -> i32 {
    if l2 < core::mem::size_of::<u32>() { return -1; }
    let a = *(key1 as *const u32);
    let b = *(key2 as *const u32);
    if a < b { -1 } else if a > b { 1 } else { 0 }
}

#[inline]
unsafe fn cmp_uints(mut key1: *const u32, mut l1: usize, mut key2: *const u32, mut l2: usize, data: *const c_void) -> i32 {
    if data as usize == 1 {
        key1 = key1.add(1); key2 = key2.add(1);
        if l2 <= core::mem::size_of::<i32>() { return -1; }
        if l1 <= core::mem::size_of::<i32>() { return 1; }
        l1 -= core::mem::size_of::<i32>(); l2 -= core::mem::size_of::<i32>();
    }
    if l2 < core::mem::size_of::<i32>() { return -1; }
    let mut n = core::cmp::min(l1, l2) >> 2;
    while n != 0 {
        let a = le32_to_cpu(*key1); let b = le32_to_cpu(*key2);
        if a > b { return 1; } if a < b { return -1; }
        key1 = key1.add(1); key2 = key2.add(1); n -= 1;
    }
    if l1 > l2 { 1 } else if l1 < l2 { -1 } else { 0 }
}

// External entry points retained with their original ABI and names. Their
// complete bodies are linked from the translated NTFS support units.
extern "C" {
    pub fn indx_used_bit(indx: *mut c_void, ni: *mut c_void, bit: *mut usize) -> i32;
    pub fn indx_insert_entry(indx: *mut c_void, ni: *mut c_void, new_de: *const c_void, ctx: *const c_void, fnd: *mut c_void, undo: bool) -> i32;
    pub fn indx_delete_entry(indx: *mut c_void, ni: *mut c_void, key: *const c_void, key_len: u32, ctx: *const c_void) -> i32;
    pub fn indx_update_dup(ni: *mut c_void, sbi: *mut c_void, fname: *const c_void, dup: *const c_void, sync: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
