/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Header file for iptables ipt_ECN target
 *
 * (C) 2002 by Harald Welte <laforge@gnumonks.org>
 *
 * This software is distributed under GNU GPL v2, 1991
 *
 * ipt_ECN.h,v 1.3 2002/05/29 12:17:40 laforge Exp
 */

/* Depends on linux/types.h and linux/netfilter/xt_DSCP.h. */

pub const IPT_ECN_IP_MASK: u8 = !XT_DSCP_MASK;

pub const IPT_ECN_OP_SET_IP: u8 = 0x01; /* set ECN bits of IPv4 header */
pub const IPT_ECN_OP_SET_ECE: u8 = 0x10; /* set ECE bit of TCP header */
pub const IPT_ECN_OP_SET_CWR: u8 = 0x20; /* set CWR bit of TCP header */

pub const IPT_ECN_OP_MASK: u8 = 0xce;

#[repr(C)]
pub struct IptEcnInfoTcp {
    /* C bit-fields ece:1, cwr:1; remaining bits are padding. */
    pub bits: u8,
}

pub const IPT_ECN_TCP_ECE_MASK: u8 = 0x01;
pub const IPT_ECN_TCP_CWR_MASK: u8 = 0x02;

#[repr(C)]
pub union IptEcnInfoProto {
    pub tcp: IptEcnInfoTcp,
}

#[repr(C)]
pub struct IptEcnInfo {
    pub operation: u8, /* bitset of operations */
    pub ip_ect: u8, /* ECT codepoint of IPv4 header, pre-shifted */
    pub proto: IptEcnInfoProto,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
