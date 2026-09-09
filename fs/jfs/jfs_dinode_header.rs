/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2001
 */

/* jfs_dinode.h: on-disk inode manager */

pub const INODESLOTSIZE: usize = 128;
pub const L2INODESLOTSIZE: usize = 7;
pub const log2INODESIZE: usize = 9; /* log2(bytes per dinode) */

/* on-disk inode: 512 bytes.  External types are supplied by other headers. */
#[repr(C)]
pub struct dinode {
    pub di_inostamp: __le32,
    pub di_fileset: __le32,
    pub di_number: __le32,
    pub di_gen: __le32,
    pub di_ixpxd: pxd_t,
    pub di_size: __le64,
    pub di_nblocks: __le64,
    pub di_nlink: __le32,
    pub di_uid: __le32,
    pub di_gid: __le32,
    pub di_mode: __le32,
    pub di_atime: timestruc_t,
    pub di_ctime: timestruc_t,
    pub di_mtime: timestruc_t,
    pub di_otime: timestruc_t,
    pub di_acl: dxd_t,
    pub di_ea: dxd_t,
    pub di_next_index: __le32,
    pub di_acltype: __le32,
    pub u: dinode_u,
}

#[repr(C)]
pub union dinode_u {
    pub _dir: ManuallyDrop<dinode_dir>,
    pub _file: ManuallyDrop<dinode_file>,
}

#[repr(C)]
pub struct dinode_dir {
    pub _table: [dir_table_slot; 12],
    pub _dtroot: dtroot_t,
}

#[repr(C)]
pub struct dinode_file {
    pub _u1: dinode_file_u1,
    pub _u2: dinode_file_u2,
}

#[repr(C)]
pub union dinode_file_u1 {
    pub _data: [u8; 96],
    pub _imap: ManuallyDrop<dinode_imap>,
}

#[repr(C)]
pub struct dinode_imap {
    pub _imap: *mut core::ffi::c_void,
    pub _gengen: __le32,
}

#[repr(C)]
pub union dinode_file_u2 {
    pub _xtroot: ManuallyDrop<xtroot_t>,
    pub _special: ManuallyDrop<dinode_special>,
}

#[repr(C)]
pub struct dinode_special {
    pub unused: [u8; 16],
    pub _dxd: dxd_t,
    pub _u: dinode_special_u,
}

#[repr(C)]
pub union dinode_special_u {
    pub _inline_all: [u8; 256],
    pub _fast: ManuallyDrop<dinode_fast>,
}

#[repr(C)]
pub struct dinode_fast {
    pub _u: dinode_rdev_or_symlink,
    pub _inlineea: [u8; 128],
}

#[repr(C)]
pub union dinode_rdev_or_symlink {
    pub _rdev: __le32,
    pub _fastsymlink: [u8; 128],
}

pub const IFJOURNAL: u32 = 0x0001_0000;
pub const ISPARSE: u32 = 0x0002_0000;
pub const INLINEEA: u32 = 0x0004_0000;
pub const ISWAPFILE: u32 = 0x0080_0000;
pub const IREADONLY: u32 = 0x0200_0000;
pub const IHIDDEN: u32 = 0x0400_0000;
pub const ISYSTEM: u32 = 0x0800_0000;
pub const IDIRECTORY: u32 = 0x2000_0000;
pub const IARCHIVE: u32 = 0x4000_0000;
pub const INEWNAME: u32 = 0x8000_0000;
pub const IRASH: u32 = 0x4E00_0000;
pub const ATTRSHIFT: u32 = 25;
pub const JFS_NOATIME_FL: u32 = 0x0008_0000;
pub const JFS_DIRSYNC_FL: u32 = 0x0010_0000;
pub const JFS_SYNC_FL: u32 = 0x0020_0000;
pub const JFS_SECRM_FL: u32 = 0x0040_0000;
pub const JFS_UNRM_FL: u32 = 0x0080_0000;
pub const JFS_APPEND_FL: u32 = 0x0100_0000;
pub const JFS_IMMUTABLE_FL: u32 = 0x0200_0000;
pub const JFS_FL_USER_VISIBLE: u32 = 0x03F8_0000;
pub const JFS_FL_USER_MODIFIABLE: u32 = 0x03F8_0000;
pub const JFS_FL_INHERIT: u32 = 0x03C8_0000;

use core::mem::ManuallyDrop;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
