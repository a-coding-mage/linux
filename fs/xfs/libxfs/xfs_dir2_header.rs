/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Translated from xfs_dir2.h. Definitions supplied by included headers remain external.

#[repr(C)]
pub struct xfs_da_args;
#[repr(C)]
pub struct xfs_inode;
#[repr(C)]
pub struct xfs_mount;
#[repr(C)]
pub struct xfs_trans;
#[repr(C)]
pub struct xfs_dir2_sf_hdr;
#[repr(C)]
pub struct xfs_dir2_sf_entry;
#[repr(C)]
pub struct xfs_dir2_data_hdr;
#[repr(C)]
pub struct xfs_dir2_data_entry;
#[repr(C)]
pub struct xfs_dir2_data_unused;
#[repr(C)]
pub struct xfs_dir3_icfree_hdr;
#[repr(C)]
pub struct xfs_dir3_icleaf_hdr;

extern "C" {
    pub static xfs_name_dotdot: xfs_name;
    pub static xfs_name_dot: xfs_name;
}

#[inline]
pub unsafe fn xfs_dir2_samename(n1: *const xfs_name, n2: *const xfs_name) -> bool {
    if n1 == n2 { return true; }
    if (*n1).len != (*n2).len { return false; }
    libc::memcmp((*n1).name as *const _, (*n2).name as *const _, (*n1).len as usize) == 0
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xfs_dir2_fmt {
    XFS_DIR2_FMT_SF,
    XFS_DIR2_FMT_BLOCK,
    XFS_DIR2_FMT_LEAF,
    XFS_DIR2_FMT_NODE,
    XFS_DIR2_FMT_ERROR,
}

pub type xfs_dir2_off_t = u64;
pub type xfs_dir2_dataptr_t = u32;
pub type xfs_dir2_db_t = u32;
pub type xfs_dablk_t = u32;
pub type xfs_dir2_data_aoff_t = u16;
pub type xfs_ino_t = u64;
pub type xfs_extlen_t = u32;
pub type xfs_failaddr_t = *const core::ffi::c_void;

#[repr(C)] pub struct xfs_name { pub name: *const u8, pub len: u8 }
#[repr(C)] pub struct xfs_buf;
#[repr(C)] pub struct xfs_buf_ops;
#[repr(C)] pub struct xfs_da_geometry { pub blklog: u8, pub fsblog: u8, pub blksize: u32 }
#[repr(C)] pub struct xfs_dir2_data_free;
#[repr(C)] pub struct xfs_dir2_block_tail;
#[repr(C)] pub struct xfs_dir2_leaf_tail;
#[repr(C)] pub struct xfs_dir2_leaf;
#[repr(C)] pub struct xfs_parent_args;

extern "C" {
    pub fn xfs_dir2_format(args: *mut xfs_da_args, error: *mut i32) -> xfs_dir2_fmt;
    pub fn xfs_mode_to_ftype(mode: i32) -> u8;
    pub fn xfs_dir_startup();
    pub fn xfs_da_mount(mp: *mut xfs_mount) -> i32;
    pub fn xfs_da_unmount(mp: *mut xfs_mount);
    pub fn xfs_dir_init(tp: *mut xfs_trans, dp: *mut xfs_inode, pdp: *mut xfs_inode) -> i32;
    pub fn xfs_dir_createname(tp: *mut xfs_trans, dp: *mut xfs_inode, name: *const xfs_name, inum: xfs_ino_t, tot: xfs_extlen_t) -> i32;
    pub fn xfs_dir_lookup(tp: *mut xfs_trans, dp: *mut xfs_inode, name: *const xfs_name, inum: *mut xfs_ino_t, ci_name: *mut xfs_name) -> i32;
    pub fn xfs_dir_removename(tp: *mut xfs_trans, dp: *mut xfs_inode, name: *const xfs_name, ino: xfs_ino_t, tot: xfs_extlen_t) -> i32;
    pub fn xfs_dir_replace(tp: *mut xfs_trans, dp: *mut xfs_inode, name: *const xfs_name, inum: xfs_ino_t, tot: xfs_extlen_t) -> i32;
    pub fn xfs_dir_canenter(tp: *mut xfs_trans, dp: *mut xfs_inode, name: *const xfs_name) -> i32;
    pub fn xfs_dir_lookup_args(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir_createname_args(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir_removename_args(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir_replace_args(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_sf_to_block(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_shrink_inode(args: *mut xfs_da_args, db: xfs_dir2_db_t, bp: *mut xfs_buf) -> i32;
    pub fn xfs_dir2_data_freescan(mp: *mut xfs_mount, hdr: *mut xfs_dir2_data_hdr, loghead: *mut i32);
    pub fn xfs_dir2_data_log_entry(args: *mut xfs_da_args, bp: *mut xfs_buf, dep: *mut xfs_dir2_data_entry);
    pub fn xfs_dir2_data_log_header(args: *mut xfs_da_args, bp: *mut xfs_buf);
    pub fn xfs_dir2_data_log_unused(args: *mut xfs_da_args, bp: *mut xfs_buf, dup: *mut xfs_dir2_data_unused);
    pub fn xfs_dir2_data_make_free(args: *mut xfs_da_args, bp: *mut xfs_buf, offset: xfs_dir2_data_aoff_t, len: xfs_dir2_data_aoff_t, needlogp: *mut i32, needscanp: *mut i32);
    pub fn xfs_dir2_data_use_free(args: *mut xfs_da_args, bp: *mut xfs_buf, dup: *mut xfs_dir2_data_unused, offset: xfs_dir2_data_aoff_t, len: xfs_dir2_data_aoff_t, needlogp: *mut i32, needscanp: *mut i32) -> i32;
    pub fn xfs_dir2_data_freefind(hdr: *mut xfs_dir2_data_hdr, bf: *mut xfs_dir2_data_free, dup: *mut xfs_dir2_data_unused) -> *mut xfs_dir2_data_free;
    pub fn xfs_dir_ino_validate(mp: *mut xfs_mount, ino: xfs_ino_t) -> i32;
    pub fn xfs_dir3_leaf_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
    pub fn xfs_dir3_data_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
    pub fn xfs_dir3_block_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
    pub static xfs_dir3_block_buf_ops: xfs_buf_ops;
    pub static xfs_dir3_leafn_buf_ops: xfs_buf_ops;
    pub static xfs_dir3_leaf1_buf_ops: xfs_buf_ops;
    pub static xfs_dir3_free_buf_ops: xfs_buf_ops;
    pub static xfs_dir3_data_buf_ops: xfs_buf_ops;
}

pub const XFS_READDIR_BUFSIZE: u32 = 32768;

#[inline] pub fn xfs_dir2_dataptr_to_byte(dp: xfs_dir2_dataptr_t) -> xfs_dir2_off_t { (dp as xfs_dir2_off_t) << XFS_DIR2_DATA_ALIGN_LOG }
#[inline] pub fn xfs_dir2_byte_to_dataptr(by: xfs_dir2_off_t) -> xfs_dir2_dataptr_t { (by >> XFS_DIR2_DATA_ALIGN_LOG) as xfs_dir2_dataptr_t }
#[inline] pub unsafe fn xfs_dir2_byte_to_db(geo: *const xfs_da_geometry, by: xfs_dir2_off_t) -> xfs_dir2_db_t { (by >> (*geo).blklog) as xfs_dir2_db_t }
#[inline] pub unsafe fn xfs_dir2_dataptr_to_db(geo: *const xfs_da_geometry, dp: xfs_dir2_dataptr_t) -> xfs_dir2_db_t { xfs_dir2_byte_to_db(geo, xfs_dir2_dataptr_to_byte(dp)) }
#[inline] pub unsafe fn xfs_dir2_byte_to_off(geo: *const xfs_da_geometry, by: xfs_dir2_off_t) -> xfs_dir2_data_aoff_t { (by & ((*geo).blksize as u64 - 1)) as xfs_dir2_data_aoff_t }
#[inline] pub unsafe fn xfs_dir2_dataptr_to_off(geo: *const xfs_da_geometry, dp: xfs_dir2_dataptr_t) -> xfs_dir2_data_aoff_t { xfs_dir2_byte_to_off(geo, xfs_dir2_dataptr_to_byte(dp)) }
#[inline] pub unsafe fn xfs_dir2_db_off_to_byte(geo: *const xfs_da_geometry, db: xfs_dir2_db_t, o: xfs_dir2_data_aoff_t) -> xfs_dir2_off_t { ((db as u64) << (*geo).blklog) + o as u64 }
#[inline] pub unsafe fn xfs_dir2_db_to_da(geo: *const xfs_da_geometry, db: xfs_dir2_db_t) -> xfs_dablk_t { (db << ((*geo).blklog - (*geo).fsblog)) as xfs_dablk_t }
#[inline] pub unsafe fn xfs_dir2_byte_to_da(geo: *const xfs_da_geometry, by: xfs_dir2_off_t) -> xfs_dablk_t { xfs_dir2_db_to_da(geo, xfs_dir2_byte_to_db(geo, by)) }
#[inline] pub unsafe fn xfs_dir2_db_off_to_dataptr(geo: *const xfs_da_geometry, db: xfs_dir2_db_t, o: xfs_dir2_data_aoff_t) -> xfs_dir2_dataptr_t { xfs_dir2_byte_to_dataptr(xfs_dir2_db_off_to_byte(geo, db, o)) }
#[inline] pub unsafe fn xfs_dir2_da_to_db(geo: *const xfs_da_geometry, da: xfs_dablk_t) -> xfs_dir2_db_t { (da >> ((*geo).blklog - (*geo).fsblog)) as xfs_dir2_db_t }
#[inline] pub unsafe fn xfs_dir2_da_to_byte(geo: *const xfs_da_geometry, da: xfs_dablk_t) -> xfs_dir2_off_t { xfs_dir2_db_off_to_byte(geo, xfs_dir2_da_to_db(geo, da), 0) }

#[inline] pub unsafe fn xfs_dir2_block_tail_p(geo: *mut xfs_da_geometry, hdr: *mut xfs_dir2_data_hdr) -> *mut xfs_dir2_block_tail { (hdr as *mut u8).add((*geo).blksize as usize) as *mut xfs_dir2_block_tail - 1 }
#[inline] pub unsafe fn xfs_dir2_leaf_tail_p(geo: *mut xfs_da_geometry, lp: *mut xfs_dir2_leaf) -> *mut xfs_dir2_leaf_tail { (lp as *mut u8).add((*geo).blksize as usize - core::mem::size_of::<xfs_dir2_leaf_tail>()) as *mut xfs_dir2_leaf_tail }

extern "C" {
    pub fn xfs_dir3_get_dtype(mp: *mut xfs_mount, filetype: u8) -> u8;
    pub fn xfs_dir3_data_end_offset(geo: *mut xfs_da_geometry, hdr: *mut xfs_dir2_data_hdr) -> u32;
    pub fn xfs_dir2_namecheck(name: *const core::ffi::c_void, length: usize) -> bool;
}

#[inline] pub fn xfs_ascii_ci_need_xfrm(c: u8) -> bool { (0x41..=0x5a).contains(&c) || (0xc0..=0xd6).contains(&c) || (0xd8..=0xde).contains(&c) }
#[inline] pub fn xfs_ascii_ci_xfrm(mut c: u8) -> u8 { if xfs_ascii_ci_need_xfrm(c) { c -= b'A' - b'a'; } c }

#[repr(C)] pub struct xfs_dir_update_params { pub dp: *const xfs_inode, pub ip: *const xfs_inode, pub name: *const xfs_name, pub delta: i32 }
#[repr(C)] pub struct xfs_dir_update { pub dp: *mut xfs_inode, pub name: *const xfs_name, pub ip: *mut xfs_inode, pub ppargs: *mut xfs_parent_args }

extern "C" {
    pub fn xfs_dir_create_child(tp: *mut xfs_trans, resblks: u32, du: *mut xfs_dir_update) -> i32;
    pub fn xfs_dir_add_child(tp: *mut xfs_trans, resblks: u32, du: *mut xfs_dir_update) -> i32;
    pub fn xfs_dir_remove_child(tp: *mut xfs_trans, resblks: u32, du: *mut xfs_dir_update) -> i32;
    pub fn xfs_dir_exchange_children(tp: *mut xfs_trans, du1: *mut xfs_dir_update, du2: *mut xfs_dir_update, spaceres: u32) -> i32;
    pub fn xfs_dir_rename_children(tp: *mut xfs_trans, du_src: *mut xfs_dir_update, du_tgt: *mut xfs_dir_update, spaceres: u32, du_wip: *mut xfs_dir_update) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
