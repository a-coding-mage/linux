/*
 * Copyright (c) 1982, 1986 Regents of the University of California.
 * All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Robert Elz at The University of Melbourne.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

// Dependency supplied by the corresponding Linux types translation.

pub const __DQUOT_VERSION__: &str = "dquot_6.6.0";

pub const MAXQUOTAS: i32 = 3;
pub const USRQUOTA: i32 = 0; // element used for user quotas
pub const GRPQUOTA: i32 = 1; // element used for group quotas
pub const PRJQUOTA: i32 = 2; // element used for project quotas

// Definitions for the default names of the quotas files.
pub const INITQFNAMES: [&str; 4] = ["user", "group", "project", "undefined"];

// Command definitions for the 'quotactl' system call.
// The commands are broken into a main command defined below
// and a subcommand that is used to convey the type of
// quota that is being manipulated (see above).
pub const SUBCMDMASK: u32 = 0x00ff;
pub const SUBCMDSHIFT: u32 = 8;
pub const fn QCMD(cmd: u32, type_: u32) -> u32 {
    (cmd << SUBCMDSHIFT) | (type_ & SUBCMDMASK)
}

pub const Q_SYNC: u32 = 0x800001; // sync disk copy of a filesystems quotas
pub const Q_QUOTAON: u32 = 0x800002; // turn quotas on
pub const Q_QUOTAOFF: u32 = 0x800003; // turn quotas off
pub const Q_GETFMT: u32 = 0x800004; // get quota format used on given filesystem
pub const Q_GETINFO: u32 = 0x800005; // get information about quota files
pub const Q_SETINFO: u32 = 0x800006; // set information about quota files
pub const Q_GETQUOTA: u32 = 0x800007; // get user quota structure
pub const Q_SETQUOTA: u32 = 0x800008; // set user quota structure
pub const Q_GETNEXTQUOTA: u32 = 0x800009; // get disk limits and usage >= ID

// Quota format type IDs
pub const QFMT_VFS_OLD: i32 = 1;
pub const QFMT_VFS_V0: i32 = 2;
pub const QFMT_OCFS2: i32 = 3;
pub const QFMT_VFS_V1: i32 = 4;
pub const QFMT_SHMEM: i32 = 5;

// Size of block in which space limits are passed through the quota
// interface
pub const QIF_DQBLKSIZE_BITS: i32 = 10;
pub const QIF_DQBLKSIZE: i32 = 1 << QIF_DQBLKSIZE_BITS;

// Quota structure used for communication with userspace via quotactl
// Following flags are used to specify which fields are valid
pub const QIF_BLIMITS_B: i32 = 0;
pub const QIF_SPACE_B: i32 = 1;
pub const QIF_ILIMITS_B: i32 = 2;
pub const QIF_INODES_B: i32 = 3;
pub const QIF_BTIME_B: i32 = 4;
pub const QIF_ITIME_B: i32 = 5;

pub const QIF_BLIMITS: u32 = 1 << QIF_BLIMITS_B;
pub const QIF_SPACE: u32 = 1 << QIF_SPACE_B;
pub const QIF_ILIMITS: u32 = 1 << QIF_ILIMITS_B;
pub const QIF_INODES: u32 = 1 << QIF_INODES_B;
pub const QIF_BTIME: u32 = 1 << QIF_BTIME_B;
pub const QIF_ITIME: u32 = 1 << QIF_ITIME_B;
pub const QIF_LIMITS: u32 = QIF_BLIMITS | QIF_ILIMITS;
pub const QIF_USAGE: u32 = QIF_SPACE | QIF_INODES;
pub const QIF_TIMES: u32 = QIF_BTIME | QIF_ITIME;
pub const QIF_ALL: u32 = QIF_LIMITS | QIF_USAGE | QIF_TIMES;

#[repr(C)]
pub struct if_dqblk {
    pub dqb_bhardlimit: __u64,
    pub dqb_bsoftlimit: __u64,
    pub dqb_curspace: __u64,
    pub dqb_ihardlimit: __u64,
    pub dqb_isoftlimit: __u64,
    pub dqb_curinodes: __u64,
    pub dqb_btime: __u64,
    pub dqb_itime: __u64,
    pub dqb_valid: __u32,
}

#[repr(C)]
pub struct if_nextdqblk {
    pub dqb_bhardlimit: __u64,
    pub dqb_bsoftlimit: __u64,
    pub dqb_curspace: __u64,
    pub dqb_ihardlimit: __u64,
    pub dqb_isoftlimit: __u64,
    pub dqb_curinodes: __u64,
    pub dqb_btime: __u64,
    pub dqb_itime: __u64,
    pub dqb_valid: __u32,
    pub dqb_id: __u32,
}

// Structure used for setting quota information about file via quotactl
// Following flags are used to specify which fields are valid
pub const IIF_BGRACE: i32 = 1;
pub const IIF_IGRACE: i32 = 2;
pub const IIF_FLAGS: i32 = 4;
pub const IIF_ALL: i32 = IIF_BGRACE | IIF_IGRACE | IIF_FLAGS;

pub const DQF_ROOT_SQUASH_B: i32 = 0;
pub const DQF_SYS_FILE_B: i32 = 16;
// Kernel internal flags invisible to userspace
pub const DQF_PRIVATE: i32 = 17;

// Root squash enabled (for v1 quota format)
pub const DQF_ROOT_SQUASH: u32 = 1 << DQF_ROOT_SQUASH_B;
// Quota stored in a system file
pub const DQF_SYS_FILE: u32 = 1 << DQF_SYS_FILE_B;

#[repr(C)]
pub struct if_dqinfo {
    pub dqi_bgrace: __u64,
    pub dqi_igrace: __u64,
    pub dqi_flags: __u32, // DFQ_*
    pub dqi_valid: __u32,
}

// Definitions for quota netlink interface
pub const QUOTA_NL_NOWARN: i32 = 0;
pub const QUOTA_NL_IHARDWARN: i32 = 1; // Inode hardlimit reached
pub const QUOTA_NL_ISOFTLONGWARN: i32 = 2; // Inode grace time expired
pub const QUOTA_NL_ISOFTWARN: i32 = 3; // Inode softlimit reached
pub const QUOTA_NL_BHARDWARN: i32 = 4; // Block hardlimit reached
pub const QUOTA_NL_BSOFTLONGWARN: i32 = 5; // Block grace time expired
pub const QUOTA_NL_BSOFTWARN: i32 = 6; // Block softlimit reached
pub const QUOTA_NL_IHARDBELOW: i32 = 7; // Usage got below inode hardlimit
pub const QUOTA_NL_ISOFTBELOW: i32 = 8; // Usage got below inode softlimit
pub const QUOTA_NL_BHARDBELOW: i32 = 9; // Usage got below block hardlimit
pub const QUOTA_NL_BSOFTBELOW: i32 = 10; // Usage got below block softlimit

pub const QUOTA_NL_C_UNSPEC: i32 = 0;
pub const QUOTA_NL_C_WARNING: i32 = 1;
pub const __QUOTA_NL_C_MAX: i32 = 2;
pub const QUOTA_NL_C_MAX: i32 = __QUOTA_NL_C_MAX - 1;

pub const QUOTA_NL_A_UNSPEC: i32 = 0;
pub const QUOTA_NL_A_QTYPE: i32 = 1;
pub const QUOTA_NL_A_EXCESS_ID: i32 = 2;
pub const QUOTA_NL_A_WARNING: i32 = 3;
pub const QUOTA_NL_A_DEV_MAJOR: i32 = 4;
pub const QUOTA_NL_A_DEV_MINOR: i32 = 5;
pub const QUOTA_NL_A_CAUSED_ID: i32 = 6;
pub const QUOTA_NL_A_PAD: i32 = 7;
pub const __QUOTA_NL_A_MAX: i32 = 8;
pub const QUOTA_NL_A_MAX: i32 = __QUOTA_NL_A_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
