// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Dependency: xfs_quota_defs.h

use core::ffi::c_char;

#[repr(C)]
pub struct xfs_rtgroup;
#[repr(C)]
pub struct xchk_stats_run;
#[repr(C)]
pub struct xfs_scrub;
#[repr(C)]
pub struct xfs_mount;
#[repr(C)]
pub struct xfs_perag;
#[repr(C)]
pub struct xfs_buf;
#[repr(C)]
pub struct xchk_ag;
#[repr(C)]
pub struct xchk_rt;
#[repr(C)]
pub struct xfs_imap;
#[repr(C)]
pub struct xfs_rmap_irec;
#[repr(C)]
pub struct xfs_buf_ops;
#[repr(C)]
pub struct xbitmap;
#[repr(C)]
pub struct xagb_bitmap;
#[repr(C)]
pub struct xrgb_bitmap;
#[repr(C)]
pub struct xfsb_bitmap;
#[repr(C)]
pub struct xrtb_bitmap;

pub type xfs_extlen_t = u64;
pub type xfs_agblock_t = u32;
pub type xfs_extnum_t = u32;
pub type xfs_dqtype_t = u32;
pub type xfs_rgblock_t = u32;
pub type xfs_filblks_t = u64;

extern "C" {
    pub fn xchk_needs_repair(sm: *const core::ffi::c_void) -> bool;
    pub fn xfs_trans_commit(tp: *mut core::ffi::c_void) -> i32;
}

