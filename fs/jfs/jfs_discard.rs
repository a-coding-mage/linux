// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) Tino Reichardt, 2012
 */

// Dependencies supplied by the surrounding kernel/JFS translation.

use core::ffi::c_void;

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
}

#[repr(C)]
pub struct super_block {
    pub s_blocksize: u32,
    pub s_blocksize_bits: u8,
}

#[repr(C)]
pub struct fstrim_range {
    pub start: u64,
    pub len: u64,
    pub minlen: u64,
}

#[repr(C)]
pub struct bmap {
    pub db_agsize: u64,
    pub db_mapsize: u64,
}

#[repr(C)]
pub struct jfs_sb_info {
    pub ipbmap: *mut inode,
    pub bmap: *mut bmap,
}

extern "C" {
    pub fn sb_issue_discard(
        sb: *mut super_block,
        blkno: u64,
        nblocks: u64,
        gfp_mask: u32,
        flags: u32,
    ) -> i32;
    pub fn jfs_err(fmt: *const u8, ...);
    pub fn jfs_info(fmt: *const u8, ...);
    pub fn JFS_SBI(sb: *mut super_block) -> *mut jfs_sb_info;
    pub fn dbDiscardAG(ip: *mut inode, agno: i32, minlen: u64) -> u64;
    pub fn down_read(sem: *mut c_void);
    pub fn up_read(sem: *mut c_void);
}

const GFP_NOFS: u32 = 0;

// BLKTOAG is supplied by jfs_dmap.h.
extern "C" {
    pub fn BLKTOAG(block: u64, sbi: *mut jfs_sb_info) -> i32;
}

/*
 * NAME: jfs_issue_discard()
 *
 * FUNCTION: TRIM the specified block range on device, if supported
 */
pub unsafe fn jfs_issue_discard(ip: *mut inode, blkno: u64, nblocks: u64) {
    let sb = (*ip).i_sb;
    let mut r: i32 = 0;

    r = sb_issue_discard(sb, blkno, nblocks, GFP_NOFS, 0);
    if r != 0 {
        jfs_err(
            b"JFS: sb_issue_discard(%p, %llu, %llu, GFP_NOFS, 0) = %d => failed!\0".as_ptr(),
            sb,
            blkno,
            nblocks,
            r,
        );
    }

    jfs_info(
        b"JFS: sb_issue_discard(%p, %llu, %llu, GFP_NOFS, 0) = %d\0".as_ptr(),
        sb,
        blkno,
        nblocks,
        r,
    );
}

/*
 * NAME: jfs_ioc_trim()
 *
 * FUNCTION: attempt to discard (TRIM) all free blocks from the filesystem.
 */
pub unsafe fn jfs_ioc_trim(ip: *mut inode, range: *mut fstrim_range) -> i32 {
    let ipbmap = (*JFS_SBI((*ip).i_sb)).ipbmap;
    let mut bmp: *mut bmap;
    let sb = (*ipbmap).i_sb;
    let mut agno: i32;
    let mut agno_end: i32;
    let start: u64;
    let mut end: u64;
    let minlen: u64;
    let mut trimmed: u64 = 0;

    start = (*range).start >> (*sb).s_blocksize_bits;
    end = start + ((*range).len >> (*sb).s_blocksize_bits) - 1;
    minlen = ((*range).minlen >> (*sb).s_blocksize_bits).max(1);

    down_read(core::ptr::null_mut());
    bmp = (*JFS_SBI((*ip).i_sb)).bmap;

    if bmp.is_null()
        || minlen > (*bmp).db_agsize
        || start >= (*bmp).db_mapsize
        || (*range).len < (*sb).s_blocksize as u64
    {
        up_read(core::ptr::null_mut());
        return -22;
    }

    if end >= (*bmp).db_mapsize {
        end = (*bmp).db_mapsize - 1;
    }

    agno = BLKTOAG(start, JFS_SBI((*ip).i_sb));
    agno_end = BLKTOAG(end, JFS_SBI((*ip).i_sb));
    while agno <= agno_end {
        trimmed = trimmed.wrapping_add(dbDiscardAG(ip, agno, minlen));
        agno += 1;
    }

    up_read(core::ptr::null_mut());
    (*range).len = trimmed << (*sb).s_blocksize_bits;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
