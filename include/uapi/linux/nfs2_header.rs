/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * NFS protocol definitions
 *
 * This file contains constants for Version 2 of the protocol.
 */

pub const NFS2_PORT: i32 = 2049;
pub const NFS2_MAXDATA: i32 = 8192;
pub const NFS2_MAXPATHLEN: i32 = 1024;
pub const NFS2_MAXNAMLEN: i32 = 255;
pub const NFS2_MAXGROUPS: i32 = 16;
pub const NFS2_FHSIZE: usize = 32;
pub const NFS2_COOKIESIZE: i32 = 4;
pub const NFS2_FIFO_DEV: i32 = -1;
pub const NFS2MODE_FMT: u32 = 0o170000;
pub const NFS2MODE_DIR: u32 = 0o040000;
pub const NFS2MODE_CHR: u32 = 0o020000;
pub const NFS2MODE_BLK: u32 = 0o060000;
pub const NFS2MODE_REG: u32 = 0o100000;
pub const NFS2MODE_LNK: u32 = 0o120000;
pub const NFS2MODE_SOCK: u32 = 0o140000;
pub const NFS2MODE_FIFO: u32 = 0o010000;

/* NFSv2 file types - beware, these are not the same in NFSv3 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfs2_ftype {
    NF2NON = 0,
    NF2REG = 1,
    NF2DIR = 2,
    NF2BLK = 3,
    NF2CHR = 4,
    NF2LNK = 5,
    NF2SOCK = 6,
    NF2BAD = 7,
    NF2FIFO = 8,
}

#[repr(C)]
pub struct nfs2_fh {
    pub data: [i8; NFS2_FHSIZE],
}

/*
 * Procedure numbers for NFSv2
 */
pub const NFS2_VERSION: i32 = 2;
pub const NFSPROC_NULL: i32 = 0;
pub const NFSPROC_GETATTR: i32 = 1;
pub const NFSPROC_SETATTR: i32 = 2;
pub const NFSPROC_ROOT: i32 = 3;
pub const NFSPROC_LOOKUP: i32 = 4;
pub const NFSPROC_READLINK: i32 = 5;
pub const NFSPROC_READ: i32 = 6;
pub const NFSPROC_WRITECACHE: i32 = 7;
pub const NFSPROC_WRITE: i32 = 8;
pub const NFSPROC_CREATE: i32 = 9;
pub const NFSPROC_REMOVE: i32 = 10;
pub const NFSPROC_RENAME: i32 = 11;
pub const NFSPROC_LINK: i32 = 12;
pub const NFSPROC_SYMLINK: i32 = 13;
pub const NFSPROC_MKDIR: i32 = 14;
pub const NFSPROC_RMDIR: i32 = 15;
pub const NFSPROC_READDIR: i32 = 16;
pub const NFSPROC_STATFS: i32 = 17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
