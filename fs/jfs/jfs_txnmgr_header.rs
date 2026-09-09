/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 */

// Dependency declarations supplied by the surrounding translation unit:
// #include "jfs_logmgr.h"

/* Hide implementation of TxBlock and TxLock */
#[inline]
pub unsafe fn tid_to_tblock(tid: tid_t) -> *mut tblock {
    TxBlock.add(tid as usize)
}

#[inline]
pub unsafe fn lid_to_tlock(lid: lid_t) -> *mut tlock {
    TxLock.add(lid as usize)
}

/* transaction block */
#[repr(C)]
pub struct tblock {
    pub xflag: u16,
    pub flag: u16,
    pub dummy: lid_t,
    pub lsn: i32,
    pub synclist: list_head,
    pub sb: *mut super_block,
    pub next: lid_t,
    pub last: lid_t,
    pub waitor: wait_queue_head_t,
    pub logtid: u32,
    pub cqueue: list_head,
    pub clsn: i32,
    pub bp: *mut lbuf,
    pub pn: i32,
    pub eor: i32,
    pub gcwait: wait_queue_head_t,
    pub u: tblock_u,
    pub ino: u32,
}

#[repr(C)]
pub union tblock_u {
    pub ip: *mut inode,
    pub ixpxd: pxd_t,
}

extern "C" {
    pub static mut TxBlock: *mut tblock;
}

/* commit flags: tblk->xflag */
pub const COMMIT_SYNC: u16 = 0x0001;
pub const COMMIT_FORCE: u16 = 0x0002;
pub const COMMIT_FLUSH: u16 = 0x0004;
pub const COMMIT_MAP: u16 = 0x00f0;
pub const COMMIT_PMAP: u16 = 0x0010;
pub const COMMIT_WMAP: u16 = 0x0020;
pub const COMMIT_PWMAP: u16 = 0x0040;
pub const COMMIT_FREE: u16 = 0x0f00;
pub const COMMIT_DELETE: u16 = 0x0100;
pub const COMMIT_TRUNCATE: u16 = 0x0200;
pub const COMMIT_CREATE: u16 = 0x0400;
pub const COMMIT_LAZY: u16 = 0x0800;
pub const COMMIT_PAGE: u16 = 0x1000;
pub const COMMIT_INODE: u16 = 0x2000;

/* transaction lock */
#[repr(C)]
pub struct tlock {
    pub next: lid_t,
    pub tid: tid_t,
    pub flag: u16,
    pub type_: u16,
    pub mp: *mut metapage,
    pub ip: *mut inode,
    pub lock: [i16; 24],
}

extern "C" {
    pub static mut TxLock: *mut tlock;
}

pub const tlckPAGELOCK: u16 = 0x8000;
pub const tlckINODELOCK: u16 = 0x4000;
pub const tlckLINELOCK: u16 = 0x2000;
pub const tlckINLINELOCK: u16 = 0x1000;
pub const tlckLOG: u16 = 0x0800;
pub const tlckUPDATEMAP: u16 = 0x0080;
pub const tlckDIRECTORY: u16 = 0x0040;
pub const tlckFREELOCK: u16 = 0x0008;
pub const tlckWRITEPAGE: u16 = 0x0004;
pub const tlckFREEPAGE: u16 = 0x0002;

pub const tlckTYPE: u16 = 0xfe00;
pub const tlckINODE: u16 = 0x8000;
pub const tlckXTREE: u16 = 0x4000;
pub const tlckDTREE: u16 = 0x2000;
pub const tlckMAP: u16 = 0x1000;
pub const tlckEA: u16 = 0x0800;
pub const tlckACL: u16 = 0x0400;
pub const tlckDATA: u16 = 0x0200;
pub const tlckBTROOT: u16 = 0x0100;
pub const tlckOPERATION: u16 = 0x00ff;
pub const tlckGROW: u16 = 0x0001;
pub const tlckREMOVE: u16 = 0x0002;
pub const tlckTRUNCATE: u16 = 0x0004;
pub const tlckRELOCATE: u16 = 0x0008;
pub const tlckENTRY: u16 = 0x0001;
pub const tlckEXTEND: u16 = 0x0002;
pub const tlckSPLIT: u16 = 0x0010;
pub const tlckNEW: u16 = 0x0020;
pub const tlckFREE: u16 = 0x0040;
pub const tlckRELINK: u16 = 0x0080;

