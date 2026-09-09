/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// C dependency: #include "incore.h"

// Types supplied by the translated incore dependency: workqueue_struct,
// gfs2_jdesc, buffer_head, work_struct, gfs2_sbd, gfs2_log_header, and
// gfs2_log_header_host.

extern "C" {
    pub static mut gfs2_recovery_wq: *mut workqueue_struct;
}

#[inline]
pub unsafe fn gfs2_replay_incr_blk(jd: *mut gfs2_jdesc, blk: *mut u32) {
    *blk = (*blk).wrapping_add(1);
    if *blk == (*jd).jd_blocks {
        *blk = 0;
    }
}

extern "C" {
    pub fn gfs2_replay_read_block(
        jd: *mut gfs2_jdesc,
        blk: u32,
        bh: *mut *mut buffer_head,
    ) -> ::core::ffi::c_int;

    pub fn gfs2_revoke_add(
        jd: *mut gfs2_jdesc,
        blkno: u64,
        where_: u32,
    ) -> i32;
    pub fn gfs2_revoke_check(
        jd: *mut gfs2_jdesc,
        blkno: u64,
        where_: u32,
    ) -> i32;
    pub fn gfs2_revoke_clean(jd: *mut gfs2_jdesc);

    pub fn gfs2_recover_journal(jd: *mut gfs2_jdesc, wait: bool) -> i32;
    pub fn gfs2_recover_func(work: *mut work_struct);
    pub fn __get_log_header(
        sdp: *mut gfs2_sbd,
        lh: *const gfs2_log_header,
        blkno: u32,
        head: *mut gfs2_log_header_host,
    ) -> i32;
    pub fn gfs2_log_pointers_init(sdp: *mut gfs2_sbd, head: *mut gfs2_log_header_host);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
