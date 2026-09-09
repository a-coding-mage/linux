// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <linux/compat.h>

#[repr(C)]
pub struct compat_if_dqblk {
    pub dqb_bhardlimit: u64,
    pub dqb_bsoftlimit: u64,
    pub dqb_curspace: u64,
    pub dqb_ihardlimit: u64,
    pub dqb_isoftlimit: u64,
    pub dqb_curinodes: u64,
    pub dqb_btime: u64,
    pub dqb_itime: u64,
    pub dqb_valid: u32,
}

#[repr(C)]
pub struct compat_fs_qfilestat {
    pub dqb_bhardlimit: u64,
    pub qfs_nblks: u64,
    pub qfs_nextents: u32,
}

#[repr(C)]
pub struct compat_fs_quota_stat {
    pub qs_version: i8,
    pub qs_flags: u16,
    pub qs_pad: i8,
    pub qs_uquota: compat_fs_qfilestat,
    pub qs_gquota: compat_fs_qfilestat,
    pub qs_incoredqs: u32,
    pub qs_btimelimit: i32,
    pub qs_itimelimit: i32,
    pub qs_rtbtimelimit: i32,
    pub qs_bwarnlimit: u16,
    pub qs_iwarnlimit: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
