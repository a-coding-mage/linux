/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  coda_fs_i.h
 *
 *  Copyright (C) 1998 Carnegie Mellon University
 *
 */

// C dependencies: <linux/types.h>, <linux/list.h>, <linux/spinlock.h>,
// and <linux/coda.h>.

/*
 * coda fs inode data
 * c_lock protects accesses to c_flags, c_mapcount, c_cached_epoch, c_uid and
 * c_cached_perm.
 * vfs_inode is set only when the inode is created and never changes.
 * c_fid is set when the inode is created and should be considered immutable.
 */
#[repr(C)]
pub struct CodaInodeInfo {
    pub c_fid: CodaFid,             /* Coda identifier */
    pub c_flags: u16,               /* flags (see below) */
    pub c_mapcount: u32,            /* nr of times this inode is mapped */
    pub c_cached_epoch: u32,        /* epoch for cached permissions */
    pub c_uid: KuidT,               /* fsuid for cached permissions */
    pub c_cached_perm: u32,         /* cached access permissions */
    pub c_lock: SpinlockT,
    pub vfs_inode: Inode,
}

/*
 * coda fs file private data
 */
pub const CODA_MAGIC: i32 = 0xC0DA_C0DAu32 as i32;

#[repr(C)]
pub struct CodaFileInfo {
    pub cfi_magic: i32,             /* magic number */
    pub cfi_container: *mut File,   /* container file for this cnode */
    pub cfi_mapcount: u32,          /* nr of times this file is mapped */
    pub cfi_access_intent: bool,    /* is access intent supported */
}

/* flags */
pub const C_VATTR: u32 = 0x1;       /* Validity of vattr in inode */
pub const C_FLUSH: u32 = 0x2;       /* used after a flush */
pub const C_DYING: u32 = 0x4;       /* from venus (which died) */
pub const C_PURGE: u32 = 0x8;

extern "C" {
    pub fn coda_cnode_make(fid: *mut CodaFid, sb: *mut SuperBlock) -> *mut Inode;
    pub fn coda_iget(
        sb: *mut SuperBlock,
        fid: *mut CodaFid,
        attr: *mut CodaVattr,
    ) -> *mut Inode;
    pub fn coda_cnode_makectl(sb: *mut SuperBlock) -> *mut Inode;
    pub fn coda_fid_to_inode(fid: *mut CodaFid, sb: *mut SuperBlock) -> *mut Inode;
    pub fn coda_ftoc(file: *mut File) -> *mut CodaFileInfo;
    pub fn coda_replace_fid(inode: *mut Inode, oldfid: *mut CodaFid, fid: *mut CodaFid);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
