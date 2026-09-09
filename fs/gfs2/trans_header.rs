/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

/* Translated from trans.h.  Linux dependencies are supplied by other files. */

use core::ffi::{c_uint, c_ulong};

pub const RES_DINODE: c_uint = 1;
pub const RES_INDIRECT: c_uint = 1;
pub const RES_JDATA: c_uint = 1;
pub const RES_DATA: c_uint = 1;
pub const RES_LEAF: c_uint = 1;
pub const RES_RG_HDR: c_uint = 1;
pub const RES_RG_BIT: c_uint = 2;
pub const RES_EATTR: c_uint = 1;
pub const RES_STATFS: c_uint = 1;
pub const RES_QUOTA: c_uint = 2;

/* Forward declarations corresponding to the C header's external types. */
#[repr(C)]
pub struct gfs2_sbd {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gfs2_rgrpd {
    pub rd_length: c_uint,
}
#[repr(C)]
pub struct gfs2_glock {
    _private: [u8; 0],
}
#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gfs2_bufdata {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gfs2_trans {
    _private: [u8; 0],
}

/* The inline function accesses these members supplied by the gfs2 inode types. */
#[repr(C)]
pub struct gfs2_inode_reservation {
    pub rs_rgd: *mut gfs2_rgrpd,
}
#[repr(C)]
pub struct gfs2_inode {
    pub i_res: gfs2_inode_reservation,
}

#[inline]
pub unsafe fn gfs2_rg_blocks(ip: *const gfs2_inode, requested: c_uint) -> c_uint {
    let rgd = (*ip).i_res.rs_rgd;
    let rd_length = (*rgd).rd_length;
    if requested < rd_length {
        requested.wrapping_add(1)
    } else {
        rd_length
    }
}

extern "C" {
    pub fn __gfs2_trans_begin(
        tr: *mut gfs2_trans,
        sdp: *mut gfs2_sbd,
        blocks: c_uint,
        revokes: c_uint,
        ip: c_ulong,
    ) -> i32;
    pub fn gfs2_trans_begin(sdp: *mut gfs2_sbd, blocks: c_uint, revokes: c_uint) -> i32;
    pub fn gfs2_trans_end(sdp: *mut gfs2_sbd);
    pub fn gfs2_trans_add_data(gl: *mut gfs2_glock, bh: *mut buffer_head);
    pub fn gfs2_trans_add_databufs(
        gl: *mut gfs2_glock,
        folio: *mut folio,
        from: usize,
        len: usize,
    );
    pub fn gfs2_trans_add_meta(gl: *mut gfs2_glock, bh: *mut buffer_head);
    pub fn gfs2_trans_add_revoke(sdp: *mut gfs2_sbd, bd: *mut gfs2_bufdata);
    pub fn gfs2_trans_remove_revoke(sdp: *mut gfs2_sbd, blkno: u64, len: c_uint);
    pub fn gfs2_trans_free(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
