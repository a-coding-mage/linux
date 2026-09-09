/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * NFSv3 protocol definitions
 */

pub const NFS3_PORT: i32 = 2049;
pub const NFS3_MAXDATA: i32 = 32768;
// PATH_MAX and NAME_MAX are supplied by the surrounding environment.
pub const NFS3_MAXPATHLEN: usize = PATH_MAX;
pub const NFS3_MAXNAMLEN: usize = NAME_MAX;
pub const NFS3_MAXGROUPS: i32 = 16;
pub const NFS3_FHSIZE: usize = 64;
pub const NFS3_COOKIESIZE: usize = 4;
pub const NFS3_CREATEVERFSIZE: usize = 8;
pub const NFS3_COOKIEVERFSIZE: usize = 8;
pub const NFS3_WRITEVERFSIZE: usize = 8;
pub const NFS3_FIFO_DEV: i32 = -1;
pub const NFS3MODE_FMT: u32 = 0o170000;
pub const NFS3MODE_DIR: u32 = 0o040000;
pub const NFS3MODE_CHR: u32 = 0o020000;
pub const NFS3MODE_BLK: u32 = 0o060000;
pub const NFS3MODE_REG: u32 = 0o100000;
pub const NFS3MODE_LNK: u32 = 0o120000;
pub const NFS3MODE_SOCK: u32 = 0o140000;
pub const NFS3MODE_FIFO: u32 = 0o010000;

/* Flags for access() call */
pub const NFS3_ACCESS_READ: u32 = 0x0001;
pub const NFS3_ACCESS_LOOKUP: u32 = 0x0002;
pub const NFS3_ACCESS_MODIFY: u32 = 0x0004;
pub const NFS3_ACCESS_EXTEND: u32 = 0x0008;
pub const NFS3_ACCESS_DELETE: u32 = 0x0010;
pub const NFS3_ACCESS_EXECUTE: u32 = 0x0020;
pub const NFS3_ACCESS_FULL: u32 = 0x003f;

/* Flags for create mode */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfs3_createmode {
    NFS3_CREATE_UNCHECKED = 0,
    NFS3_CREATE_GUARDED = 1,
    NFS3_CREATE_EXCLUSIVE = 2,
}

/* NFSv3 file system properties */
pub const NFS3_FSF_LINK: u32 = 0x0001;
pub const NFS3_FSF_SYMLINK: u32 = 0x0002;
pub const NFS3_FSF_HOMOGENEOUS: u32 = 0x0008;
pub const NFS3_FSF_CANSETTIME: u32 = 0x0010;
/* Some shorthands. See fs/nfsd/nfs3proc.c */
pub const NFS3_FSF_DEFAULT: u32 = 0x001B;
pub const NFS3_FSF_BILLYBOY: u32 = 0x0018;
pub const NFS3_FSF_READONLY: u32 = 0x0008;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfs3_ftype {
    NF3NON = 0,
    NF3REG = 1,
    NF3DIR = 2,
    NF3BLK = 3,
    NF3CHR = 4,
    NF3LNK = 5,
    NF3SOCK = 6,
    NF3FIFO = 7, /* changed from NFSv2 (was 8) */
    NF3BAD = 8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfs3_time_how {
    DONT_CHANGE = 0,
    SET_TO_SERVER_TIME = 1,
    SET_TO_CLIENT_TIME = 2,
}

#[repr(C)]
pub struct nfs3_fh {
    pub size: u16,
    pub data: [u8; NFS3_FHSIZE],
}

pub const NFS3_VERSION: i32 = 3;
pub const NFS3PROC_NULL: i32 = 0;
pub const NFS3PROC_GETATTR: i32 = 1;
pub const NFS3PROC_SETATTR: i32 = 2;
pub const NFS3PROC_LOOKUP: i32 = 3;
pub const NFS3PROC_ACCESS: i32 = 4;
pub const NFS3PROC_READLINK: i32 = 5;
pub const NFS3PROC_READ: i32 = 6;
pub const NFS3PROC_WRITE: i32 = 7;
pub const NFS3PROC_CREATE: i32 = 8;
pub const NFS3PROC_MKDIR: i32 = 9;
pub const NFS3PROC_SYMLINK: i32 = 10;
pub const NFS3PROC_MKNOD: i32 = 11;
pub const NFS3PROC_REMOVE: i32 = 12;
pub const NFS3PROC_RMDIR: i32 = 13;
pub const NFS3PROC_RENAME: i32 = 14;
pub const NFS3PROC_LINK: i32 = 15;
pub const NFS3PROC_READDIR: i32 = 16;
pub const NFS3PROC_READDIRPLUS: i32 = 17;
pub const NFS3PROC_FSSTAT: i32 = 18;
pub const NFS3PROC_FSINFO: i32 = 19;
pub const NFS3PROC_PATHCONF: i32 = 20;
pub const NFS3PROC_COMMIT: i32 = 21;

pub const NFS_MNT3_VERSION: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
