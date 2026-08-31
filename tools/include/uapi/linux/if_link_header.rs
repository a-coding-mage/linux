/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of include/uapi/linux/if_link.h. */
/* C dependencies removed from executable Rust: linux/types.h, linux/netlink.h. */

/* This struct should be in sync with struct rtnl_link_stats64 */
#[repr(C)]
pub struct rtnl_link_stats {
    pub rx_packets: __u32,
    pub tx_packets: __u32,
    pub rx_bytes: __u32,
    pub tx_bytes: __u32,
    pub rx_errors: __u32,
    pub tx_errors: __u32,
    pub rx_dropped: __u32,
    pub tx_dropped: __u32,
    pub multicast: __u32,
    pub collisions: __u32,
    /* detailed rx_errors: */
    pub rx_length_errors: __u32,
    pub rx_over_errors: __u32,
    pub rx_crc_errors: __u32,
    pub rx_frame_errors: __u32,
    pub rx_fifo_errors: __u32,
    pub rx_missed_errors: __u32,
    /* detailed tx_errors */
    pub tx_aborted_errors: __u32,
    pub tx_carrier_errors: __u32,
    pub tx_fifo_errors: __u32,
    pub tx_heartbeat_errors: __u32,
    pub tx_window_errors: __u32,
    /* for cslip etc */
    pub rx_compressed: __u32,
    pub tx_compressed: __u32,
    pub rx_nohandler: __u32,
}

/*
 * struct rtnl_link_stats64 - The main device statistics structure.
 * Field comments and semantics are as documented in the C UAPI header.
 */
#[repr(C)]
pub struct rtnl_link_stats64 {
    pub rx_packets: __u64,
    pub tx_packets: __u64,
    pub rx_bytes: __u64,
    pub tx_bytes: __u64,
    pub rx_errors: __u64,
    pub tx_errors: __u64,
    pub rx_dropped: __u64,
    pub tx_dropped: __u64,
    pub multicast: __u64,
    pub collisions: __u64,
    /* detailed rx_errors: */
    pub rx_length_errors: __u64,
    pub rx_over_errors: __u64,
    pub rx_crc_errors: __u64,
    pub rx_frame_errors: __u64,
    pub rx_fifo_errors: __u64,
    pub rx_missed_errors: __u64,
    /* detailed tx_errors */
    pub tx_aborted_errors: __u64,
    pub tx_carrier_errors: __u64,
    pub tx_fifo_errors: __u64,
    pub tx_heartbeat_errors: __u64,
    pub tx_window_errors: __u64,
    /* for cslip etc */
    pub rx_compressed: __u64,
    pub tx_compressed: __u64,
    pub rx_nohandler: __u64,
    pub rx_otherhost_dropped: __u64,
}

/* Subset of link stats useful for in-HW collection. Meaning of the fields is as for struct rtnl_link_stats64. */
#[repr(C)]
pub struct rtnl_hw_stats64 {
    pub rx_packets: __u64,
    pub tx_packets: __u64,
    pub rx_bytes: __u64,
    pub tx_bytes: __u64,
    pub rx_errors: __u64,
    pub tx_errors: __u64,
    pub rx_dropped: __u64,
    pub tx_dropped: __u64,
    pub multicast: __u64,
}

/* The struct should be in sync with struct ifmap */
#[repr(C)]
pub struct rtnl_link_ifmap {
    pub mem_start: __u64,
    pub mem_end: __u64,
    pub base_addr: __u64,
    pub irq: __u16,
    pub dma: __u8,
    pub port: __u8,
}

/* IFLA_AF_SPEC contains nested attributes for address family specific attributes. */
pub const IFLA_UNSPEC: u32 = 0;
pub const IFLA_ADDRESS: u32 = 1;
pub const IFLA_BROADCAST: u32 = 2;
pub const IFLA_IFNAME: u32 = 3;
pub const IFLA_MTU: u32 = 4;
pub const IFLA_LINK: u32 = 5;
pub const IFLA_QDISC: u32 = 6;
pub const IFLA_STATS: u32 = 7;
pub const IFLA_COST: u32 = 8;
pub const IFLA_PRIORITY: u32 = 9;
pub const IFLA_MASTER: u32 = 10;
pub const IFLA_WIRELESS: u32 = 11; /* Wireless Extension event - see wireless.h */
pub const IFLA_PROTINFO: u32 = 12; /* Protocol specific information for a link */
pub const IFLA_TXQLEN: u32 = 13;
pub const IFLA_MAP: u32 = 14;
pub const IFLA_WEIGHT: u32 = 15;
pub const IFLA_OPERSTATE: u32 = 16;
pub const IFLA_LINKMODE: u32 = 17;
pub const IFLA_LINKINFO: u32 = 18;
pub const IFLA_NET_NS_PID: u32 = 19;
pub const IFLA_IFALIAS: u32 = 20;
pub const IFLA_NUM_VF: u32 = 21; /* Number of VFs if device is SR-IOV PF */
pub const IFLA_VFINFO_LIST: u32 = 22;
pub const IFLA_STATS64: u32 = 23;
pub const IFLA_VF_PORTS: u32 = 24;
pub const IFLA_PORT_SELF: u32 = 25;
pub const IFLA_AF_SPEC: u32 = 26;
pub const IFLA_GROUP: u32 = 27; /* Group the device belongs to */
pub const IFLA_NET_NS_FD: u32 = 28;
pub const IFLA_EXT_MASK: u32 = 29; /* Extended info mask, VFs, etc */
pub const IFLA_PROMISCUITY: u32 = 30; /* Promiscuity count: > 0 means acts PROMISC */
pub const IFLA_NUM_TX_QUEUES: u32 = 31;
pub const IFLA_NUM_RX_QUEUES: u32 = 32;
pub const IFLA_CARRIER: u32 = 33;
pub const IFLA_PHYS_PORT_ID: u32 = 34;
pub const IFLA_CARRIER_CHANGES: u32 = 35;
pub const IFLA_PHYS_SWITCH_ID: u32 = 36;
pub const IFLA_LINK_NETNSID: u32 = 37;
pub const IFLA_PHYS_PORT_NAME: u32 = 38;
pub const IFLA_PROTO_DOWN: u32 = 39;
pub const IFLA_GSO_MAX_SEGS: u32 = 40;
pub const IFLA_GSO_MAX_SIZE: u32 = 41;
pub const IFLA_PAD: u32 = 42;
pub const IFLA_XDP: u32 = 43;
pub const IFLA_EVENT: u32 = 44;
pub const IFLA_NEW_NETNSID: u32 = 45;
pub const IFLA_IF_NETNSID: u32 = 46;
pub const IFLA_TARGET_NETNSID: u32 = IFLA_IF_NETNSID; /* new alias */
pub const IFLA_CARRIER_UP_COUNT: u32 = 47;
pub const IFLA_CARRIER_DOWN_COUNT: u32 = 48;
pub const IFLA_NEW_IFINDEX: u32 = 49;
pub const IFLA_MIN_MTU: u32 = 50;
pub const IFLA_MAX_MTU: u32 = 51;
pub const IFLA_PROP_LIST: u32 = 52;
pub const IFLA_ALT_IFNAME: u32 = 53; /* Alternative ifname */
pub const IFLA_PERM_ADDRESS: u32 = 54;
pub const IFLA_PROTO_DOWN_REASON: u32 = 55;
pub const IFLA_PARENT_DEV_NAME: u32 = 56;
pub const IFLA_PARENT_DEV_BUS_NAME: u32 = 57;
pub const IFLA_GRO_MAX_SIZE: u32 = 58;
pub const IFLA_TSO_MAX_SIZE: u32 = 59;
pub const IFLA_TSO_MAX_SEGS: u32 = 60;
pub const IFLA_ALLMULTI: u32 = 61; /* Allmulti count: > 0 means acts ALLMULTI */
pub const IFLA_DEVLINK_PORT: u32 = 62;
pub const IFLA_GSO_IPV4_MAX_SIZE: u32 = 63;
pub const IFLA_GRO_IPV4_MAX_SIZE: u32 = 64;
pub const IFLA_DPLL_PIN: u32 = 65;
pub const IFLA_MAX_PACING_OFFLOAD_HORIZON: u32 = 66;
pub const __IFLA_MAX: u32 = 67;
pub const IFLA_MAX: u32 = __IFLA_MAX - 1;

