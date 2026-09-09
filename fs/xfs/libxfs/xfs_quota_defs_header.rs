// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/*
 * Quota definitions shared between user and kernel source trees.
 */

/*
 * Even though users may not have quota limits occupying all 64-bits,
 * they may need 64-bit accounting. Hence, 64-bit quota-counters,
 * and quota-limits. This is a waste in the common case, but hey ...
 */
pub type xfs_qcnt_t = u64;
pub type xfs_dqtype_t = u8;

pub const XFS_DQTYPE_STRINGS: &[(u32, &str)] = &[
    (XFS_DQTYPE_USER, "USER"),
    (XFS_DQTYPE_PROJ, "PROJ"),
    (XFS_DQTYPE_GROUP, "GROUP"),
    (XFS_DQTYPE_BIGTIME, "BIGTIME"),
];

/* flags for q_flags field in the dquot. */
pub const XFS_DQFLAG_DIRTY: u32 = 1u32 << 0; /* dquot is dirty */
pub const XFS_DQFLAG_STRINGS: &[(u32, &str)] = &[(XFS_DQFLAG_DIRTY, "DIRTY")];

/* See the source comments for the dquot log reservation rationale. */
pub const XFS_DQUOT_LOGRES: usize =
    (core::mem::size_of::<xfs_dq_logformat>() + core::mem::size_of::<xfs_disk_dquot>()) * 6;

#[inline]
pub unsafe fn XFS_IS_QUOTA_ON(mp: *const xfs_mount) -> _ {
    (*mp).m_qflags & XFS_ALL_QUOTA_ACCT
}
#[inline]
pub unsafe fn XFS_IS_UQUOTA_ON(mp: *const xfs_mount) -> _ { (*mp).m_qflags & XFS_UQUOTA_ACCT }
#[inline]
pub unsafe fn XFS_IS_PQUOTA_ON(mp: *const xfs_mount) -> _ { (*mp).m_qflags & XFS_PQUOTA_ACCT }
#[inline]
pub unsafe fn XFS_IS_GQUOTA_ON(mp: *const xfs_mount) -> _ { (*mp).m_qflags & XFS_GQUOTA_ACCT }
#[inline]
pub unsafe fn XFS_IS_UQUOTA_ENFORCED(mp: *const xfs_mount) -> _ { (*mp).m_qflags & XFS_UQUOTA_ENFD }
#[inline]
pub unsafe fn XFS_IS_GQUOTA_ENFORCED(mp: *const xfs_mount) -> _ { (*mp).m_qflags & XFS_GQUOTA_ENFD }
#[inline]
pub unsafe fn XFS_IS_PQUOTA_ENFORCED(mp: *const xfs_mount) -> _ { (*mp).m_qflags & XFS_PQUOTA_ENFD }

/* Flags to tell various functions what to do. */
pub const XFS_QMOPT_UQUOTA: u32 = 1u32 << 0;
pub const XFS_QMOPT_GQUOTA: u32 = 1u32 << 1;
pub const XFS_QMOPT_PQUOTA: u32 = 1u32 << 2;
pub const XFS_QMOPT_FORCE_RES: u32 = 1u32 << 3;
pub const XFS_QMOPT_SBVERSION: u32 = 1u32 << 4;
pub const XFS_QMOPT_RES_REGBLKS: u32 = 1u32 << 7;
pub const XFS_QMOPT_RES_RTBLKS: u32 = 1u32 << 8;
pub const XFS_QMOPT_BCOUNT: u32 = 1u32 << 9;
pub const XFS_QMOPT_ICOUNT: u32 = 1u32 << 10;
pub const XFS_QMOPT_RTBCOUNT: u32 = 1u32 << 11;
pub const XFS_QMOPT_DELBCOUNT: u32 = 1u32 << 12;
pub const XFS_QMOPT_DELRTBCOUNT: u32 = 1u32 << 13;
pub const XFS_QMOPT_RES_INOS: u32 = 1u32 << 14;
pub const XFS_QMOPT_INHERIT: u32 = 1u32 << 31;

