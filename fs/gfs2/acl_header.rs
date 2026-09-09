/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// C header guard: __ACL_DOT_H__

// Dependency supplied by the surrounding translation unit: "incore.h".

/// Equivalent of `GFS2_ACL_MAX_ENTRIES(sdp)`.
#[macro_export]
macro_rules! GFS2_ACL_MAX_ENTRIES {
    ($sdp:expr) => {
        (300usize << ($sdp).sd_sb.sb_bsize_shift >> 12)
    };
}

extern "C" {
    pub fn gfs2_get_acl(
        inode: *mut inode,
        type_: ::core::ffi::c_int,
        rcu: bool,
    ) -> *mut posix_acl;

    pub fn __gfs2_set_acl(
        inode: *mut inode,
        acl: *mut posix_acl,
        type_: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn gfs2_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