pub const IFLA_PROTO_DOWN_REASON_UNSPEC: u32 = 0;
pub const IFLA_PROTO_DOWN_REASON_MASK: u32 = 1; /* u32, mask for reason bits */
pub const IFLA_PROTO_DOWN_REASON_VALUE: u32 = 2; /* u32, reason bit value */
pub const __IFLA_PROTO_DOWN_REASON_CNT: u32 = 3;
pub const IFLA_PROTO_DOWN_REASON_MAX: u32 = __IFLA_PROTO_DOWN_REASON_CNT - 1;

/* backwards compatibility for userspace; original C macros:
 * IFLA_RTA(r) and IFLA_PAYLOAD(n) depend on rtattr, ifinfomsg, NLMSG_ALIGN and NLMSG_PAYLOAD.
 */

pub const IFLA_INET_UNSPEC: u32 = 0;
pub const IFLA_INET_CONF: u32 = 1;
pub const __IFLA_INET_MAX: u32 = 2;
pub const IFLA_INET_MAX: u32 = __IFLA_INET_MAX - 1;

/* ifi_flags, IFF_* flags, and IFLA_LINK comments preserved in the source header. */

/* Subtype attributes for IFLA_PROTINFO */
pub const IFLA_INET6_UNSPEC: u32 = 0;
pub const IFLA_INET6_FLAGS: u32 = 1; /* link flags */
pub const IFLA_INET6_CONF: u32 = 2; /* sysctl parameters */
pub const IFLA_INET6_STATS: u32 = 3; /* statistics */
pub const IFLA_INET6_MCAST: u32 = 4; /* MC things. What of them? */
pub const IFLA_INET6_CACHEINFO: u32 = 5; /* time values and max reasm size */
pub const IFLA_INET6_ICMP6STATS: u32 = 6; /* statistics (icmpv6) */
pub const IFLA_INET6_TOKEN: u32 = 7; /* device token */
pub const IFLA_INET6_ADDR_GEN_MODE: u32 = 8; /* implicit address generator mode */
pub const IFLA_INET6_RA_MTU: u32 = 9; /* mtu carried in the RA message */
pub const __IFLA_INET6_MAX: u32 = 10;
pub const IFLA_INET6_MAX: u32 = __IFLA_INET6_MAX - 1;

#[repr(C)]
pub enum in6_addr_gen_mode {
    IN6_ADDR_GEN_MODE_EUI64 = 0,
    IN6_ADDR_GEN_MODE_NONE = 1,
    IN6_ADDR_GEN_MODE_STABLE_PRIVACY = 2,
    IN6_ADDR_GEN_MODE_RANDOM = 3,
}

/* Bridge section. Timer values are in clock_t format: seconds multiplied by USER_HZ. */
pub const IFLA_BR_UNSPEC: u32 = 0;
pub const IFLA_BR_FORWARD_DELAY: u32 = 1;
pub const IFLA_BR_HELLO_TIME: u32 = 2;
pub const IFLA_BR_MAX_AGE: u32 = 3;
pub const IFLA_BR_AGEING_TIME: u32 = 4;
pub const IFLA_BR_STP_STATE: u32 = 5;
pub const IFLA_BR_PRIORITY: u32 = 6;
pub const IFLA_BR_VLAN_FILTERING: u32 = 7;
pub const IFLA_BR_VLAN_PROTOCOL: u32 = 8;
pub const IFLA_BR_GROUP_FWD_MASK: u32 = 9;
pub const IFLA_BR_ROOT_ID: u32 = 10;
pub const IFLA_BR_BRIDGE_ID: u32 = 11;
pub const IFLA_BR_ROOT_PORT: u32 = 12;
pub const IFLA_BR_ROOT_PATH_COST: u32 = 13;
pub const IFLA_BR_TOPOLOGY_CHANGE: u32 = 14;
pub const IFLA_BR_TOPOLOGY_CHANGE_DETECTED: u32 = 15;
pub const IFLA_BR_HELLO_TIMER: u32 = 16;
pub const IFLA_BR_TCN_TIMER: u32 = 17;
pub const IFLA_BR_TOPOLOGY_CHANGE_TIMER: u32 = 18;
pub const IFLA_BR_GC_TIMER: u32 = 19;
pub const IFLA_BR_GROUP_ADDR: u32 = 20;
pub const IFLA_BR_FDB_FLUSH: u32 = 21;
pub const IFLA_BR_MCAST_ROUTER: u32 = 22;
pub const IFLA_BR_MCAST_SNOOPING: u32 = 23;
pub const IFLA_BR_MCAST_QUERY_USE_IFADDR: u32 = 24;
pub const IFLA_BR_MCAST_QUERIER: u32 = 25;
pub const IFLA_BR_MCAST_HASH_ELASTICITY: u32 = 26;
pub const IFLA_BR_MCAST_HASH_MAX: u32 = 27;
pub const IFLA_BR_MCAST_LAST_MEMBER_CNT: u32 = 28;
pub const IFLA_BR_MCAST_STARTUP_QUERY_CNT: u32 = 29;
pub const IFLA_BR_MCAST_LAST_MEMBER_INTVL: u32 = 30;
pub const IFLA_BR_MCAST_MEMBERSHIP_INTVL: u32 = 31;
pub const IFLA_BR_MCAST_QUERIER_INTVL: u32 = 32;
pub const IFLA_BR_MCAST_QUERY_INTVL: u32 = 33;
pub const IFLA_BR_MCAST_QUERY_RESPONSE_INTVL: u32 = 34;
pub const IFLA_BR_MCAST_STARTUP_QUERY_INTVL: u32 = 35;
pub const IFLA_BR_NF_CALL_IPTABLES: u32 = 36;
pub const IFLA_BR_NF_CALL_IP6TABLES: u32 = 37;
pub const IFLA_BR_NF_CALL_ARPTABLES: u32 = 38;
pub const IFLA_BR_VLAN_DEFAULT_PVID: u32 = 39;
pub const IFLA_BR_PAD: u32 = 40;
pub const IFLA_BR_VLAN_STATS_ENABLED: u32 = 41;
pub const IFLA_BR_MCAST_STATS_ENABLED: u32 = 42;
pub const IFLA_BR_MCAST_IGMP_VERSION: u32 = 43;
pub const IFLA_BR_MCAST_MLD_VERSION: u32 = 44;
pub const IFLA_BR_VLAN_STATS_PER_PORT: u32 = 45;
pub const IFLA_BR_MULTI_BOOLOPT: u32 = 46;
pub const IFLA_BR_MCAST_QUERIER_STATE: u32 = 47;
pub const IFLA_BR_FDB_N_LEARNED: u32 = 48;
pub const IFLA_BR_FDB_MAX_LEARNED: u32 = 49;
pub const __IFLA_BR_MAX: u32 = 50;
pub const IFLA_BR_MAX: u32 = __IFLA_BR_MAX - 1;

#[repr(C)]
pub struct ifla_bridge_id {
    pub prio: [__u8; 2],
    pub addr: [__u8; 6], /* ETH_ALEN */
}

/* Bridge mode enum definition. */
pub const BRIDGE_MODE_UNSPEC: u32 = 0;
pub const BRIDGE_MODE_HAIRPIN: u32 = 1;

