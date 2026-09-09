// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2005-2006 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C header guard: __XFS_AOPS_H__

extern "C" {
    pub static xfs_address_space_operations: address_space_operations;
    pub static xfs_dax_aops: address_space_operations;

    pub fn xfs_setfilesize(ip: *mut xfs_inode, offset: xfs_off_t, size: usize) -> ::std::os::raw::c_int;
    pub fn xfs_end_bio(bio: *mut bio);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
