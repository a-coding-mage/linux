/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Connection state tracking for netfilter. */

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IpConntrackInfo {
    /* Part of an established connection (either direction). */
    IP_CT_ESTABLISHED = 0,
    /* Like NEW, but related to an existing connection, or ICMP error. */
    IP_CT_RELATED = 1,
    /* Started a new connection to track; may be a retransmission. */
    IP_CT_NEW = 2,
    /* >= this indicates reply direction */
    IP_CT_IS_REPLY = 3,
    IP_CT_ESTABLISHED_REPLY = 3,
    IP_CT_RELATED_REPLY = 4,
    /* Number of distinct IP_CT types. */
    IP_CT_NUMBER = 5,
    /* only for userspace compatibility */
    #[cfg(not(feature = "kernel"))]
    IP_CT_NEW_REPLY = 5,
    #[cfg(feature = "kernel")]
    IP_CT_UNTRACKED = 7,
}

pub const NF_CT_STATE_INVALID_BIT: i32 = 1 << 0;
pub const fn NF_CT_STATE_BIT(ctinfo: i32) -> i32 {
    1 << ((ctinfo % IpConntrackInfo::IP_CT_IS_REPLY as i32) + 1)
}
pub const NF_CT_STATE_UNTRACKED_BIT: i32 = 1 << 6;

/* Bitset representing status of connection. */
pub const IPS_EXPECTED_BIT: i32 = 0;
pub const IPS_EXPECTED: i32 = 1 << IPS_EXPECTED_BIT;
pub const IPS_SEEN_REPLY_BIT: i32 = 1;
pub const IPS_SEEN_REPLY: i32 = 1 << IPS_SEEN_REPLY_BIT;
pub const IPS_ASSURED_BIT: i32 = 2;
pub const IPS_ASSURED: i32 = 1 << IPS_ASSURED_BIT;
pub const IPS_CONFIRMED_BIT: i32 = 3;
pub const IPS_CONFIRMED: i32 = 1 << IPS_CONFIRMED_BIT;
pub const IPS_SRC_NAT_BIT: i32 = 4;
pub const IPS_SRC_NAT: i32 = 1 << IPS_SRC_NAT_BIT;
pub const IPS_DST_NAT_BIT: i32 = 5;
pub const IPS_DST_NAT: i32 = 1 << IPS_DST_NAT_BIT;
pub const IPS_NAT_MASK: i32 = IPS_DST_NAT | IPS_SRC_NAT;
pub const IPS_SEQ_ADJUST_BIT: i32 = 6;
pub const IPS_SEQ_ADJUST: i32 = 1 << IPS_SEQ_ADJUST_BIT;
pub const IPS_SRC_NAT_DONE_BIT: i32 = 7;
pub const IPS_SRC_NAT_DONE: i32 = 1 << IPS_SRC_NAT_DONE_BIT;
pub const IPS_DST_NAT_DONE_BIT: i32 = 8;
pub const IPS_DST_NAT_DONE: i32 = 1 << IPS_DST_NAT_DONE_BIT;
pub const IPS_NAT_DONE_MASK: i32 = IPS_DST_NAT_DONE | IPS_SRC_NAT_DONE;
pub const IPS_DYING_BIT: i32 = 9;
pub const IPS_DYING: i32 = 1 << IPS_DYING_BIT;
pub const IPS_FIXED_TIMEOUT_BIT: i32 = 10;
pub const IPS_FIXED_TIMEOUT: i32 = 1 << IPS_FIXED_TIMEOUT_BIT;
pub const IPS_TEMPLATE_BIT: i32 = 11;
pub const IPS_TEMPLATE: i32 = 1 << IPS_TEMPLATE_BIT;
pub const IPS_UNTRACKED_BIT: i32 = 12;
pub const IPS_UNTRACKED: i32 = 1 << IPS_UNTRACKED_BIT;

#[cfg(feature = "kernel")]
pub const IPS_NAT_CLASH_BIT: i32 = IPS_UNTRACKED_BIT;
#[cfg(feature = "kernel")]
pub const IPS_NAT_CLASH: i32 = IPS_UNTRACKED;

pub const IPS_HELPER_BIT: i32 = 13;
pub const IPS_HELPER: i32 = 1 << IPS_HELPER_BIT;
pub const IPS_OFFLOAD_BIT: i32 = 14;
pub const IPS_OFFLOAD: i32 = 1 << IPS_OFFLOAD_BIT;
pub const IPS_HW_OFFLOAD_BIT: i32 = 15;
pub const IPS_HW_OFFLOAD: i32 = 1 << IPS_HW_OFFLOAD_BIT;
pub const IPS_UNCHANGEABLE_MASK: i32 = IPS_NAT_DONE_MASK | IPS_NAT_MASK |
    IPS_EXPECTED | IPS_CONFIRMED | IPS_DYING | IPS_SEQ_ADJUST | IPS_TEMPLATE |
    IPS_UNTRACKED | IPS_OFFLOAD | IPS_HW_OFFLOAD;
pub const __IPS_MAX_BIT: i32 = 16;

/* Connection tracking event types */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IpConntrackEvents {
    IPCT_NEW = 0,
    IPCT_RELATED = 1,
    IPCT_DESTROY = 2,
    IPCT_REPLY = 3,
    IPCT_ASSURED = 4,
    IPCT_PROTOINFO = 5,
    IPCT_HELPER = 6,
    IPCT_MARK = 7,
    IPCT_SEQADJ = 8,
    IPCT_NATSEQADJ = 8,
    IPCT_SECMARK = 9,
    IPCT_LABEL = 10,
    IPCT_SYNPROXY = 11,
    #[cfg(feature = "kernel")]
    __IPCT_MAX = 12,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IpConntrackExpectEvents {
    IPEXP_NEW = 0,
    IPEXP_DESTROY = 1,
}

/* expectation flags */
pub const NF_CT_EXPECT_PERMANENT: i32 = 0x1;
pub const NF_CT_EXPECT_INACTIVE: i32 = 0x2;
pub const NF_CT_EXPECT_USERSPACE: i32 = 0x4;
#[cfg(feature = "kernel")]
pub const NF_CT_EXPECT_DEAD: i32 = 0x8;
#[cfg(feature = "kernel")]
pub const NF_CT_EXPECT_MASK: i32 = NF_CT_EXPECT_PERMANENT | NF_CT_EXPECT_INACTIVE |
    NF_CT_EXPECT_USERSPACE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
