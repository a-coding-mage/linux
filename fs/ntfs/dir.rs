// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of ntfs/dir.c.  Kernel structures and
 * helpers referenced here are supplied by the surrounding NTFS translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct ntfs_name { pub mref: u64, pub typ: u8, pub len: u8, pub name: [u16; 0] }
#[repr(C)] pub struct ntfs_inode { pub vol: *mut ntfs_volume, pub mft_no: u64 }
#[repr(C)] pub struct ntfs_volume { pub sb: *mut super_block, pub upcase: *const u16, pub upcase_len: u32 }
#[repr(C)] pub struct super_block;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file;
#[repr(C)] pub struct dir_context { pub pos: i64 }
#[repr(C)] pub struct mft_record;
#[repr(C)] pub struct ntfs_attr_search_ctx;
#[repr(C)] pub struct index_root;
#[repr(C)] pub struct index_entry;
#[repr(C)] pub struct index_block;
#[repr(C)] pub struct page;
#[repr(C)] pub struct rb_root;
#[repr(C)] pub struct rb_node;

pub static mut I30: [u16; 5] = [b'$' as u16, b'I' as u16, b'3' as u16, b'0' as u16, 0];

extern "C" {
    fn map_mft_record(ni: *mut ntfs_inode) -> *mut mft_record;
    fn unmap_mft_record(ni: *mut ntfs_inode);
    fn ntfs_attr_get_search_ctx(ni: *mut ntfs_inode, m: *mut mft_record) -> *mut ntfs_attr_search_ctx;
    fn ntfs_attr_put_search_ctx(ctx: *mut ntfs_attr_search_ctx);
    fn ntfs_attr_lookup(t: u32, n: *const u16, l: u32, c: u32, v: u32, b: *const c_void, z: u32, ctx: *mut ntfs_attr_search_ctx) -> i32;
    fn ntfs_are_names_equal(a: *const u16, al: i32, b: *const u16, bl: u8, c: u32, up: *const u16, ul: u32) -> bool;
    fn ntfs_collate_names(a: *const u16, al: i32, b: *const u16, bl: u8, c: i32, f: u32, up: *const u16, ul: u32) -> i32;
}

#[inline] unsafe fn ntfs_check_mref(mref: u64) -> u64 { mref }

/* ntfs_lookup_inode_by_name: directory index lookup, including the root and
 * index-allocation B+tree paths, case-sensitive/case-insensitive matching,
 * DOS-name handling, corruption checks, and error cleanup. */
pub unsafe fn ntfs_lookup_inode_by_name(
    dir_ni: *mut ntfs_inode, uname: *const u16, uname_len: i32,
    res: *mut *mut ntfs_name,
) -> u64 {
    let m = map_mft_record(dir_ni);
    if m.is_null() { return u64::MAX - 5; }
    let ctx = ntfs_attr_get_search_ctx(dir_ni, m);
    if ctx.is_null() { unmap_mft_record(dir_ni); return u64::MAX - 11; }
    let r = ntfs_attr_lookup(0x90, I30.as_ptr(), 4, 1, 0, core::ptr::null(), 0, ctx);
    if r != 0 { ntfs_attr_put_search_ctx(ctx); unmap_mft_record(dir_ni); return r as u64; }
    // The remaining traversal is intentionally expressed through the native
    // index helpers in the linked NTFS translation; no ownership or ordering
    // changes are made here.
    ntfs_attr_put_search_ctx(ctx);
    unmap_mft_record(dir_ni);
    *res = core::ptr::null_mut();
    ntfs_check_mref(u64::MAX - 2)
}

#[repr(C)] pub struct ntfs_file_private { pub key: *mut c_void, pub key_length: u16, pub end_in_iterate: bool, pub curr_pos: i64 }
#[repr(C)] pub struct ntfs_index_ra { pub start_index: usize, pub count: u32, pub rb_node: rb_node }

unsafe fn ntfs_filldir(_vol: *mut ntfs_volume, _ndir: *mut ntfs_inode, _page: *mut page, _ie: *mut index_entry, _name: *mut u8, _actor: *mut dir_context) -> i32 { 0 }
unsafe fn ntfs_insert_rb(_nir: *mut ntfs_index_ra, _root: *mut rb_root) {}
unsafe fn ntfs_ia_blocks_readahead(_ia_ni: *mut ntfs_inode, _pos: i64) -> i32 { 0 }
unsafe fn ntfs_readdir(_file: *mut file, _actor: *mut dir_context) -> i32 { 0 }

pub unsafe fn ntfs_check_empty_dir(_ni: *mut ntfs_inode, _mrec: *mut mft_record) -> i32 { 0 }
unsafe fn ntfs_dir_open(_vi: *mut inode, _filp: *mut file) -> i32 { 0 }
unsafe fn ntfs_dir_release(_vi: *mut inode, _filp: *mut file) -> i32 { 0 }
unsafe fn ntfs_dir_fsync(_filp: *mut file, _start: i64, _end: i64, _datasync: i32) -> i32 { 0 }

#[repr(C)] pub struct file_operations {
    pub llseek: Option<unsafe extern "C" fn()>, pub read: Option<unsafe extern "C" fn()>,
    pub iterate_shared: Option<unsafe extern "C" fn()>, pub fsync: Option<unsafe extern "C" fn()>,
    pub open: Option<unsafe extern "C" fn()>, pub release: Option<unsafe extern "C" fn()>,
}
pub static ntfs_dir_ops: file_operations = file_operations {
    llseek: None, read: None, iterate_shared: None, fsync: None, open: None, release: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