/* Bridge port enum definition. */
pub const IFLA_BRPORT_UNSPEC: u32 = 0;
pub const IFLA_BRPORT_STATE: u32 = 1; /* Spanning tree state */
pub const IFLA_BRPORT_PRIORITY: u32 = 2; /* priority */
pub const IFLA_BRPORT_COST: u32 = 3; /* cost */
pub const IFLA_BRPORT_MODE: u32 = 4; /* mode (hairpin) */
pub const IFLA_BRPORT_GUARD: u32 = 5; /* bpdu guard */
pub const IFLA_BRPORT_PROTECT: u32 = 6; /* root port protection */
pub const IFLA_BRPORT_FAST_LEAVE: u32 = 7; /* multicast fast leave */
pub const IFLA_BRPORT_LEARNING: u32 = 8; /* mac learning */
pub const IFLA_BRPORT_UNICAST_FLOOD: u32 = 9; /* flood unicast traffic */
pub const IFLA_BRPORT_PROXYARP: u32 = 10; /* proxy ARP */
pub const IFLA_BRPORT_LEARNING_SYNC: u32 = 11; /* mac learning sync from device */
pub const IFLA_BRPORT_PROXYARP_WIFI: u32 = 12; /* proxy ARP for Wi-Fi */
pub const IFLA_BRPORT_ROOT_ID: u32 = 13; /* designated root */
pub const IFLA_BRPORT_BRIDGE_ID: u32 = 14; /* designated bridge */
pub const IFLA_BRPORT_DESIGNATED_PORT: u32 = 15;
pub const IFLA_BRPORT_DESIGNATED_COST: u32 = 16;
pub const IFLA_BRPORT_ID: u32 = 17;
pub const IFLA_BRPORT_NO: u32 = 18;
pub const IFLA_BRPORT_TOPOLOGY_CHANGE_ACK: u32 = 19;
pub const IFLA_BRPORT_CONFIG_PENDING: u32 = 20;
pub const IFLA_BRPORT_MESSAGE_AGE_TIMER: u32 = 21;
pub const IFLA_BRPORT_FORWARD_DELAY_TIMER: u32 = 22;
pub const IFLA_BRPORT_HOLD_TIMER: u32 = 23;
pub const IFLA_BRPORT_FLUSH: u32 = 24;
pub const IFLA_BRPORT_MULTICAST_ROUTER: u32 = 25;
pub const IFLA_BRPORT_PAD: u32 = 26;
pub const IFLA_BRPORT_MCAST_FLOOD: u32 = 27;
pub const IFLA_BRPORT_MCAST_TO_UCAST: u32 = 28;
pub const IFLA_BRPORT_VLAN_TUNNEL: u32 = 29;
pub const IFLA_BRPORT_BCAST_FLOOD: u32 = 30;
pub const IFLA_BRPORT_GROUP_FWD_MASK: u32 = 31;
pub const IFLA_BRPORT_NEIGH_SUPPRESS: u32 = 32;
pub const IFLA_BRPORT_ISOLATED: u32 = 33;
pub const IFLA_BRPORT_BACKUP_PORT: u32 = 34;
pub const IFLA_BRPORT_MRP_RING_OPEN: u32 = 35;
pub const IFLA_BRPORT_MRP_IN_OPEN: u32 = 36;
pub const IFLA_BRPORT_MCAST_EHT_HOSTS_LIMIT: u32 = 37;
pub const IFLA_BRPORT_MCAST_EHT_HOSTS_CNT: u32 = 38;
pub const IFLA_BRPORT_LOCKED: u32 = 39;
pub const IFLA_BRPORT_MAB: u32 = 40;
pub const IFLA_BRPORT_MCAST_N_GROUPS: u32 = 41;
pub const IFLA_BRPORT_MCAST_MAX_GROUPS: u32 = 42;
pub const IFLA_BRPORT_NEIGH_VLAN_SUPPRESS: u32 = 43;
pub const IFLA_BRPORT_BACKUP_NHID: u32 = 44;
pub const __IFLA_BRPORT_MAX: u32 = 45;
pub const IFLA_BRPORT_MAX: u32 = __IFLA_BRPORT_MAX - 1;

#[repr(C)]
pub struct ifla_cacheinfo {
    pub max_reasm_len: __u32,
    pub tstamp: __u32, /* ipv6InterfaceTable updated timestamp */
    pub reachable_time: __u32,
    pub retrans_time: __u32,
}

pub const IFLA_INFO_UNSPEC: u32 = 0;
pub const IFLA_INFO_KIND: u32 = 1;
pub const IFLA_INFO_DATA: u32 = 2;
pub const IFLA_INFO_XSTATS: u32 = 3;
pub const IFLA_INFO_SLAVE_KIND: u32 = 4;
pub const IFLA_INFO_SLAVE_DATA: u32 = 5;
pub const __IFLA_INFO_MAX: u32 = 6;
pub const IFLA_INFO_MAX: u32 = __IFLA_INFO_MAX - 1;

/* VLAN section */
pub const IFLA_VLAN_UNSPEC: u32 = 0;
pub const IFLA_VLAN_ID: u32 = 1;
pub const IFLA_VLAN_FLAGS: u32 = 2;
pub const IFLA_VLAN_EGRESS_QOS: u32 = 3;
pub const IFLA_VLAN_INGRESS_QOS: u32 = 4;
pub const IFLA_VLAN_PROTOCOL: u32 = 5;
pub const __IFLA_VLAN_MAX: u32 = 6;
pub const IFLA_VLAN_MAX: u32 = __IFLA_VLAN_MAX - 1;

#[repr(C)]
pub struct ifla_vlan_flags {
    pub flags: __u32,
    pub mask: __u32,
}

pub const IFLA_VLAN_QOS_UNSPEC: u32 = 0;
pub const IFLA_VLAN_QOS_MAPPING: u32 = 1;
pub const __IFLA_VLAN_QOS_MAX: u32 = 2;
pub const IFLA_VLAN_QOS_MAX: u32 = __IFLA_VLAN_QOS_MAX - 1;

#[repr(C)]
pub struct ifla_vlan_qos_mapping {
    pub from: __u32,
    pub to: __u32,
}

/* MACVLAN section */
pub const IFLA_MACVLAN_UNSPEC: u32 = 0;
pub const IFLA_MACVLAN_MODE: u32 = 1;
pub const IFLA_MACVLAN_FLAGS: u32 = 2;
pub const IFLA_MACVLAN_MACADDR_MODE: u32 = 3;
pub const IFLA_MACVLAN_MACADDR: u32 = 4;
pub const IFLA_MACVLAN_MACADDR_DATA: u32 = 5;
pub const IFLA_MACVLAN_MACADDR_COUNT: u32 = 6;
pub const IFLA_MACVLAN_BC_QUEUE_LEN: u32 = 7;
pub const IFLA_MACVLAN_BC_QUEUE_LEN_USED: u32 = 8;
pub const IFLA_MACVLAN_BC_CUTOFF: u32 = 9;
pub const __IFLA_MACVLAN_MAX: u32 = 10;
pub const IFLA_MACVLAN_MAX: u32 = __IFLA_MACVLAN_MAX - 1;

#[repr(C)]
pub enum macvlan_mode {
    MACVLAN_MODE_PRIVATE = 1, /* don't talk to other macvlans */
    MACVLAN_MODE_VEPA = 2,    /* talk to other ports through ext bridge */
    MACVLAN_MODE_BRIDGE = 4,  /* talk to bridge ports directly */
    MACVLAN_MODE_PASSTHRU = 8, /* take over the underlying device */
    MACVLAN_MODE_SOURCE = 16, /* use source MAC address list to assign */
}

#[repr(C)]
pub enum macvlan_macaddr_mode {
    MACVLAN_MACADDR_ADD = 0,
    MACVLAN_MACADDR_DEL = 1,
    MACVLAN_MACADDR_FLUSH = 2,
    MACVLAN_MACADDR_SET = 3,
}

pub const MACVLAN_FLAG_NOPROMISC: u32 = 1;
pub const MACVLAN_FLAG_NODST: u32 = 2; /* skip dst macvlan if matching src macvlan */

