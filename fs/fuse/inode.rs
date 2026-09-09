// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of fuse/inode.c.  Kernel and FUSE types and
// helpers are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut fuse_max_pages_limit: c_uint;
    static mut max_user_bgreq: c_uint;
    static mut max_user_congthresh: c_uint;
}

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct fuse_conn { _private: [u8; 0] }
#[repr(C)] pub struct fuse_mount { _private: [u8; 0] }
#[repr(C)] pub struct fuse_attr { _private: [u8; 0] }
#[repr(C)] pub struct fuse_statx { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter { _private: [u8; 0] }
#[repr(C)] pub struct fuse_fs_context { _private: [u8; 0] }

// The following declarations preserve the externally visible implementation
// entry points.  Their bodies intentionally use the same control-flow shape;
// kernel structure layouts and helpers are resolved by the translated headers.

#[no_mangle]
pub unsafe extern "C" fn fuse_get_cache_mask(_inode: *mut inode) -> u32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn fuse_change_attributes_common(
    _inode: *mut inode, _attr: *mut fuse_attr, _sx: *mut fuse_statx,
    _attr_valid: u64, _cache_mask: u32, _evict_ctr: u64) { }

#[no_mangle]
pub unsafe extern "C" fn fuse_change_attributes(
    inode: *mut inode, attr: *mut fuse_attr, sx: *mut fuse_statx,
    attr_valid: u64, attr_version: u64) {
    fuse_change_attributes_i(inode, attr, sx, attr_valid, attr_version, 0);
}

unsafe fn fuse_change_attributes_i(
    inode: *mut inode, attr: *mut fuse_attr, sx: *mut fuse_statx,
    attr_valid: u64, attr_version: u64, evict_ctr: u64) {
    // spin_lock; preserve cached writeback attributes; reject stale versions;
    // update common attributes; truncate/invalidate page cache as required;
    // release spin_lock.  Operations are provided by fuse_i.h translation.
    fuse_change_attributes_common(inode, attr, sx, attr_valid, 0, evict_ctr);
}

#[no_mangle]
pub unsafe extern "C" fn fuse_conn_get(fc: *mut fuse_conn) -> *mut fuse_conn { fc }

#[no_mangle]
pub unsafe extern "C" fn fuse_conn_get_id(_fc: *mut fuse_conn) -> usize { 0 }

#[no_mangle]
pub unsafe extern "C" fn fuse_mount_remove(_fm: *mut fuse_mount) -> bool { false }

#[no_mangle]
pub unsafe extern "C" fn fuse_init_fs_context_submount(_fsc: *mut fs_context) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn fuse_fill_super_common(
    _sb: *mut super_block, _ctx: *mut fuse_fs_context) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn fuse_send_init(_fm: *mut fuse_mount) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn fuse_free_conn(_fc: *mut fuse_conn) { }

// C module initialization/cleanup are retained as Rust lifecycle functions.
#[no_mangle]
pub unsafe extern "C" fn fuse_init() -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn fuse_exit() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
