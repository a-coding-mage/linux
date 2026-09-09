/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2002
 */

// Dependency declarations supplied by the translated project.

pub const EXTSPERIAG: usize = 128;
pub const IMAPBLKNO: usize = 0;
pub const SMAPSZ: usize = 4;
pub const EXTSPERSUM: usize = 32;
pub const L2EXTSPERSUM: usize = 5;
pub const PGSPERIEXT: usize = 4;
pub const MAXIAGS: u32 = (1u32 << 20) - 1;
pub const MAXAG: usize = 128;
pub const AMAPSIZE: usize = 512;
pub const SMAPSIZE: usize = 16;

#[inline]
pub const fn inotoiag(ino: u64) -> u64 { ino >> L2INOSPERIAG }

#[inline]
pub const fn iagtolblk(iagno: u64, l2nbperpg: u32) -> u64 {
    (iagno + 1) << l2nbperpg
}

#[inline]
pub unsafe fn inopblk(pxd: *const pxd_t, ino: u64, l2nbperpg: u32) -> u64 {
    addressPXD(pxd) + ((((ino & (INOSPEREXT - 1)) >> L2INOSPERPAGE) << l2nbperpg))
}

#[repr(C)]
pub struct iag {
    pub agstart: __le64,
    pub iagnum: __le32,
    pub inofreefwd: __le32,
    pub inofreeback: __le32,
    pub extfreefwd: __le32,
    pub extfreeback: __le32,
    pub iagfree: __le32,
    pub inosmap: [__le32; SMAPSZ],
    pub extsmap: [__le32; SMAPSZ],
    pub nfreeinos: __le32,
    pub nfreeexts: __le32,
    pub pad: [u8; 1976],
    pub wmap: [__le32; EXTSPERIAG],
    pub pmap: [__le32; EXTSPERIAG],
    pub inoext: [pxd_t; EXTSPERIAG],
}

#[repr(C)]
pub struct iagctl_disk {
    pub inofree: __le32,
    pub extfree: __le32,
    pub numinos: __le32,
    pub numfree: __le32,
}

#[repr(C)]
pub struct iagctl {
    pub inofree: i32,
    pub extfree: i32,
    pub numinos: i32,
    pub numfree: i32,
}

#[repr(C)]
pub struct dinomap_disk {
    pub in_freeiag: __le32,
    pub in_nextiag: __le32,
    pub in_numinos: __le32,
    pub in_numfree: __le32,
    pub in_nbperiext: __le32,
    pub in_l2nbperiext: __le32,
    pub in_diskblock: __le32,
    pub in_maxag: __le32,
    pub pad: [u8; 2016],
    pub in_agctl: [iagctl_disk; MAXAG],
}

#[repr(C)]
pub struct dinomap {
    pub in_freeiag: i32,
    pub in_nextiag: i32,
    pub in_numinos: i32,
    pub in_numfree: i32,
    pub in_nbperiext: i32,
    pub in_l2nbperiext: i32,
    pub in_diskblock: i32,
    pub in_maxag: i32,
    pub in_agctl: [iagctl; MAXAG],
}

#[repr(C)]
pub struct inomap {
    pub im_imap: dinomap,
    pub im_ipimap: *mut inode,
    pub im_freelock: mutex,
    pub im_aglock: [mutex; MAXAG],
    pub im_DBGdimap: *mut u32,
    pub im_numinos: atomic_t,
    pub im_numfree: atomic_t,
}

pub const unsafe fn im_freeiag(p: *const inomap) -> *mut i32 { &(*(p)).im_imap.in_freeiag as *const i32 as *mut i32 }
pub const unsafe fn im_nextiag(p: *const inomap) -> *mut i32 { &(*(p)).im_imap.in_nextiag as *const i32 as *mut i32 }
pub const unsafe fn im_agctl(p: *const inomap) -> *mut [iagctl; MAXAG] { &(*(p)).im_imap.in_agctl as *const _ as *mut _ }
pub const unsafe fn im_nbperiext(p: *const inomap) -> *mut i32 { &(*(p)).im_imap.in_nbperiext as *const i32 as *mut i32 }
pub const unsafe fn im_l2nbperiext(p: *const inomap) -> *mut i32 { &(*(p)).im_imap.in_l2nbperiext as *const i32 as *mut i32 }
pub const unsafe fn im_diskblock(p: *const inomap) -> *mut i32 { &(*(p)).im_imap.in_diskblock as *const i32 as *mut i32 }
pub const unsafe fn im_maxag(p: *const inomap) -> *mut i32 { &(*(p)).im_imap.in_maxag as *const i32 as *mut i32 }

unsafe extern "C" {
    pub fn diFree(ip: *mut inode) -> i32;
    pub fn diAlloc(ip: *mut inode, is_anon: bool, ipimap: *mut inode) -> i32;
    pub fn diSync(ip: *mut inode) -> i32;
    pub fn diUpdatePMap(ipimap: *mut inode, inum: c_ulong, is_free: bool, tblk: *mut tblock) -> i32;
    pub fn diExtendFS(ipimap: *mut inode, ipbmap: *mut inode) -> i32;
    pub fn diMount(ip: *mut inode) -> i32;
    pub fn diUnmount(ip: *mut inode, flag: i32) -> i32;
    pub fn diRead(ip: *mut inode) -> i32;
    pub fn diReadSpecial(sb: *mut super_block, ino: ino_t, flag: i32) -> *mut inode;
    pub fn diWriteSpecial(ip: *mut inode, flag: i32);
    pub fn diFreeSpecial(ip: *mut inode);
    pub fn diWrite(tid: tid_t, ip: *mut inode) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