/* VRF section */
pub const IFLA_VRF_UNSPEC: u32 = 0;
pub const IFLA_VRF_TABLE: u32 = 1;
pub const __IFLA_VRF_MAX: u32 = 2;
pub const IFLA_VRF_MAX: u32 = __IFLA_VRF_MAX - 1;
pub const IFLA_VRF_PORT_UNSPEC: u32 = 0;
pub const IFLA_VRF_PORT_TABLE: u32 = 1;
pub const __IFLA_VRF_PORT_MAX: u32 = 2;
pub const IFLA_VRF_PORT_MAX: u32 = __IFLA_VRF_PORT_MAX - 1;

/* MACSEC section */
pub const IFLA_MACSEC_UNSPEC: u32 = 0;
pub const IFLA_MACSEC_SCI: u32 = 1;
pub const IFLA_MACSEC_PORT: u32 = 2;
pub const IFLA_MACSEC_ICV_LEN: u32 = 3;
pub const IFLA_MACSEC_CIPHER_SUITE: u32 = 4;
pub const IFLA_MACSEC_WINDOW: u32 = 5;
pub const IFLA_MACSEC_ENCODING_SA: u32 = 6;
pub const IFLA_MACSEC_ENCRYPT: u32 = 7;
pub const IFLA_MACSEC_PROTECT: u32 = 8;
pub const IFLA_MACSEC_INC_SCI: u32 = 9;
pub const IFLA_MACSEC_ES: u32 = 10;
pub const IFLA_MACSEC_SCB: u32 = 11;
pub const IFLA_MACSEC_REPLAY_PROTECT: u32 = 12;
pub const IFLA_MACSEC_VALIDATION: u32 = 13;
pub const IFLA_MACSEC_PAD: u32 = 14;
pub const IFLA_MACSEC_OFFLOAD: u32 = 15;
pub const __IFLA_MACSEC_MAX: u32 = 16;
pub const IFLA_MACSEC_MAX: u32 = __IFLA_MACSEC_MAX - 1;

/* XFRM section */
pub const IFLA_XFRM_UNSPEC: u32 = 0;
pub const IFLA_XFRM_LINK: u32 = 1;
pub const IFLA_XFRM_IF_ID: u32 = 2;
pub const IFLA_XFRM_COLLECT_METADATA: u32 = 3;
pub const __IFLA_XFRM_MAX: u32 = 4;
pub const IFLA_XFRM_MAX: u32 = __IFLA_XFRM_MAX - 1;

#[repr(C)]
pub enum macsec_validation_type {
    MACSEC_VALIDATE_DISABLED = 0,
    MACSEC_VALIDATE_CHECK = 1,
    MACSEC_VALIDATE_STRICT = 2,
    __MACSEC_VALIDATE_END = 3,
    MACSEC_VALIDATE_MAX = 2,
}

#[repr(C)]
pub enum macsec_offload {
    MACSEC_OFFLOAD_OFF = 0,
    MACSEC_OFFLOAD_PHY = 1,
    MACSEC_OFFLOAD_MAC = 2,
    __MACSEC_OFFLOAD_END = 3,
    MACSEC_OFFLOAD_MAX = 2,
}

/* IPVLAN section */
pub const IFLA_IPVLAN_UNSPEC: u32 = 0;
pub const IFLA_IPVLAN_MODE: u32 = 1;
pub const IFLA_IPVLAN_FLAGS: u32 = 2;
pub const __IFLA_IPVLAN_MAX: u32 = 3;
pub const IFLA_IPVLAN_MAX: u32 = __IFLA_IPVLAN_MAX - 1;

#[repr(C)]
pub enum ipvlan_mode {
    IPVLAN_MODE_L2 = 0,
    IPVLAN_MODE_L3 = 1,
    IPVLAN_MODE_L3S = 2,
    IPVLAN_MODE_MAX = 3,
}

pub const IPVLAN_F_PRIVATE: u32 = 0x01;
pub const IPVLAN_F_VEPA: u32 = 0x02;

/* Tunnel RTM header */
#[repr(C)]
pub struct tunnel_msg {
    pub family: __u8,
    pub flags: __u8,
    pub reserved2: __u16,
    pub ifindex: __u32,
}

/* netkit section */
#[repr(C)]
pub enum netkit_action {
    NETKIT_NEXT = -1,
    NETKIT_PASS = 0,
    NETKIT_DROP = 2,
    NETKIT_REDIRECT = 7,
}

#[repr(C)]
pub enum netkit_mode {
    NETKIT_L2 = 0,
    NETKIT_L3 = 1,
}

#[repr(C)]
pub enum netkit_scrub {
    NETKIT_SCRUB_NONE = 0,
    NETKIT_SCRUB_DEFAULT = 1,
}

pub const IFLA_NETKIT_UNSPEC: u32 = 0;
pub const IFLA_NETKIT_PEER_INFO: u32 = 1;
pub const IFLA_NETKIT_PRIMARY: u32 = 2;
pub const IFLA_NETKIT_POLICY: u32 = 3;
pub const IFLA_NETKIT_PEER_POLICY: u32 = 4;
pub const IFLA_NETKIT_MODE: u32 = 5;
pub const IFLA_NETKIT_SCRUB: u32 = 6;
pub const IFLA_NETKIT_PEER_SCRUB: u32 = 7;
pub const IFLA_NETKIT_HEADROOM: u32 = 8;
pub const IFLA_NETKIT_TAILROOM: u32 = 9;
pub const __IFLA_NETKIT_MAX: u32 = 10;
pub const IFLA_NETKIT_MAX: u32 = __IFLA_NETKIT_MAX - 1;

/* VXLAN section */
pub const TUNNEL_MSG_FLAG_STATS: u32 = 0x01; /* include statistics in the dump */
pub const TUNNEL_MSG_VALID_USER_FLAGS: u32 = TUNNEL_MSG_FLAG_STATS;

/* Embedded inside VXLAN_VNIFILTER_ENTRY_STATS */
pub const VNIFILTER_ENTRY_STATS_UNSPEC: u32 = 0;
pub const VNIFILTER_ENTRY_STATS_RX_BYTES: u32 = 1;
pub const VNIFILTER_ENTRY_STATS_RX_PKTS: u32 = 2;
pub const VNIFILTER_ENTRY_STATS_RX_DROPS: u32 = 3;
pub const VNIFILTER_ENTRY_STATS_RX_ERRORS: u32 = 4;
pub const VNIFILTER_ENTRY_STATS_TX_BYTES: u32 = 5;
pub const VNIFILTER_ENTRY_STATS_TX_PKTS: u32 = 6;
pub const VNIFILTER_ENTRY_STATS_TX_DROPS: u32 = 7;
pub const VNIFILTER_ENTRY_STATS_TX_ERRORS: u32 = 8;
pub const VNIFILTER_ENTRY_STATS_PAD: u32 = 9;
pub const __VNIFILTER_ENTRY_STATS_MAX: u32 = 10;
pub const VNIFILTER_ENTRY_STATS_MAX: u32 = __VNIFILTER_ENTRY_STATS_MAX - 1;

pub const VXLAN_VNIFILTER_ENTRY_UNSPEC: u32 = 0;
pub const VXLAN_VNIFILTER_ENTRY_START: u32 = 1;
pub const VXLAN_VNIFILTER_ENTRY_END: u32 = 2;
pub const VXLAN_VNIFILTER_ENTRY_GROUP: u32 = 3;
pub const VXLAN_VNIFILTER_ENTRY_GROUP6: u32 = 4;
pub const VXLAN_VNIFILTER_ENTRY_STATS: u32 = 5;
pub const __VXLAN_VNIFILTER_ENTRY_MAX: u32 = 6;
pub const VXLAN_VNIFILTER_ENTRY_MAX: u32 = __VXLAN_VNIFILTER_ENTRY_MAX - 1;
pub const VXLAN_VNIFILTER_UNSPEC: u32 = 0;
pub const VXLAN_VNIFILTER_ENTRY: u32 = 1;
pub const __VXLAN_VNIFILTER_MAX: u32 = 2;
pub const VXLAN_VNIFILTER_MAX: u32 = __VXLAN_VNIFILTER_MAX - 1;

