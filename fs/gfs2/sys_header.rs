/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Dependency intent from <linux/spinlock.h> is preserved here; this header
// contains no declarations that require a local Rust translation of it.

#[repr(C)]
pub struct gfs2_sbd {
    _private: [u8; 0],
}

extern "C" {
    pub fn gfs2_sys_fs_add(sdp: *mut gfs2_sbd) -> ::core::ffi::c_int;
    pub fn gfs2_sys_fs_del(sdp: *mut gfs2_sbd);

    pub fn gfs2_sys_init() -> ::core::ffi::c_int;
    pub fn gfs2_sys_uninit();

    pub fn gfs2_recover_set(sdp: *mut gfs2_sbd, jid: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
