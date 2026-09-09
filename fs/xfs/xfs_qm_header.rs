// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding translation unit:
// xfs_dquot_item.h, xfs_dquot.h

pub struct xfs_inode;

extern "C" {
    pub static mut xfs_dqtrx_cache: *mut kmem_cache;
}

/* Number of bmaps requested from bmapi during quotacheck. */
pub const XFS_DQITER_MAP_SIZE: usize = 10;

#[macro_export]
macro_rules! XFS_IS_DQUOT_UNINITIALIZED {
    ($dqp:expr) => {
        !$dqp.q_blk.hardlimit &&
        !$dqp.q_blk.softlimit &&
        !$dqp.q_rtb.hardlimit &&
        !$dqp.q_rtb.softlimit &&
        !$dqp.q_ino.hardlimit &&
        !$dqp.q_ino.softlimit &&
        !$dqp.q_blk.count &&
        !$dqp.q_rtb.count &&
        !$dqp.q_ino.count
    };
}

#[repr(C)]
pub struct xfs_quota_limits {
    pub hard: xfs_qcnt_t,
    pub soft: xfs_qcnt_t,
    pub time: time64_t,
}

#[repr(C)]
pub struct xfs_def_quota {
    pub blk: xfs_quota_limits,
    pub ino: xfs_quota_limits,
    pub rtb: xfs_quota_limits,
}

#[repr(C)]
pub struct xfs_quotainfo {
    pub qi_uquota_tree: radix_tree_root,
    pub qi_gquota_tree: radix_tree_root,
    pub qi_pquota_tree: radix_tree_root,
    pub qi_tree_lock: mutex,
    pub qi_uquotaip: *mut xfs_inode,
    pub qi_gquotaip: *mut xfs_inode,
    pub qi_pquotaip: *mut xfs_inode,
    pub qi_dirip: *mut xfs_inode,
    pub qi_lru: list_lru,
    pub qi_dquots: u64,
    pub qi_quotaofflock: mutex,
    pub qi_dqchunklen: xfs_filblks_t,
    pub qi_dqperchunk: uint,
    pub qi_usr_default: xfs_def_quota,
    pub qi_grp_default: xfs_def_quota,
    pub qi_prj_default: xfs_def_quota,
    pub qi_shrinker: *mut shrinker,
    pub qi_expiry_min: time64_t,
    pub qi_expiry_max: time64_t,
    pub qi_mod_ino_dqtrx_hooks: xfs_hooks,
    pub qi_apply_dqtrx_hooks: xfs_hooks,
}

#[inline]
pub unsafe fn xfs_dquot_tree(
    qi: *mut xfs_quotainfo,
    r#type: xfs_dqtype_t,
) -> *mut radix_tree_root {
    match r#type {
        XFS_DQTYPE_USER => &mut (*qi).qi_uquota_tree,
        XFS_DQTYPE_GROUP => &mut (*qi).qi_gquota_tree,
        XFS_DQTYPE_PROJ => &mut (*qi).qi_pquota_tree,
        _ => {
            ASSERT(0);
            core::ptr::null_mut()
        }
    }
}

#[inline]
pub unsafe fn xfs_quota_inode(
    mp: *mut xfs_mount,
    r#type: xfs_dqtype_t,
) -> *mut xfs_inode {
    match r#type {
        XFS_DQTYPE_USER => (*(*mp).m_quotainfo).qi_uquotaip,
        XFS_DQTYPE_GROUP => (*(*mp).m_quotainfo).qi_gquotaip,
        XFS_DQTYPE_PROJ => (*(*mp).m_quotainfo).qi_pquotaip,
        _ => {
            ASSERT(0);
            core::ptr::null_mut()
        }
    }
}

#[repr(C)]
pub struct xfs_mod_ino_dqtrx_params {
    pub tx_id: usize,
    pub ino: xfs_ino_t,
    pub q_type: xfs_dqtype_t,
    pub q_id: xfs_dqid_t,
    pub delta: i64,
}

extern "C" {
    pub fn xfs_trans_mod_dquot(tp: *mut xfs_trans, dqp: *mut xfs_dquot, field: uint, delta: i64);
    pub fn xfs_trans_dqjoin(tp: *mut xfs_trans, dqp: *mut xfs_dquot);
    pub fn xfs_trans_log_dquot(tp: *mut xfs_trans, dqp: *mut xfs_dquot);
}

pub const XFS_QM_TRANS_USR: usize = 0;
pub const XFS_QM_TRANS_GRP: usize = 1;
pub const XFS_QM_TRANS_PRJ: usize = 2;
pub const XFS_QM_TRANS_DQTYPES: usize = 3;
pub const XFS_QM_TRANS_MAXDQS: usize = 5;

#[repr(C)]
pub struct xfs_dquot_acct {
    pub dqs: [[xfs_dqtrx; XFS_QM_TRANS_MAXDQS]; XFS_QM_TRANS_DQTYPES],
}

pub const XFS_QM_BTIMELIMIT: i32 = 7 * 24 * 60 * 60;
pub const XFS_QM_RTBTIMELIMIT: i32 = 7 * 24 * 60 * 60;
pub const XFS_QM_ITIMELIMIT: i32 = 7 * 24 * 60 * 60;

extern "C" {
    pub fn xfs_qm_destroy_quotainfo(mp: *mut xfs_mount);
    pub fn xfs_qm_scall_trunc_qfiles(mp: *mut xfs_mount, flags: uint) -> i32;
    pub fn xfs_qm_scall_getquota(mp: *mut xfs_mount, id: xfs_dqid_t, r#type: xfs_dqtype_t, dst: *mut qc_dqblk) -> i32;
    pub fn xfs_qm_scall_getquota_next(mp: *mut xfs_mount, id: *mut xfs_dqid_t, r#type: xfs_dqtype_t, dst: *mut qc_dqblk) -> i32;
    pub fn xfs_qm_scall_setqlim(mp: *mut xfs_mount, id: xfs_dqid_t, r#type: xfs_dqtype_t, newlim: *mut qc_dqblk) -> i32;
    pub fn xfs_qm_scall_quotaon(mp: *mut xfs_mount, flags: uint) -> i32;
    pub fn xfs_qm_scall_quotaoff(mp: *mut xfs_mount, flags: uint) -> i32;
}

#[inline]
pub unsafe fn xfs_get_defquota(qi: *mut xfs_quotainfo, r#type: xfs_dqtype_t) -> *mut xfs_def_quota {
    match r#type {
        XFS_DQTYPE_USER => &mut (*qi).qi_usr_default,
        XFS_DQTYPE_GROUP => &mut (*qi).qi_grp_default,
        XFS_DQTYPE_PROJ => &mut (*qi).qi_prj_default,
        _ => { ASSERT(0); core::ptr::null_mut() }
    }
}

extern "C" {
    pub fn xfs_qm_qino_load(mp: *mut xfs_mount, r#type: xfs_dqtype_t, ipp: *mut *mut xfs_inode) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
