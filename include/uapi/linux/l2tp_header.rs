/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * L2TP-over-IP socket for L2TPv3.
 *
 * Author: James Chapman <jchapman@katalix.com>
 */

// Translated from the Linux UAPI header. The included kernel types are
// expected to be supplied by the surrounding bindings.

pub const __SOCK_SIZE__: usize = 16;

#[repr(C)]
pub struct sockaddr_l2tpip {
    pub l2tp_family: __kernel_sa_family_t,
    pub l2tp_unused: __be16,
    pub l2tp_addr: in_addr,
    pub l2tp_conn_id: __u32,
    pub __pad: [u8; __SOCK_SIZE__ - core::mem::size_of::<__kernel_sa_family_t>()
        - core::mem::size_of::<__be16>() - core::mem::size_of::<in_addr>()
        - core::mem::size_of::<__u32>()],
}

#[repr(C)]
pub struct sockaddr_l2tpip6 {
    pub l2tp_family: __kernel_sa_family_t,
    pub l2tp_unused: __be16,
    pub l2tp_flowinfo: __be32,
    pub l2tp_addr: in6_addr,
    pub l2tp_scope_id: __u32,
    pub l2tp_conn_id: __u32,
}

pub const L2TP_CMD_NOOP: i32 = 0;
pub const L2TP_CMD_TUNNEL_CREATE: i32 = 1;
pub const L2TP_CMD_TUNNEL_DELETE: i32 = 2;
pub const L2TP_CMD_TUNNEL_MODIFY: i32 = 3;
pub const L2TP_CMD_TUNNEL_GET: i32 = 4;
pub const L2TP_CMD_SESSION_CREATE: i32 = 5;
pub const L2TP_CMD_SESSION_DELETE: i32 = 6;
pub const L2TP_CMD_SESSION_MODIFY: i32 = 7;
pub const L2TP_CMD_SESSION_GET: i32 = 8;
pub const __L2TP_CMD_MAX: i32 = 9;
pub const L2TP_CMD_MAX: i32 = __L2TP_CMD_MAX - 1;

pub const L2TP_ATTR_NONE: i32 = 0;
pub const L2TP_ATTR_PW_TYPE: i32 = 1;
pub const L2TP_ATTR_ENCAP_TYPE: i32 = 2;
pub const L2TP_ATTR_OFFSET: i32 = 3;
pub const L2TP_ATTR_DATA_SEQ: i32 = 4;
pub const L2TP_ATTR_L2SPEC_TYPE: i32 = 5;
pub const L2TP_ATTR_L2SPEC_LEN: i32 = 6;
pub const L2TP_ATTR_PROTO_VERSION: i32 = 7;
pub const L2TP_ATTR_IFNAME: i32 = 8;
pub const L2TP_ATTR_CONN_ID: i32 = 9;
pub const L2TP_ATTR_PEER_CONN_ID: i32 = 10;
pub const L2TP_ATTR_SESSION_ID: i32 = 11;
pub const L2TP_ATTR_PEER_SESSION_ID: i32 = 12;
pub const L2TP_ATTR_UDP_CSUM: i32 = 13;
pub const L2TP_ATTR_VLAN_ID: i32 = 14;
pub const L2TP_ATTR_COOKIE: i32 = 15;
pub const L2TP_ATTR_PEER_COOKIE: i32 = 16;
pub const L2TP_ATTR_DEBUG: i32 = 17;
pub const L2TP_ATTR_RECV_SEQ: i32 = 18;
pub const L2TP_ATTR_SEND_SEQ: i32 = 19;
pub const L2TP_ATTR_LNS_MODE: i32 = 20;
pub const L2TP_ATTR_USING_IPSEC: i32 = 21;
pub const L2TP_ATTR_RECV_TIMEOUT: i32 = 22;
pub const L2TP_ATTR_FD: i32 = 23;
pub const L2TP_ATTR_IP_SADDR: i32 = 24;
pub const L2TP_ATTR_IP_DADDR: i32 = 25;
pub const L2TP_ATTR_UDP_SPORT: i32 = 26;
pub const L2TP_ATTR_UDP_DPORT: i32 = 27;
pub const L2TP_ATTR_MTU: i32 = 28;
pub const L2TP_ATTR_MRU: i32 = 29;
pub const L2TP_ATTR_STATS: i32 = 30;
pub const L2TP_ATTR_IP6_SADDR: i32 = 31;
pub const L2TP_ATTR_IP6_DADDR: i32 = 32;
pub const L2TP_ATTR_UDP_ZERO_CSUM6_TX: i32 = 33;
pub const L2TP_ATTR_UDP_ZERO_CSUM6_RX: i32 = 34;
pub const L2TP_ATTR_PAD: i32 = 35;
pub const __L2TP_ATTR_MAX: i32 = 36;
pub const L2TP_ATTR_MAX: i32 = __L2TP_ATTR_MAX - 1;

pub const L2TP_ATTR_STATS_NONE: i32 = 0;
pub const L2TP_ATTR_TX_PACKETS: i32 = 1;
pub const L2TP_ATTR_TX_BYTES: i32 = 2;
pub const L2TP_ATTR_TX_ERRORS: i32 = 3;
pub const L2TP_ATTR_RX_PACKETS: i32 = 4;
pub const L2TP_ATTR_RX_BYTES: i32 = 5;
pub const L2TP_ATTR_RX_SEQ_DISCARDS: i32 = 6;
pub const L2TP_ATTR_RX_OOS_PACKETS: i32 = 7;
pub const L2TP_ATTR_RX_ERRORS: i32 = 8;
pub const L2TP_ATTR_STATS_PAD: i32 = 9;
pub const L2TP_ATTR_RX_COOKIE_DISCARDS: i32 = 10;
pub const L2TP_ATTR_RX_INVALID: i32 = 11;
pub const __L2TP_ATTR_STATS_MAX: i32 = 12;
pub const L2TP_ATTR_STATS_MAX: i32 = __L2TP_ATTR_STATS_MAX - 1;

#[repr(i32)]
pub enum l2tp_pwtype {
    L2TP_PWTYPE_NONE = 0x0000,
    L2TP_PWTYPE_ETH_VLAN = 0x0004,
    L2TP_PWTYPE_ETH = 0x0005,
    L2TP_PWTYPE_PPP = 0x0007,
    L2TP_PWTYPE_PPP_AC = 0x0008,
    L2TP_PWTYPE_IP = 0x000b,
    __L2TP_PWTYPE_MAX,
}

#[repr(i32)]
pub enum l2tp_l2spec_type {
    L2TP_L2SPECTYPE_NONE = 0,
    L2TP_L2SPECTYPE_DEFAULT = 1,
}

#[repr(i32)]
pub enum l2tp_encap_type {
    L2TP_ENCAPTYPE_UDP = 0,
    L2TP_ENCAPTYPE_IP = 1,
}

#[repr(i32)]
pub enum l2tp_seqmode {
    L2TP_SEQ_NONE = 0,
    L2TP_SEQ_IP = 1,
    L2TP_SEQ_ALL = 2,
}

#[repr(i32)]
pub enum l2tp_debug_flags {
    L2TP_MSG_DEBUG = 1 << 0,
    L2TP_MSG_CONTROL = 1 << 1,
    L2TP_MSG_SEQ = 1 << 2,
    L2TP_MSG_DATA = 1 << 3,
}

pub const L2TP_GENL_NAME: &str = "l2tp";
pub const L2TP_GENL_VERSION: i32 = 0x1;
pub const L2TP_GENL_MCGROUP: &str = "l2tp";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
