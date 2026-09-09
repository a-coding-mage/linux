/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * dir.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

/* C header guard: OCFS2_DIR_H */

#[repr(C)]
pub struct ocfs2_dx_hinfo {
    pub major_hash: u32,
    pub minor_hash: u32,
}

#[repr(C)]
pub struct ocfs2_dir_lookup_result {
    pub dl_leaf_bh: *mut buffer_head, /* Unindexed leaf block */
    pub dl_entry: *mut ocfs2_dir_entry, /* Target dirent in unindexed leaf */
    pub dl_dx_root_bh: *mut buffer_head, /* Root of indexed tree */
    pub dl_dx_leaf_bh: *mut buffer_head, /* Indexed leaf block */
    pub dl_dx_entry: *mut ocfs2_dx_entry, /* Target dx_entry in indexed leaf */
    pub dl_hinfo: ocfs2_dx_hinfo, /* Name hash results */
    /* Previous entry in dir free space list. NULL if previous entry is dx root block. */
    pub dl_prev_leaf_bh: *mut buffer_head,
}

pub enum buffer_head {}
pub enum ocfs2_dir_entry {}
pub enum ocfs2_dx_entry {}
pub enum inode {}
pub enum dentry {}
pub enum file {}
pub enum dir_context {}
pub enum ocfs2_super {}
pub enum ocfs2_alloc_context {}
pub enum handle_t {}

extern "C" {
    pub fn ocfs2_free_dir_lookup_result(res: *mut ocfs2_dir_lookup_result);

    pub fn ocfs2_find_entry(
        name: *const ::core::ffi::c_char,
        namelen: ::core::ffi::c_int,
        dir: *mut inode,
        lookup: *mut ocfs2_dir_lookup_result,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_delete_entry(
        handle: *mut handle_t,
        dir: *mut inode,
        res: *mut ocfs2_dir_lookup_result,
    ) -> ::core::ffi::c_int;
    pub fn __ocfs2_add_entry(
        handle: *mut handle_t,
        dir: *mut inode,
        name: *const ::core::ffi::c_char,
        namelen: ::core::ffi::c_int,
        inode: *mut inode,
        blkno: u64,
        parent_fe_bh: *mut buffer_head,
        lookup: *mut ocfs2_dir_lookup_result,
    ) -> ::core::ffi::c_int;

    /* The C inline wrapper depends on the external dentry layout and helpers. */
    pub fn ocfs2_add_entry(
        handle: *mut handle_t,
        dentry: *mut dentry,
        inode: *mut inode,
        blkno: u64,
        parent_fe_bh: *mut buffer_head,
        lookup: *mut ocfs2_dir_lookup_result,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_update_entry(
        dir: *mut inode,
        handle: *mut handle_t,
        res: *mut ocfs2_dir_lookup_result,
        new_entry_inode: *mut inode,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_check_dir_for_entry(
        dir: *mut inode,
        name: *const ::core::ffi::c_char,
        namelen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_empty_dir(inode: *mut inode) -> ::core::ffi::c_int;
    pub fn ocfs2_find_files_on_disk(
        name: *const ::core::ffi::c_char,
        namelen: ::core::ffi::c_int,
        blkno: *mut u64,
        inode: *mut inode,
        res: *mut ocfs2_dir_lookup_result,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_lookup_ino_from_name(
        dir: *mut inode,
        name: *const ::core::ffi::c_char,
        namelen: ::core::ffi::c_int,
        blkno: *mut u64,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_readdir(file: *mut file, ctx: *mut dir_context) -> ::core::ffi::c_int;
    pub fn ocfs2_dir_foreach(inode: *mut inode, ctx: *mut dir_context) -> ::core::ffi::c_int;
    pub fn ocfs2_prepare_dir_for_insert(
        osb: *mut ocfs2_super,
        dir: *mut inode,
        parent_fe_bh: *mut buffer_head,
        name: *const ::core::ffi::c_char,
        namelen: ::core::ffi::c_int,
        lookup: *mut ocfs2_dir_lookup_result,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_fill_new_dir(
        osb: *mut ocfs2_super,
        handle: *mut handle_t,
        parent: *mut inode,
        inode: *mut inode,
        fe_bh: *mut buffer_head,
        data_ac: *mut ocfs2_alloc_context,
        meta_ac: *mut ocfs2_alloc_context,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_dx_dir_truncate(dir: *mut inode, di_bh: *mut buffer_head) -> ::core::ffi::c_int;
    pub fn ocfs2_dir_trailer_from_size(
        blocksize: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_void,
    ) -> *mut ocfs2_dir_block_trailer;
}

pub enum ocfs2_dir_block_trailer {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