pub const XFS_QMOPT_QUOTALL: u32 = XFS_QMOPT_UQUOTA | XFS_QMOPT_PQUOTA | XFS_QMOPT_GQUOTA;
pub const XFS_QMOPT_RESBLK_MASK: u32 = XFS_QMOPT_RES_REGBLKS | XFS_QMOPT_RES_RTBLKS;

pub const XFS_TRANS_DQ_RES_BLKS: u32 = XFS_QMOPT_RES_REGBLKS;
pub const XFS_TRANS_DQ_RES_RTBLKS: u32 = XFS_QMOPT_RES_RTBLKS;
pub const XFS_TRANS_DQ_RES_INOS: u32 = XFS_QMOPT_RES_INOS;
pub const XFS_TRANS_DQ_BCOUNT: u32 = XFS_QMOPT_BCOUNT;
pub const XFS_TRANS_DQ_DELBCOUNT: u32 = XFS_QMOPT_DELBCOUNT;
pub const XFS_TRANS_DQ_ICOUNT: u32 = XFS_QMOPT_ICOUNT;
pub const XFS_TRANS_DQ_RTBCOUNT: u32 = XFS_QMOPT_RTBCOUNT;
pub const XFS_TRANS_DQ_DELRTBCOUNT: u32 = XFS_QMOPT_DELRTBCOUNT;

extern "C" {
    pub fn xfs_dquot_verify(mp: *mut xfs_mount, ddq: *mut xfs_disk_dquot, id: xfs_dqid_t) -> xfs_failaddr_t;
    pub fn xfs_dqblk_verify(mp: *mut xfs_mount, dqb: *mut xfs_dqblk, id: xfs_dqid_t) -> xfs_failaddr_t;
    pub fn xfs_calc_dquots_per_chunk(nbblks: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn xfs_dqblk_repair(mp: *mut xfs_mount, dqb: *mut xfs_dqblk, id: xfs_dqid_t, type_: xfs_dqtype_t);
    pub fn xfs_dqinode_sick_mask(type_: xfs_dqtype_t) -> core::ffi::c_uint;
    pub fn xfs_dqinode_load(tp: *mut xfs_trans, dp: *mut xfs_inode, type_: xfs_dqtype_t, ipp: *mut *mut xfs_inode) -> core::ffi::c_int;
    pub fn xfs_dqinode_metadir_create(dp: *mut xfs_inode, type_: xfs_dqtype_t, ipp: *mut *mut xfs_inode) -> core::ffi::c_int;
    pub fn xfs_dqinode_metadir_link(dp: *mut xfs_inode, type_: xfs_dqtype_t, ip: *mut xfs_inode) -> core::ffi::c_int;
    pub fn xfs_dqinode_mkdir_parent(mp: *mut xfs_mount, dpp: *mut *mut xfs_inode) -> core::ffi::c_int;
    pub fn xfs_dqinode_load_parent(tp: *mut xfs_trans, dpp: *mut *mut xfs_inode) -> core::ffi::c_int;
    pub fn xfs_dquot_from_disk_ts(ddq: *mut xfs_disk_dquot, dtimer: __be32) -> time64_t;
    pub fn xfs_dquot_to_disk_ts(ddq: *mut xfs_dquot, timer: time64_t) -> __be32;
}

#[inline]
pub unsafe fn xfs_dqinode_path(type_: xfs_dqtype_t) -> *const core::ffi::c_char {
    match type_ {
        XFS_DQTYPE_USER => b"user\0".as_ptr() as *const _,
        XFS_DQTYPE_GROUP => b"group\0".as_ptr() as *const _,
        XFS_DQTYPE_PROJ => b"project\0".as_ptr() as *const _,
        _ => { ASSERT!(false); core::ptr::null() }
    }
}

#[inline]
pub unsafe fn xfs_dqinode_metafile_type(type_: xfs_dqtype_t) -> xfs_metafile_type {
    match type_ {
        XFS_DQTYPE_USER => XFS_METAFILE_USRQUOTA,
        XFS_DQTYPE_GROUP => XFS_METAFILE_GRPQUOTA,
        XFS_DQTYPE_PROJ => XFS_METAFILE_PRJQUOTA,
        _ => { ASSERT!(false); XFS_METAFILE_UNKNOWN }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