#[inline]
pub unsafe fn xrep_notsupported(_sc: *mut xfs_scrub) -> i32 { -95 }

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
extern "C" {
    pub fn xrep_attempt(sc: *mut xfs_scrub, run: *mut xchk_stats_run) -> i32;
    pub fn xrep_will_attempt(sc: *mut xfs_scrub) -> bool;
    pub fn xrep_failure(mp: *mut xfs_mount);
    pub fn xrep_roll_ag_trans(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_roll_trans(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_defer_finish(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_ag_has_space(pag: *mut xfs_perag, nr_blocks: xfs_extlen_t, ty: i32) -> bool;
    pub fn xrep_calc_ag_resblks(sc: *mut xfs_scrub) -> xfs_extlen_t;
    pub fn xrep_fix_freelist(sc: *mut xfs_scrub, alloc_flags: i32) -> i32;
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
#[inline]
pub unsafe fn xrep_trans_commit(sc: *mut xfs_scrub, tp: *mut core::ffi::c_void) -> i32 {
    let error = xfs_trans_commit(tp);
    let _ = sc;
    error
}

#[repr(C)]
pub struct xrep_find_ag_btree {
    pub rmap_owner: u64,
    pub buf_ops: *const xfs_buf_ops,
    pub maxlevels: u32,
    pub root: xfs_agblock_t,
    pub height: u32,
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
extern "C" {
    pub fn xrep_find_ag_btree_roots(sc: *mut xfs_scrub, agf_bp: *mut xfs_buf,
        btree_info: *mut xrep_find_ag_btree, agfl_bp: *mut xfs_buf) -> i32;
    pub fn xrep_setup_xfbtree(sc: *mut xfs_scrub, descr: *const c_char) -> i32;
    pub fn xrep_ino_ensure_extent_count(sc: *mut xfs_scrub, whichfork: i32, nextents: xfs_extnum_t) -> i32;
    pub fn xrep_reset_perag_resv(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_bmap(sc: *mut xfs_scrub, whichfork: i32, allow_unwritten: bool) -> i32;
    pub fn xrep_metadata_inode_forks(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_ag_rmapbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_ag_refcountbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_xattr(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_directory(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_parent(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_nlinks(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_symlink(sc: *mut xfs_scrub, resblks: *mut u32) -> i32;
    pub fn xrep_setup_dirtree(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_rtrmapbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_rtrefcountbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_ag_allocbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_setup_inode(sc: *mut xfs_scrub, imap: *const xfs_imap) -> i32;
    pub fn xrep_ag_btcur_init(sc: *mut xfs_scrub, sa: *mut xchk_ag);
    pub fn xrep_ag_init(sc: *mut xfs_scrub, pag: *mut xfs_perag, sa: *mut xchk_ag) -> i32;
    pub fn xrep_check_ino_btree_mapping(sc: *mut xfs_scrub, rec: *const xfs_rmap_irec) -> i32;
    pub fn xrep_revalidate_allocbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_revalidate_iallocbt(sc: *mut xfs_scrub) -> i32;
}

// Metadata repairer declarations retain their C ABI and external linkage.
#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
extern "C" {
    pub fn xrep_probe(sc: *mut xfs_scrub) -> i32; pub fn xrep_superblock(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_agf(sc: *mut xfs_scrub) -> i32; pub fn xrep_agfl(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_agi(sc: *mut xfs_scrub) -> i32; pub fn xrep_allocbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_iallocbt(sc: *mut xfs_scrub) -> i32; pub fn xrep_rmapbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_refcountbt(sc: *mut xfs_scrub) -> i32; pub fn xrep_inode(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_bmap_data(sc: *mut xfs_scrub) -> i32; pub fn xrep_bmap_attr(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_bmap_cow(sc: *mut xfs_scrub) -> i32; pub fn xrep_nlinks(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_fscounters(sc: *mut xfs_scrub) -> i32; pub fn xrep_xattr(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_directory(sc: *mut xfs_scrub) -> i32; pub fn xrep_parent(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_symlink(sc: *mut xfs_scrub) -> i32; pub fn xrep_dirtree(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_metapath(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_reinit_pagf(sc: *mut xfs_scrub) -> i32; pub fn xrep_reinit_pagi(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_buf_verify_struct(bp: *mut xfs_buf, ops: *const xfs_buf_ops) -> bool;
    pub fn xrep_inode_set_nblocks(sc: *mut xfs_scrub, new_blocks: i64);
    pub fn xrep_reset_metafile_resv(sc: *mut xfs_scrub) -> i32;
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_attempt(_sc: *mut xfs_scrub, _run: *mut xchk_stats_run) -> i32 { -95 }

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_will_attempt(_sc: *const xfs_scrub) -> bool { true }

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_failure(_mp: *mut xfs_mount) {}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_calc_ag_resblks(_sc: *mut xfs_scrub) -> xfs_extlen_t { 0 }

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_calc_rtgroup_resblks(sc: *mut xfs_scrub) -> xfs_extlen_t { xrep_calc_ag_resblks(sc) }

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_reset_perag_resv(_sc: *mut xfs_scrub) -> i32 { -95 }

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_setup_nothing(_sc: *mut xfs_scrub) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_setup_symlink(_sc: *mut xfs_scrub, _x: *mut u32) -> i32 { 0 }

#[cfg(feature = "CONFIG_XFS_RT")]
extern "C" {
    pub fn xrep_rtgroup_init(sc: *mut xfs_scrub, rtg: *mut xfs_rtgroup, sr: *mut xchk_rt, rtglock_flags: u32) -> i32;
    pub fn xrep_rtgroup_btcur_init(sc: *mut xfs_scrub, sr: *mut xchk_rt);
    pub fn xrep_require_rtext_inuse(sc: *mut xfs_scrub, rgbno: xfs_rgblock_t, len: xfs_filblks_t) -> i32;
    pub fn xrep_calc_rtgroup_resblks(sc: *mut xfs_scrub) -> xfs_extlen_t;
    pub fn xrep_rtbitmap(sc: *mut xfs_scrub) -> i32; pub fn xrep_rtsummary(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_rgsuperblock(sc: *mut xfs_scrub) -> i32; pub fn xrep_rtrmapbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_rtrefcountbt(sc: *mut xfs_scrub) -> i32;
}

#[cfg(feature = "CONFIG_XFS_QUOTA")]
extern "C" {
    pub fn xrep_update_qflags(sc: *mut xfs_scrub, clear_flags: u32, set_flags: u32);
    pub fn xrep_force_quotacheck(sc: *mut xfs_scrub, ty: xfs_dqtype_t);
    pub fn xrep_ino_dqattach(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_quota(sc: *mut xfs_scrub) -> i32; pub fn xrep_quotacheck(sc: *mut xfs_scrub) -> i32;
}

#[cfg(not(feature = "CONFIG_XFS_QUOTA"))]
#[inline]
pub unsafe fn xrep_force_quotacheck(_sc: *mut xfs_scrub, _ty: xfs_dqtype_t) {}
#[cfg(not(feature = "CONFIG_XFS_QUOTA"))]
#[inline]
pub unsafe fn xrep_ino_dqattach(_sc: *mut xfs_scrub) -> i32 { 0 }

// C preprocessor aliases retained as Rust-level aliases for no-repair builds.
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_probe;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_superblock;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_agf;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_agfl;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_agi;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_allocbt;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_iallocbt;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_rmapbt;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_refcountbt;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_inode;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_bmap_data;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_bmap_attr;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_bmap_cow;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_nlinks;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_fscounters;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_xattr;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_directory;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_parent;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_symlink;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_dirtree;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_metapath;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_quota;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_quotacheck;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_rtbitmap;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_rtsummary;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_rgsuperblock;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_rtrmapbt;
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub use xrep_notsupported as xrep_rtrefcountbt;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