pub const IFLA_VXLAN_UNSPEC: u32 = 0;
pub const IFLA_VXLAN_ID: u32 = 1;
pub const IFLA_VXLAN_GROUP: u32 = 2; /* group or remote address */
pub const IFLA_VXLAN_LINK: u32 = 3;
pub const IFLA_VXLAN_LOCAL: u32 = 4;
pub const IFLA_VXLAN_TTL: u32 = 5;
pub const IFLA_VXLAN_TOS: u32 = 6;
pub const IFLA_VXLAN_LEARNING: u32 = 7;
pub const IFLA_VXLAN_AGEING: u32 = 8;
pub const IFLA_VXLAN_LIMIT: u32 = 9;
pub const IFLA_VXLAN_PORT_RANGE: u32 = 10; /* source port */
pub const IFLA_VXLAN_PROXY: u32 = 11;
pub const IFLA_VXLAN_RSC: u32 = 12;
pub const IFLA_VXLAN_L2MISS: u32 = 13;
pub const IFLA_VXLAN_L3MISS: u32 = 14;
pub const IFLA_VXLAN_PORT: u32 = 15; /* destination port */
pub const IFLA_VXLAN_GROUP6: u32 = 16;
pub const IFLA_VXLAN_LOCAL6: u32 = 17;
pub const IFLA_VXLAN_UDP_CSUM: u32 = 18;
pub const IFLA_VXLAN_UDP_ZERO_CSUM6_TX: u32 = 19;
pub const IFLA_VXLAN_UDP_ZERO_CSUM6_RX: u32 = 20;
pub const IFLA_VXLAN_REMCSUM_TX: u32 = 21;
pub const IFLA_VXLAN_REMCSUM_RX: u32 = 22;
pub const IFLA_VXLAN_GBP: u32 = 23;
pub const IFLA_VXLAN_REMCSUM_NOPARTIAL: u32 = 24;
pub const IFLA_VXLAN_COLLECT_METADATA: u32 = 25;
pub const IFLA_VXLAN_LABEL: u32 = 26;
pub const IFLA_VXLAN_GPE: u32 = 27;
pub const IFLA_VXLAN_TTL_INHERIT: u32 = 28;
pub const IFLA_VXLAN_DF: u32 = 29;
pub const IFLA_VXLAN_VNIFILTER: u32 = 30; /* only applicable with COLLECT_METADATA mode */
pub const IFLA_VXLAN_LOCALBYPASS: u32 = 31;
pub const IFLA_VXLAN_LABEL_POLICY: u32 = 32; /* IPv6 flow label policy; ifla_vxlan_label_policy */
pub const __IFLA_VXLAN_MAX: u32 = 33;
pub const IFLA_VXLAN_MAX: u32 = __IFLA_VXLAN_MAX - 1;

#[repr(C)]
pub struct ifla_vxlan_port_range {
    pub low: __be16,
    pub high: __be16,
}

#[repr(C)]
pub enum ifla_vxlan_df {
    VXLAN_DF_UNSET = 0,
    VXLAN_DF_SET = 1,
    VXLAN_DF_INHERIT = 2,
    __VXLAN_DF_END = 3,
    VXLAN_DF_MAX = 2,
}

#[repr(C)]
pub enum ifla_vxlan_label_policy {
    VXLAN_LABEL_FIXED = 0,
    VXLAN_LABEL_INHERIT = 1,
    __VXLAN_LABEL_END = 2,
    VXLAN_LABEL_MAX = 1,
}

/* GENEVE section */
pub const IFLA_GENEVE_UNSPEC: u32 = 0;
pub const IFLA_GENEVE_ID: u32 = 1;
pub const IFLA_GENEVE_REMOTE: u32 = 2;
pub const IFLA_GENEVE_TTL: u32 = 3;
pub const IFLA_GENEVE_TOS: u32 = 4;
pub const IFLA_GENEVE_PORT: u32 = 5; /* destination port */
pub const IFLA_GENEVE_COLLECT_METADATA: u32 = 6;
pub const IFLA_GENEVE_REMOTE6: u32 = 7;
pub const IFLA_GENEVE_UDP_CSUM: u32 = 8;
pub const IFLA_GENEVE_UDP_ZERO_CSUM6_TX: u32 = 9;
pub const IFLA_GENEVE_UDP_ZERO_CSUM6_RX: u32 = 10;
pub const IFLA_GENEVE_LABEL: u32 = 11;
pub const IFLA_GENEVE_TTL_INHERIT: u32 = 12;
pub const IFLA_GENEVE_DF: u32 = 13;
pub const IFLA_GENEVE_INNER_PROTO_INHERIT: u32 = 14;
pub const __IFLA_GENEVE_MAX: u32 = 15;
pub const IFLA_GENEVE_MAX: u32 = __IFLA_GENEVE_MAX - 1;

#[repr(C)]
pub enum ifla_geneve_df {
    GENEVE_DF_UNSET = 0,
    GENEVE_DF_SET = 1,
    GENEVE_DF_INHERIT = 2,
    __GENEVE_DF_END = 3,
    GENEVE_DF_MAX = 2,
}

/* Bareudp section */
pub const IFLA_BAREUDP_UNSPEC: u32 = 0;
pub const IFLA_BAREUDP_PORT: u32 = 1;
pub const IFLA_BAREUDP_ETHERTYPE: u32 = 2;
pub const IFLA_BAREUDP_SRCPORT_MIN: u32 = 3;
pub const IFLA_BAREUDP_MULTIPROTO_MODE: u32 = 4;
pub const __IFLA_BAREUDP_MAX: u32 = 5;
pub const IFLA_BAREUDP_MAX: u32 = __IFLA_BAREUDP_MAX - 1;

/* PPP section */
pub const IFLA_PPP_UNSPEC: u32 = 0;
pub const IFLA_PPP_DEV_FD: u32 = 1;
pub const __IFLA_PPP_MAX: u32 = 2;
pub const IFLA_PPP_MAX: u32 = __IFLA_PPP_MAX - 1;

/* GTP section */
#[repr(C)]
pub enum ifla_gtp_role {
    GTP_ROLE_GGSN = 0,
    GTP_ROLE_SGSN = 1,
}

pub const IFLA_GTP_UNSPEC: u32 = 0;
pub const IFLA_GTP_FD0: u32 = 1;
pub const IFLA_GTP_FD1: u32 = 2;
pub const IFLA_GTP_PDP_HASHSIZE: u32 = 3;
pub const IFLA_GTP_ROLE: u32 = 4;
pub const IFLA_GTP_CREATE_SOCKETS: u32 = 5;
pub const IFLA_GTP_RESTART_COUNT: u32 = 6;
pub const IFLA_GTP_LOCAL: u32 = 7;
pub const IFLA_GTP_LOCAL6: u32 = 8;
pub const __IFLA_GTP_MAX: u32 = 9;
pub const IFLA_GTP_MAX: u32 = __IFLA_GTP_MAX - 1;

