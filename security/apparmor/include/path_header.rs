// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor basic path manipulation function definitions.
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Dependencies from linux kernel headers: linux/path.h, linux/types.h

use std::ffi::c_char;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PathFlags {
    PATH_IS_DIR = 0x1,                    // path is a directory
    PATH_SOCK_COND = 0x2,
    PATH_CONNECT_PATH = 0x4,              // connect disconnected paths to /
    PATH_CHROOT_REL = 0x8,                // do path lookup relative to chroot
    PATH_CHROOT_NSCONNECT = 0x10,         // connect paths that are at ns root

    PATH_DELEGATE_DELETED = 0x10000,      // delegate deleted files
    PATH_MEDIATE_DELETED = 0x20000,       // mediate deleted paths
}

// Opaque type from linux/path.h
#[repr(C)]
pub struct path {
    _opaque: [u8; 0],
}

pub const IN_ATOMIC: bool = true;

extern "C" {
    pub fn aa_path_name(
        path: *const path,
        flags: i32,
        buffer: *mut c_char,
        name: *mut *const c_char,
        info: *mut *const c_char,
        disconnected: *const c_char,
    ) -> i32;

    pub fn aa_get_buffer(in_atomic: bool) -> *mut c_char;

    pub fn aa_put_buffer(buf: *mut c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
