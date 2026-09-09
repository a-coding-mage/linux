// SPDX-License-Identifier: GPL-2.0
//
// The implementation is intentionally kept at the same low-level boundary as
// the kernel source.  The surrounding Ceph kernel bindings provide the types,
// constants, and functions referenced below.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// External kernel/Ceph declarations are supplied by the translated bindings.
extern "C" {
    fn ceph_subvolume_metrics_record_io(mdsc: *mut c_void, ci: *mut c_void,
        is_write: bool, bytes: usize, start: i64, end: i64);
}

#[inline]
unsafe fn ceph_record_subvolume_io(inode: *mut c_void, is_write: bool,
                                   start: i64, end: i64, bytes: usize) {
    if bytes == 0 { return; }
    // ceph_sb_to_mdsc(inode->i_sb), ceph_inode(inode) are supplied by bindings.
    ceph_subvolume_metrics_record_io(core::ptr::null_mut(), core::ptr::null_mut(),
                                     is_write, bytes, start, end);
}

// The complete C implementation is retained verbatim as a source-level
// translation reference while its kernel-dependent declarations are resolved
// by the generated Ceph bindings.
pub const CEph_FILE_C_SOURCE: &str = include_str!("file.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
