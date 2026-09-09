/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/slab.h and linux/ceph/auth.h.

/*
 * null security mode.
 *
 * we use a single static authorizer that simply encodes our entity name
 * and global id.
 */

#[repr(C)]
pub struct ceph_none_authorizer {
    pub base: ceph_authorizer,
    pub buf: [core::ffi::c_char; 128],
    pub buf_len: core::ffi::c_int,
}

#[repr(C)]
pub struct ceph_auth_none_info {
    pub starting: bool,
}

unsafe extern "C" {
    pub fn ceph_auth_none_init(ac: *mut ceph_auth_client) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
