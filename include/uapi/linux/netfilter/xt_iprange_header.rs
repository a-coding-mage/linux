/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependencies:
// #include <linux/types.h>
// #include <linux/netfilter.h>

pub const IPRANGE_SRC: i32 = 1 << 0; // match source IP address
pub const IPRANGE_DST: i32 = 1 << 1; // match destination IP address
pub const IPRANGE_SRC_INV: i32 = 1 << 4; // negate the condition
pub const IPRANGE_DST_INV: i32 = 1 << 5; // -"-

#[repr(C)]
pub struct xt_iprange_mtinfo {
    pub src_min: nf_inet_addr,
    pub src_max: nf_inet_addr,
    pub dst_min: nf_inet_addr,
    pub dst_max: nf_inet_addr,
    pub flags: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