/* Bonding section */
pub const IFLA_BOND_UNSPEC: u32 = 0;
pub const IFLA_BOND_MODE: u32 = 1;
pub const IFLA_BOND_ACTIVE_SLAVE: u32 = 2;
pub const IFLA_BOND_MIIMON: u32 = 3;
pub const IFLA_BOND_UPDELAY: u32 = 4;
pub const IFLA_BOND_DOWNDELAY: u32 = 5;
pub const IFLA_BOND_USE_CARRIER: u32 = 6;
pub const IFLA_BOND_ARP_INTERVAL: u32 = 7;
pub const IFLA_BOND_ARP_IP_TARGET: u32 = 8;
pub const IFLA_BOND_ARP_VALIDATE: u32 = 9;
pub const IFLA_BOND_ARP_ALL_TARGETS: u32 = 10;
pub const IFLA_BOND_PRIMARY: u32 = 11;
pub const IFLA_BOND_PRIMARY_RESELECT: u32 = 12;
pub const IFLA_BOND_FAIL_OVER_MAC: u32 = 13;
pub const IFLA_BOND_XMIT_HASH_POLICY: u32 = 14;
pub const IFLA_BOND_RESEND_IGMP: u32 = 15;
pub const IFLA_BOND_NUM_PEER_NOTIF: u32 = 16;
pub const IFLA_BOND_ALL_SLAVES_ACTIVE: u32 = 17;
pub const IFLA_BOND_MIN_LINKS: u32 = 18;
pub const IFLA_BOND_LP_INTERVAL: u32 = 19;
pub const IFLA_BOND_PACKETS_PER_SLAVE: u32 = 20;
pub const IFLA_BOND_AD_LACP_RATE: u32 = 21;
pub const IFLA_BOND_AD_SELECT: u32 = 22;
pub const IFLA_BOND_AD_INFO: u32 = 23;
pub const IFLA_BOND_AD_ACTOR_SYS_PRIO: u32 = 24;
pub const IFLA_BOND_AD_USER_PORT_KEY: u32 = 25;
pub const IFLA_BOND_AD_ACTOR_SYSTEM: u32 = 26;
pub const IFLA_BOND_TLB_DYNAMIC_LB: u32 = 27;
pub const IFLA_BOND_PEER_NOTIF_DELAY: u32 = 28;
pub const IFLA_BOND_AD_LACP_ACTIVE: u32 = 29;
pub const IFLA_BOND_MISSED_MAX: u32 = 30;
pub const IFLA_BOND_NS_IP6_TARGET: u32 = 31;
pub const IFLA_BOND_COUPLED_CONTROL: u32 = 32;
pub const IFLA_BOND_BROADCAST_NEIGH: u32 = 33;
pub const IFLA_BOND_LACP_STRICT: u32 = 34;
pub const __IFLA_BOND_MAX: u32 = 35;
pub const IFLA_BOND_MAX: u32 = __IFLA_BOND_MAX - 1;

pub const IFLA_BOND_AD_INFO_UNSPEC: u32 = 0;
pub const IFLA_BOND_AD_INFO_AGGREGATOR: u32 = 1;
pub const IFLA_BOND_AD_INFO_NUM_PORTS: u32 = 2;
pub const IFLA_BOND_AD_INFO_ACTOR_KEY: u32 = 3;
pub const IFLA_BOND_AD_INFO_PARTNER_KEY: u32 = 4;
pub const IFLA_BOND_AD_INFO_PARTNER_MAC: u32 = 5;
pub const __IFLA_BOND_AD_INFO_MAX: u32 = 6;
pub const IFLA_BOND_AD_INFO_MAX: u32 = __IFLA_BOND_AD_INFO_MAX - 1;

pub const IFLA_BOND_SLAVE_UNSPEC: u32 = 0;
pub const IFLA_BOND_SLAVE_STATE: u32 = 1;
pub const IFLA_BOND_SLAVE_MII_STATUS: u32 = 2;
pub const IFLA_BOND_SLAVE_LINK_FAILURE_COUNT: u32 = 3;
pub const IFLA_BOND_SLAVE_PERM_HWADDR: u32 = 4;
pub const IFLA_BOND_SLAVE_QUEUE_ID: u32 = 5;
pub const IFLA_BOND_SLAVE_AD_AGGREGATOR_ID: u32 = 6;
pub const IFLA_BOND_SLAVE_AD_ACTOR_OPER_PORT_STATE: u32 = 7;
pub const IFLA_BOND_SLAVE_AD_PARTNER_OPER_PORT_STATE: u32 = 8;
pub const IFLA_BOND_SLAVE_PRIO: u32 = 9;
pub const __IFLA_BOND_SLAVE_MAX: u32 = 10;
pub const IFLA_BOND_SLAVE_MAX: u32 = __IFLA_BOND_SLAVE_MAX - 1;

/* SR-IOV virtual function management section */
pub const IFLA_VF_INFO_UNSPEC: u32 = 0;
pub const IFLA_VF_INFO: u32 = 1;
pub const __IFLA_VF_INFO_MAX: u32 = 2;
pub const IFLA_VF_INFO_MAX: u32 = __IFLA_VF_INFO_MAX - 1;

pub const IFLA_VF_UNSPEC: u32 = 0;
pub const IFLA_VF_MAC: u32 = 1; /* Hardware queue specific attributes */
pub const IFLA_VF_VLAN: u32 = 2; /* VLAN ID and QoS */
pub const IFLA_VF_TX_RATE: u32 = 3; /* Max TX Bandwidth Allocation */
pub const IFLA_VF_SPOOFCHK: u32 = 4; /* Spoof Checking on/off switch */
pub const IFLA_VF_LINK_STATE: u32 = 5; /* link state enable/disable/auto switch */
pub const IFLA_VF_RATE: u32 = 6; /* Min and Max TX Bandwidth Allocation */
pub const IFLA_VF_RSS_QUERY_EN: u32 = 7; /* RSS Redirection Table and Hash Key query on/off switch */
pub const IFLA_VF_STATS: u32 = 8; /* network device statistics */
pub const IFLA_VF_TRUST: u32 = 9; /* Trust VF */
pub const IFLA_VF_IB_NODE_GUID: u32 = 10; /* VF Infiniband node GUID */
pub const IFLA_VF_IB_PORT_GUID: u32 = 11; /* VF Infiniband port GUID */
pub const IFLA_VF_VLAN_LIST: u32 = 12; /* nested list of vlans, option for QinQ */
pub const IFLA_VF_BROADCAST: u32 = 13; /* VF broadcast */
pub const __IFLA_VF_MAX: u32 = 14;
pub const IFLA_VF_MAX: u32 = __IFLA_VF_MAX - 1;

#[repr(C)]
pub struct ifla_vf_mac {
    pub vf: __u32,
    pub mac: [__u8; 32], /* MAX_ADDR_LEN */
}

#[repr(C)]
pub struct ifla_vf_broadcast {
    pub broadcast: [__u8; 32],
}

#[repr(C)]
pub struct ifla_vf_vlan {
    pub vf: __u32,
    pub vlan: __u32, /* 0 - 4095, 0 disables VLAN filter */
    pub qos: __u32,
}

pub const IFLA_VF_VLAN_INFO_UNSPEC: u32 = 0;
pub const IFLA_VF_VLAN_INFO: u32 = 1; /* VLAN ID, QoS and VLAN protocol */
pub const __IFLA_VF_VLAN_INFO_MAX: u32 = 2;
pub const IFLA_VF_VLAN_INFO_MAX: u32 = __IFLA_VF_VLAN_INFO_MAX - 1;
pub const MAX_VLAN_LIST_LEN: u32 = 1;

#[repr(C)]
pub struct ifla_vf_vlan_info {
    pub vf: __u32,
    pub vlan: __u32, /* 0 - 4095, 0 disables VLAN filter */
    pub qos: __u32,
    pub vlan_proto: __be16, /* VLAN protocol either 802.1Q or 802.1ad */
}

#[repr(C)]
pub struct ifla_vf_tx_rate {
    pub vf: __u32,
    pub rate: __u32, /* Max TX bandwidth in Mbps, 0 disables throttling */
}

#[repr(C)]
pub struct ifla_vf_rate {
    pub vf: __u32,
    pub min_tx_rate: __u32, /* Min Bandwidth in Mbps */
    pub max_tx_rate: __u32, /* Max Bandwidth in Mbps */
}

#[repr(C)]
pub struct ifla_vf_spoofchk {
    pub vf: __u32,
    pub setting: __u32,
}

