/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * symlink.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

/* C header guard: OCFS2_SYMLINK_H */

/* These types are supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct inode {
    pub i_mode: u16,
    pub i_blocks: u64,
}

#[repr(C)]
pub struct inode_operations {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct address_space_operations {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    pub static ocfs2_symlink_inode_operations: inode_operations;
    pub static ocfs2_fast_symlink_aops: address_space_operations;
}

/* Test whether an inode is a fast symlink. */
#[inline]
pub unsafe fn ocfs2_inode_is_fast_symlink(inode: *mut inode) -> i32 {
    /*
     * S_ISLNK is a C macro and i_blocks is provided by the external inode
     * definition; these accesses preserve the original expression's intent.
     */
    let is_symlink = ((*inode).i_mode & 0o170000) == 0o120000;
    if is_symlink && (*inode).i_blocks == 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
