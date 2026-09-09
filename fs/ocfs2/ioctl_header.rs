/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ioctl.h
 *
 * Function prototypes
 *
 * Copyright (C) 2006 Herbert Poetzl
 *
 */

// The C header guard is omitted; this file is a Rust translation unit.
// Types such as `dentry`, `file_kattr`, `mnt_idmap`, and `file` are supplied
// by other translated dependencies.

unsafe extern "C" {
    pub fn ocfs2_fileattr_get(
        dentry: *mut dentry,
        fa: *mut file_kattr,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_fileattr_set(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        fa: *mut file_kattr,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_ioctl(
        filp: *mut file,
        cmd: ::core::ffi::c_uint,
        arg: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;

    pub fn ocfs2_compat_ioctl(
        file: *mut file,
        cmd: ::core::ffi::c_uint,
        arg: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