#[repr(C)]
pub struct lv { pub offset: u8, pub length: u8 }
pub const TLOCKSHORT: usize = 20;
pub const TLOCKLONG: usize = 28;

#[repr(C)]
pub struct linelock {
    pub next: lid_t, pub maxcnt: i8, pub index: i8, pub flag: u16,
    pub type_: u8, pub l2linesize: u8, pub lv: [lv; 20],
}
pub type dt_lock = linelock;

#[repr(C)]
pub struct xtlock {
    pub next: lid_t, pub maxcnt: i8, pub index: i8, pub flag: u16,
    pub type_: u8, pub l2linesize: u8, pub header: lv, pub lwm: lv,
    pub hwm: lv, pub twm: lv, pub pxdlock: [i32; 8],
}

#[repr(C)]
pub struct maplock {
    pub next: lid_t, pub maxcnt: u8, pub index: u8, pub flag: u16,
    pub type_: u8, pub count: u8, pub pxd: pxd_t,
}
pub const mlckALLOC: u16 = 0x00f0;
pub const mlckALLOCXADLIST: u16 = 0x0080;
pub const mlckALLOCPXDLIST: u16 = 0x0040;
pub const mlckALLOCXAD: u16 = 0x0020;
pub const mlckALLOCPXD: u16 = 0x0010;
pub const mlckFREE: u16 = 0x000f;
pub const mlckFREEXADLIST: u16 = 0x0008;
pub const mlckFREEPXDLIST: u16 = 0x0004;
pub const mlckFREEXAD: u16 = 0x0002;
pub const mlckFREEPXD: u16 = 0x0001;
pub type pxd_lock = maplock;

#[repr(C)]
pub union xdlist_union {
    pub _xdlist: *mut core::ffi::c_void,
    pub pad: i64,
}
#[repr(C)]
pub struct xdlistlock {
    pub next: lid_t, pub maxcnt: u8, pub index: u8, pub flag: u16,
    pub type_: u8, pub count: u8, pub union64: xdlist_union,
}

#[repr(C)]
pub struct commit {
    pub tid: tid_t, pub flag: i32, pub log: *mut jfs_log, pub sb: *mut super_block,
    pub nip: i32, pub iplist: *mut *mut inode, pub lrd: lrd,
}

extern "C" {
    pub static mut jfs_tlocks_low: i32;
    pub fn txInit() -> i32;
    pub fn txExit();
    pub fn txLock(tid: tid_t, ip: *mut inode, mp: *mut metapage, flag: i32) -> *mut tlock;
    pub fn txMaplock(tid: tid_t, ip: *mut inode, flag: i32) -> *mut tlock;
    pub fn txCommit(tid: tid_t, flag: i32, iplist: *mut *mut inode, nip: i32) -> i32;
    pub fn txBegin(sb: *mut super_block, flag: i32) -> tid_t;
    pub fn txBeginAnon(sb: *mut super_block);
    pub fn txEnd(tid: tid_t);
    pub fn txAbort(tid: tid_t, flag: i32);
    pub fn txLinelock(lock: *mut linelock) -> *mut linelock;
    pub fn txFreeMap(ip: *mut inode, maplock: *mut maplock, tblk: *mut tblock, flag: i32);
    pub fn txEA(tid: tid_t, ip: *mut inode, dxda: *mut dxd_t, dxdb: *mut dxd_t);
    pub fn txFreelock(ip: *mut inode);
    pub fn lmLog(log: *mut jfs_log, tblk: *mut tblock, lrd: *mut lrd, tlck: *mut tlock) -> i32;
    pub fn txQuiesce(sb: *mut super_block);
    pub fn txResume(sb: *mut super_block);
    pub fn txLazyUnlock(tblk: *mut tblock);
    pub fn jfs_lazycommit(arg: *mut core::ffi::c_void) -> i32;
    pub fn jfs_sync(arg: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
