// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_sb {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_dsb {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_fsop_geom {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_perag {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_buf {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_log_sb(tp: *mut xfs_trans);
    pub fn xfs_sync_sb(mp: *mut xfs_mount, wait: bool) -> i32;
    pub fn xfs_sync_sb_buf(mp: *mut xfs_mount, update_rtsb: bool) -> i32;
    pub fn xfs_sb_mount_common(mp: *mut xfs_mount, sbp: *mut xfs_sb);
    pub fn xfs_sb_mount_rextsize(mp: *mut xfs_mount, sbp: *mut xfs_sb);
    pub fn xfs_mount_sb_set_rextsize(
        mp: *mut xfs_mount,
        sbp: *mut xfs_sb,
        rextsize: xfs_agblock_t,
    );
    pub fn xfs_sb_from_disk(to: *mut xfs_sb, from: *mut xfs_dsb);
    pub fn xfs_sb_to_disk(to: *mut xfs_dsb, from: *mut xfs_sb);
    pub fn xfs_sb_quota_from_disk(sbp: *mut xfs_sb);
    pub fn xfs_sb_good_version(sbp: *mut xfs_sb) -> bool;
    pub fn xfs_sb_version_to_features(sbp: *mut xfs_sb) -> u64;

    pub fn xfs_update_secondary_sbs(mp: *mut xfs_mount) -> i32;

    pub fn xfs_fs_geometry(
        mp: *mut xfs_mount,
        geo: *mut xfs_fsop_geom,
        struct_version: i32,
    );
    pub fn xfs_sb_read_secondary(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        agno: xfs_agnumber_t,
        bpp: *mut *mut xfs_buf,
    ) -> i32;
    pub fn xfs_sb_get_secondary(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        agno: xfs_agnumber_t,
        bpp: *mut *mut xfs_buf,
    ) -> i32;

    pub fn xfs_validate_stripe_geometry(
        mp: *mut xfs_mount,
        sunit: i64,
        swidth: i64,
        sectorsize: i32,
        may_repair: bool,
        silent: bool,
    ) -> bool;
    pub fn xfs_validate_rt_geometry(sbp: *mut xfs_sb) -> bool;

    pub fn xfs_compute_rextslog(rtextents: xfs_rtbxlen_t) -> u8;
    pub fn xfs_compute_rgblklog(
        rgextents: xfs_rtxlen_t,
        rextsize: xfs_rgblock_t,
    ) -> i32;
}

pub const XFS_FS_GEOM_MAX_STRUCT_VER: i32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
