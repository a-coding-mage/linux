/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2002
 */

// Dependency supplied by the surrounding repository: jfs_txnmgr.h

pub const BMAPVERSION: i32 = 1;
pub const TREESIZE: usize = 256 + 64 + 16 + 4 + 1;
pub const LEAFIND: usize = 64 + 16 + 4 + 1;
pub const LPERDMAP: usize = 256;
pub const L2LPERDMAP: i32 = 8;
pub const DBWORD: i32 = 32;
pub const L2DBWORD: i32 = 5;
pub const BUDMIN: i32 = L2DBWORD;
pub const BPERDMAP: i32 = (LPERDMAP as i32) * DBWORD;
pub const L2BPERDMAP: i32 = 13;
pub const CTLTREESIZE: usize = 1024 + 256 + 64 + 16 + 4 + 1;
pub const CTLLEAFIND: usize = 256 + 64 + 16 + 4 + 1;
pub const LPERCTL: usize = 1024;
pub const L2LPERCTL: i32 = 10;
pub const ROOT: i32 = 0;
pub const NOFREE: i8 = -1;
pub const MAXAG: usize = 128;
pub const L2MAXAG: i32 = 7;
pub const L2MINAGSZ: i32 = 25;
pub const BMAPBLKNO: i32 = 0;

pub const L2MAXL0SIZE: i32 = L2BPERDMAP + L2LPERCTL + 0;
pub const L2MAXL1SIZE: i32 = L2BPERDMAP + 2 * L2LPERCTL;
pub const L2MAXL2SIZE: i32 = L2BPERDMAP + 3 * L2LPERCTL;
pub const MAXL0SIZE: i64 = 1i64 << L2MAXL0SIZE;
pub const MAXL1SIZE: i64 = 1i64 << L2MAXL1SIZE;
pub const MAXL2SIZE: i64 = 1i64 << L2MAXL2SIZE;
pub const MAXMAPSIZE: i64 = MAXL2SIZE;

#[inline]
pub unsafe fn TREEMAX(cp: *mut i8) -> i8 {
    let tmp1 = *cp.add(2).max(cp.add(3));
    let tmp2 = *cp.max(cp.add(1));
    tmp1.max(tmp2)
}

#[inline]
pub const fn BLKTODMAP(b: i64, s: u32) -> i64 {
    (((b >> 13) + (b >> 23) + (b >> 33) + 3 + 1) << s)
}

#[inline]
pub const fn BLKTOL0(b: i64, s: u32) -> i64 {
    (((((b >> 23) << 10) + (b >> 23) + (b >> 33) + 2 + 1)) << s))
}

#[inline]
pub const fn BLKTOL1(b: i64, s: u32) -> i64 {
    (((((b >> 33) << 20) + ((b >> 33) << 10) + (b >> 33) + 1 + 1)) << s)
}

#[inline]
pub const fn BLKTOCTL(b: i64, s: u32, l: i32) -> i64 {
    if l == 2 { 1 } else if l == 1 { BLKTOL1(b, s) } else { BLKTOL0(b, s) }
}

#[inline]
pub const fn BMAPSZTOLEV(size: i64) -> i32 {
    if size <= MAXL0SIZE { 0 } else if size <= MAXL1SIZE { 1 } else { 2 }
}

#[repr(C)]
pub struct dmaptree {
    pub nleafs: u32,
    pub l2nleafs: u32,
    pub leafidx: u32,
    pub height: u32,
    pub budmin: i8,
    pub stree: [i8; TREESIZE],
    pub pad: [u8; 2],
}

#[repr(C)]
pub struct dmap {
    pub nblocks: u32,
    pub nfree: u32,
    pub start: u64,
    pub tree: dmaptree,
    pub pad: [u8; 1672],
    pub wmap: [u32; LPERDMAP],
    pub pmap: [u32; LPERDMAP],
}

