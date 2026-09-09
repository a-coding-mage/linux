/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2008 Red Hat, Inc.  All rights reserved.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const RGRP_RSRV_MINBLKS: u32 = 32;
pub const RGRP_RSRV_ADDBLKS: u32 = 64;

pub const GFS2_AF_ORLOV: i32 = 1;

pub enum gfs2_rgrpd {}
pub enum gfs2_sbd {}
pub enum gfs2_holder {}
pub enum gfs2_glock {}
pub enum gfs2_inode {}
pub enum gfs2_alloc {}
pub enum gfs2_alloc_parms {}
pub enum gfs2_blkreserv {}
pub enum inode {}
pub enum seq_file {}
pub enum buffer_head {}
pub enum gfs2_bitmap {}
pub enum file {}

#[repr(C)]
pub struct gfs2_rgrp_list {
    pub rl_rgrps: ::core::ffi::c_uint,
    pub rl_space: ::core::ffi::c_uint,
    pub rl_rgd: *mut *mut gfs2_rgrpd,
    pub rl_ghs: *mut gfs2_holder,
}

extern "C" {
    pub fn gfs2_rgrp_verify(rgd: *mut gfs2_rgrpd);
    pub fn gfs2_blk2rgrpd(sdp: *mut gfs2_sbd, blk: u64, exact: bool) -> *mut gfs2_rgrpd;
    pub fn gfs2_rgrpd_get_first(sdp: *mut gfs2_sbd) -> *mut gfs2_rgrpd;
    pub fn gfs2_rgrpd_get_next(rgd: *mut gfs2_rgrpd) -> *mut gfs2_rgrpd;
    pub fn gfs2_clear_rgrpd(sdp: *mut gfs2_sbd);
    pub fn gfs2_rindex_update(sdp: *mut gfs2_sbd) -> ::core::ffi::c_int;
    pub fn gfs2_free_clones(rgd: *mut gfs2_rgrpd);
    pub fn gfs2_rgrp_go_instantiate(gl: *mut gfs2_glock) -> ::core::ffi::c_int;
    pub fn gfs2_rgrp_brelse(rgd: *mut gfs2_rgrpd);
    pub fn gfs2_alloc_get(ip: *mut gfs2_inode) -> *mut gfs2_alloc;
    pub fn gfs2_inplace_reserve(ip: *mut gfs2_inode, ap: *mut gfs2_alloc_parms) -> ::core::ffi::c_int;
    pub fn gfs2_inplace_release(ip: *mut gfs2_inode);
    pub fn gfs2_alloc_blocks(ip: *mut gfs2_inode, bn: *mut u64, n: *mut ::core::ffi::c_uint, dinode: bool) -> ::core::ffi::c_int;
    pub fn gfs2_rs_deltree(rs: *mut gfs2_blkreserv);
    pub fn gfs2_rs_delete(ip: *mut gfs2_inode);
    pub fn __gfs2_free_blocks(ip: *mut gfs2_inode, rgd: *mut gfs2_rgrpd, bstart: u64, blen: u32, meta: ::core::ffi::c_int);
    pub fn gfs2_free_meta(ip: *mut gfs2_inode, rgd: *mut gfs2_rgrpd, bstart: u64, blen: u32);
    pub fn gfs2_free_di(rgd: *mut gfs2_rgrpd, ip: *mut gfs2_inode);
    pub fn gfs2_unlink_di(inode: *mut inode);
    pub fn gfs2_check_blk_type(sdp: *mut gfs2_sbd, no_addr: u64, type_: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn gfs2_rlist_add(ip: *mut gfs2_inode, rlist: *mut gfs2_rgrp_list, block: u64);
    pub fn gfs2_rlist_alloc(rlist: *mut gfs2_rgrp_list, state: ::core::ffi::c_uint, flags: u16);
    pub fn gfs2_rlist_free(rlist: *mut gfs2_rgrp_list);
    pub fn gfs2_ri_total(sdp: *mut gfs2_sbd) -> u64;
    pub fn gfs2_rgrp_dump(seq: *mut seq_file, rgd: *mut gfs2_rgrpd, fs_id_buf: *const ::core::ffi::c_char);
    pub fn gfs2_rgrp_send_discards(sdp: *mut gfs2_sbd, offset: u64, bh: *mut buffer_head, bi: *const gfs2_bitmap, minlen: ::core::ffi::c_uint, ptrimmed: *mut u64) -> ::core::ffi::c_int;
    pub fn gfs2_fitrim(filp: *mut file, argp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn RB_EMPTY_NODE(node: *const ::core::ffi::c_void) -> bool;
}

/* Inline functions retain their original field and RB-tree dependencies. */
#[inline]
pub unsafe fn gfs2_rs_active(rs: *const gfs2_blkreserv) -> bool {
    !RB_EMPTY_NODE(rs as *const ::core::ffi::c_void)
}

#[inline]
pub unsafe fn rgrp_contains_block(rgd: *const gfs2_rgrpd, block: u64) -> bool {
    /* rd_data0 and rd_data are fields of the externally supplied gfs2_rgrpd. */
    let first = (*(rgd as *const gfs2_rgrpd)).rd_data0;
    let last = first + (*(rgd as *const gfs2_rgrpd)).rd_data;
    first <= block && block < last
}

extern "C" {
    pub fn check_and_update_goal(ip: *mut gfs2_inode);
    pub fn rgrp_lock_local(rgd: *mut gfs2_rgrpd);
    pub fn rgrp_unlock_local(rgd: *mut gfs2_rgrpd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
