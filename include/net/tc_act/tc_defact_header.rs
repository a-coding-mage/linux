/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <net/act_api.h> in the C source.
// The containing translation unit must provide `tc_action`.

#[repr(C)]
pub struct tcf_defact {
    pub common: tc_action,
    pub tcfd_datalen: u32,
    pub tcfd_defdata: *mut core::ffi::c_void,
}

#[inline]
pub unsafe fn to_defact(a: *mut tc_action) -> *mut tcf_defact {
    a as *mut tcf_defact
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
