/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * extent_map.h
 *
 * In-memory file extent mappings for OCFS2.
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

use core::ffi::{c_int, c_uint, c_ulonglong, c_void};

// Types supplied by other headers/dependencies.
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ocfs2_extent_rec {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fiemap_extent_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ocfs2_extent_list {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ocfs2_caching_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

pub type u32 = u32;
pub type u64 = u64;
pub type loff_t = i64;

#[repr(C)]
pub struct ocfs2_extent_map_item {
    pub ei_cpos: c_uint,
    pub ei_phys: c_uint,
    pub ei_clusters: c_uint,
    pub ei_flags: c_uint,
    pub ei_list: list_head,
}

pub const OCFS2_MAX_EXTENT_MAP_ITEMS: c_uint = 3;

#[repr(C)]
pub struct ocfs2_extent_map {
    pub em_num_items: c_uint,
    pub em_list: list_head,
}

extern "C" {
    pub fn ocfs2_extent_map_init(inode: *mut inode);
    pub fn ocfs2_extent_map_trunc(inode: *mut inode, cluster: c_uint);
    pub fn ocfs2_extent_map_insert_rec(inode: *mut inode, rec: *mut ocfs2_extent_rec);

    pub fn ocfs2_get_clusters(
        inode: *mut inode,
        v_cluster: u32,
        p_cluster: *mut u32,
        num_clusters: *mut u32,
        extent_flags: *mut c_uint,
    ) -> c_int;
    pub fn ocfs2_extent_map_get_blocks(
        inode: *mut inode,
        v_blkno: u64,
        p_blkno: *mut u64,
        ret_count: *mut u64,
        extent_flags: *mut c_uint,
    ) -> c_int;

    pub fn ocfs2_fiemap(
        inode: *mut inode,
        fieinfo: *mut fiemap_extent_info,
        map_start: u64,
        map_len: u64,
    ) -> c_int;

    pub fn ocfs2_overwrite_io(
        inode: *mut inode,
        di_bh: *mut buffer_head,
        map_start: u64,
        map_len: u64,
    ) -> c_int;

    pub fn ocfs2_seek_data_hole_offset(
        file: *mut file,
        offset: *mut loff_t,
        origin: c_int,
    ) -> c_int;

    pub fn ocfs2_xattr_get_clusters(
        inode: *mut inode,
        v_cluster: u32,
        p_cluster: *mut u32,
        num_clusters: *mut u32,
        el: *mut ocfs2_extent_list,
        extent_flags: *mut c_uint,
    ) -> c_int;

    pub fn ocfs2_read_virt_blocks(
        inode: *mut inode,
        v_block: u64,
        nr: c_int,
        bhs: *mut *mut buffer_head,
        flags: c_int,
        validate: Option<unsafe extern "C" fn(*mut super_block, *mut buffer_head) -> c_int>,
    ) -> c_int;
    pub fn ocfs2_figure_hole_clusters(
        ci: *mut ocfs2_caching_info,
        el: *mut ocfs2_extent_list,
        eb_bh: *mut buffer_head,
        v_cluster: u32,
        num_clusters: *mut u32,
    ) -> c_int;
    pub fn printk(format: *const core::ffi::c_char, ...) -> c_int;
}

pub const EINVAL: c_int = 22;

#[inline]
pub unsafe fn ocfs2_read_virt_block(
    inode: *mut inode,
    v_block: u64,
    bh: *mut *mut buffer_head,
    validate: Option<unsafe extern "C" fn(*mut super_block, *mut buffer_head) -> c_int>,
) -> c_int {
    let mut status: c_int = 0;

    if bh.is_null() {
        let message = b"ocfs2: bh == NULL\0";
        printk(message.as_ptr() as *const core::ffi::c_char);
        status = -EINVAL;
        return status;
    }

    status = ocfs2_read_virt_blocks(inode, v_block, 1, bh, 0, validate);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
