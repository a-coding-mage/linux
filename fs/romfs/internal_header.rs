/* SPDX-License-Identifier: GPL-2.0-or-later */
/* RomFS internal definitions
 *
 * Copyright © 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependency: <linux/romfs_fs.h> */

#[repr(C)]
pub struct romfs_inode_info {
    pub vfs_inode: crate::inode,
    pub i_metasize: libc::c_ulong,   /* size of non-data area */
    pub i_dataoffset: libc::c_ulong, /* from the start of fs */
}

pub unsafe fn romfs_maxsize(sb: *mut crate::super_block) -> usize {
    (*sb).s_fs_info as libc::c_ulong as usize
}

pub unsafe fn ROMFS_I(inode: *mut crate::inode) -> *mut romfs_inode_info {
    (inode as *mut u8).sub(core::mem::offset_of!(romfs_inode_info, vfs_inode))
        as *mut romfs_inode_info
}

/*
 * mmap-nommu.c
 */
/* The CONFIG_MMU/CONFIG_ROMFS_ON_MTD conditional is supplied by the build. */
#[cfg(all(not(feature = "CONFIG_MMU"), feature = "CONFIG_ROMFS_ON_MTD"))]
unsafe extern "C" {
    pub static romfs_ro_fops: crate::file_operations;
}

#[cfg(any(feature = "CONFIG_MMU", not(feature = "CONFIG_ROMFS_ON_MTD")))]
pub use crate::generic_ro_fops as romfs_ro_fops;

/*
 * storage.c
 */
unsafe extern "C" {
    pub fn romfs_dev_read(
        sb: *mut crate::super_block,
        pos: libc::c_ulong,
        buf: *mut libc::c_void,
        buflen: usize,
    ) -> libc::c_int;

    pub fn romfs_dev_strnlen(
        sb: *mut crate::super_block,
        pos: libc::c_ulong,
        maxlen: usize,
    ) -> libc::ssize_t;

    pub fn romfs_dev_strcmp(
        sb: *mut crate::super_block,
        pos: libc::c_ulong,
        str_: *const libc::c_char,
        size: usize,
    ) -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
