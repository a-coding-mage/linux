/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * move_extents.h
 *
 * Copyright (C) 2011 Oracle.  All rights reserved.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

extern "C" {
    pub fn ocfs2_ioctl_move_extents(filp: *mut file, argp: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
