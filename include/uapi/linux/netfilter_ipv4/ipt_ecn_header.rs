/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalent of: #include <linux/netfilter/xt_ecn.h>

pub type ipt_ecn_info = xt_ecn_info;

pub const IPT_ECN_IP_MASK: i32 = XT_ECN_IP_MASK;
pub const IPT_ECN_OP_MATCH_IP: i32 = XT_ECN_OP_MATCH_IP;
pub const IPT_ECN_OP_MATCH_ECE: i32 = XT_ECN_OP_MATCH_ECE;
pub const IPT_ECN_OP_MATCH_CWR: i32 = XT_ECN_OP_MATCH_CWR;
pub const IPT_ECN_OP_MATCH_MASK: i32 = XT_ECN_OP_MATCH_MASK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
