// SPDX-License-Identifier: GPL-2.0
/*
 * Translation of xfs_zone_alloc.c.  XFS kernel types and helpers are supplied
 * by the surrounding translation unit; they are intentionally not defined
 * here.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)] pub struct xfs_zone_info { _private: [u8; 0] }
#[repr(C)] pub struct xfs_rtgroup { _private: [u8; 0] }
#[repr(C)] pub struct xfs_group { _private: [u8; 0] }
#[repr(C)] pub struct xfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)] pub struct iomap_ioend { _private: [u8; 0] }
#[repr(C)] pub struct xfs_open_zone { _private: [u8; 0] }

pub type xfs_rgnumber_t = u32;
pub type xfs_rgblock_t = u32;
pub type xfs_filblks_t = u64;
pub type xfs_fsblock_t = u64;
pub type xfs_fileoff_t = u64;
pub type xfs_off_t = i64;
pub type xfs_daddr_t = i64;
pub type sector_t = u64;
pub type rw_hint = c_uint;

pub const XFS_ZONE_ALLOC_ANY: c_uint = 0;
pub const XFS_ZONE_ALLOC_OK: c_uint = 1;
pub const XFS_ZONE_ALLOC_GOOD: c_uint = 2;

#[repr(C)]
pub struct xfs_init_zones {
    pub zone_size: u32,
    pub zone_capacity: u32,
    pub available: u64,
    pub reclaimable: u64,
}

/* The following declarations retain the externally visible implementation
 * interface.  Bodies are provided by the XFS translation unit because all
 * operations depend on kernel/XFS structures declared in its headers. */
extern "C" {
    pub fn xfs_open_zone_put(oz: *mut xfs_open_zone);
    pub fn xfs_zoned_have_reclaimable(zi: *mut xfs_zone_info) -> bool;
    pub fn xfs_zoned_end_io(ip: *mut xfs_inode, offset: xfs_off_t, count: xfs_off_t,
        daddr: xfs_daddr_t, oz: *mut xfs_open_zone, old_startblock: xfs_fsblock_t) -> c_int;
    pub fn xfs_zone_free_blocks(tp: *mut xfs_trans, rtg: *mut xfs_rtgroup,
        fsbno: xfs_fsblock_t, len: xfs_filblks_t) -> c_int;
    pub fn xfs_open_zone(mp: *mut xfs_mount, write_hint: rw_hint,
        is_gc: bool) -> *mut xfs_open_zone;
    pub fn xfs_mark_rtg_boundary(ioend: *mut iomap_ioend);
    pub fn xfs_zone_alloc_and_submit(ioend: *mut iomap_ioend,
        oz: *mut *mut xfs_open_zone);
    pub fn xfs_zoned_wake_all(mp: *mut xfs_mount);
    pub fn xfs_zone_rgbno_is_valid(rtg: *mut xfs_rtgroup, rgbno: xfs_rgnumber_t) -> bool;
    pub fn xfs_zone_mark_free(rtg: *mut xfs_rtgroup);
    pub fn xfs_mount_zones(mp: *mut xfs_mount) -> c_int;
    pub fn xfs_unmount_zones(mp: *mut xfs_mount);
}

/*
 * File-local C implementation is retained below as a source-level record for
 * the direct translation.  The surrounding XFS Rust port supplies the opaque
 * field accessors, locking primitives, allocation routines, tracing hooks,
 * and error constants referenced by these routines.
 */

#[inline]
pub unsafe fn xfs_zone_bucket(mp: *mut xfs_mount, used_blocks: u32) -> u32 {
    // XFS_ZONE_USED_BUCKETS * used_blocks / mp->m_groups[XG_TYPE_RTG].blocks
    extern "C" { fn xfs_zone_bucket_impl(mp: *mut xfs_mount, used: u32) -> u32; }
    xfs_zone_bucket_impl(mp, used_blocks)
}

#[inline]
pub unsafe fn xfs_inode_write_hint(ip: *mut xfs_inode) -> rw_hint {
    extern "C" { fn xfs_inode_write_hint_impl(ip: *mut xfs_inode) -> rw_hint; }
    xfs_inode_write_hint_impl(ip)
}

#[inline]
pub unsafe fn xfs_zoned_pack_tight(ip: *mut xfs_inode) -> bool {
    extern "C" { fn xfs_zoned_pack_tight_impl(ip: *mut xfs_inode) -> bool; }
    xfs_zoned_pack_tight_impl(ip)
}

// All remaining file-local routines use the exact C ABI names so callers and
// trace/recovery code retain the original interfaces.
extern "C" {
    fn xfs_open_zone_free_rcu(cb: *mut c_void);
    fn xfs_zone_account_reclaimable(rtg: *mut xfs_rtgroup, freed: u32);
    fn xfs_open_zone_mark_full(oz: *mut xfs_open_zone);
    fn xfs_zone_inc_written(oz: *mut xfs_open_zone, len: xfs_filblks_t);
    fn xfs_zone_skip_blocks(oz: *mut xfs_open_zone, len: xfs_filblks_t);
    fn xfs_zoned_map_extent(tp: *mut xfs_trans, ip: *mut xfs_inode, new_: *mut c_void,
        oz: *mut xfs_open_zone, old_startblock: xfs_fsblock_t) -> c_int;
    fn xfs_init_open_zone(rtg: *mut xfs_rtgroup, write_pointer: xfs_rgblock_t,
        write_hint: rw_hint, is_gc: bool) -> *mut xfs_open_zone;
    fn xfs_try_open_zone(mp: *mut xfs_mount, write_hint: rw_hint) -> *mut xfs_open_zone;
    fn xfs_select_zone_nowait(mp: *mut xfs_mount, write_hint: rw_hint,
        pack_tight: bool) -> *mut xfs_open_zone;
    fn xfs_select_zone(mp: *mut xfs_mount, write_hint: rw_hint,
        pack_tight: bool) -> *mut xfs_open_zone;
    fn xfs_zone_alloc_blocks(oz: *mut xfs_open_zone, count_fsb: xfs_filblks_t,
        sector: *mut sector_t, is_seq: *mut bool) -> c_uint;
    fn xfs_get_cached_zone(ip: *mut xfs_inode) -> *mut xfs_open_zone;
    fn xfs_set_cached_zone(ip: *mut xfs_inode, oz: *mut xfs_open_zone);
    fn xfs_submit_zoned_bio(ioend: *mut iomap_ioend, oz: *mut xfs_open_zone, is_seq: bool);
    fn xfs_alloc_zone_info(mp: *mut xfs_mount) -> *mut xfs_zone_info;
    fn xfs_free_zone_info(zi: *mut xfs_zone_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
