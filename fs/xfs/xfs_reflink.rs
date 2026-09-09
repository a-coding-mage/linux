// SPDX-License-Identifier: GPL-2.0+
//
// Faithful low-level Rust interface translation of xfs_reflink.c.
// Kernel/XFS types and helpers are supplied by the surrounding translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_long, c_void};

#[repr(C)]
pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_inode { pub i_mount: *mut xfs_mount, pub i_cowfp: *mut xfs_ifork, pub i_diflags2: u64, pub i_disk_size: i64, pub i_cowextsize: u32 }
#[repr(C)]
pub struct xfs_ifork { pub if_bytes: usize }
#[repr(C)]
pub struct xfs_iext_cursor { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_btree_cur { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_buf { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_perag { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_rtgroup { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_bmbt_irec { pub br_startoff: u64, pub br_startblock: u64, pub br_blockcount: u64, pub br_state: u32 }

pub type xfs_extlen_t = u64;
pub type xfs_fileoff_t = u64;
pub type xfs_filblks_t = u64;
pub type xfs_fsblock_t = u64;
pub type xfs_off_t = i64;
pub type xfs_agblock_t = u32;
pub type xfs_rgblock_t = u32;
pub type xfs_agnumber_t = u32;
pub type loff_t = i64;
pub type uint = u32;

/* The following declarations retain the externally visible implementation
 * entry points.  Their XFS helper operations are external dependencies. */
extern "C" {
    pub fn xfs_reflink_trim_around_shared(ip: *mut xfs_inode, irec: *mut xfs_bmbt_irec, shared: *mut bool) -> c_int;
    pub fn xfs_bmap_trim_cow(ip: *mut xfs_inode, imap: *mut xfs_bmbt_irec, shared: *mut bool) -> c_int;
    pub fn xfs_reflink_convert_cow_locked(ip: *mut xfs_inode, offset_fsb: xfs_fileoff_t, count_fsb: xfs_filblks_t) -> c_int;
    pub fn xfs_reflink_convert_cow(ip: *mut xfs_inode, offset: xfs_off_t, count: xfs_off_t) -> c_int;
    pub fn xfs_reflink_allocate_cow(ip: *mut xfs_inode, imap: *mut xfs_bmbt_irec, cmap: *mut xfs_bmbt_irec, shared: *mut bool, lockmode: *mut uint, convert_now: bool) -> c_int;
    pub fn xfs_reflink_cancel_cow_blocks(ip: *mut xfs_inode, tpp: *mut *mut xfs_trans, offset_fsb: xfs_fileoff_t, end_fsb: xfs_fileoff_t, cancel_real: bool) -> c_int;
    pub fn xfs_reflink_cancel_cow_range(ip: *mut xfs_inode, offset: xfs_off_t, count: xfs_off_t, cancel_real: bool) -> c_int;
    pub fn xfs_reflink_end_cow(ip: *mut xfs_inode, offset: xfs_off_t, count: xfs_off_t) -> c_int;
    pub fn xfs_reflink_end_atomic_cow(ip: *mut xfs_inode, offset: xfs_off_t, count: xfs_off_t) -> c_int;
    pub fn xfs_reflink_max_atomic_cow(mp: *mut xfs_mount) -> xfs_extlen_t;
    pub fn xfs_reflink_recover_cow(mp: *mut xfs_mount) -> c_int;
    pub fn xfs_reflink_update_dest(dest: *mut xfs_inode, newlen: xfs_off_t, cowextsize: xfs_extlen_t, remap_flags: u32) -> c_int;
    pub fn xfs_reflink_remap_blocks(src: *mut xfs_inode, pos_in: loff_t, dest: *mut xfs_inode, pos_out: loff_t, remap_len: loff_t, remapped: *mut loff_t) -> c_int;
    pub fn xfs_reflink_remap_prep(file_in: *mut c_void, pos_in: loff_t, file_out: *mut c_void, pos_out: loff_t, len: *mut loff_t, remap_flags: u32) -> c_int;
    pub fn xfs_reflink_inode_has_shared_extents(tp: *mut xfs_trans, ip: *mut xfs_inode, has_shared: *mut bool) -> c_int;
    pub fn xfs_reflink_clear_inode_flag(ip: *mut xfs_inode, tpp: *mut *mut xfs_trans) -> c_int;
    pub fn xfs_reflink_unshare(ip: *mut xfs_inode, offset: xfs_off_t, len: xfs_off_t) -> c_int;
    pub fn xfs_reflink_supports_rextsize(mp: *mut xfs_mount, rextsize: u32) -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
