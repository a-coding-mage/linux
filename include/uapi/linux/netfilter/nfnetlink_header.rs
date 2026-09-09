/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translation of <linux/netfilter/nfnetlink.h>.
// The original includes <linux/types.h> and
// <linux/netfilter/nfnetlink_compat.h>; their externally supplied symbols are
// referenced as needed below.

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnetlink_groups {
    NFNLGRP_NONE = 0,
    NFNLGRP_CONNTRACK_NEW,
    NFNLGRP_CONNTRACK_UPDATE,
    NFNLGRP_CONNTRACK_DESTROY,
    NFNLGRP_CONNTRACK_EXP_NEW,
    NFNLGRP_CONNTRACK_EXP_UPDATE,
    NFNLGRP_CONNTRACK_EXP_DESTROY,
    NFNLGRP_NFTABLES,
    NFNLGRP_ACCT_QUOTA,
    NFNLGRP_NFTRACE,
    __NFNLGRP_MAX,
}

pub const NFNLGRP_MAX: i32 = nfnetlink_groups::__NFNLGRP_MAX as i32 - 1;

/* General form of address family dependent message. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfgenmsg {
    pub nfgen_family: u8, /* AF_xxx */
    pub version: u8,      /* nfnetlink version */
    pub res_id: u16,      /* resource id, big-endian */
}

pub const NFNETLINK_V0: i32 = 0;

/* netfilter netlink message types are split in two pieces:
 * 8 bit subsystem, 8bit operation.
 */
#[macro_export]
macro_rules! NFNL_SUBSYS_ID {
    ($x:expr) => {
        (($x & 0xff00) >> 8)
    };
}

#[macro_export]
macro_rules! NFNL_MSG_TYPE {
    ($x:expr) => {
        ($x & 0x00ff)
    };
}

/* No enum here, otherwise __stringify() trick of MODULE_ALIAS_NFNL_SUBSYS()
 * won't work anymore */
pub const NFNL_SUBSYS_NONE: i32 = 0;
pub const NFNL_SUBSYS_CTNETLINK: i32 = 1;
pub const NFNL_SUBSYS_CTNETLINK_EXP: i32 = 2;
pub const NFNL_SUBSYS_QUEUE: i32 = 3;
pub const NFNL_SUBSYS_ULOG: i32 = 4;
pub const NFNL_SUBSYS_OSF: i32 = 5;
pub const NFNL_SUBSYS_IPSET: i32 = 6;
pub const NFNL_SUBSYS_ACCT: i32 = 7;
pub const NFNL_SUBSYS_CTNETLINK_TIMEOUT: i32 = 8;
pub const NFNL_SUBSYS_CTHELPER: i32 = 9;
pub const NFNL_SUBSYS_NFTABLES: i32 = 10;
pub const NFNL_SUBSYS_NFT_COMPAT: i32 = 11;
pub const NFNL_SUBSYS_HOOK: i32 = 12;
pub const NFNL_SUBSYS_COUNT: i32 = 13;

/* Reserved control nfnetlink messages */
pub const NFNL_MSG_BATCH_BEGIN: i32 = NLMSG_MIN_TYPE;
pub const NFNL_MSG_BATCH_END: i32 = NLMSG_MIN_TYPE + 1;

/**
 * enum nfnl_batch_attributes - nfnetlink batch netlink attributes
 *
 * @NFNL_BATCH_GENID: generation ID for this changeset (NLA_U32)
 */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_batch_attributes {
    NFNL_BATCH_UNSPEC = 0,
    NFNL_BATCH_GENID,
    __NFNL_BATCH_MAX,
}

pub const NFNL_BATCH_MAX: i32 = nfnl_batch_attributes::__NFNL_BATCH_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