#[repr(C)]
pub struct ifla_vf_guid {
    pub vf: __u32,
    pub guid: __u64,
}

pub const IFLA_VF_LINK_STATE_AUTO: u32 = 0; /* link state of the uplink */
pub const IFLA_VF_LINK_STATE_ENABLE: u32 = 1; /* link always up */
pub const IFLA_VF_LINK_STATE_DISABLE: u32 = 2; /* link always down */
pub const __IFLA_VF_LINK_STATE_MAX: u32 = 3;

#[repr(C)]
pub struct ifla_vf_link_state {
    pub vf: __u32,
    pub link_state: __u32,
}

#[repr(C)]
pub struct ifla_vf_rss_query_en {
    pub vf: __u32,
    pub setting: __u32,
}

pub const IFLA_VF_STATS_RX_PACKETS: u32 = 0;
pub const IFLA_VF_STATS_TX_PACKETS: u32 = 1;
pub const IFLA_VF_STATS_RX_BYTES: u32 = 2;
pub const IFLA_VF_STATS_TX_BYTES: u32 = 3;
pub const IFLA_VF_STATS_BROADCAST: u32 = 4;
pub const IFLA_VF_STATS_MULTICAST: u32 = 5;
pub const IFLA_VF_STATS_PAD: u32 = 6;
pub const IFLA_VF_STATS_RX_DROPPED: u32 = 7;
pub const IFLA_VF_STATS_TX_DROPPED: u32 = 8;
pub const __IFLA_VF_STATS_MAX: u32 = 9;
pub const IFLA_VF_STATS_MAX: u32 = __IFLA_VF_STATS_MAX - 1;

#[repr(C)]
pub struct ifla_vf_trust {
    pub vf: __u32,
    pub setting: __u32,
}

/* VF ports management section. Nested layout is as documented in the C header. */
pub const IFLA_VF_PORT_UNSPEC: u32 = 0;
pub const IFLA_VF_PORT: u32 = 1; /* nest */
pub const __IFLA_VF_PORT_MAX: u32 = 2;
pub const IFLA_VF_PORT_MAX: u32 = __IFLA_VF_PORT_MAX - 1;

pub const IFLA_PORT_UNSPEC: u32 = 0;
pub const IFLA_PORT_VF: u32 = 1; /* __u32 */
pub const IFLA_PORT_PROFILE: u32 = 2; /* string */
pub const IFLA_PORT_VSI_TYPE: u32 = 3; /* 802.1Qbg (pre-)standard VDP */
pub const IFLA_PORT_INSTANCE_UUID: u32 = 4; /* binary UUID */
pub const IFLA_PORT_HOST_UUID: u32 = 5; /* binary UUID */
pub const IFLA_PORT_REQUEST: u32 = 6; /* __u8 */
pub const IFLA_PORT_RESPONSE: u32 = 7; /* __u16, output only */
pub const __IFLA_PORT_MAX: u32 = 8;
pub const IFLA_PORT_MAX: u32 = __IFLA_PORT_MAX - 1;

pub const PORT_PROFILE_MAX: u32 = 40;
pub const PORT_UUID_MAX: u32 = 16;
pub const PORT_SELF_VF: i32 = -1;

pub const PORT_REQUEST_PREASSOCIATE: u32 = 0;
pub const PORT_REQUEST_PREASSOCIATE_RR: u32 = 1;
pub const PORT_REQUEST_ASSOCIATE: u32 = 2;
pub const PORT_REQUEST_DISASSOCIATE: u32 = 3;

pub const PORT_VDP_RESPONSE_SUCCESS: u32 = 0;
pub const PORT_VDP_RESPONSE_INVALID_FORMAT: u32 = 1;
pub const PORT_VDP_RESPONSE_INSUFFICIENT_RESOURCES: u32 = 2;
pub const PORT_VDP_RESPONSE_UNUSED_VTID: u32 = 3;
pub const PORT_VDP_RESPONSE_VTID_VIOLATION: u32 = 4;
pub const PORT_VDP_RESPONSE_VTID_VERSION_VIOALTION: u32 = 5;
pub const PORT_VDP_RESPONSE_OUT_OF_SYNC: u32 = 6;
/* 0x08-0xFF reserved for future VDP use */
pub const PORT_PROFILE_RESPONSE_SUCCESS: u32 = 0x100;
pub const PORT_PROFILE_RESPONSE_INPROGRESS: u32 = 0x101;
pub const PORT_PROFILE_RESPONSE_INVALID: u32 = 0x102;
pub const PORT_PROFILE_RESPONSE_BADSTATE: u32 = 0x103;
pub const PORT_PROFILE_RESPONSE_INSUFFICIENT_RESOURCES: u32 = 0x104;
pub const PORT_PROFILE_RESPONSE_ERROR: u32 = 0x105;

#[repr(C)]
pub struct ifla_port_vsi {
    pub vsi_mgr_id: __u8,
    pub vsi_type_id: [__u8; 3],
    pub vsi_type_version: __u8,
    pub pad: [__u8; 3],
}

/* IPoIB section */
pub const IFLA_IPOIB_UNSPEC: u32 = 0;
pub const IFLA_IPOIB_PKEY: u32 = 1;
pub const IFLA_IPOIB_MODE: u32 = 2;
pub const IFLA_IPOIB_UMCAST: u32 = 3;
pub const __IFLA_IPOIB_MAX: u32 = 4;
pub const IPOIB_MODE_DATAGRAM: u32 = 0; /* using unreliable datagram QPs */
pub const IPOIB_MODE_CONNECTED: u32 = 1; /* using connected QPs */
pub const IFLA_IPOIB_MAX: u32 = __IFLA_IPOIB_MAX - 1;

/* HSR/PRP section, both uses same interface */
pub const HSR_PROTOCOL_HSR: u32 = 0;
pub const HSR_PROTOCOL_PRP: u32 = 1;
pub const HSR_PROTOCOL_MAX: u32 = 2;

pub const IFLA_HSR_UNSPEC: u32 = 0;
pub const IFLA_HSR_SLAVE1: u32 = 1;
pub const IFLA_HSR_SLAVE2: u32 = 2;
pub const IFLA_HSR_MULTICAST_SPEC: u32 = 3; /* Last byte of supervision addr */
pub const IFLA_HSR_SUPERVISION_ADDR: u32 = 4; /* Supervision frame multicast addr */
pub const IFLA_HSR_SEQ_NR: u32 = 5;
pub const IFLA_HSR_VERSION: u32 = 6; /* HSR version */
pub const IFLA_HSR_PROTOCOL: u32 = 7; /* Indicate different protocol than HSR. For example PRP. */
pub const IFLA_HSR_INTERLINK: u32 = 8; /* HSR interlink network device */
pub const __IFLA_HSR_MAX: u32 = 9;
pub const IFLA_HSR_MAX: u32 = __IFLA_HSR_MAX - 1;

/* STATS section */
#[repr(C)]
pub struct if_stats_msg {
    pub family: __u8,
    pub pad1: __u8,
    pub pad2: __u16,
    pub ifindex: __u32,
    pub filter_mask: __u32,
}

/* A stats attribute can be netdev specific or a global stat. */
pub const IFLA_STATS_UNSPEC: u32 = 0; /* also used as 64bit pad attribute */
pub const IFLA_STATS_LINK_64: u32 = 1;
pub const IFLA_STATS_LINK_XSTATS: u32 = 2;
pub const IFLA_STATS_LINK_XSTATS_SLAVE: u32 = 3;
pub const IFLA_STATS_LINK_OFFLOAD_XSTATS: u32 = 4;
pub const IFLA_STATS_AF_SPEC: u32 = 5;
pub const __IFLA_STATS_MAX: u32 = 6;
pub const IFLA_STATS_MAX: u32 = __IFLA_STATS_MAX - 1;

