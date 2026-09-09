/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) International Business Machines Corp., 2000-2002 */
/* jfs_dtree.h: directory B+-tree manager */

use core::mem::ManuallyDrop;

/* Supplied by the surrounding JFS translation unit: tid_t, pxd_t, dasd,
 * inode, file, dir_context, component_name, btstack, ino_t, and integer
 * aliases, together with the referenced byte-order and filesystem helpers. */

#[repr(C)]
pub union DdataT {
    pub leaf: ManuallyDrop<DdataLeaf>,
    pub xd: ManuallyDrop<pxd_t>,
}

#[repr(C)]
pub struct DdataLeaf {
    pub tid: tid_t,
    pub ip: *mut inode,
    pub ino: u32,
}

#[repr(C)]
pub struct Dtslot {
    pub next: i8,
    pub cnt: i8,
    pub name: [__le16; 15],
}

pub const DATASLOTSIZE: usize = 16;
pub const L2DATASLOTSIZE: usize = 4;
pub const DTSLOTSIZE: usize = 32;
pub const L2DTSLOTSIZE: usize = 5;
pub const DTSLOTHDRSIZE: usize = 2;
pub const DTSLOTDATASIZE: usize = 30;
pub const DTSLOTDATALEN: usize = 15;

#[repr(C)]
pub struct Idtentry {
    pub xd: pxd_t,
    pub next: i8,
    pub namlen: u8,
    pub name: [__le16; 11],
}

pub const DTIHDRSIZE: usize = 10;
pub const DTIHDRDATALEN: usize = 11;

#[inline]
pub const fn ndtinternal(klen: usize) -> usize { (4 + klen + 14) / 15 }

#[repr(C)]
pub struct Ldtentry {
    pub inumber: __le32,
    pub next: i8,
    pub namlen: u8,
    pub name: [__le16; 11],
    pub index: __le32,
}

pub const DTLHDRSIZE: usize = 6;
pub const DTLHDRDATALEN_LEGACY: usize = 13;
pub const DTLHDRDATALEN: usize = 11;

/* DO_INDEX(INODE): JFS_SBI((INODE)->i_sb)->mntflag & JFS_DIR_INDEX */
pub const MAX_INLINE_DIRTABLE_ENTRY: usize = 13;

#[repr(C)]
pub struct DirTableSlot {
    pub rsrvd: u8,
    pub flag: u8,
    pub slot: u8,
    pub addr1: u8,
    pub addr2: __le32,
}

pub const DIR_INDEX_VALID: u8 = 1;
pub const DIR_INDEX_FREE: u8 = 0;

#[inline]
pub unsafe fn dts_address(slot: *mut DirTableSlot, address64: u64) {
    (*slot).addr1 = (address64 >> 32) as u8;
    (*slot).addr2 = __cpu_to_le32((address64 & 0xffff_ffff) as u32);
}

#[inline]
pub unsafe fn address_dts(dts: *const DirTableSlot) -> i64 {
    (((*dts).addr1 as i64) << 32) | (__le32_to_cpu((*dts).addr2) as i64)
}

#[inline]
pub const fn ndtleaf_legacy(klen: usize) -> usize { (2 + klen + 14) / 15 }
#[inline]
pub const fn ndtleaf(klen: usize) -> usize { ndtinternal(klen) }

#[repr(C)]
pub struct DtrootHeader {
    pub dasd: dasd,
    pub flag: u8,
    pub nextindex: u8,
    pub freecnt: i8,
    pub freelist: i8,
    pub idotdot: __le32,
    pub stbl: [i8; 8],
}

#[repr(C)]
pub union Dtroot {
    pub header: ManuallyDrop<DtrootHeader>,
    pub slot: ManuallyDrop<[Dtslot; 9]>,
}
pub type DtrootT = Dtroot;
pub const DTROOTMAXSLOT: usize = 9;

/* PARENT(IP) and dtEmpty(IP) use JFS_IP supplied by the surrounding code. */

#[repr(C)]
pub struct DtpageHeader {
    pub next: __le64,
    pub prev: __le64,
    pub flag: u8,
    pub nextindex: u8,
    pub freecnt: i8,
    pub freelist: i8,
    pub maxslot: u8,
    pub stblindex: u8,
    pub rsrvd: [u8; 2],
    pub self_: pxd_t,
}

#[repr(C)]
pub union Dtpage {
    pub header: ManuallyDrop<DtpageHeader>,
    pub slot: ManuallyDrop<[Dtslot; 128]>,
}
pub type DtpageT = Dtpage;
pub const DTPAGEMAXSLOT: usize = 128;
pub const DT8THPGNODEBYTES: usize = 512;
pub const DT8THPGNODETSLOTS: usize = 1;
pub const DT8THPGNODESLOTS: usize = 16;
pub const DTQTRPGNODEBYTES: usize = 1024;
pub const DTQTRPGNODETSLOTS: usize = 1;
pub const DTQTRPGNODESLOTS: usize = 32;
pub const DTHALFPGNODEBYTES: usize = 2048;
pub const DTHALFPGNODETSLOTS: usize = 2;
pub const DTHALFPGNODESLOTS: usize = 64;
pub const DTFULLPGNODEBYTES: usize = 4096;
pub const DTFULLPGNODETSLOTS: usize = 4;
pub const DTFULLPGNODESLOTS: usize = 128;
pub const DTENTRYSTART: i32 = 1;

pub const JFS_CREATE: i32 = 1;
pub const JFS_LOOKUP: i32 = 2;
pub const JFS_REMOVE: i32 = 3;
pub const JFS_RENAME: i32 = 4;
pub const DIREND: i32 = i32::MAX;

extern "C" {
    pub fn dtInitRoot(tid: tid_t, ip: *mut inode, idotdot: u32);
    pub fn dtSearch(ip: *mut inode, key: *mut component_name, data: *mut ino_t, btstack: *mut btstack, flag: i32) -> i32;
    pub fn dtInsert(tid: tid_t, ip: *mut inode, key: *mut component_name, ino: *mut ino_t, btstack: *mut btstack) -> i32;
    pub fn dtDelete(tid: tid_t, ip: *mut inode, key: *mut component_name, data: *mut ino_t, flag: i32) -> i32;
    pub fn dtModify(tid: tid_t, ip: *mut inode, key: *mut component_name, orig_ino: *mut ino_t, new_ino: ino_t, flag: i32) -> i32;
    pub fn jfs_readdir(file: *mut file, ctx: *mut dir_context) -> i32;
    pub fn check_dtroot(p: *mut DtrootT) -> bool;
    pub fn check_dtpage(p: *mut DtpageT) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
