/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file describes the netlink messages (i.e. 'protocol packets'),
 * and not any kind of function definitions. It is shared between kernel and
 * userspace. Don't put kernel specific stuff in here.
 *
 * C dependencies: <linux/types.h>, <linux/netfilter/nfnetlink.h>
 */

#[repr(u32)]
pub enum nfulnl_msg_types {
    NFULNL_MSG_PACKET, // packet from kernel to userspace
    NFULNL_MSG_CONFIG, // connect to a particular queue

    NFULNL_MSG_MAX,
}

#[repr(C)]
pub struct nfulnl_msg_packet_hdr {
    pub hw_protocol: u16, // __be16, hw protocol (network order)
    pub hook: u8,         // netfilter hook
    pub _pad: u8,
}

#[repr(C)]
pub struct nfulnl_msg_packet_hw {
    pub hw_addrlen: u16, // __be16
    pub _pad: u16,
    pub hw_addr: [u8; 8],
}

#[repr(C)]
pub struct nfulnl_msg_packet_timestamp {
    pub sec: u64,  // __aligned_be64
    pub usec: u64, // __aligned_be64
}

#[repr(u32)]
pub enum nfulnl_vlan_attr {
    NFULA_VLAN_UNSPEC,
    NFULA_VLAN_PROTO, // __be16 skb vlan_proto
    NFULA_VLAN_TCI,   // __be16 skb htons(vlan_tci)
    __NFULA_VLAN_MAX,
}

pub const NFULA_VLAN_MAX: u32 = __NFULA_VLAN_MAX as u32 + 1;

#[repr(u32)]
pub enum nfulnl_attr_type {
    NFULA_UNSPEC,
    NFULA_PACKET_HDR,
    NFULA_MARK,            // __u32 nfmark
    NFULA_TIMESTAMP,       // nfulnl_msg_packet_timestamp
    NFULA_IFINDEX_INDEV,   // __u32 ifindex
    NFULA_IFINDEX_OUTDEV,  // __u32 ifindex
    NFULA_IFINDEX_PHYSINDEV,  // __u32 ifindex
    NFULA_IFINDEX_PHYSOUTDEV, // __u32 ifindex
    NFULA_HWADDR,          // nfulnl_msg_packet_hw
    NFULA_PAYLOAD,         // opaque data payload
    NFULA_PREFIX,          // string prefix
    NFULA_UID,             // user id of socket
    NFULA_SEQ,             // instance-local sequence number
    NFULA_SEQ_GLOBAL,      // global sequence number
    NFULA_GID,             // group id of socket
    NFULA_HWTYPE,          // hardware type
    NFULA_HWHEADER,        // hardware header
    NFULA_HWLEN,           // hardware header length
    NFULA_CT,              // nfnetlink_conntrack.h
    NFULA_CT_INFO,         // enum ip_conntrack_info
    NFULA_VLAN,             // nested attribute: packet vlan info
    NFULA_L2HDR,            // full L2 header

    __NFULA_MAX,
}

pub const NFULA_MAX: u32 = __NFULA_MAX as u32 - 1;

#[repr(u32)]
pub enum nfulnl_msg_config_cmds {
    NFULNL_CFG_CMD_NONE,
    NFULNL_CFG_CMD_BIND,
    NFULNL_CFG_CMD_UNBIND,
    NFULNL_CFG_CMD_PF_BIND,
    NFULNL_CFG_CMD_PF_UNBIND,
}

#[repr(C, packed)]
pub struct nfulnl_msg_config_cmd {
    pub command: u8, // nfulnl_msg_config_cmds
}

#[repr(C, packed)]
pub struct nfulnl_msg_config_mode {
    pub copy_range: u32, // __be32
    pub copy_mode: u8,
    pub _pad: u8,
}

#[repr(u32)]
pub enum nfulnl_attr_config {
    NFULA_CFG_UNSPEC,
    NFULA_CFG_CMD,       // nfulnl_msg_config_cmd
    NFULA_CFG_MODE,      // nfulnl_msg_config_mode
    NFULA_CFG_NLBUFSIZ,  // __u32 buffer size
    NFULA_CFG_TIMEOUT,   // __u32 in 1/100 s
    NFULA_CFG_QTHRESH,   // __u32
    NFULA_CFG_FLAGS,     // __u16
    __NFULA_CFG_MAX,
}

pub const NFULA_CFG_MAX: u32 = __NFULA_CFG_MAX as u32 - 1;

pub const NFULNL_COPY_NONE: u32 = 0x00;
pub const NFULNL_COPY_META: u32 = 0x01;
pub const NFULNL_COPY_PACKET: u32 = 0x02;
/* 0xff is reserved, don't use it for new copy modes. */

pub const NFULNL_CFG_F_SEQ: u32 = 0x0001;
pub const NFULNL_CFG_F_SEQ_GLOBAL: u32 = 0x0002;
pub const NFULNL_CFG_F_CONNTRACK: u32 = 0x0004;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
