/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Squashfs
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * squashfs_fs_i.h
 */

#[repr(C)]
pub struct squashfs_inode_info_fragment {
    pub fragment_block: u64,
    pub fragment_size: ::std::ffi::c_int,
    pub fragment_offset: ::std::ffi::c_int,
    pub block_list_start: u64,
}

#[repr(C)]
pub struct squashfs_inode_info_dir {
    pub dir_idx_start: u64,
    pub dir_idx_offset: ::std::ffi::c_int,
    pub dir_idx_cnt: ::std::ffi::c_int,
}

#[repr(C)]
pub union squashfs_inode_info_union {
    pub fragment: squashfs_inode_info_fragment,
    pub dir: squashfs_inode_info_dir,
}

#[repr(C)]
pub struct squashfs_inode_info {
    pub start: u64,
    pub offset: ::std::ffi::c_int,
    pub xattr: u64,
    pub xattr_size: ::std::ffi::c_uint,
    pub xattr_count: ::std::ffi::c_int,
    pub parent: ::std::ffi::c_int,
    pub _anon: squashfs_inode_info_union,
    pub vfs_inode: inode,
}

pub unsafe fn squashfs_i(inode: *mut inode) -> *mut squashfs_inode_info {
    container_of!(inode, squashfs_inode_info, vfs_inode)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
