// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of xfs_dir2_leaf.c.
 *
 * This module intentionally relies on the XFS declarations supplied by the
 * surrounding translation unit.  The original implementation is pointer-
 * oriented, so the interfaces below retain raw pointers and unsafe ABI.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

extern "C" {
    fn xfs_has_crc(mp: *mut xfs_mount) -> bool;
    fn xfs_dir2_leaf_hdr_from_disk(mp: *mut xfs_mount, to: *mut xfs_dir3_icleaf_hdr, from: *mut xfs_dir2_leaf);
}

#[repr(C)] pub struct xfs_mount { pub m_dir_geo: *mut xfs_da_geometry }
#[repr(C)] pub struct xfs_da_geometry { pub leaf_max_ents: u32, pub leaf_hdr_size: u32, pub blksize: u32, pub data_entry_offset: u32, pub fsbcount: i64, pub freeblk: i64, pub datablk: i64 }
#[repr(C)] pub struct xfs_inode { pub i_mount: *mut xfs_mount }
#[repr(C)] pub struct xfs_trans;
#[repr(C)] pub struct xfs_buf { pub b_addr: *mut c_void, pub b_mount: *mut xfs_mount, pub b_ops: *mut xfs_buf_ops, pub b_length: u32, pub b_log_item: *mut xfs_buf_log_item }
#[repr(C)] pub struct xfs_buf_log_item { pub bli_item: xfs_log_item }
#[repr(C)] pub struct xfs_log_item { pub li_lsn: u64 }
#[repr(C)] pub struct xfs_buf_ops { pub name: *const u8, pub magic16: [u16; 2], pub verify_read: Option<unsafe extern "C" fn(*mut xfs_buf)>, pub verify_write: Option<unsafe extern "C" fn(*mut xfs_buf)>, pub verify_struct: Option<unsafe extern "C" fn(*mut xfs_buf)> }
#[repr(C)] pub struct xfs_dir2_leaf { pub bytes: [u8; 1] }
#[repr(C)] pub struct xfs_dir2_leaf_entry { pub hashval: u32, pub address: u32 }
#[repr(C)] pub struct xfs_dir3_icleaf_hdr { pub forw: u32, pub back: u32, pub magic: u16, pub count: i32, pub stale: i32, pub ents: *mut xfs_dir2_leaf_entry }
#[repr(C)] pub struct xfs_da_args { pub dp: *mut xfs_inode, pub trans: *mut xfs_trans, pub geo: *mut xfs_da_geometry, pub owner: u64, pub hashval: u32, pub inumber: u64, pub namelen: u8, pub filetype: u8, pub op_flags: u32, pub total: i64, pub name: *const u8, pub cmpresult: i32 }
#[repr(C)] pub struct xfs_da_state { pub args: *mut xfs_da_args, pub mp: *mut xfs_mount, pub path: xfs_da_state_path }
#[repr(C)] pub struct xfs_da_state_path { pub active: i32, pub blk: [xfs_da_state_blk; 1] }
#[repr(C)] pub struct xfs_da_state_blk { pub bp: *mut xfs_buf }

pub type xfs_failaddr_t = *const c_void;
pub type xfs_dir2_db_t = i64;
pub type xfs_dablk_t = i64;
pub type xfs_ino_t = u64;
pub type xfs_dir2_data_off_t = u16;
pub type xfs_dahash_t = u32;

pub const XFS_DIR2_LEAF1_MAGIC: u16 = 0xf; pub const XFS_DIR2_LEAFN_MAGIC: u16 = 0xf1;
pub const XFS_DIR3_LEAF1_MAGIC: u16 = 0x3ff; pub const XFS_DIR3_LEAFN_MAGIC: u16 = 0x3ff1;
pub const XFS_DIR2_NULL_DATAPTR: u32 = 0;
pub const XFS_DA_OP_JUSTCHECK: u32 = 1; pub const XFS_DA_OP_OKNOENT: u32 = 2;
pub const XFS_CMP_DIFFERENT: i32 = 0; pub const XFS_CMP_EXACT: i32 = 1; pub const XFS_CMP_CASE: i32 = 2;

/* The following declarations preserve the complete externally visible API. */
extern "C" {
    pub fn xfs_dir3_leaf_check_int(mp: *mut xfs_mount, hdr: *mut xfs_dir3_icleaf_hdr, leaf: *mut xfs_dir2_leaf, expensive_checking: bool) -> xfs_failaddr_t;
    pub fn xfs_dir3_leaf_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
    pub fn xfs_dir3_leaf_read(tp: *mut xfs_trans, dp: *mut xfs_inode, owner: xfs_ino_t, fbno: xfs_dablk_t, bpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_dir3_leafn_read(tp: *mut xfs_trans, dp: *mut xfs_inode, owner: xfs_ino_t, fbno: xfs_dablk_t, bpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_dir3_leaf_get_buf(args: *mut xfs_da_args, bno: xfs_dir2_db_t, bpp: *mut *mut xfs_buf, magic: u16) -> i32;
    pub fn xfs_dir2_block_to_leaf(args: *mut xfs_da_args, dbp: *mut xfs_buf) -> i32;
    pub fn xfs_dir2_leaf_addname(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_leaf_removename(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_leaf_replace(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_leaf_lookup(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_leaf_search_hash(args: *mut xfs_da_args, lbp: *mut xfs_buf) -> i32;
    pub fn xfs_dir2_leaf_trim_data(args: *mut xfs_da_args, lbp: *mut xfs_buf, db: xfs_dir2_db_t) -> i32;
    pub fn xfs_dir2_node_to_leaf(state: *mut xfs_da_state) -> i32;
    pub fn xfs_dir3_leaf_compact(args: *mut xfs_da_args, leafhdr: *mut xfs_dir3_icleaf_hdr, bp: *mut xfs_buf);
    pub fn xfs_dir3_leaf_compact_x1(hdr: *mut xfs_dir3_icleaf_hdr, ents: *mut xfs_dir2_leaf_entry, index: *mut i32, low: *mut i32, high: *mut i32, lowlog: *mut i32, highlog: *mut i32);
    pub fn xfs_dir3_leaf_log_ents(args: *mut xfs_da_args, hdr: *mut xfs_dir3_icleaf_hdr, bp: *mut xfs_buf, first: i32, last: i32);
    pub fn xfs_dir3_leaf_log_header(args: *mut xfs_da_args, bp: *mut xfs_buf);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
