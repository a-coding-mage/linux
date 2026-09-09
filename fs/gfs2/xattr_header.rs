/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Forward declarations supplied by other translation units.
pub struct Gfs2Inode;
pub struct Iattr;

#[macro_export]
macro_rules! GFS2_EA_REC_LEN {
    ($ea:expr) => { be32_to_cpu(unsafe { (*$ea).ea_rec_len }) };
}

#[macro_export]
macro_rules! GFS2_EA_DATA_LEN {
    ($ea:expr) => { be32_to_cpu(unsafe { (*$ea).ea_data_len }) };
}

#[macro_export]
macro_rules! GFS2_EA_SIZE {
    ($ea:expr) => {
        ALIGN(
            core::mem::size_of::<gfs2_ea_header>()
                + unsafe { (*$ea).ea_name_len as usize }
                + if GFS2_EA_IS_STUFFED!($ea) {
                    GFS2_EA_DATA_LEN!($ea) as usize
                } else {
                    core::mem::size_of::<__be64>() * unsafe { (*$ea).ea_num_ptrs as usize }
                },
            8,
        )
    };
}

#[macro_export]
macro_rules! GFS2_EA_IS_STUFFED {
    ($ea:expr) => { unsafe { (*$ea).ea_num_ptrs == 0 } };
}

#[macro_export]
macro_rules! GFS2_EA_IS_LAST {
    ($ea:expr) => { unsafe { (*$ea).ea_flags & GFS2_EAFLAG_LAST } };
}

#[macro_export]
macro_rules! GFS2_EAREQ_SIZE_STUFFED {
    ($er:expr) => {
        ALIGN(
            core::mem::size_of::<gfs2_ea_header>()
                + unsafe { (*$er).er_name_len as usize }
                + unsafe { (*$er).er_data_len as usize },
            8,
        )
    };
}

#[macro_export]
macro_rules! GFS2_EA2NAME {
    ($ea:expr) => {
        (($ea as *mut gfs2_ea_header).wrapping_add(1) as *mut i8)
    };
}

#[macro_export]
macro_rules! GFS2_EA2DATA {
    ($ea:expr) => {
        (GFS2_EA2NAME!($ea).wrapping_add(unsafe { (*$ea).ea_name_len as usize }))
    };
}

#[macro_export]
macro_rules! GFS2_EA2DATAPTRS {
    ($ea:expr) => {
        (GFS2_EA2NAME!($ea)
            .wrapping_add(ALIGN(unsafe { (*$ea).ea_name_len as usize }, 8)) as *mut __be64)
    };
}

#[macro_export]
macro_rules! GFS2_EA2NEXT {
    ($ea:expr) => {
        (($ea as *mut u8).wrapping_add(GFS2_EA_REC_LEN!($ea) as usize) as *mut gfs2_ea_header)
    };
}

#[macro_export]
macro_rules! GFS2_EA_BH2FIRST {
    ($bh:expr) => {
        ((unsafe { (*$bh).b_data }.wrapping_add(core::mem::size_of::<gfs2_meta_header>()))
            as *mut gfs2_ea_header)
    };
}

#[repr(C)]
pub struct gfs2_ea_request {
    pub er_name: *const i8,
    pub er_data: *mut i8,
    pub er_name_len: core::ffi::c_uint,
    pub er_data_len: core::ffi::c_uint,
    pub er_type: core::ffi::c_uint, /* GFS2_EATYPE_... */
}

#[repr(C)]
pub struct gfs2_ea_location {
    pub el_bh: *mut buffer_head,
    pub el_ea: *mut gfs2_ea_header,
    pub el_prev: *mut gfs2_ea_header,
}

unsafe extern "C" {
    pub fn __gfs2_xattr_set(
        inode: *mut inode,
        name: *const i8,
        value: *const core::ffi::c_void,
        size: usize,
        flags: core::ffi::c_int,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn gfs2_listxattr(dentry: *mut dentry, buffer: *mut i8, size: usize) -> isize;
    pub fn gfs2_ea_dealloc(ip: *mut gfs2_inode, initialized: bool) -> core::ffi::c_int;

    /* Exported to acl.c */
    pub fn gfs2_xattr_acl_get(
        ip: *mut gfs2_inode,
        name: *const i8,
        data: *mut *mut i8,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
