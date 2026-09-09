/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Red Hat, Inc.  All rights reserved.
 */

// The C header includes "incore.h"; the types below are declarations for
// those externally supplied C structures.
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct address_space {
    _private: [u8; 0],
}

#[repr(C)]
pub struct writeback_control {
    _private: [u8; 0],
}

extern "C" {
    pub fn adjust_fs_space(inode: *mut inode);
    pub fn gfs2_jdata_writeback(
        mapping: *mut address_space,
        wbc: *mut writeback_control,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
