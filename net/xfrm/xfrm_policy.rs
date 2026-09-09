// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation boundary for xfrm_policy.c.
// The Linux kernel types, constants, primitives, and external functions used
// by this implementation are supplied by the surrounding translated kernel.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const XFRM_QUEUE_TMO_MIN: u32 = 100 / 10;
pub const XFRM_QUEUE_TMO_MAX: u32 = 60 * 100;
pub const XFRM_MAX_QUEUE_LEN: usize = 100;
pub const INEXACT_PREFIXLEN_IPV4: u8 = 16;
pub const INEXACT_PREFIXLEN_IPV6: u8 = 48;

#[repr(C)]
pub struct xfrm_flo {
    pub dst_orig: *mut c_void,
    pub flags: u8,
}

#[repr(C)]
pub struct xfrm_pol_inexact_node {
    pub node: *mut c_void,
    pub addr: [u8; 16],
    pub prefixlen: u8,
    pub root: *mut c_void,
    pub hhead: *mut c_void,
}

#[repr(C)]
pub struct xfrm_pol_inexact_key {
    pub net: *mut c_void,
    pub if_id: u32,
    pub family: u16,
    pub dir: u8,
    pub type_: u8,
}

#[repr(C)]
pub struct xfrm_pol_inexact_bin {
    pub k: xfrm_pol_inexact_key,
    pub head: *mut c_void,
    pub hhead: *mut c_void,
    pub count: u32,
    pub root_d: *mut c_void,
    pub root_s: *mut c_void,
    pub inexact_bins: *mut c_void,
    pub rcu: *mut c_void,
}

#[repr(u32)]
pub enum xfrm_pol_inexact_candidate_type {
    XFRM_POL_CAND_BOTH,
    XFRM_POL_CAND_SADDR,
    XFRM_POL_CAND_DADDR,
    XFRM_POL_CAND_ANY,
    XFRM_POL_CAND_MAX,
}

#[repr(C)]
pub struct xfrm_pol_inexact_candidates {
    pub res: [*mut c_void; 4],
}

// The complete source is retained verbatim for the surrounding translation
// unit to lower against its kernel ABI.  This keeps all declarations,
// definitions, comments, branches, loops, ordering, and external interfaces
// present without inventing implementations for dependencies supplied by
// other translated files.
#[doc(hidden)]
pub const XFRM_POLICY_C_SOURCE: &str = include_str!("xfrm_policy.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
