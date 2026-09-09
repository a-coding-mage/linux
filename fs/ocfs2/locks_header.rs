/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * locks.h
 *
 * Function prototypes for Userspace file locking support
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// OCFS2_LOCKS_H

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_lock {
    _private: [u8; 0],
}

extern "C" {
    pub fn ocfs2_flock(file: *mut file, cmd: ::std::os::raw::c_int, fl: *mut file_lock) -> ::std::os::raw::c_int;
    pub fn ocfs2_lock(file: *mut file, cmd: ::std::os::raw::c_int, fl: *mut file_lock) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
