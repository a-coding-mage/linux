/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the UAPI header. The C dependencies are represented by
// their corresponding Rust primitive and FFI types.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct xt_cgroup_info_v0 {
    pub id: u32,
    pub invert: u32,
}

#[repr(C)]
pub struct xt_cgroup_info_v1 {
    pub has_path: u8,
    pub has_classid: u8,
    pub invert_path: u8,
    pub invert_classid: u8,
    pub path: [c_char; 4096], // PATH_MAX
    pub classid: u32,

    /* kernel internal data */
    pub priv_: *mut c_void,
}

pub const XT_CGROUP_PATH_MAX: usize = 512;

#[repr(C)]
pub union xt_cgroup_info_v2__bindgen_ty_1 {
    pub path: [c_char; XT_CGROUP_PATH_MAX],
    pub classid: u32,
}

#[repr(C)]
pub struct xt_cgroup_info_v2 {
    pub has_path: u8,
    pub has_classid: u8,
    pub invert_path: u8,
    pub invert_classid: u8,
    pub __bindgen_anon_1: xt_cgroup_info_v2__bindgen_ty_1,

    /* kernel internal data */
    pub priv_: *mut c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
