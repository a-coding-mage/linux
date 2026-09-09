/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* iptables module for matching the ECN header in IPv4 and TCP header
 *
 * (C) 2002 Harald Welte <laforge@gnumonks.org>
 *
 * This software is distributed under GNU GPL v2, 1991
 * 
 * ipt_ecn.h,v 1.4 2002/08/05 19:39:00 laforge Exp
*/

// Dependency: XT_DSCP_MASK is supplied by linux/netfilter/xt_dscp.h.

pub const XT_ECN_IP_MASK: _ = !XT_DSCP_MASK;

pub const XT_ECN_OP_MATCH_IP: u8 = 0x01;
pub const XT_ECN_OP_MATCH_ECE: u8 = 0x10;
pub const XT_ECN_OP_MATCH_CWR: u8 = 0x20;

pub const XT_ECN_OP_MATCH_MASK: u8 = 0xce;

/* match info */
#[repr(C)]
pub struct xt_ecn_info_proto_tcp {
	pub ect: u8,
}

#[repr(C)]
pub union xt_ecn_info_proto {
	pub tcp: xt_ecn_info_proto_tcp,
}

#[repr(C)]
pub struct xt_ecn_info {
	pub operation: u8,
	pub invert: u8,
	pub ip_ect: u8,
	pub proto: xt_ecn_info_proto,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
