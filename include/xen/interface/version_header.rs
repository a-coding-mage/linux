/* SPDX-License-Identifier: MIT */
/******************************************************************************
 * version.h
 *
 * Xen version, type, and compile information.
 *
 * Copyright (c) 2005, Nguyen Anh Quynh <aquynh@gmail.com>
 * Copyright (c) 2005, Keir Fraser <keir@xensource.com>
 */

/* NB. All ops return zero on success, except XENVER_version. */

/* arg == NULL; returns major:minor (16:16). */
pub const XENVER_version: u32 = 0;

/* arg == xen_extraversion_t. */
pub const XENVER_extraversion: u32 = 1;
#[repr(C)]
pub struct xen_extraversion {
    pub extraversion: [core::ffi::c_char; 16],
}
pub const XEN_EXTRAVERSION_LEN: usize = core::mem::size_of::<xen_extraversion>();

/* arg == xen_compile_info_t. */
pub const XENVER_compile_info: u32 = 2;
#[repr(C)]
pub struct xen_compile_info {
    pub compiler: [core::ffi::c_char; 64],
    pub compile_by: [core::ffi::c_char; 16],
    pub compile_domain: [core::ffi::c_char; 32],
    pub compile_date: [core::ffi::c_char; 32],
}

pub const XENVER_capabilities: u32 = 3;
#[repr(C)]
pub struct xen_capabilities_info {
    pub info: [core::ffi::c_char; 1024],
}
pub const XEN_CAPABILITIES_INFO_LEN: usize = core::mem::size_of::<xen_capabilities_info>();

pub const XENVER_changeset: u32 = 4;
#[repr(C)]
pub struct xen_changeset_info {
    pub info: [core::ffi::c_char; 64],
}
pub const XEN_CHANGESET_INFO_LEN: usize = core::mem::size_of::<xen_changeset_info>();

pub const XENVER_platform_parameters: u32 = 5;
#[repr(C)]
pub struct xen_platform_parameters {
    pub virt_start: xen_ulong_t,
}

pub const XENVER_get_features: u32 = 6;
#[repr(C)]
pub struct xen_feature_info {
    pub submap_idx: u32, /* IN: which 32-bit submap to return */
    pub submap: u32,     /* OUT: 32-bit submap */
}

/* Declares the features reported by XENVER_get_features.
 * Dependency: xen/interface/features.h
 */

/* arg == NULL; returns host memory page size. */
pub const XENVER_pagesize: u32 = 7;

/* arg == xen_domain_handle_t. */
pub const XENVER_guest_handle: u32 = 8;

pub const XENVER_commandline: u32 = 9;
#[repr(C)]
pub struct xen_commandline {
    pub buf: [core::ffi::c_char; 1024],
}

/*
 * Return value is the number of bytes written, or XEN_Exx on error.
 * Calling with empty parameter returns the size of build_id.
 */
pub const XENVER_build_id: u32 = 10;
#[repr(C)]
pub struct xen_build_id {
    pub len: u32, /* IN: size of buf[]. */
    pub buf: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
