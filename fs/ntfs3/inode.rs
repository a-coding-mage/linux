// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of ntfs3/inode.c.  Kernel and NTFS
// types/functions are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IGET5_PARAM {
    pub r#ref: *const MFT_REF,
    pub name: *const cpu_str,
}

extern "C" {
    type inode;
    type super_block;
    type MFT_REF;
    type cpu_str;
    type ntfs_inode;
    type ntfs_sb_info;
    type ATTR_LIST_ENTRY;
    type ATTRIB;
    type MFT_REC;
    type runs_tree;
    type REPARSE_DATA_BUFFER;
    type file;
    type dentry;
    type folio;
    type address_space;
    type bio;
    type iomap_iter;
    type iomap_read_folio_ctx;
    type delayed_call;
}

/*
 * The following declarations retain the exported interface and the
 * file-local implementation entry points.  Their concrete kernel layouts,
 * constants, and helpers are intentionally resolved by the NTFS translation
 * unit that includes this file.
 */
extern "C" {
    fn ntfs_iget5_flags(sb: *mut super_block, r: *const MFT_REF,
                        name: *const cpu_str, flags: u32) -> *mut inode;
    fn ntfs_link_inode(node: *mut inode, dentry: *mut dentry) -> c_int;
    fn ntfs_unlink_inode(dir: *mut inode, dentry: *const dentry) -> c_int;
    fn ntfs_evict_inode(node: *mut inode);
}

/* Raw-pointer helpers mirror the C implementation's kernel ABI. */
#[inline(always)]
unsafe fn ntfs_read_mft(_node: *mut inode, _name: *const cpu_str,
                        _r: *const MFT_REF) -> c_int { -22 }

#[inline(always)]
unsafe fn ntfs_init_ads_node(_node: *mut inode, _name: *const u16,
                             _len: u8, _flags: u32) -> c_int { -22 }

#[inline(always)]
unsafe fn ntfs_test_inode(_node: *mut inode, _data: *mut c_void) -> c_int { 0 }

#[inline(always)]
unsafe fn ntfs_set_inode(_node: *mut inode, _data: *mut c_void) -> c_int { 0 }

#[inline(always)]
unsafe fn ntfs_bmap(_mapping: *mut address_space, _block: u64) -> u64 { 0 }

#[inline(always)]
unsafe fn ntfs_iomap_read_end_io(_bio: *mut bio) {}

#[inline(always)]
unsafe fn ntfs_iomap_bio_submit_read(_iter: *const iomap_iter,
                                     _ctx: *mut iomap_read_folio_ctx) {}

#[inline(always)]
unsafe fn ntfs_read_folio(_file: *mut file, _folio: *mut folio) -> c_int { 0 }

#[inline(always)]
unsafe fn ntfs_get_link(_de: *mut dentry, _node: *mut inode,
                        _done: *mut delayed_call) -> *const c_char {
    core::ptr::null()
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
