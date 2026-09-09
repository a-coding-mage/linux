/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <net/act_api.h>.
use core::ffi::c_void;

// Opaque types supplied by external kernel headers.
pub enum net {}
pub enum rcu_head {}
pub enum tc_action {}

#[repr(C)]
pub struct tcf_connmark_parms {
    pub net: *mut net,
    pub zone: u16,
    pub action: i32,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct tcf_connmark_info {
    pub common: tc_action,
    // C __rcu annotation is a pointer-access/RCU convention.
    pub parms: *mut tcf_connmark_parms,
}

#[inline]
pub unsafe fn to_connmark(a: *mut c_void) -> *mut tcf_connmark_info {
    a as *mut tcf_connmark_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