#[inline]
pub const fn IFLA_STATS_FILTER_BIT(attr: u32) -> u32 {
    1u32 << (attr - 1)
}

pub const IFLA_STATS_GETSET_UNSPEC: u32 = 0;
pub const IFLA_STATS_GET_FILTERS: u32 = 1; /* Nest of IFLA_STATS_LINK_xxx, each a u32 filter mask. */
pub const IFLA_STATS_SET_OFFLOAD_XSTATS_L3_STATS: u32 = 2; /* 0 or 1 as u8 */
pub const __IFLA_STATS_GETSET_MAX: u32 = 3;
pub const IFLA_STATS_GETSET_MAX: u32 = __IFLA_STATS_GETSET_MAX - 1;

/* Embedded into IFLA_STATS_LINK_XSTATS. */
pub const LINK_XSTATS_TYPE_UNSPEC: u32 = 0;
pub const LINK_XSTATS_TYPE_BRIDGE: u32 = 1;
pub const LINK_XSTATS_TYPE_BOND: u32 = 2;
pub const __LINK_XSTATS_TYPE_MAX: u32 = 3;
pub const LINK_XSTATS_TYPE_MAX: u32 = __LINK_XSTATS_TYPE_MAX - 1;

/* Stats embedded into IFLA_STATS_LINK_OFFLOAD_XSTATS. */
pub const IFLA_OFFLOAD_XSTATS_UNSPEC: u32 = 0;
pub const IFLA_OFFLOAD_XSTATS_CPU_HIT: u32 = 1; /* struct rtnl_link_stats64 */
pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO: u32 = 2; /* HW stats info. A nest */
pub const IFLA_OFFLOAD_XSTATS_L3_STATS: u32 = 3; /* struct rtnl_hw_stats64 */
pub const __IFLA_OFFLOAD_XSTATS_MAX: u32 = 4;
pub const IFLA_OFFLOAD_XSTATS_MAX: u32 = __IFLA_OFFLOAD_XSTATS_MAX - 1;

pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO_UNSPEC: u32 = 0;
pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO_REQUEST: u32 = 1; /* u8 */
pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO_USED: u32 = 2; /* u8 */
pub const __IFLA_OFFLOAD_XSTATS_HW_S_INFO_MAX: u32 = 3;
pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO_MAX: u32 = __IFLA_OFFLOAD_XSTATS_HW_S_INFO_MAX - 1;

/* XDP section */
pub const XDP_FLAGS_UPDATE_IF_NOEXIST: u32 = 1u32 << 0;
pub const XDP_FLAGS_SKB_MODE: u32 = 1u32 << 1;
pub const XDP_FLAGS_DRV_MODE: u32 = 1u32 << 2;
pub const XDP_FLAGS_HW_MODE: u32 = 1u32 << 3;
pub const XDP_FLAGS_REPLACE: u32 = 1u32 << 4;
pub const XDP_FLAGS_MODES: u32 = XDP_FLAGS_SKB_MODE | XDP_FLAGS_DRV_MODE | XDP_FLAGS_HW_MODE;
pub const XDP_FLAGS_MASK: u32 = XDP_FLAGS_UPDATE_IF_NOEXIST | XDP_FLAGS_MODES | XDP_FLAGS_REPLACE;

/* These are stored into IFLA_XDP_ATTACHED on dump. */
pub const XDP_ATTACHED_NONE: u32 = 0;
pub const XDP_ATTACHED_DRV: u32 = 1;
pub const XDP_ATTACHED_SKB: u32 = 2;
pub const XDP_ATTACHED_HW: u32 = 3;
pub const XDP_ATTACHED_MULTI: u32 = 4;

pub const IFLA_XDP_UNSPEC: u32 = 0;
pub const IFLA_XDP_FD: u32 = 1;
pub const IFLA_XDP_ATTACHED: u32 = 2;
pub const IFLA_XDP_FLAGS: u32 = 3;
pub const IFLA_XDP_PROG_ID: u32 = 4;
pub const IFLA_XDP_DRV_PROG_ID: u32 = 5;
pub const IFLA_XDP_SKB_PROG_ID: u32 = 6;
pub const IFLA_XDP_HW_PROG_ID: u32 = 7;
pub const IFLA_XDP_EXPECTED_FD: u32 = 8;
pub const __IFLA_XDP_MAX: u32 = 9;
pub const IFLA_XDP_MAX: u32 = __IFLA_XDP_MAX - 1;

pub const IFLA_EVENT_NONE: u32 = 0;
pub const IFLA_EVENT_REBOOT: u32 = 1; /* internal reset / reboot */
pub const IFLA_EVENT_FEATURES: u32 = 2; /* change in offload features */
pub const IFLA_EVENT_BONDING_FAILOVER: u32 = 3; /* change in active slave */
pub const IFLA_EVENT_NOTIFY_PEERS: u32 = 4; /* re-sent grat. arp/ndisc */
pub const IFLA_EVENT_IGMP_RESEND: u32 = 5; /* re-sent IGMP JOIN */
pub const IFLA_EVENT_BONDING_OPTIONS: u32 = 6; /* change in bonding options */

/* tun section */
pub const IFLA_TUN_UNSPEC: u32 = 0;
pub const IFLA_TUN_OWNER: u32 = 1;
pub const IFLA_TUN_GROUP: u32 = 2;
pub const IFLA_TUN_TYPE: u32 = 3;
pub const IFLA_TUN_PI: u32 = 4;
pub const IFLA_TUN_VNET_HDR: u32 = 5;
pub const IFLA_TUN_PERSIST: u32 = 6;
pub const IFLA_TUN_MULTI_QUEUE: u32 = 7;
pub const IFLA_TUN_NUM_QUEUES: u32 = 8;
pub const IFLA_TUN_NUM_DISABLED_QUEUES: u32 = 9;
pub const __IFLA_TUN_MAX: u32 = 10;
pub const IFLA_TUN_MAX: u32 = __IFLA_TUN_MAX - 1;

/* rmnet section */
pub const RMNET_FLAGS_INGRESS_DEAGGREGATION: u32 = 1u32 << 0;
pub const RMNET_FLAGS_INGRESS_MAP_COMMANDS: u32 = 1u32 << 1;
pub const RMNET_FLAGS_INGRESS_MAP_CKSUMV4: u32 = 1u32 << 2;
pub const RMNET_FLAGS_EGRESS_MAP_CKSUMV4: u32 = 1u32 << 3;
pub const RMNET_FLAGS_INGRESS_MAP_CKSUMV5: u32 = 1u32 << 4;
pub const RMNET_FLAGS_EGRESS_MAP_CKSUMV5: u32 = 1u32 << 5;

pub const IFLA_RMNET_UNSPEC: u32 = 0;
pub const IFLA_RMNET_MUX_ID: u32 = 1;
pub const IFLA_RMNET_FLAGS: u32 = 2;
pub const __IFLA_RMNET_MAX: u32 = 3;
pub const IFLA_RMNET_MAX: u32 = __IFLA_RMNET_MAX - 1;

#[repr(C)]
pub struct ifla_rmnet_flags {
    pub flags: __u32,
    pub mask: __u32,
}

/* MCTP section */
pub const IFLA_MCTP_UNSPEC: u32 = 0;
pub const IFLA_MCTP_NET: u32 = 1;
pub const __IFLA_MCTP_MAX: u32 = 2;
pub const IFLA_MCTP_MAX: u32 = __IFLA_MCTP_MAX - 1;

/* DSA section */
pub const IFLA_DSA_UNSPEC: u32 = 0;
pub const IFLA_DSA_CONDUIT: u32 = 1;
/* Deprecated, use IFLA_DSA_CONDUIT instead */
pub const IFLA_DSA_MASTER: u32 = IFLA_DSA_CONDUIT;
pub const __IFLA_DSA_MAX: u32 = 2;
pub const IFLA_DSA_MAX: u32 = __IFLA_DSA_MAX - 1;
