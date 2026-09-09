/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h> and <linux/netfilter.h>; the referenced
// integer aliases and `nf_inet_addr` are supplied by those dependencies.

/* TPROXY target is capable of marking the packet to perform
 * redirection. We can get rid of that whenever we get support for
 * mutliple targets in the same rule. */
#[repr(C)]
pub struct xt_tproxy_target_info {
    pub mark_mask: __u32,
    pub mark_value: __u32,
    pub laddr: __be32,
    pub lport: __be16,
}

#[repr(C)]
pub struct xt_tproxy_target_info_v1 {
    pub mark_mask: __u32,
    pub mark_value: __u32,
    pub laddr: nf_inet_addr,
    pub lport: __be16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
