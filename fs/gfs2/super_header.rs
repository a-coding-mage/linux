/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Translated from super.h. C include dependencies are supplied by other files.

/* Supported fs format version range */
pub const GFS2_FS_FORMAT_MIN: i32 = 1801;
pub const GFS2_FS_FORMAT_MAX: i32 = 1802;

extern "C" {
    pub fn gfs2_lm_unmount(sdp: *mut gfs2_sbd);

    pub fn gfs2_jindex_free(sdp: *mut gfs2_sbd);
    pub fn gfs2_jdesc_find(sdp: *mut gfs2_sbd, jid: core::ffi::c_uint) -> *mut gfs2_jdesc;
    pub fn gfs2_jdesc_check(jd: *mut gfs2_jdesc) -> core::ffi::c_int;
    pub fn gfs2_lookup_in_master_dir(
        sdp: *mut gfs2_sbd,
        filename: *mut core::ffi::c_char,
        ipp: *mut *mut gfs2_inode,
    ) -> core::ffi::c_int;

    pub fn gfs2_make_fs_rw(sdp: *mut gfs2_sbd) -> core::ffi::c_int;
    pub fn gfs2_make_fs_ro(sdp: *mut gfs2_sbd);
    pub fn gfs2_online_uevent(sdp: *mut gfs2_sbd);
    pub fn gfs2_destroy_threads(sdp: *mut gfs2_sbd);
    pub fn gfs2_statfs_init(sdp: *mut gfs2_sbd) -> core::ffi::c_int;
    pub fn gfs2_statfs_change(sdp: *mut gfs2_sbd, total: s64, free: s64, dinodes: s64);
    pub fn gfs2_statfs_change_in(sc: *mut gfs2_statfs_change_host, buf: *const core::ffi::c_void);
    pub fn gfs2_statfs_change_out(sc: *const gfs2_statfs_change_host, buf: *mut core::ffi::c_void);
    pub fn update_statfs(sdp: *mut gfs2_sbd, m_bh: *mut buffer_head);
    pub fn gfs2_statfs_sync(sb: *mut super_block, type_: core::ffi::c_int) -> core::ffi::c_int;
    pub fn gfs2_freeze_func(work: *mut work_struct);

    pub fn free_local_statfs_inodes(sdp: *mut gfs2_sbd);
    pub fn find_local_statfs_inode(sdp: *mut gfs2_sbd, index: core::ffi::c_uint) -> *mut inode;
    pub fn free_sbd(sdp: *mut gfs2_sbd);

    pub static mut gfs2_fs_type: file_system_type;
    pub static mut gfs2meta_fs_type: file_system_type;
    pub static gfs2_export_ops: export_operations;
    pub static gfs2_super_ops: super_operations;
    pub static gfs2_dops: dentry_operations;

    pub static gfs2_xattr_handlers_max: *const *const xattr_handler;
    pub static gfs2_xattr_handlers_min: *const *const xattr_handler;
}

// The following declarations are provided by the translated dependency headers.
extern "C" {
    pub type gfs2_sbd;
    pub type gfs2_jdesc;
    pub type gfs2_inode;
    pub type gfs2_statfs_change_host;
    pub type buffer_head;
    pub type super_block;
    pub type work_struct;
    pub type inode;
    pub type file_system_type;
    pub type export_operations;
    pub type super_operations;
    pub type dentry_operations;
    pub type xattr_handler;
    pub type s64;
}

/*
 * C inline equivalent. The referenced spin lock and fields belong to the
 * supplied gfs2_sbd definition.
 */
#[inline]
pub unsafe fn gfs2_jindex_size(sdp: *mut gfs2_sbd) -> core::ffi::c_uint {
    let mut x: core::ffi::c_uint;
    spin_lock(&mut (*sdp).sd_jindex_spin);
    x = (*sdp).sd_journals;
    spin_unlock(&mut (*sdp).sd_jindex_spin);
    x
}

extern "C" {
    fn spin_lock(lock: *mut core::ffi::c_void);
    fn spin_unlock(lock: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
