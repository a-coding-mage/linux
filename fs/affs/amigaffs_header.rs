/* SPDX-License-Identifier: GPL-2.0 */

// Translated from amigaffs.h. The original Linux endian-qualified integer
// types are represented by their fixed-width integer storage types here.

pub const FS_OFS: u32 = 0x444F5300;
pub const FS_FFS: u32 = 0x444F5301;
pub const FS_INTLOFS: u32 = 0x444F5302;
pub const FS_INTLFFS: u32 = 0x444F5303;
pub const FS_DCOFS: u32 = 0x444F5304;
pub const FS_DCFFS: u32 = 0x444F5305;
pub const MUFS_FS: u32 = 0x6d754653; // 'muFS'
pub const MUFS_OFS: u32 = 0x6d754600; // 'muF\0'
pub const MUFS_FFS: u32 = 0x6d754601; // 'muF\1'
pub const MUFS_INTLOFS: u32 = 0x6d754602; // 'muF\2'
pub const MUFS_INTLFFS: u32 = 0x6d754603; // 'muF\3'
pub const MUFS_DCOFS: u32 = 0x6d754604; // 'muF\4'
pub const MUFS_DCFFS: u32 = 0x6d754605; // 'muF\5'

pub const T_SHORT: u32 = 2;
pub const T_LIST: u32 = 16;
pub const T_DATA: u32 = 8;

pub const ST_LINKFILE: i32 = -4;
pub const ST_FILE: i32 = -3;
pub const ST_ROOT: i32 = 1;
pub const ST_USERDIR: i32 = 2;
pub const ST_SOFTLINK: i32 = 3;
pub const ST_LINKDIR: i32 = 4;

pub const AFFS_ROOT_BMAPS: usize = 25;

/* Seconds since Amiga epoch of 1978/01/01 to UNIX */
pub const AFFS_EPOCH_DELTA: i64 = ((8 * 365 + 2) * 86400_i64);

#[repr(C)]
pub struct affs_date {
    pub days: u32,
    pub mins: u32,
    pub ticks: u32,
}

#[repr(C)]
pub struct affs_short_date {
    pub days: u16,
    pub mins: u16,
    pub ticks: u16,
}

#[repr(C)]
pub struct affs_root_head {
    pub ptype: u32,
    /* The following fields are not used, but kept as documentation. */
    pub spare1: u32,
    pub spare2: u32,
    pub hash_size: u32,
    pub spare3: u32,
    pub checksum: u32,
    pub hashtable: [u32; 0],
}

#[repr(C)]
pub struct affs_root_tail {
    pub bm_flag: u32,
    pub bm_blk: [u32; AFFS_ROOT_BMAPS],
    pub bm_ext: u32,
    pub root_change: affs_date,
    pub disk_name: [u8; 32],
    pub spare1: u32,
    pub spare2: u32,
    pub disk_change: affs_date,
    pub disk_create: affs_date,
    pub spare3: u32,
    pub spare4: u32,
    pub dcache: u32,
    pub stype: u32,
}

#[repr(C)]
pub struct affs_head {
    pub ptype: u32,
    pub key: u32,
    pub block_count: u32,
    pub spare1: u32,
    pub first_data: u32,
    pub checksum: u32,
    pub table: [u32; 0],
}

#[repr(C)]
pub struct affs_tail {
    pub spare1: u32,
    pub uid: u16,
    pub gid: u16,
    pub protect: u32,
    pub size: u32,
    pub comment: [u8; 92],
    pub change: affs_date,
    pub name: [u8; 32],
    pub spare2: u32,
    pub original: u32,
    pub link_chain: u32,
    pub spare: [u32; 5],
    pub hash_chain: u32,
    pub parent: u32,
    pub extension: u32,
    pub stype: u32,
}

#[repr(C)]
pub struct slink_front {
    pub ptype: u32,
    pub key: u32,
    pub spare1: [u32; 3],
    pub checksum: u32,
    pub symname: [u8; 0], // depends on block size
}

#[repr(C)]
pub struct affs_data_head {
    pub ptype: u32,
    pub key: u32,
    pub sequence: u32,
    pub size: u32,
    pub next: u32,
    pub checksum: u32,
    pub data: [u8; 0], // depends on block size
}

/* Permission bits */

pub const FIBF_OTR_READ: u16 = 0x8000;
pub const FIBF_OTR_WRITE: u16 = 0x4000;
pub const FIBF_OTR_EXECUTE: u16 = 0x2000;
pub const FIBF_OTR_DELETE: u16 = 0x1000;
pub const FIBF_GRP_READ: u16 = 0x0800;
pub const FIBF_GRP_WRITE: u16 = 0x0400;
pub const FIBF_GRP_EXECUTE: u16 = 0x0200;
pub const FIBF_GRP_DELETE: u16 = 0x0100;

pub const FIBF_HIDDEN: u16 = 0x0080;
pub const FIBF_SCRIPT: u16 = 0x0040;
pub const FIBF_PURE: u16 = 0x0020; // no use under linux
pub const FIBF_ARCHIVED: u16 = 0x0010; // never set, always cleared on write
pub const FIBF_NOREAD: u16 = 0x0008; // 0 means allowed
pub const FIBF_NOWRITE: u16 = 0x0004; // 0 means allowed
pub const FIBF_NOEXECUTE: u16 = 0x0002; // 0 means allowed, ignored under linux
pub const FIBF_NODELETE: u16 = 0x0001; // 0 means allowed

pub const FIBF_OWNER: u16 = 0x000F; // Bits pertaining to owner
pub const FIBF_MASK: u16 = 0xEE0E; // Bits modified by Linux

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
