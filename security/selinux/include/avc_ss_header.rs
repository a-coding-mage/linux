/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Access vector cache interface for the security server.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

// C dependency: <linux/types.h> provides u32.

unsafe extern "C" {
    pub fn avc_ss_reset(seqno: u32) -> core::ffi::c_int;
}

/* Class/perm mapping support */
#[repr(C)]
pub struct security_class_mapping {
    pub name: *const core::ffi::c_char,
    pub perms: [*const core::ffi::c_char; core::mem::size_of::<u32>() * 8 + 1],
}

unsafe extern "C" {
    pub static secclass_map: [security_class_mapping; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
