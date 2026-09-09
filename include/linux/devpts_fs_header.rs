/* SPDX-License-Identifier: GPL-2.0-or-later */
/* -*- linux-c -*- --------------------------------------------------------- *
 *
 * linux/include/linux/devpts_fs.h
 *
 *  Copyright 1998-2004 H. Peter Anvin -- All Rights Reserved
 *
 * ------------------------------------------------------------------------- */

// Translated from the C header. The CONFIG_UNIX98_PTYS build-time condition
// is preserved as a Rust cfg condition.

#[cfg(CONFIG_UNIX98_PTYS)]
pub struct pts_fs_info;

#[cfg(CONFIG_UNIX98_PTYS)]
extern "C" {
    pub fn devpts_mntget(
        file: *mut file,
        info: *mut pts_fs_info,
    ) -> *mut vfsmount;
    pub fn devpts_acquire(file: *mut file) -> *mut pts_fs_info;
    pub fn devpts_release(info: *mut pts_fs_info);

    pub fn devpts_new_index(info: *mut pts_fs_info) -> ::std::os::raw::c_int;
    pub fn devpts_kill_index(
        info: *mut pts_fs_info,
        index: ::std::os::raw::c_int,
    );

    /* mknod in devpts */
    pub fn devpts_pty_new(
        info: *mut pts_fs_info,
        index: ::std::os::raw::c_int,
        priv_data: *mut ::std::ffi::c_void,
    ) -> *mut dentry;
    /* get private structure */
    pub fn devpts_get_priv(dentry: *mut dentry) -> *mut ::std::ffi::c_void;
    /* unlink */
    pub fn devpts_pty_kill(dentry: *mut dentry);

    /* in pty.c */
    pub fn ptm_open_peer(
        master: *mut file,
        tty: *mut tty_struct,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

// These declarations are supplied by other translated headers.
#[allow(non_camel_case_types)]
pub enum file {}
#[allow(non_camel_case_types)]
pub enum vfsmount {}
#[allow(non_camel_case_types)]
pub enum dentry {}
#[allow(non_camel_case_types)]
pub enum tty_struct {}

// When CONFIG_UNIX98_PTYS is disabled, the C inline function returns -EIO.
// The errno constant is supplied by the translated linux/errno.h dependency.
#[cfg(not(CONFIG_UNIX98_PTYS))]
#[inline]
pub unsafe fn ptm_open_peer(
    _master: *mut file,
    _tty: *mut tty_struct,
    _flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    -EIO
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
