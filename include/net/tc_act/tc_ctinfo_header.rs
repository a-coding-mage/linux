/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <net/act_api.h>

#[repr(C)]
pub struct tcf_ctinfo_params {
    pub rcu: rcu_head,
    pub net: *mut net,
    pub action: ::core::ffi::c_int,
    pub dscpmask: u32,
    pub dscpstatemask: u32,
    pub cpmarkmask: u32,
    pub zone: u16,
    pub mode: u8,
    pub dscpmaskshift: u8,
}

#[repr(C)]
pub struct tcf_ctinfo {
    pub common: tc_action,
    pub params: *mut tcf_ctinfo_params,
    pub stats_dscp_set: atomic64_t,
    pub stats_dscp_error: atomic64_t,
    pub stats_cpmark_set: atomic64_t,
}

pub const CTINFO_MODE_DSCP: u32 = 1u32 << 0;
pub const CTINFO_MODE_CPMARK: u32 = 1u32 << 1;

#[inline]
pub unsafe fn to_ctinfo(a: *mut ::core::ffi::c_void) -> *mut tcf_ctinfo {
    a as *mut tcf_ctinfo
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
