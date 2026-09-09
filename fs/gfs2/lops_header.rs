/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2008 Red Hat, Inc.  All rights reserved.
 */

// C dependencies: <linux/list.h> and "incore.h" supply the referenced types.

use core::ffi::c_int;

#[allow(non_camel_case_types)]
pub type u32 = core::ffi::c_uint;
#[allow(non_camel_case_types)]
pub type u64 = core::ffi::c_ulonglong;
#[allow(non_camel_case_types)]
pub type __be64 = u64;
#[allow(non_camel_case_types)]
pub type blk_opf_t = u32;

#[repr(C)]
pub struct gfs2_sbd {
    pub sd_ldptrs: core::ffi::c_uint,
}
#[repr(C)]
pub struct gfs2_jdesc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}
#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bio {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gfs2_trans {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gfs2_log_header_host {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gfs2_log_descriptor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gfs2_log_operations {
    pub lo_before_commit: Option<unsafe extern "C" fn(*mut gfs2_sbd, *mut gfs2_trans)>,
    pub lo_after_commit: Option<unsafe extern "C" fn(*mut gfs2_sbd, *mut gfs2_trans)>,
    pub lo_before_scan:
        Option<unsafe extern "C" fn(*mut gfs2_jdesc, *mut gfs2_log_header_host, u32)>,
    pub lo_scan_elements: Option<unsafe extern "C" fn(
        *mut gfs2_jdesc,
        u32,
        *mut gfs2_log_descriptor,
        *mut __be64,
        u32,
    ) -> c_int>,
    pub lo_after_scan: Option<unsafe extern "C" fn(*mut gfs2_jdesc, c_int, u32)>,
}

extern "C" {
    pub static gfs2_log_ops: *const *const gfs2_log_operations;

    pub fn gfs2_log_incr_head(sdp: *mut gfs2_sbd);
    pub fn gfs2_log_bmap(jd: *mut gfs2_jdesc, lbn: u32) -> u64;
    pub fn gfs2_log_write(
        sdp: *mut gfs2_sbd,
        jd: *mut gfs2_jdesc,
        page: *mut page,
        size: u32,
        offset: u32,
        blkno: u64,
        opf: blk_opf_t,
    );
    pub fn gfs2_log_submit_write(biop: *mut *mut bio);
    pub fn gfs2_pin(sdp: *mut gfs2_sbd, bh: *mut buffer_head);
    pub fn gfs2_find_jhead(jd: *mut gfs2_jdesc, head: *mut gfs2_log_header_host) -> c_int;
    pub fn gfs2_drain_revokes(sdp: *mut gfs2_sbd);
}

#[inline]
pub unsafe fn buf_limit(sdp: *mut gfs2_sbd) -> u32 {
    (*sdp).sd_ldptrs
}

#[inline]
pub unsafe fn databuf_limit(sdp: *mut gfs2_sbd) -> u32 {
    (*sdp).sd_ldptrs / 2
}

#[inline]
pub unsafe fn lops_before_commit(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans) {
    let mut x = 0usize;
    while !(*gfs2_log_ops.add(x)).is_null() {
        if let Some(callback) = (*(*gfs2_log_ops.add(x))).lo_before_commit {
            callback(sdp, tr);
        }
        x += 1;
    }
}

#[inline]
pub unsafe fn lops_after_commit(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans) {
    let mut x = 0usize;
    while !(*gfs2_log_ops.add(x)).is_null() {
        if let Some(callback) = (*(*gfs2_log_ops.add(x))).lo_after_commit {
            callback(sdp, tr);
        }
        x += 1;
    }
}

#[inline]
pub unsafe fn lops_before_scan(
    jd: *mut gfs2_jdesc,
    head: *mut gfs2_log_header_host,
    pass: u32,
) {
    let mut x = 0usize;
    while !(*gfs2_log_ops.add(x)).is_null() {
        if let Some(callback) = (*(*gfs2_log_ops.add(x))).lo_before_scan {
            callback(jd, head, pass);
        }
        x += 1;
    }
}

#[inline]
pub unsafe fn lops_scan_elements(
    jd: *mut gfs2_jdesc,
    start: u32,
    ld: *mut gfs2_log_descriptor,
    ptr: *mut __be64,
    pass: u32,
) -> c_int {
    let mut x = 0usize;
    while !(*gfs2_log_ops.add(x)).is_null() {
        if let Some(callback) = (*(*gfs2_log_ops.add(x))).lo_scan_elements {
            let error = callback(jd, start, ld, ptr, pass);
            if error != 0 {
                return error;
            }
        }
        x += 1;
    }
    0
}

#[inline]
pub unsafe fn lops_after_scan(jd: *mut gfs2_jdesc, error: c_int, pass: u32) {
    let mut x = 0usize;
    while !(*gfs2_log_ops.add(x)).is_null() {
        if let Some(callback) = (*(*gfs2_log_ops.add(x))).lo_before_scan {
            let _ = callback;
            if let Some(after_scan) = (*(*gfs2_log_ops.add(x))).lo_after_scan {
                after_scan(jd, error, pass);
            }
        }
        x += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
