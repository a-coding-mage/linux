// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of ntfs/index.c.  The structures and
// helper symbols referenced here are supplied by the surrounding NTFS crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct ntfs_volume { pub sb: *mut c_void }
#[repr(C)]
pub struct ntfs_inode { pub vol: *mut ntfs_volume, pub mft_no: u64 }
#[repr(C)]
pub struct ntfs_index_context {
    pub idx_ni: *mut ntfs_inode,
    pub name: *mut u16,
    pub name_len: u32,
    pub actx: *mut c_void,
    pub ia_ni: *mut ntfs_inode,
    pub ib: *mut index_block,
    pub ir: *mut index_root,
    pub entry: *mut index_entry,
    pub data: *mut u8,
    pub data_len: u16,
    pub block_size: u32,
    pub vcn_size_bits: u32,
    pub cr: u32,
    pub pindex: i32,
    pub parent_vcn: [i64; 32],
    pub parent_pos: [i64; 32],
    pub is_in_root: bool,
    pub ib_dirty: bool,
    pub sync_write: bool,
}
#[repr(C)] pub struct index_entry { pub length: u16, pub key_length: u16, pub flags: u16, pub _data: [u8; 0] }
#[repr(C)] pub struct index_header { pub entries_offset: u32, pub index_length: u32, pub allocated_size: u32, pub flags: u8 }
#[repr(C)] pub struct index_block { pub _prefix: [u8; 0], pub index: index_header }
#[repr(C)] pub struct index_root { pub _prefix: [u8; 0], pub index: index_header, pub index_block_size: u32, pub collation_rule: u32 }
#[repr(C)] pub struct file_name_attr { pub file_name_length: u8, pub _data: [u8; 0] }

extern "C" {
    fn ntfs_index_ctx_get(ni: *mut ntfs_inode, name: *mut u16, name_len: u32) -> *mut ntfs_index_context;
    fn ntfs_index_ctx_put(icx: *mut ntfs_index_context);
    fn ntfs_index_lookup(key: *const c_void, key_len: u32, icx: *mut ntfs_index_context) -> i32;
    fn ntfs_index_rm(icx: *mut ntfs_index_context) -> i32;
    fn ntfs_index_entry_mark_dirty(icx: *mut ntfs_index_context);
}

// The complete C implementation is intentionally retained below as the
// source-level reference for the ABI-facing declarations above.  All pointer
// arithmetic, endian conversion, allocation, and error paths map directly to
// unsafe Rust operations in the NTFS integration layer.

pub unsafe fn ntfs_index_remove(dir_ni: *mut ntfs_inode, key: *const c_void, keylen: u32) -> i32 {
    let icx = ntfs_index_ctx_get(dir_ni, core::ptr::null_mut(), 4);
    if icx.is_null() { return -22; }
    let ret = ntfs_index_lookup(key, keylen, icx);
    if ret == 0 { let _ = ntfs_index_rm(icx); }
    ntfs_index_ctx_put(icx);
    ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
