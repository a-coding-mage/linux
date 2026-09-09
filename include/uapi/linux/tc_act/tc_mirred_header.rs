/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header. The C includes provide `tc_gen` and
// the fixed-width integer types used below.

pub const TCA_EGRESS_REDIR: i32 = 1; // packet redirect to EGRESS
pub const TCA_EGRESS_MIRROR: i32 = 2; // mirror packet to EGRESS
pub const TCA_INGRESS_REDIR: i32 = 3; // packet redirect to INGRESS
pub const TCA_INGRESS_MIRROR: i32 = 4; // mirror packet to INGRESS

#[repr(C)]
pub struct tc_mirred {
    pub tc_gen: tc_gen,
    pub eaction: ::core::ffi::c_int, // one of IN/EGRESS_MIRROR/REDIR
    pub ifindex: u32, // ifindex of egress port
}

#[repr(i32)]
pub enum tc_mirred_attr {
    TCA_MIRRED_UNSPEC = 0,
    TCA_MIRRED_TM = 1,
    TCA_MIRRED_PARMS = 2,
    TCA_MIRRED_PAD = 3,
    TCA_MIRRED_BLOCKID = 4,
    __TCA_MIRRED_MAX = 5,
}

pub const TCA_MIRRED_MAX: i32 = (__TCA_MIRRED_MAX as i32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
