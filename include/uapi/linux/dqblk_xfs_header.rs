/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * Copyright (c) 1995-2001,2004 Silicon Graphics, Inc.  All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation.
 */

/* Disk quota - quotactl(2) commands for the XFS Quota Manager (XQM). */

#[inline]
pub const fn xqm_cmd(x: u32) -> u32 { ((b'X' as u32) << 8) + x }
#[inline]
pub const fn xqm_command(x: u32) -> bool { (x & (0xff << 8)) == ((b'X' as u32) << 8) }

pub const XQM_USRQUOTA: u32 = 0;
pub const XQM_GRPQUOTA: u32 = 1;
pub const XQM_PRJQUOTA: u32 = 2;
pub const XQM_MAXQUOTAS: u32 = 3;

pub const Q_XQUOTAON: u32 = xqm_cmd(1);
pub const Q_XQUOTAOFF: u32 = xqm_cmd(2);
pub const Q_XGETQUOTA: u32 = xqm_cmd(3);
pub const Q_XSETQLIM: u32 = xqm_cmd(4);
pub const Q_XGETQSTAT: u32 = xqm_cmd(5);
pub const Q_XQUOTARM: u32 = xqm_cmd(6);
pub const Q_XQUOTASYNC: u32 = xqm_cmd(7);
pub const Q_XGETQSTATV: u32 = xqm_cmd(8);
pub const Q_XGETNEXTQUOTA: u32 = xqm_cmd(9);

pub const FS_DQUOT_VERSION: i8 = 1;

#[repr(C)]
pub struct fs_disk_quota {
    pub d_version: i8,
    pub d_flags: i8,
    pub d_fieldmask: u16,
    pub d_id: u32,
    pub d_blk_hardlimit: u64,
    pub d_blk_softlimit: u64,
    pub d_ino_hardlimit: u64,
    pub d_ino_softlimit: u64,
    pub d_bcount: u64,
    pub d_icount: u64,
    pub d_itimer: i32,
    pub d_btimer: i32,
    pub d_iwarns: u16,
    pub d_bwarns: u16,
    pub d_itimer_hi: i8,
    pub d_btimer_hi: i8,
    pub d_rtbtimer_hi: i8,
    pub d_padding2: i8,
    pub d_rtb_hardlimit: u64,
    pub d_rtb_softlimit: u64,
    pub d_rtbcount: u64,
    pub d_rtbtimer: i32,
    pub d_rtbwarns: u16,
    pub d_padding3: i16,
    pub d_padding4: [i8; 8],
}
pub type fs_disk_quota_t = fs_disk_quota;

pub const FS_DQ_ISOFT: u16 = 1 << 0;
pub const FS_DQ_IHARD: u16 = 1 << 1;
pub const FS_DQ_BSOFT: u16 = 1 << 2;
pub const FS_DQ_BHARD: u16 = 1 << 3;
pub const FS_DQ_RTBSOFT: u16 = 1 << 4;
pub const FS_DQ_RTBHARD: u16 = 1 << 5;
pub const FS_DQ_LIMIT_MASK: u16 = FS_DQ_ISOFT | FS_DQ_IHARD | FS_DQ_BSOFT | FS_DQ_BHARD | FS_DQ_RTBSOFT | FS_DQ_RTBHARD;
pub const FS_DQ_BTIMER: u16 = 1 << 6;
pub const FS_DQ_ITIMER: u16 = 1 << 7;
pub const FS_DQ_RTBTIMER: u16 = 1 << 8;
pub const FS_DQ_TIMER_MASK: u16 = FS_DQ_BTIMER | FS_DQ_ITIMER | FS_DQ_RTBTIMER;
pub const FS_DQ_BWARNS: u16 = 1 << 9;
pub const FS_DQ_IWARNS: u16 = 1 << 10;
pub const FS_DQ_RTBWARNS: u16 = 1 << 11;
pub const FS_DQ_WARNS_MASK: u16 = FS_DQ_BWARNS | FS_DQ_IWARNS | FS_DQ_RTBWARNS;
pub const FS_DQ_BCOUNT: u16 = 1 << 12;
pub const FS_DQ_ICOUNT: u16 = 1 << 13;
pub const FS_DQ_RTBCOUNT: u16 = 1 << 14;
pub const FS_DQ_ACCT_MASK: u16 = FS_DQ_BCOUNT | FS_DQ_ICOUNT | FS_DQ_RTBCOUNT;
pub const FS_DQ_BIGTIME: u16 = 1 << 15;

pub const FS_QUOTA_UDQ_ACCT: u16 = 1 << 0;
pub const FS_QUOTA_UDQ_ENFD: u16 = 1 << 1;
pub const FS_QUOTA_GDQ_ACCT: u16 = 1 << 2;
pub const FS_QUOTA_GDQ_ENFD: u16 = 1 << 3;
pub const FS_QUOTA_PDQ_ACCT: u16 = 1 << 4;
pub const FS_QUOTA_PDQ_ENFD: u16 = 1 << 5;
pub const FS_USER_QUOTA: u16 = 1 << 0;
pub const FS_PROJ_QUOTA: u16 = 1 << 1;
pub const FS_GROUP_QUOTA: u16 = 1 << 2;

pub const FS_QSTAT_VERSION: i8 = 1;

#[repr(C)]
pub struct fs_qfilestat {
    pub qfs_ino: u64,
    pub qfs_nblks: u64,
    pub qfs_nextents: u32,
}
pub type fs_qfilestat_t = fs_qfilestat;

#[repr(C)]
pub struct fs_quota_stat {
    pub qs_version: i8,
    pub qs_flags: u16,
    pub qs_pad: i8,
    pub qs_uquota: fs_qfilestat_t,
    pub qs_gquota: fs_qfilestat_t,
    pub qs_incoredqs: u32,
    pub qs_btimelimit: i32,
    pub qs_itimelimit: i32,
    pub qs_rtbtimelimit: i32,
    pub qs_bwarnlimit: u16,
    pub qs_iwarnlimit: u16,
}
pub type fs_quota_stat_t = fs_quota_stat;

pub const FS_QSTATV_VERSION1: i8 = 1;

#[repr(C)]
pub struct fs_qfilestatv {
    pub qfs_ino: u64,
    pub qfs_nblks: u64,
    pub qfs_nextents: u32,
    pub qfs_pad: u32,
}

#[repr(C)]
pub struct fs_quota_statv {
    pub qs_version: i8,
    pub qs_pad1: u8,
    pub qs_flags: u16,
    pub qs_incoredqs: u32,
    pub qs_uquota: fs_qfilestatv,
    pub qs_gquota: fs_qfilestatv,
    pub qs_pquota: fs_qfilestatv,
    pub qs_btimelimit: i32,
    pub qs_itimelimit: i32,
    pub qs_rtbtimelimit: i32,
    pub qs_bwarnlimit: u16,
    pub qs_iwarnlimit: u16,
    pub qs_rtbwarnlimit: u16,
    pub qs_pad3: u16,
    pub qs_pad4: u32,
    pub qs_pad2: [u64; 7],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
