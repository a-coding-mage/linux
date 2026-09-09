/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux headers:
// linux/types.h and linux/netfilter/nfnetlink.h

#[repr(i32)]
pub enum nfqnl_msg_types {
    NFQNL_MSG_PACKET,       /* packet from kernel to userspace */
    NFQNL_MSG_VERDICT,      /* verdict from userspace to kernel */
    NFQNL_MSG_CONFIG,       /* connect to a particular queue */
    NFQNL_MSG_VERDICT_BATCH, /* batchv from userspace to kernel */

    NFQNL_MSG_MAX,
}

#[repr(C, packed)]
pub struct nfqnl_msg_packet_hdr {
    pub packet_id: __be32,   /* unique ID of packet in queue */
    pub hw_protocol: __be16, /* hw protocol (network order) */
    pub hook: __u8,          /* netfilter hook */
}

#[repr(C)]
pub struct nfqnl_msg_packet_hw {
    pub hw_addrlen: __be16,
    pub _pad: __u16,
    pub hw_addr: [__u8; 8],
}

#[repr(C)]
pub struct nfqnl_msg_packet_timestamp {
    pub sec: __aligned_be64,
    pub usec: __aligned_be64,
}

#[repr(i32)]
pub enum nfqnl_vlan_attr {
    NFQA_VLAN_UNSPEC,
    NFQA_VLAN_PROTO, /* __be16 skb vlan_proto */
    NFQA_VLAN_TCI,   /* __be16 skb htons(vlan_tci) */
    __NFQA_VLAN_MAX,
}

pub const NFQA_VLAN_MAX: i32 = __NFQA_VLAN_MAX as i32 - 1;

#[repr(i32)]
pub enum nfqnl_attr_type {
    NFQA_UNSPEC,
    NFQA_PACKET_HDR,
    NFQA_VERDICT_HDR,      /* nfqnl_msg_verdict_hrd */
    NFQA_MARK,             /* __u32 nfmark */
    NFQA_TIMESTAMP,        /* nfqnl_msg_packet_timestamp */
    NFQA_IFINDEX_INDEV,    /* __u32 ifindex */
    NFQA_IFINDEX_OUTDEV,   /* __u32 ifindex */
    NFQA_IFINDEX_PHYSINDEV, /* __u32 ifindex */
    NFQA_IFINDEX_PHYSOUTDEV, /* __u32 ifindex */
    NFQA_HWADDR,           /* nfqnl_msg_packet_hw */
    NFQA_PAYLOAD,          /* opaque data payload */
    NFQA_CT,               /* nfnetlink_conntrack.h */
    NFQA_CT_INFO,          /* enum ip_conntrack_info */
    NFQA_CAP_LEN,          /* __u32 length of captured packet */
    NFQA_SKB_INFO,         /* __u32 skb meta information */
    NFQA_EXP,              /* nfnetlink_conntrack.h */
    NFQA_UID,              /* __u32 sk uid */
    NFQA_GID,              /* __u32 sk gid */
    NFQA_SECCTX,           /* security context string */
    NFQA_VLAN,              /* nested attribute: packet vlan info */
    NFQA_L2HDR,             /* full L2 header */
    NFQA_PRIORITY,          /* skb->priority */
    NFQA_CGROUP_CLASSID,    /* __u32 cgroup classid */

    __NFQA_MAX,
}

pub const NFQA_MAX: i32 = __NFQA_MAX as i32 - 1;

#[repr(C)]
pub struct nfqnl_msg_verdict_hdr {
    pub verdict: __be32,
    pub id: __be32,
}

#[repr(i32)]
pub enum nfqnl_msg_config_cmds {
    NFQNL_CFG_CMD_NONE,
    NFQNL_CFG_CMD_BIND,
    NFQNL_CFG_CMD_UNBIND,
    NFQNL_CFG_CMD_PF_BIND,
    NFQNL_CFG_CMD_PF_UNBIND,
}

#[repr(C)]
pub struct nfqnl_msg_config_cmd {
    pub command: __u8, /* nfqnl_msg_config_cmds */
    pub _pad: __u8,
    pub pf: __be16, /* AF_xxx for PF_[UN]BIND */
}

#[repr(i32)]
pub enum nfqnl_config_mode {
    NFQNL_COPY_NONE,
    NFQNL_COPY_META,
    NFQNL_COPY_PACKET,
}

#[repr(C, packed)]
pub struct nfqnl_msg_config_params {
    pub copy_range: __be32,
    pub copy_mode: __u8, /* enum nfqnl_config_mode */
}

#[repr(i32)]
pub enum nfqnl_attr_config {
    NFQA_CFG_UNSPEC,
    NFQA_CFG_CMD,         /* nfqnl_msg_config_cmd */
    NFQA_CFG_PARAMS,      /* nfqnl_msg_config_params */
    NFQA_CFG_QUEUE_MAXLEN, /* __u32 */
    NFQA_CFG_MASK,        /* identify which flags to change */
    NFQA_CFG_FLAGS,       /* value of these flags (__u32) */
    __NFQA_CFG_MAX,
}

pub const NFQA_CFG_MAX: i32 = __NFQA_CFG_MAX as i32 - 1;

/* Flags for NFQA_CFG_FLAGS */
pub const NFQA_CFG_F_FAIL_OPEN: i32 = 1 << 0;
pub const NFQA_CFG_F_CONNTRACK: i32 = 1 << 1;
pub const NFQA_CFG_F_GSO: i32 = 1 << 2;
pub const NFQA_CFG_F_UID_GID: i32 = 1 << 3;
pub const NFQA_CFG_F_SECCTX: i32 = 1 << 4;
pub const NFQA_CFG_F_MAX: i32 = 1 << 5;

/* flags for NFQA_SKB_INFO */
/* packet appears to have wrong checksums, but they are ok */
pub const NFQA_SKB_CSUMNOTREADY: i32 = 1 << 0;
/* packet is GSO (i.e., exceeds device mtu) */
pub const NFQA_SKB_GSO: i32 = 1 << 1;
/* csum not validated (incoming device doesn't support hw checksum, etc.) */
pub const NFQA_SKB_CSUM_NOTVERIFIED: i32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
