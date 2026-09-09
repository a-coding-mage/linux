/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by linux/netfilter/nf_synproxy.h.

pub const XT_SYNPROXY_OPT_MSS: u32 = NF_SYNPROXY_OPT_MSS;
pub const XT_SYNPROXY_OPT_WSCALE: u32 = NF_SYNPROXY_OPT_WSCALE;
pub const XT_SYNPROXY_OPT_SACK_PERM: u32 = NF_SYNPROXY_OPT_SACK_PERM;
pub const XT_SYNPROXY_OPT_TIMESTAMP: u32 = NF_SYNPROXY_OPT_TIMESTAMP;
pub const XT_SYNPROXY_OPT_ECN: u32 = NF_SYNPROXY_OPT_ECN;

pub type xt_synproxy_info = nf_synproxy_info;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
