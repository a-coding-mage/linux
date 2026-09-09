/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS disk address translation.
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nilfs_palloc_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nilfs_inode {
    _private: [u8; 0],
}

pub type __u64 = u64;
pub type sector_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;

extern "C" {
    pub fn nilfs_dat_translate(inode: *mut inode, vbn: __u64, blocknr: *mut sector_t) -> i32;

    pub fn nilfs_dat_prepare_alloc(
        inode: *mut inode,
        req: *mut nilfs_palloc_req,
    ) -> i32;
    pub fn nilfs_dat_commit_alloc(inode: *mut inode, req: *mut nilfs_palloc_req);
    pub fn nilfs_dat_abort_alloc(inode: *mut inode, req: *mut nilfs_palloc_req);
    pub fn nilfs_dat_prepare_start(
        inode: *mut inode,
        req: *mut nilfs_palloc_req,
    ) -> i32;
    pub fn nilfs_dat_commit_start(
        inode: *mut inode,
        req: *mut nilfs_palloc_req,
        blocknr: sector_t,
    );
    pub fn nilfs_dat_prepare_end(inode: *mut inode, req: *mut nilfs_palloc_req) -> i32;
    pub fn nilfs_dat_commit_end(inode: *mut inode, req: *mut nilfs_palloc_req, dead: i32);
    pub fn nilfs_dat_abort_end(inode: *mut inode, req: *mut nilfs_palloc_req);
    pub fn nilfs_dat_prepare_update(
        inode: *mut inode,
        oldreq: *mut nilfs_palloc_req,
        newreq: *mut nilfs_palloc_req,
    ) -> i32;
    pub fn nilfs_dat_commit_update(
        inode: *mut inode,
        oldreq: *mut nilfs_palloc_req,
        newreq: *mut nilfs_palloc_req,
        dead: i32,
    );
    pub fn nilfs_dat_abort_update(
        inode: *mut inode,
        oldreq: *mut nilfs_palloc_req,
        newreq: *mut nilfs_palloc_req,
    );

    pub fn nilfs_dat_mark_dirty(inode: *mut inode, vbn: __u64) -> i32;
    pub fn nilfs_dat_freev(inode: *mut inode, vbn: *mut __u64, nitems: size_t) -> i32;
    pub fn nilfs_dat_move(inode: *mut inode, vbn: __u64, blocknr: sector_t) -> i32;
    pub fn nilfs_dat_get_vinfo(
        inode: *mut inode,
        buf: *mut core::ffi::c_void,
        vinfo: u32,
        size: size_t,
    ) -> ssize_t;

    pub fn nilfs_dat_read(
        sb: *mut super_block,
        entry_size: size_t,
        raw_inode: *mut nilfs_inode,
        inodep: *mut *mut inode,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
