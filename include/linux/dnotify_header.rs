/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Directory notification for Linux
 *
 * Copyright (C) 2000,2002 Stephen Rothwell
 *
 * Dependency intent: declarations from <linux/fs.h> are supplied by other
 * translated files. The original header guard and include are omitted here.
 */

#[repr(C)]
pub struct dnotify_struct {
    pub dn_next: *mut dnotify_struct,
    pub dn_mask: u32,
    pub dn_fd: i32,
    pub dn_filp: *mut file,
    pub dn_owner: fl_owner_t,
}

/* The following items are kernel-only in the original header. */

#[cfg(feature = "CONFIG_DNOTIFY")]
pub const DNOTIFY_ALL_EVENTS: u32 = FS_DELETE
    | FS_DELETE_CHILD
    | FS_MODIFY
    | FS_MODIFY_CHILD
    | FS_ACCESS
    | FS_ACCESS_CHILD
    | FS_ATTRIB
    | FS_ATTRIB_CHILD
    | FS_CREATE
    | FS_RENAME
    | FS_MOVED_FROM
    | FS_MOVED_TO;

#[cfg(feature = "CONFIG_DNOTIFY")]
extern "C" {
    pub fn dnotify_flush(filp: *mut file, id: fl_owner_t);
    pub fn fcntl_dirnotify(fd: i32, filp: *mut file, arg: u32) -> i32;
}

#[cfg(not(feature = "CONFIG_DNOTIFY"))]
#[inline]
pub unsafe fn dnotify_flush(_filp: *mut file, _id: fl_owner_t) {}

#[cfg(not(feature = "CONFIG_DNOTIFY"))]
#[inline]
pub unsafe fn fcntl_dirnotify(_fd: i32, _filp: *mut file, _arg: u32) -> i32 {
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
