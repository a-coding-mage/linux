// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level translation unit for gfs2/rgrp.c.
// The implementation depends on the surrounding kernel/GFS2 Rust bindings;
// those externally supplied declarations are intentionally not recreated here.
#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// C-compatible opaque types supplied by the surrounding translation unit.
#[repr(C)]
pub struct gfs2_rgrpd { _private: [u8; 0] }
#[repr(C)]
pub struct gfs2_sbd { _private: [u8; 0] }
#[repr(C)]
pub struct gfs2_inode { _private: [u8; 0] }
#[repr(C)]
pub struct gfs2_blkreserv { _private: [u8; 0] }
#[repr(C)]
pub struct gfs2_bitmap { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }

pub const BFITNOENT: u32 = u32::MAX;
pub const NO_BLOCK: u64 = u64::MAX;

#[repr(C)]
pub struct gfs2_rbm {
    pub rgd: *mut gfs2_rgrpd,
    pub offset: u32,
    pub bii: i32,
}

#[repr(C)]
pub struct gfs2_extent {
    pub rbm: gfs2_rbm,
    pub len: u32,
}

// The remaining declarations and definitions are supplied by the generated
// bindings for the Linux/GFS2 headers.  Keep this translation unit ABI-shaped
// and preserve the source-level entry points below.
extern "C" {
    pub fn gfs2_rgrp_verify(rgd: *mut gfs2_rgrpd);
    pub fn gfs2_blk2rgrpd(sdp: *mut gfs2_sbd, blk: u64, exact: bool) -> *mut gfs2_rgrpd;
    pub fn gfs2_rgrpd_get_first(sdp: *mut gfs2_sbd) -> *mut gfs2_rgrpd;
    pub fn gfs2_rgrpd_get_next(rgd: *mut gfs2_rgrpd) -> *mut gfs2_rgrpd;
    pub fn gfs2_free_clones(rgd: *mut gfs2_rgrpd);
    pub fn gfs2_rs_deltree(rs: *mut gfs2_blkreserv);
    pub fn gfs2_rs_delete(ip: *mut gfs2_inode);
    pub fn gfs2_clear_rgrpd(sdp: *mut gfs2_sbd);
    pub fn gfs2_inplace_reserve(ip: *mut gfs2_inode, ap: *mut c_void) -> i32;
    pub fn gfs2_inplace_release(ip: *mut gfs2_inode);
    pub fn gfs2_rgrp_dump(seq: *mut seq_file, rgd: *mut gfs2_rgrpd, fs_id_buf: *const i8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
