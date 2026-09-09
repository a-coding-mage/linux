/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2002
 */
/* Translated from jfs_xtree.h.  Types and functions supplied by dependencies
 * (including jfs_btree.h) are intentionally left external. */

use core::mem::ManuallyDrop;

#[repr(C)]
pub struct xad_t {
    pub flag: u8,
    pub rsvrd: [u8; 2],
    pub off1: u8,
    pub off2: __le32,
    pub loc: pxd_t,
}

pub const MAXXLEN: u32 = (1u32 << 24) - 1;
pub const XTSLOTSIZE: usize = 16;
pub const L2XTSLOTSIZE: usize = 4;

#[inline]
pub unsafe fn XADoffset(xad: *mut xad_t, offset64: u64) {
    (*xad).off1 = (offset64 >> 32) as u8;
    (*xad).off2 = __cpu_to_le32((offset64 & 0xffff_ffff) as u32);
}

#[inline]
pub unsafe fn XADaddress(xad: *mut xad_t, address64: u64) {
    PXDaddress(&mut (*xad).loc, address64);
}

#[inline]
pub unsafe fn XADlength(xad: *mut xad_t, length32: u32) {
    PXDlength(&mut (*xad).loc, length32);
}

#[inline]
pub unsafe fn offsetXAD(xad: *const xad_t) -> i64 {
    (((*xad).off1 as i64) << 32) | (__le32_to_cpu((*xad).off2) as i64)
}

#[inline]
pub unsafe fn addressXAD(xad: *const xad_t) -> u64 {
    addressPXD(&(*xad).loc)
}

#[inline]
pub unsafe fn lengthXAD(xad: *const xad_t) -> u32 {
    lengthPXD(&(*xad).loc)
}

#[repr(C)]
pub struct xadlist {
    pub maxnxad: i16,
    pub nxad: i16,
    pub xad: *mut xad_t,
}

pub const XAD_NEW: u8 = 0x01;
pub const XAD_EXTENDED: u8 = 0x02;
pub const XAD_COMPRESSED: u8 = 0x04;
pub const XAD_NOTRECORDED: u8 = 0x08;
pub const XAD_COW: u8 = 0x10;

pub const XTROOTINITSLOT_DIR: usize = 6;
pub const XTROOTINITSLOT: usize = 10;
pub const XTROOTMAXSLOT: usize = 18;
pub const XTPAGEMAXSLOT: usize = 256;
pub const XTENTRYSTART: usize = 2;

#[repr(C)]
pub struct xtheader {
    pub next: __le64,
    pub prev: __le64,
    pub flag: u8,
    pub rsrvd1: u8,
    pub nextindex: __le16,
    pub maxentry: __le16,
    pub rsrvd2: __le16,
    pub self_: pxd_t,
}

#[repr(C)]
pub union xtroot_t {
    pub header: ManuallyDrop<xtheader>,
    pub xad: ManuallyDrop<[xad_t; XTROOTMAXSLOT]>,
}

#[repr(C)]
pub union xtpage_t {
    pub header: ManuallyDrop<xtheader>,
    pub xad: ManuallyDrop<[xad_t; XTPAGEMAXSLOT]>,
}

extern "C" {
    pub fn xtLookup(
        ip: *mut inode, lstart: i64, llen: i64, pflag: *mut i32,
        paddr: *mut i64, plen: *mut i32, flag: i32,
    ) -> i32;
    pub fn xtInitRoot(tid: tid_t, ip: *mut inode);
    pub fn xtInsert(
        tid: tid_t, ip: *mut inode, xflag: i32, xoff: i64, xlen: i32,
        xaddrp: *mut i64, flag: i32,
    ) -> i32;
    pub fn xtExtend(tid: tid_t, ip: *mut inode, xoff: i64, xlen: i32, flag: i32) -> i32;
    pub fn xtUpdate(tid: tid_t, ip: *mut inode, nxad: *mut xad_t) -> i32;
    pub fn xtTruncate(tid: tid_t, ip: *mut inode, newsize: i64, type_: i32) -> i64;
    pub fn xtTruncate_pmap(tid: tid_t, ip: *mut inode, committed_size: i64) -> i64;
    pub fn xtAppend(
        tid: tid_t, ip: *mut inode, xflag: i32, xoff: i64, maxblocks: i32,
        xlenp: *mut i32, xaddrp: *mut i64, flag: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
