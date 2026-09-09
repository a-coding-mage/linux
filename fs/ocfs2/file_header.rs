/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * file.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// OCFS2_FILE_H

extern "C" {
    pub static ocfs2_fops: file_operations;
    pub static ocfs2_dops: file_operations;
    pub static ocfs2_fops_no_plocks: file_operations;
    pub static ocfs2_dops_no_plocks: file_operations;
    pub static ocfs2_file_iops: inode_operations;
    pub static ocfs2_special_file_iops: inode_operations;
}

pub enum ocfs2_alloc_context {}
pub enum ocfs2_alloc_restarted {}

#[repr(C)]
pub struct ocfs2_file_private {
    pub cookie: u64,
    pub fp_file: *mut file,
    pub fp_mutex: mutex,
    pub fp_flock: ocfs2_lock_res,
}

extern "C" {
    pub fn ocfs2_add_inode_data(
        osb: *mut ocfs2_super,
        inode: *mut inode,
        logical_offset: *mut u32,
        clusters_to_add: u32,
        mark_unwritten: ::core::ffi::c_int,
        fe_bh: *mut buffer_head,
        handle: *mut handle_t,
        data_ac: *mut ocfs2_alloc_context,
        meta_ac: *mut ocfs2_alloc_context,
        reason_ret: *mut ocfs2_alloc_restarted,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_set_inode_size(
        handle: *mut handle_t,
        inode: *mut inode,
        fe_bh: *mut buffer_head,
        new_i_size: u64,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_simple_size_update(
        inode: *mut inode,
        di_bh: *mut buffer_head,
        new_i_size: u64,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_truncate_file(
        inode: *mut inode,
        di_bh: *mut buffer_head,
        new_i_size: u64,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_extend_no_holes(
        inode: *mut inode,
        di_bh: *mut buffer_head,
        new_i_size: u64,
        zero_to: u64,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_zero_extend(
        inode: *mut inode,
        di_bh: *mut buffer_head,
        zero_to: loff_t,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_setattr(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        attr: *mut iattr,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_getattr(
        idmap: *mut mnt_idmap,
        path: *const path,
        stat: *mut kstat,
        request_mask: u32,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_permission(
        idmap: *mut mnt_idmap,
        inode: *mut inode,
        mask: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_should_update_atime(
        inode: *mut inode,
        vfsmnt: *mut vfsmount,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_update_inode_atime(
        inode: *mut inode,
        bh: *mut buffer_head,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_change_file_space(
        file: *mut file,
        cmd: ::core::ffi::c_uint,
        sr: *mut ocfs2_space_resv,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_check_range_for_refcount(
        inode: *mut inode,
        pos: loff_t,
        count: usize,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_remove_inode_range(
        inode: *mut inode,
        di_bh: *mut buffer_head,
        byte_start: u64,
        byte_len: u64,
    ) -> ::core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
