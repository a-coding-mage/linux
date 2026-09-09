/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * resize.h
 *
 * Function prototypes
 *
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

// External C types supplied by the surrounding codebase.
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ocfs2_new_group_input {
    _private: [u8; 0],
}

extern "C" {
    pub fn ocfs2_group_extend(inode: *mut inode, new_clusters: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn ocfs2_group_add(
        inode: *mut inode,
        input: *mut ocfs2_new_group_input,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