#[repr(C)]
pub struct dmapctl {
    pub nleafs: u32,
    pub l2nleafs: u32,
    pub leafidx: u32,
    pub height: u32,
    pub budmin: i8,
    pub stree: [i8; CTLTREESIZE],
    pub pad: [u8; 2714],
}

#[repr(C)]
pub union dmtree {
    pub t1: dmaptree,
    pub t2: dmapctl,
}
pub type dmtree_t = dmtree;

#[repr(C)]
pub struct dbmap_disk {
    pub dn_mapsize: u64,
    pub dn_nfree: u64,
    pub dn_l2nbperpage: u32,
    pub dn_numag: u32,
    pub dn_maxlevel: u32,
    pub dn_maxag: u32,
    pub dn_agpref: u32,
    pub dn_aglevel: u32,
    pub dn_agheight: u32,
    pub dn_agwidth: u32,
    pub dn_agstart: u32,
    pub dn_agl2size: u32,
    pub dn_agfree: [u64; MAXAG],
    pub dn_agsize: u64,
    pub dn_maxfreebud: i8,
    pub pad: [u8; 3007],
}

#[repr(C)]
pub struct dbmap {
    pub dn_mapsize: i64,
    pub dn_nfree: i64,
    pub dn_l2nbperpage: i32,
    pub dn_numag: i32,
    pub dn_maxlevel: i32,
    pub dn_maxag: i32,
    pub dn_agpref: i32,
    pub dn_aglevel: i32,
    pub dn_agheight: i32,
    pub dn_agwidth: i32,
    pub dn_agstart: i32,
    pub dn_agl2size: i32,
    pub dn_agfree: [i64; MAXAG],
    pub dn_agsize: i64,
    pub dn_maxfreebud: i8,
}

#[repr(C)]
pub struct bmap {
    pub db_bmap: dbmap,
    pub db_ipbmap: *mut inode,
    pub db_bmaplock: mutex,
    pub db_active: [atomic_t; MAXAG],
    pub db_DBmap: *mut u32,
}

#[inline] pub fn BLKSTOL2(d: i64) -> i32 { blkstol2(d) }
#[inline] pub fn NLSTOL2BSZ(n: i32) -> i32 { 31 - cntlz(n) + BUDMIN }
#[inline] pub fn LITOL2BSZ(n: i32, m: i32, b: i32) -> i32 { (if n == 0 { m } else { cnttz(n) }) + b }
#[inline] pub const fn BLKTOCTLLEAF(b: i64, m: u32) -> i64 { (b & ((1i64 << (m + L2LPERCTL as u32)) - 1)) >> m }
#[inline] pub const fn BUDSIZE(s: i32, m: i32) -> i32 { 1 << (s - m) }

extern "C" {
    pub fn dbMount(ipbmap: *mut inode) -> i32;
    pub fn dbUnmount(ipbmap: *mut inode, mounterror: i32) -> i32;
    pub fn dbFree(ipbmap: *mut inode, blkno: i64, nblocks: i64) -> i32;
    pub fn dbUpdatePMap(ipbmap: *mut inode, free: i32, blkno: i64, nblocks: i64, tblk: *mut tblock) -> i32;
    pub fn dbNextAG(ipbmap: *mut inode) -> i32;
    pub fn dbAlloc(ipbmap: *mut inode, hint: i64, nblocks: i64, results: *mut i64) -> i32;
    pub fn dbReAlloc(ipbmap: *mut inode, blkno: i64, nblocks: i64, addnblocks: i64, results: *mut i64) -> i32;
    pub fn dbSync(ipbmap: *mut inode) -> i32;
    pub fn dbAllocBottomUp(ip: *mut inode, blkno: i64, nblocks: i64) -> i32;
    pub fn dbExtendFS(ipbmap: *mut inode, blkno: i64, nblocks: i64) -> i32;
    pub fn dbFinalizeBmap(ipbmap: *mut inode);
    pub fn dbMapFileSizeToMapSize(ipbmap: *mut inode) -> i64;
    pub fn dbDiscardAG(ip: *mut inode, agno: i32, minlen: i64) -> i64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
