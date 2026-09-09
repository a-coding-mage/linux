/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux headers:
// `nf_inet_addr`, `__be16`, and `__u8`.

pub const XT_IPVS_IPVS_PROPERTY: i32 = 1 << 0; // all other options imply this one
pub const XT_IPVS_PROTO: i32 = 1 << 1;
pub const XT_IPVS_VADDR: i32 = 1 << 2;
pub const XT_IPVS_VPORT: i32 = 1 << 3;
pub const XT_IPVS_DIR: i32 = 1 << 4;
pub const XT_IPVS_METHOD: i32 = 1 << 5;
pub const XT_IPVS_VPORTCTL: i32 = 1 << 6;
pub const XT_IPVS_MASK: i32 = (1 << 7) - 1;
pub const XT_IPVS_ONCE_MASK: i32 = XT_IPVS_MASK & !XT_IPVS_IPVS_PROPERTY;

#[repr(C)]
pub struct xt_ipvs_mtinfo {
    pub vaddr: nf_inet_addr,
    pub vmask: nf_inet_addr,
    pub vport: __be16,
    pub l4proto: __u8,
    pub fwd_method: __u8,
    pub vportctl: __be16,
    pub invert: __u8,
    pub bitmask: __u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
