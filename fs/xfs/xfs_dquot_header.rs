// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/* Dquots hold quota information about a user or group. */

use core::ffi::c_int;

#[repr(C)]
pub struct xfs_mount;
#[repr(C)]
pub struct xfs_trans;

pub const XFS_QLOWSP_1_PCNT: usize = 0;
pub const XFS_QLOWSP_3_PCNT: usize = 1;
pub const XFS_QLOWSP_5_PCNT: usize = 2;
pub const XFS_QLOWSP_MAX: usize = 3;

#[repr(C)]
pub struct xfs_dquot_res {
    pub reserved: xfs_qcnt_t,
    pub count: xfs_qcnt_t,
    pub hardlimit: xfs_qcnt_t,
    pub softlimit: xfs_qcnt_t,
    pub timer: time64_t,
}

#[inline]
pub unsafe fn xfs_dquot_res_over_limits(qres: *const xfs_dquot_res) -> bool {
    ((*qres).softlimit != 0 && (*qres).softlimit < (*qres).reserved)
        || ((*qres).hardlimit != 0 && (*qres).hardlimit < (*qres).reserved)
}

#[repr(C)]
pub struct xfs_dquot_pre {
    pub q_prealloc_lo_wmark: xfs_qcnt_t,
    pub q_prealloc_hi_wmark: xfs_qcnt_t,
    pub q_low_space: [i64; XFS_QLOWSP_MAX],
}

#[repr(C)]
pub struct xfs_dquot {
    pub q_lru: list_head,
    pub q_mount: *mut xfs_mount,
    pub q_type: xfs_dqtype_t,
    pub q_flags: u16,
    pub q_id: xfs_dqid_t,
    pub q_lockref: lockref,
    pub q_bufoffset: c_int,
    pub q_blkno: xfs_daddr_t,
    pub q_fileoffset: xfs_fileoff_t,
    pub q_blk: xfs_dquot_res,
    pub q_ino: xfs_dquot_res,
    pub q_rtb: xfs_dquot_res,
    pub q_logitem: xfs_dq_logitem,
    pub q_blk_prealloc: xfs_dquot_pre,
    pub q_rtb_prealloc: xfs_dquot_pre,
    pub q_qlock: mutex,
    pub q_flush: completion,
    pub q_pincount: atomic_t,
    pub q_pinwait: wait_queue_head,
}

pub const XFS_QLOCK_NORMAL: c_int = 0;
pub const XFS_QLOCK_NESTED: c_int = 1;

#[inline]
pub unsafe fn xfs_dqflock(dqp: *mut xfs_dquot) {
    wait_for_completion(&mut (*dqp).q_flush);
}

#[inline]
pub unsafe fn xfs_dqflock_nowait(dqp: *mut xfs_dquot) -> bool {
    try_wait_for_completion(&mut (*dqp).q_flush)
}

#[inline]
pub unsafe fn xfs_dqfunlock(dqp: *mut xfs_dquot) {
    complete(&mut (*dqp).q_flush);
}

#[inline]
pub unsafe fn xfs_dquot_type(dqp: *const xfs_dquot) -> c_int {
    ((*dqp).q_type & XFS_DQTYPE_REC_MASK) as c_int
}

#[inline]
pub unsafe fn xfs_this_quota_on(mp: *mut xfs_mount, type_: xfs_dqtype_t) -> c_int {
    match type_ {
        XFS_DQTYPE_USER => XFS_IS_UQUOTA_ON(mp),
        XFS_DQTYPE_GROUP => XFS_IS_GQUOTA_ON(mp),
        XFS_DQTYPE_PROJ => XFS_IS_PQUOTA_ON(mp),
        _ => 0,
    }
}

#[inline]
pub unsafe fn xfs_inode_dquot(ip: *mut xfs_inode, type_: xfs_dqtype_t) -> *mut xfs_dquot {
    if xfs_is_metadir_inode(ip) { return core::ptr::null_mut(); }
    match type_ {
        XFS_DQTYPE_USER => (*ip).i_udquot,
        XFS_DQTYPE_GROUP => (*ip).i_gdquot,
        XFS_DQTYPE_PROJ => (*ip).i_pdquot,
        _ => core::ptr::null_mut(),
    }
}

