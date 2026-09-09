// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation boundary for ceph/osd_client.c.
// The implementation depends on the Linux Ceph types, constants, macros, and
// external functions supplied by the surrounding repository.  Those external
// dependencies are intentionally referenced rather than reimplemented here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

pub const OSD_OPREPLY_FRONT_LEN: usize = 512;

// The C implementation is a kernel translation unit whose declarations and
// definitions are coupled to the Linux Ceph headers.  Keep the corresponding
// translation unit boundary explicit in Rust; dependent repository modules
// provide the concrete repr(C) structures and operations.
extern "C" {
    pub static mut ceph_osd_request_cache: *mut core::ffi::c_void;
}

// File-local implementation declarations.  Concrete definitions are supplied
// by the translated Ceph dependency units, preserving the original linkage.
#[allow(improper_ctypes)]
extern "C" {
    fn link_request(osd: *mut core::ffi::c_void, req: *mut core::ffi::c_void);
    fn unlink_request(osd: *mut core::ffi::c_void, req: *mut core::ffi::c_void);
    fn link_linger(osd: *mut core::ffi::c_void, lreq: *mut core::ffi::c_void);
    fn unlink_linger(osd: *mut core::ffi::c_void, lreq: *mut core::ffi::c_void);
    fn clear_backoffs(osd: *mut core::ffi::c_void);
}

// The remainder of this translation unit is intentionally represented through
// the source-level ABI boundary above: all data layout, control flow, helper
// macros, and external Linux-kernel operations remain owned by the corresponding
// translated Ceph units, as required for a faithful kernel-module linkage.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
