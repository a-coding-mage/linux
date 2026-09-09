/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * namei.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

/* OCFS2_NAMEI_H: C header guard. */

pub const OCFS2_DIO_ORPHAN_PREFIX: &str = "dio-";
pub const OCFS2_DIO_ORPHAN_PREFIX_LEN: usize = 4;

/* These types are supplied by the corresponding kernel/dependency headers. */
pub enum inode_operations {}
pub enum dentry {}
pub enum ocfs2_super {}
pub enum handle_t {}
pub enum inode {}
pub enum buffer_head {}

extern "C" {
    pub static ocfs2_dir_iops: inode_operations;

    pub fn ocfs2_get_parent(child: *mut dentry) -> *mut dentry;

    pub fn ocfs2_orphan_del(
        osb: *mut ocfs2_super,
        handle: *mut handle_t,
        orphan_dir_inode: *mut inode,
        inode: *mut inode,
        orphan_dir_bh: *mut buffer_head,
        dio: bool,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_create_inode_in_orphan(
        dir: *mut inode,
        mode: ::core::ffi::c_int,
        new_inode: *mut *mut inode,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_add_inode_to_orphan(
        osb: *mut ocfs2_super,
        inode: *mut inode,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_del_inode_from_orphan(
        osb: *mut ocfs2_super,
        inode: *mut inode,
        di_bh: *mut buffer_head,
        update_isize: ::core::ffi::c_int,
        end: i64,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_mv_orphaned_inode_to_new(
        dir: *mut inode,
        new_inode: *mut inode,
        new_dentry: *mut dentry,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