#[inline]
pub unsafe fn xfs_dquot_is_enforced(dqp: *const xfs_dquot) -> bool {
    match xfs_dquot_type(dqp) as xfs_dqtype_t {
        XFS_DQTYPE_USER => XFS_IS_UQUOTA_ENFORCED((*dqp).q_mount),
        XFS_DQTYPE_GROUP => XFS_IS_GQUOTA_ENFORCED((*dqp).q_mount),
        XFS_DQTYPE_PROJ => XFS_IS_PQUOTA_ENFORCED((*dqp).q_mount),
        _ => { ASSERT(false); false }
    }
}

#[inline]
pub unsafe fn xfs_dquot_lowsp(dqp: *mut xfs_dquot) -> bool {
    let mut freesp = (*dqp).q_blk.hardlimit - (*dqp).q_blk.reserved;
    if freesp < (*dqp).q_blk_prealloc.q_low_space[XFS_QLOWSP_1_PCNT] { return true; }
    freesp = (*dqp).q_rtb.hardlimit - (*dqp).q_rtb.reserved;
    if freesp < (*dqp).q_rtb_prealloc.q_low_space[XFS_QLOWSP_1_PCNT] { return true; }
    false
}

#[inline]
pub unsafe fn XFS_DQ_IS_LOCKED(dqp: *mut xfs_dquot) -> bool {
    mutex_is_locked(&mut (*dqp).q_qlock)
}

#[inline]
pub unsafe fn XFS_DQ_IS_DIRTY(dqp: *mut xfs_dquot) -> u16 {
    (*dqp).q_flags & XFS_DQFLAG_DIRTY
}

extern "C" {
    pub fn xfs_dquot_to_disk(ddqp: *mut xfs_disk_dquot, dqp: *mut xfs_dquot);
    pub fn xfs_qm_dqdestroy(dqp: *mut xfs_dquot);
    pub fn xfs_qm_dqflush(dqp: *mut xfs_dquot, bp: *mut xfs_buf) -> c_int;
    pub fn xfs_qm_dqunpin_wait(dqp: *mut xfs_dquot);
    pub fn xfs_qm_adjust_dqtimers(d: *mut xfs_dquot);
    pub fn xfs_qm_adjust_dqlimits(d: *mut xfs_dquot);
    pub fn xfs_qm_id_for_quotatype(ip: *mut xfs_inode, type_: xfs_dqtype_t) -> xfs_dqid_t;
    pub fn xfs_qm_dqget(mp: *mut xfs_mount, id: xfs_dqid_t, type_: xfs_dqtype_t, can_alloc: bool, dqpp: *mut *mut xfs_dquot) -> c_int;
    pub fn xfs_qm_dqget_inode(ip: *mut xfs_inode, type_: xfs_dqtype_t, can_alloc: bool, dqpp: *mut *mut xfs_dquot) -> c_int;
    pub fn xfs_qm_dqget_next(mp: *mut xfs_mount, id: xfs_dqid_t, type_: xfs_dqtype_t, dqpp: *mut *mut xfs_dquot) -> c_int;
    pub fn xfs_qm_dqget_uncached(mp: *mut xfs_mount, id: xfs_dqid_t, type_: xfs_dqtype_t, dqpp: *mut *mut xfs_dquot) -> c_int;
    pub fn xfs_dqlock2(a: *mut xfs_dquot, b: *mut xfs_dquot);
    pub fn xfs_dqlockn(q: *mut xfs_dqtrx);
    pub fn xfs_dquot_set_prealloc_limits(d: *mut xfs_dquot);
    pub fn xfs_dquot_attach_buf(tp: *mut xfs_trans, dqp: *mut xfs_dquot) -> c_int;
    pub fn xfs_dquot_use_attached_buf(dqp: *mut xfs_dquot, bpp: *mut *mut xfs_buf) -> c_int;
    pub fn xfs_dquot_detach_buf(dqp: *mut xfs_dquot);
    pub fn xfs_dquot_set_timeout(mp: *mut xfs_mount, timeout: time64_t) -> time64_t;
    pub fn xfs_dquot_set_grace_period(grace: time64_t) -> time64_t;
    pub fn xfs_qm_init_dquot_blk(tp: *mut xfs_trans, id: xfs_dqid_t, type_: xfs_dqtype_t, bp: *mut xfs_buf);
}

#[inline]
pub unsafe fn xfs_qm_dqhold(dqp: *mut xfs_dquot) -> *mut xfs_dquot {
    lockref_get(&mut (*dqp).q_lockref);
    dqp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
