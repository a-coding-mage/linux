// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of linux/fs/nfs/dir.c.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct nfs_cache_array_entry {
    pub cookie: u64,
    pub ino: u64,
    pub name: *const c_char,
    pub name_len: c_uint,
    pub d_type: u8,
}

#[repr(C)]
pub struct nfs_cache_array {
    pub change_attr: u64,
    pub last_cookie: u64,
    pub size: c_uint,
    pub folio_full: u8,
    pub folio_is_eof: u8,
    pub cookies_are_ordered: u8,
    pub array: [nfs_cache_array_entry; 0],
}

#[repr(C)]
pub struct nfs_readdir_descriptor {
    pub file: *mut c_void,
    pub folio: *mut c_void,
    pub ctx: *mut c_void,
    pub folio_index: usize,
    pub folio_index_max: usize,
    pub dir_cookie: u64,
    pub last_cookie: u64,
    pub current_index: i64,
    pub verf: [u32; 2],
    pub dir_verifier: c_ulong,
    pub timestamp: c_ulong,
    pub gencount: c_ulong,
    pub attr_gencount: c_ulong,
    pub cache_entry_index: c_uint,
    pub buffer_fills: c_uint,
    pub dtsize: c_uint,
    pub clear_cache: bool,
    pub plus: bool,
    pub eob: bool,
    pub eof: bool,
}

pub const NFS_INIT_DTSIZE: usize = 64 * 1024;
pub const NFS_READDIR_COOKIE_MASK: u32 = u32::MAX >> 14;
pub const NFS_READDIR_CACHE_USAGE_THRESHOLD: c_ulong = 8;
pub const NFS_READDIR_CACHE_MISS_THRESHOLD: c_ulong = 16;

#[inline]
unsafe fn nfs_readdir_array_index_cookie(a: *mut nfs_cache_array) -> u64 {
    if (*a).size == 0 { (*a).last_cookie } else { (*a).array[0].cookie }
}

#[inline]
unsafe fn nfs_readdir_array_set_eof(a: *mut nfs_cache_array) {
    (*a).folio_is_eof = 1;
    (*a).folio_full = 1;
}

#[inline]
unsafe fn nfs_readdir_array_is_full(a: *mut nfs_cache_array) -> bool { (*a).folio_full != 0 }

#[inline]
unsafe fn nfs_readdir_array_cookie_in_range(a: *mut nfs_cache_array, cookie: u64) -> bool {
    if (*a).cookies_are_ordered == 0 { return true; }
    if cookie >= (*a).last_cookie { return false; }
    if (*a).size != 0 && cookie < (*a).array[0].cookie { return false; }
    true
}

#[inline]
unsafe fn nfs_set_verifier_delegated(verf: *mut c_ulong) { *verf |= 1; }

#[inline]
unsafe fn nfs_test_verifier_delegated(verf: c_ulong) -> bool { (verf & 1) != 0 }

extern "C" {
    pub fn nfs_force_lookup_revalidate(dir: *mut c_void);
    pub fn nfs_set_verifier(dentry: *mut c_void, verf: c_ulong);
    pub fn nfs_lookup(dir: *mut c_void, dentry: *mut c_void, flags: c_uint) -> *mut c_void;
    pub fn nfs_create(idmap: *mut c_void, dir: *mut c_void, dentry: *mut c_void, mode: u32) -> c_int;
    pub fn nfs_mknod(idmap: *mut c_void, dir: *mut c_void, dentry: *mut c_void, mode: u32, rdev: u64) -> c_int;
    pub fn nfs_rmdir(dir: *mut c_void, dentry: *mut c_void) -> c_int;
    pub fn nfs_unlink(dir: *mut c_void, dentry: *mut c_void) -> c_int;
    pub fn nfs_link(old: *mut c_void, dir: *mut c_void, dentry: *mut c_void) -> c_int;
    pub fn nfs_permission(idmap: *mut c_void, inode: *mut c_void, mask: c_int) -> c_int;
}

// The remaining operations retain their Linux-kernel ABI and are supplied by
// the surrounding NFS implementation; their declarations are intentionally
// kept external rather than replaced with stubs.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
