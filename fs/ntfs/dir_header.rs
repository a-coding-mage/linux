/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for directory handling in NTFS Linux kernel driver.
 *
 * Copyright (c) 2002-2004 Anton Altaparmakov
 */

// Dependency equivalent of: #include "inode.h"

/*
 * ntfs_name is used to return the file name to the caller of
 * ntfs_lookup_inode_by_name() in order for the caller (namei.c::ntfs_lookup())
 * to be able to deal with dcache aliasing issues.
 */
#[repr(C, packed)]
pub struct ntfs_name {
    pub mref: u64,
    pub r#type: u8,
    pub len: u8,
    pub name: [__le16; 0],
}

/* The little endian Unicode string $I30 as a global constant. */
extern "C" {
    pub static mut I30: [__le16; 5];

    pub fn ntfs_lookup_inode_by_name(
        dir_ni: *mut ntfs_inode,
        uname: *const __le16,
        uname_len: ::std::os::raw::c_int,
        res: *mut *mut ntfs_name,
    ) -> u64;

    pub fn ntfs_check_empty_dir(
        ni: *mut ntfs_inode,
        ni_mrec: *mut mft_record,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
