/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

/* Translated from bmap.h. Linux iomap and inode dependencies are supplied externally. */

use core::ffi::c_int;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gfs2_inode {
    pub i_inode: inode,
    _private: [u8; 0],
}
#[repr(C)]
pub struct gfs2_sbd {
    pub sd_sb: gfs2_sb_holder,
    pub sd_max_height: u32,
    pub sd_diptrs: u32,
    pub sd_inptrs: u32,
}
#[repr(C)]
pub struct gfs2_sb_holder {
    pub sb_bsize_shift: u32,
}
#[repr(C)]
pub struct gfs2_jdesc { _private: [u8; 0] }
#[repr(C)]
pub struct file { _private: [u8; 0] }
#[repr(C)]
pub struct buffer_head { _private: [u8; 0] }
#[repr(C)]
pub struct iomap { _private: [u8; 0] }
#[repr(C)]
pub struct iomap_ops { _private: [u8; 0] }
#[repr(C)]
pub struct iomap_write_ops { _private: [u8; 0] }
#[repr(C)]
pub struct iomap_writeback_ops { _private: [u8; 0] }

pub type sector_t = u64;
pub type loff_t = i64;
pub type u64_ = u64;

extern "C" {
    pub static gfs2_iomap_ops: iomap_ops;
    pub static gfs2_iomap_write_ops: iomap_write_ops;
    pub static gfs2_writeback_ops: iomap_writeback_ops;

    pub fn GFS2_SB(inode: *const inode) -> *const gfs2_sbd;
    pub fn gfs2_is_dir(ip: *const gfs2_inode) -> bool;
    pub fn BUG_ON(condition: bool);

    pub fn gfs2_unstuff_dinode(ip: *mut gfs2_inode) -> c_int;
    pub fn gfs2_block_map(inode: *mut inode, lblock: sector_t,
                          bh: *mut buffer_head, create: c_int) -> c_int;
    pub fn gfs2_iomap_get(inode: *mut inode, pos: loff_t, length: loff_t,
                          iomap: *mut iomap) -> c_int;
    pub fn gfs2_iomap_alloc(inode: *mut inode, pos: loff_t, length: loff_t,
                            iomap: *mut iomap) -> c_int;
    pub fn gfs2_get_extent(inode: *mut inode, lblock: u64, dblock: *mut u64,
                           extlen: *mut u32) -> c_int;
    pub fn gfs2_alloc_extent(inode: *mut inode, lblock: u64, dblock: *mut u64,
                             extlen: *mut u32, new: *mut bool) -> c_int;
    pub fn gfs2_clear_beyond_eof(inode: *mut inode, end: loff_t) -> c_int;
    pub fn gfs2_setattr_size(inode: *mut inode, size: u64) -> c_int;
    pub fn gfs2_truncatei_resume(ip: *mut gfs2_inode) -> c_int;
    pub fn gfs2_file_dealloc(ip: *mut gfs2_inode) -> c_int;
    pub fn gfs2_write_alloc_required(ip: *mut gfs2_inode, offset: u64,
                                     len: u32) -> c_int;
    pub fn gfs2_map_journal_extents(sdp: *mut gfs2_sbd, jd: *mut gfs2_jdesc) -> c_int;
    pub fn gfs2_free_journal_extents(jd: *mut gfs2_jdesc);
    pub fn __gfs2_punch_hole(file: *mut file, offset: loff_t, length: loff_t) -> c_int;
}

/// Calculate number of blocks needed to write to a file.
#[inline]
pub unsafe fn gfs2_write_calc_reserv(
    ip: *const gfs2_inode,
    len: u32,
    data_blocks: *mut u32,
    ind_blocks: *mut u32,
) {
    let sdp = GFS2_SB(&(*ip).i_inode as *const inode);
    BUG_ON(gfs2_is_dir(ip));
    *data_blocks = (len >> (*sdp).sd_sb.sb_bsize_shift).wrapping_add(3);
    *ind_blocks = 3u32.wrapping_mul((*sdp).sd_max_height.wrapping_sub(1));

    let mut tmp = *data_blocks;
    while tmp > (*sdp).sd_diptrs {
        tmp = (tmp.wrapping_add((*sdp).sd_inptrs).wrapping_sub(1)) / (*sdp).sd_inptrs;
        *ind_blocks = (*ind_blocks).wrapping_add(tmp);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
