// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Translated from xfs_quota.h.  xfs_quota_defs.h supplies the referenced types,
// constants, fields, and quota predicate functions.

pub struct xfs_trans;
pub struct xfs_buf;
pub struct xfs_inode;
pub struct xfs_dquot;
pub struct xfs_mount;
pub struct xfs_quotainfo;
pub struct kstatfs;
pub struct xfs_hook;
pub type notifier_fn_t = unsafe extern "C" fn();
pub type xfs_ino_t = u64;
pub type xfs_dqtype_t = u32;
pub type xfs_dqid_t = u32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type prid_t = u32;
extern "C" {
    pub static XFS_DQTYPE_USER: xfs_dqtype_t;
    pub static XFS_DQTYPE_GROUP: xfs_dqtype_t;
    pub static XFS_DQTYPE_PROJ: xfs_dqtype_t;
    pub static XFS_UQUOTA_CHKD: u32;
    pub static XFS_GQUOTA_CHKD: u32;
    pub static XFS_PQUOTA_CHKD: u32;
}

#[repr(C)]
pub struct xfs_dqtrx {
    pub qt_dquot: *mut xfs_dquot,
    pub qt_blk_res: u64,
    pub qt_bcount_delta: i64,
    pub qt_delbcnt_delta: i64,
    pub qt_rtblk_res: u64,
    pub qt_rtblk_res_used: u64,
    pub qt_rtbcount_delta: i64,
    pub qt_delrtb_delta: i64,
    pub qt_ino_res: u64,
    pub qt_ino_res_used: u64,
    pub qt_icount_delta: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xfs_apply_dqtrx_type {
    XFS_APPLY_DQTRX_COMMIT = 0,
    XFS_APPLY_DQTRX_UNRESERVE,
}

#[repr(C)]
pub struct xfs_apply_dqtrx_params {
    pub tx_id: usize,
    pub ino: xfs_ino_t,
    pub q_type: xfs_dqtype_t,
    pub q_id: xfs_dqid_t,
}

#[repr(C)]
pub struct xfs_dqtrx_hook {
    pub mod_hook: xfs_hook,
    pub apply_hook: xfs_hook,
}

#[cfg(all(feature = "CONFIG_XFS_QUOTA", feature = "CONFIG_XFS_LIVE_HOOKS"))]
extern "C" {
    pub fn xfs_trans_mod_ino_dquot(tp: *mut xfs_trans, ip: *mut xfs_inode, dqp: *mut xfs_dquot, field: u32, delta: i64);
    pub fn xfs_dqtrx_hook_disable();
    pub fn xfs_dqtrx_hook_enable();
    pub fn xfs_dqtrx_hook_add(qi: *mut xfs_quotainfo, hook: *mut xfs_dqtrx_hook) -> i32;
    pub fn xfs_dqtrx_hook_del(qi: *mut xfs_quotainfo, hook: *mut xfs_dqtrx_hook);
    pub fn xfs_dqtrx_hook_setup(hook: *mut xfs_dqtrx_hook, mod_fn: notifier_fn_t, apply_fn: notifier_fn_t);
}

#[cfg(all(feature = "CONFIG_XFS_QUOTA", not(feature = "CONFIG_XFS_LIVE_HOOKS")))]
pub unsafe fn xfs_trans_mod_ino_dquot(tp: *mut xfs_trans, _ip: *mut xfs_inode, dqp: *mut xfs_dquot, field: u32, delta: i64) {
    xfs_trans_mod_dquot(tp, dqp, field, delta);
}

#[cfg(all(feature = "CONFIG_XFS_QUOTA", not(feature = "CONFIG_XFS_LIVE_HOOKS")))]
extern "C" { fn xfs_trans_mod_dquot(tp: *mut xfs_trans, dqp: *mut xfs_dquot, field: u32, delta: i64); }

#[cfg(all(not(feature = "CONFIG_XFS_QUOTA"), feature = "CONFIG_XFS_LIVE_HOOKS"))]
pub unsafe fn xfs_dqtrx_hook_enable() {}
#[cfg(all(not(feature = "CONFIG_XFS_QUOTA"), feature = "CONFIG_XFS_LIVE_HOOKS"))]
pub unsafe fn xfs_dqtrx_hook_disable() {}

pub unsafe fn xfs_quota_chkd_flag(type_: xfs_dqtype_t) -> u32 {
    match type_ {
        XFS_DQTYPE_USER => XFS_UQUOTA_CHKD,
        XFS_DQTYPE_GROUP => XFS_GQUOTA_CHKD,
        XFS_DQTYPE_PROJ => XFS_PQUOTA_CHKD,
        _ => 0,
    }
}

#[cfg(feature = "CONFIG_XFS_QUOTA")]
extern "C" {
    pub fn xfs_trans_dup_dqinfo(tp: *mut xfs_trans, tp2: *mut xfs_trans);
    pub fn xfs_trans_free_dqinfo(tp: *mut xfs_trans);
    pub fn xfs_trans_mod_dquot_byino(tp: *mut xfs_trans, ip: *mut xfs_inode, field: u32, delta: i64);
    pub fn xfs_trans_apply_dquot_deltas(tp: *mut xfs_trans);
    pub fn xfs_trans_unreserve_and_mod_dquots(tp: *mut xfs_trans, already_locked: bool);
    pub fn xfs_trans_reserve_quota_nblks(tp: *mut xfs_trans, ip: *mut xfs_inode, dblocks: i64, rblocks: i64, force: bool) -> i32;
    pub fn xfs_qm_dqattach(ip: *mut xfs_inode) -> i32;
    pub fn xfs_qm_dqattach_locked(ip: *mut xfs_inode, doalloc: bool) -> i32;
    pub fn xfs_qm_dqdetach(ip: *mut xfs_inode);
    pub fn xfs_qm_dqrele(dqp: *mut xfs_dquot);
    pub fn xfs_quota_reserve_blkres(ip: *mut xfs_inode, blocks: i64) -> i32;
    pub fn xfs_mount_reset_sbqflags(mp: *mut xfs_mount) -> i32;
    pub fn xfs_trans_reserve_quota_bydquots(tp: *mut xfs_trans, mp: *mut xfs_mount,
        udqp: *mut xfs_dquot, gdqp: *mut xfs_dquot, pdqp: *mut xfs_dquot,
        nblks: i64, nions: isize, flags: u32) -> i32;
    pub fn xfs_trans_reserve_quota_icreate(tp: *mut xfs_trans, udqp: *mut xfs_dquot,
        gdqp: *mut xfs_dquot, pdqp: *mut xfs_dquot, dblocks: i64) -> i32;
    pub fn xfs_qm_vop_dqalloc(ip: *mut xfs_inode, kuid: kuid_t, kgid: kgid_t, prid: prid_t,
        flags: u32, udqp: *mut *mut xfs_dquot, gdqp: *mut *mut xfs_dquot, pdqp: *mut *mut xfs_dquot) -> i32;
    pub fn xfs_qm_vop_create_dqattach(tp: *mut xfs_trans, ip: *mut xfs_inode, u: *mut xfs_dquot, g: *mut xfs_dquot, p: *mut xfs_dquot);
    pub fn xfs_qm_vop_rename_dqattach(it: *mut *mut xfs_inode) -> i32;
    pub fn xfs_qm_vop_chown(tp: *mut xfs_trans, ip: *mut xfs_inode, old: *mut *mut xfs_dquot, new: *mut xfs_dquot) -> *mut xfs_dquot;
    pub fn xfs_qm_statvfs(ip: *mut xfs_inode, s: *mut kstatfs);
    pub fn xfs_qm_newmount(mp: *mut xfs_mount, a: *mut u32, b: *mut u32) -> i32;
    pub fn xfs_qm_resume_quotaon(mp: *mut xfs_mount);
    pub fn xfs_qm_mount_quotas(mp: *mut xfs_mount);
    pub fn xfs_qm_unmount(mp: *mut xfs_mount);
    pub fn xfs_qm_unmount_quotas(mp: *mut xfs_mount);
    pub fn xfs_inode_near_dquot_enforcement(ip: *mut xfs_inode, type_: xfs_dqtype_t) -> bool;
}

#[cfg(not(feature = "CONFIG_XFS_QUOTA"))]
pub unsafe fn xfs_qm_vop_dqalloc(_ip: *mut xfs_inode, _kuid: kuid_t, _kgid: kgid_t, _prid: prid_t, _flags: u32,
    udqp: *mut *mut xfs_dquot, gdqp: *mut *mut xfs_dquot, pdqp: *mut *mut xfs_dquot) -> i32 {
    *udqp = core::ptr::null_mut(); *gdqp = core::ptr::null_mut(); *pdqp = core::ptr::null_mut(); 0
}

#[cfg(not(feature = "CONFIG_XFS_QUOTA"))]
pub unsafe fn xfs_quota_reserve_blkres(_ip: *mut xfs_inode, _blocks: i64) -> i32 { 0 }

pub unsafe fn xfs_quota_unreserve_blkres(ip: *mut xfs_inode, blocks: u64) {
    // don't return an error as unreserving quotas can't fail
    xfs_quota_reserve_blkres(ip, -(blocks as i64));
}

// XFS_NOT_DQATTACHED and XFS_QM_NEED_QUOTACHECK remain source-level macros;
// their field and predicate dependencies are supplied by xfs_quota_defs.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
