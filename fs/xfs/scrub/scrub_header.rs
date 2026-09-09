// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// External kernel/XFS types and symbols are supplied by other translation units.

#[repr(C)]
pub struct xfs_scrub;

#[repr(C)]
pub struct xchk_relax {
    pub next_resched: ::core::ffi::c_ulong,
    pub resched_nr: ::core::ffi::c_uint,
    pub killable: bool,
}

// Yield to the scheduler at most 10x per second.
// XCHK_RELAX_NEXT = jiffies + (HZ / 10)

#[repr(C)]
pub struct xchk_meta_ops {
    pub setup: Option<unsafe extern "C" fn(*mut xfs_scrub) -> ::core::ffi::c_int>,
    pub scrub: Option<unsafe extern "C" fn(*mut xfs_scrub) -> ::core::ffi::c_int>,
    pub repair: Option<unsafe extern "C" fn(*mut xfs_scrub) -> ::core::ffi::c_int>,
    pub repair_eval: Option<unsafe extern "C" fn(*mut xfs_scrub) -> ::core::ffi::c_int>,
    pub has: Option<unsafe extern "C" fn(*const xfs_mount) -> bool>,
    pub r#type: xchk_type,
}

#[repr(C)]
pub struct xchk_ag {
    pub pag: *mut xfs_perag,
    pub agf_bp: *mut xfs_buf,
    pub agi_bp: *mut xfs_buf,
    pub bno_cur: *mut xfs_btree_cur,
    pub cnt_cur: *mut xfs_btree_cur,
    pub ino_cur: *mut xfs_btree_cur,
    pub fino_cur: *mut xfs_btree_cur,
    pub rmap_cur: *mut xfs_btree_cur,
    pub refc_cur: *mut xfs_btree_cur,
}

#[repr(C)]
pub struct xchk_rt {
    pub rtg: *mut xfs_rtgroup,
    pub rtlock_flags: ::core::ffi::c_uint,
    pub rmap_cur: *mut xfs_btree_cur,
    pub refc_cur: *mut xfs_btree_cur,
}

#[repr(C)]
pub struct xfs_scrub {
    pub mp: *mut xfs_mount,
    pub sm: *mut xfs_scrub_metadata,
    pub ops: *const xchk_meta_ops,
    pub tp: *mut xfs_trans,
    pub file: *mut file,
    pub ip: *mut xfs_inode,
    pub buf: *mut ::core::ffi::c_void,
    pub buf_cleanup: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>,
    pub xfile: *mut xfile,
    pub xmbtp: *mut xfs_buftarg,
    pub ilock_flags: ::core::ffi::c_uint,
    pub orphanage_ilock_flags: ::core::ffi::c_uint,
    pub orphanage: *mut xfs_inode,
    pub tempip: *mut xfs_inode,
    pub temp_ilock_flags: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub sick_mask: ::core::ffi::c_uint,
    pub healthy_mask: ::core::ffi::c_uint,
    pub relax: xchk_relax,
    pub sa: xchk_ag,
    pub sr: xchk_rt,
}

#[repr(i32)]
pub enum xchk_type {
    ST_NONE = 1,
    ST_PERAG,
    ST_FS,
    ST_INODE,
    ST_GENERIC,
    ST_RTGROUP,
}

pub const XCHK_TRY_HARDER: ::core::ffi::c_uint = 1u32 << 0;
pub const XCHK_HAVE_FREEZE_PROT: ::core::ffi::c_uint = 1u32 << 1;
pub const XCHK_FSGATES_DRAIN: ::core::ffi::c_uint = 1u32 << 2;
pub const XCHK_NEED_DRAIN: ::core::ffi::c_uint = 1u32 << 3;
pub const XCHK_FSGATES_QUOTA: ::core::ffi::c_uint = 1u32 << 4;
pub const XCHK_FSGATES_DIRENTS: ::core::ffi::c_uint = 1u32 << 5;
pub const XCHK_FSGATES_RMAP: ::core::ffi::c_uint = 1u32 << 6;
pub const XREP_RESET_PERAG_RESV: ::core::ffi::c_uint = 1u32 << 30;
pub const XREP_ALREADY_FIXED: ::core::ffi::c_uint = 1u32 << 31;
pub const XCHK_FSGATES_ALL: ::core::ffi::c_uint = XCHK_FSGATES_DRAIN
    | XCHK_FSGATES_QUOTA | XCHK_FSGATES_DIRENTS | XCHK_FSGATES_RMAP;

#[repr(C)]
pub struct xfs_scrub_subord {
    pub sc: xfs_scrub,
    pub parent_sc: *mut xfs_scrub,
    pub old_smtype: ::core::ffi::c_uint,
    pub old_smflags: ::core::ffi::c_uint,
}

extern "C" {
    pub fn xchk_scrub_create_subord(sc: *mut xfs_scrub, subtype: ::core::ffi::c_uint)
        -> *mut xfs_scrub_subord;
    pub fn xchk_scrub_free_subord(sub: *mut xfs_scrub_subord);
}

#[inline]
pub unsafe fn xchk_maybe_relax(_widget: *mut xchk_relax) -> ::core::ffi::c_int {
    // The C implementation depends on kernel scheduler, signal, jiffies, and likely/unlikely APIs.
    // Preserve the declaration for integration with the kernel translation unit.
    0
}

#[inline]
pub unsafe fn xchk_should_terminate(sc: *mut xfs_scrub, error: *mut ::core::ffi::c_int) -> bool {
    if xchk_maybe_relax(&mut (*sc).relax) != 0 {
        if *error == 0 { *error = -4; }
        return true;
    }
    false
}

#[inline]
pub unsafe fn xchk_nothing(_sc: *mut xfs_scrub) -> ::core::ffi::c_int { -2 }

extern "C" {
    pub fn xchk_tester(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_superblock(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_agf(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_agfl(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_agi(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_allocbt(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_iallocbt(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_rmapbt(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_refcountbt(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_inode(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_bmap_data(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_bmap_attr(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_bmap_cow(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_directory(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_xattr(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_symlink(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_parent(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_dirtree(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_metapath(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_rtbitmap(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_rtsummary(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_rgsuperblock(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_rtrmapbt(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_rtrefcountbt(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_quota(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_quotacheck(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_fscounters(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_nlinks(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
    pub fn xchk_xref_is_used_space(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t);
    pub fn xchk_xref_is_not_inode_chunk(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t);
    pub fn xchk_xref_is_inode_chunk(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t);
    pub fn xchk_xref_is_only_owned_by(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info);
    pub fn xchk_xref_is_not_owned_by(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info);
    pub fn xchk_xref_has_no_owner(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t);
    pub fn xchk_xref_is_cow_staging(sc: *mut xfs_scrub, bno: xfs_agblock_t, len: xfs_extlen_t);
    pub fn xchk_xref_is_not_shared(sc: *mut xfs_scrub, bno: xfs_agblock_t, len: xfs_extlen_t);
    pub fn xchk_xref_is_not_cow_staging(sc: *mut xfs_scrub, bno: xfs_agblock_t, len: xfs_extlen_t);
}

// CONFIG_XFS_RT and CONFIG_XFS_QUOTA conditional declarations are supplied by the build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
