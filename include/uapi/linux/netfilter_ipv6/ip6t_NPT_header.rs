/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h> and <linux/netfilter.h> declarations.
// The external `nf_inet_addr` type is supplied by the corresponding dependency.

#[repr(C)]
pub struct ip6t_npt_tginfo {
    pub src_pfx: nf_inet_addr,
    pub dst_pfx: nf_inet_addr,
    pub src_pfx_len: u8,
    pub dst_pfx_len: u8,
    // Used internally by the kernel
    pub adjustment: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
