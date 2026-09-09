/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2025 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Declarations supplied by the surrounding kernel translation.

unsafe extern "C" {
    pub fn fserror_mount(sb: *mut super_block);
    pub fn fserror_unmount(sb: *mut super_block);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fserror_type {
    // pagecache I/O failed
    FSERR_BUFFERED_READ,
    FSERR_BUFFERED_WRITE,

    // direct I/O failed
    FSERR_DIRECTIO_READ,
    FSERR_DIRECTIO_WRITE,

    // out of band media error reported
    FSERR_DATA_LOST,

    // filesystem metadata
    FSERR_METADATA,
}

#[repr(C)]
pub struct fserror_event {
    pub work: work_struct,
    pub sb: *mut super_block,
    pub inode: *mut inode,
    pub pos: loff_t,
    pub len: u64,
    pub type_: fserror_type,

    // negative error number
    pub error: i32,
}

unsafe extern "C" {
    pub fn fserror_report(
        sb: *mut super_block,
        inode: *mut inode,
        type_: fserror_type,
        pos: loff_t,
        len: u64,
        error: i32,
        gfp: gfp_t,
    );
}

#[inline]
pub unsafe fn fserror_report_io(
    inode: *mut inode,
    type_: fserror_type,
    pos: loff_t,
    len: u64,
    error: i32,
    gfp: gfp_t,
) {
    unsafe {
        fserror_report((*inode).i_sb, inode, type_, pos, len, error, gfp);
    }
}

#[inline]
pub unsafe fn fserror_report_data_lost(
    inode: *mut inode,
    pos: loff_t,
    len: u64,
    gfp: gfp_t,
) {
    unsafe {
        fserror_report((*inode).i_sb, inode, fserror_type::FSERR_DATA_LOST, pos, len, -EIO, gfp);
    }
}

#[inline]
pub unsafe fn fserror_report_file_metadata(inode: *mut inode, error: i32, gfp: gfp_t) {
    unsafe {
        fserror_report((*inode).i_sb, inode, fserror_type::FSERR_METADATA, 0, 0, error, gfp);
    }
}

#[inline]
pub unsafe fn fserror_report_metadata(sb: *mut super_block, error: i32, gfp: gfp_t) {
    unsafe {
        fserror_report(sb, core::ptr::null_mut(), fserror_type::FSERR_METADATA, 0, 0, error, gfp);
    }
}

#[inline]
pub unsafe fn fserror_report_shutdown(sb: *mut super_block, gfp: gfp_t) {
    unsafe {
        fserror_report(
            sb,
            core::ptr::null_mut(),
            fserror_type::FSERR_METADATA,
            0,
            0,
            -ESHUTDOWN,
            gfp,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
