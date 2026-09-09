// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Translation of uapi/linux/if_link.h; external Linux types are referenced as aliases.
use core::ffi::c_char;
type __u8 = u8; type __u16 = u16; type __u32 = u32; type __u64 = u64; type __be16 = u16;

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
    pub rx_length_errors: __u32,
    pub rx_over_errors: __u32,
    pub rx_crc_errors: __u32,
    pub rx_frame_errors: __u32,
    pub rx_fifo_errors: __u32,
    pub rx_missed_errors: __u32,
    pub tx_aborted_errors: __u32,
    pub tx_carrier_errors: __u32,
    pub tx_fifo_errors: __u32,
    pub tx_heartbeat_errors: __u32,
    pub tx_window_errors: __u32,
    pub rx_compressed: __u32,
    pub tx_compressed: __u32,
    pub rx_nohandler: __u32,
}

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
    pub rx_length_errors: __u64,
    pub rx_over_errors: __u64,
    pub rx_crc_errors: __u64,
    pub rx_frame_errors: __u64,
    pub rx_fifo_errors: __u64,
    pub rx_missed_errors: __u64,
    pub tx_aborted_errors: __u64,
    pub tx_carrier_errors: __u64,
    pub tx_fifo_errors: __u64,
    pub tx_heartbeat_errors: __u64,
    pub tx_window_errors: __u64,
    pub rx_compressed: __u64,
    pub tx_compressed: __u64,
    pub rx_nohandler: __u64,
    pub rx_otherhost_dropped: __u64,
}

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

#[repr(C)]
pub struct rtnl_link_ifmap {
    pub mem_start: __u64,
    pub mem_end: __u64,
    pub base_addr: __u64,
    pub irq: __u16,
    pub dma: __u8,
    pub port: __u8,
}

pub const IFLA_UNSPEC: i32 = 0;
pub const IFLA_ADDRESS: i32 = 1;
pub const IFLA_BROADCAST: i32 = 2;
pub const IFLA_IFNAME: i32 = 3;
pub const IFLA_MTU: i32 = 4;
pub const IFLA_LINK: i32 = 5;
pub const IFLA_QDISC: i32 = 6;
pub const IFLA_STATS: i32 = 7;
pub const IFLA_COST: i32 = 8;
pub const #define IFLA_COST IFLA_COST: i32 = 9;
pub const IFLA_PRIORITY: i32 = 10;
pub const #define IFLA_PRIORITY IFLA_PRIORITY: i32 = 11;
pub const IFLA_MASTER: i32 = 12;
pub const #define IFLA_MASTER IFLA_MASTER: i32 = 13;
pub const IFLA_WIRELESS: i32 = 14;
pub const #define IFLA_WIRELESS IFLA_WIRELESS: i32 = 15;
pub const IFLA_PROTINFO: i32 = 16;
pub const #define IFLA_PROTINFO IFLA_PROTINFO: i32 = 17;
pub const IFLA_TXQLEN: i32 = 18;
pub const #define IFLA_TXQLEN IFLA_TXQLEN: i32 = 19;
pub const IFLA_MAP: i32 = 20;
pub const #define IFLA_MAP IFLA_MAP: i32 = 21;
pub const IFLA_WEIGHT: i32 = 22;
pub const #define IFLA_WEIGHT IFLA_WEIGHT: i32 = 23;
pub const IFLA_OPERSTATE: i32 = 24;
pub const IFLA_LINKMODE: i32 = 25;
pub const IFLA_LINKINFO: i32 = 26;
pub const #define IFLA_LINKINFO IFLA_LINKINFO: i32 = 27;
pub const IFLA_NET_NS_PID: i32 = 28;
pub const IFLA_IFALIAS: i32 = 29;
pub const IFLA_NUM_VF: i32 = 30;
pub const IFLA_VFINFO_LIST: i32 = 31;
pub const IFLA_STATS64: i32 = 32;
pub const IFLA_VF_PORTS: i32 = 33;
pub const IFLA_PORT_SELF: i32 = 34;
pub const IFLA_AF_SPEC: i32 = 35;
pub const IFLA_GROUP: i32 = 36;
pub const IFLA_NET_NS_FD: i32 = 37;
pub const IFLA_EXT_MASK: i32 = 38;
pub const IFLA_PROMISCUITY: i32 = 39;
pub const #define IFLA_PROMISCUITY IFLA_PROMISCUITY: i32 = 40;
pub const IFLA_NUM_TX_QUEUES: i32 = 41;
pub const IFLA_NUM_RX_QUEUES: i32 = 42;
pub const IFLA_CARRIER: i32 = 43;
pub const IFLA_PHYS_PORT_ID: i32 = 44;
pub const IFLA_CARRIER_CHANGES: i32 = 45;
pub const IFLA_PHYS_SWITCH_ID: i32 = 46;
pub const IFLA_LINK_NETNSID: i32 = 47;
pub const IFLA_PHYS_PORT_NAME: i32 = 48;
pub const IFLA_PROTO_DOWN: i32 = 49;
pub const IFLA_GSO_MAX_SEGS: i32 = 50;
pub const IFLA_GSO_MAX_SIZE: i32 = 51;
pub const IFLA_PAD: i32 = 52;
pub const IFLA_XDP: i32 = 53;
pub const IFLA_EVENT: i32 = 54;
pub const IFLA_NEW_NETNSID: i32 = 55;
pub const IFLA_IF_NETNSID: i32 = 56;
pub const IFLA_TARGET_NETNSID: i32 = IFLA_IF_NETNSID;
pub const IFLA_CARRIER_UP_COUNT: i32 = 0;
pub const IFLA_CARRIER_DOWN_COUNT: i32 = 1;
pub const IFLA_NEW_IFINDEX: i32 = 2;
pub const IFLA_MIN_MTU: i32 = 3;
pub const IFLA_MAX_MTU: i32 = 4;
pub const IFLA_PROP_LIST: i32 = 5;
pub const IFLA_ALT_IFNAME: i32 = 6;
pub const IFLA_PERM_ADDRESS: i32 = 7;
pub const IFLA_PROTO_DOWN_REASON: i32 = 8;
pub const IFLA_PARENT_DEV_NAME: i32 = 9;
pub const IFLA_PARENT_DEV_BUS_NAME: i32 = 10;
pub const IFLA_GRO_MAX_SIZE: i32 = 11;
pub const IFLA_TSO_MAX_SIZE: i32 = 12;
pub const IFLA_TSO_MAX_SEGS: i32 = 13;
pub const IFLA_ALLMULTI: i32 = 14;
pub const IFLA_DEVLINK_PORT: i32 = 15;
pub const IFLA_GSO_IPV4_MAX_SIZE: i32 = 16;
pub const IFLA_GRO_IPV4_MAX_SIZE: i32 = 17;
pub const IFLA_DPLL_PIN: i32 = 18;
pub const IFLA_MAX_PACING_OFFLOAD_HORIZON: i32 = 19;
pub const IFLA_NETNS_IMMUTABLE: i32 = 20;
pub const IFLA_HEADROOM: i32 = 21;
pub const IFLA_TAILROOM: i32 = 22;
pub const __IFLA_MAX: i32 = 23;
pub const IFLA_MAX: u32 = (__IFLA_MAX - 1);

pub const IFLA_PROTO_DOWN_REASON_UNSPEC: i32 = 0;
pub const IFLA_PROTO_DOWN_REASON_MASK: i32 = 1;
pub const IFLA_PROTO_DOWN_REASON_VALUE: i32 = 2;
pub const __IFLA_PROTO_DOWN_REASON_CNT: i32 = 3;
pub const IFLA_PROTO_DOWN_REASON_MAX: i32 = __IFLA_PROTO_DOWN_REASON_CNT - 1;

pub const IFLA_INET_UNSPEC: i32 = 0;
pub const IFLA_INET_CONF: i32 = 1;
pub const __IFLA_INET_MAX: i32 = 2;
pub const IFLA_INET_MAX: u32 = (__IFLA_INET_MAX - 1);

pub const IFLA_INET6_UNSPEC: i32 = 0;
pub const IFLA_INET6_FLAGS: i32 = 1;
pub const IFLA_INET6_CONF: i32 = 2;
pub const IFLA_INET6_STATS: i32 = 3;
pub const IFLA_INET6_MCAST: i32 = 4;
pub const IFLA_INET6_CACHEINFO: i32 = 5;
pub const IFLA_INET6_ICMP6STATS: i32 = 6;
pub const IFLA_INET6_TOKEN: i32 = 7;
pub const IFLA_INET6_ADDR_GEN_MODE: i32 = 8;
pub const IFLA_INET6_RA_MTU: i32 = 9;
pub const __IFLA_INET6_MAX: i32 = 10;
pub const IFLA_INET6_MAX: u32 = (__IFLA_INET6_MAX - 1);

pub const IN6_ADDR_GEN_MODE_EUI64: i32 = 0;
pub const IN6_ADDR_GEN_MODE_NONE: i32 = 1;
pub const IN6_ADDR_GEN_MODE_STABLE_PRIVACY: i32 = 2;
pub const IN6_ADDR_GEN_MODE_RANDOM: i32 = 3;

pub const IFLA_BR_UNSPEC: i32 = 0;
pub const IFLA_BR_FORWARD_DELAY: i32 = 1;
pub const IFLA_BR_HELLO_TIME: i32 = 2;
pub const IFLA_BR_MAX_AGE: i32 = 3;
pub const IFLA_BR_AGEING_TIME: i32 = 4;
pub const IFLA_BR_STP_STATE: i32 = 5;
pub const IFLA_BR_PRIORITY: i32 = 6;
pub const IFLA_BR_VLAN_FILTERING: i32 = 7;
pub const IFLA_BR_VLAN_PROTOCOL: i32 = 8;
pub const IFLA_BR_GROUP_FWD_MASK: i32 = 9;
pub const IFLA_BR_ROOT_ID: i32 = 10;
pub const IFLA_BR_BRIDGE_ID: i32 = 11;
pub const IFLA_BR_ROOT_PORT: i32 = 12;
pub const IFLA_BR_ROOT_PATH_COST: i32 = 13;
pub const IFLA_BR_TOPOLOGY_CHANGE: i32 = 14;
pub const IFLA_BR_TOPOLOGY_CHANGE_DETECTED: i32 = 15;
pub const IFLA_BR_HELLO_TIMER: i32 = 16;
pub const IFLA_BR_TCN_TIMER: i32 = 17;
pub const IFLA_BR_TOPOLOGY_CHANGE_TIMER: i32 = 18;
pub const IFLA_BR_GC_TIMER: i32 = 19;
pub const IFLA_BR_GROUP_ADDR: i32 = 20;
pub const IFLA_BR_FDB_FLUSH: i32 = 21;
pub const IFLA_BR_MCAST_ROUTER: i32 = 22;
pub const IFLA_BR_MCAST_SNOOPING: i32 = 23;
pub const IFLA_BR_MCAST_QUERY_USE_IFADDR: i32 = 24;
pub const IFLA_BR_MCAST_QUERIER: i32 = 25;
pub const IFLA_BR_MCAST_HASH_ELASTICITY: i32 = 26;
pub const IFLA_BR_MCAST_HASH_MAX: i32 = 27;
pub const IFLA_BR_MCAST_LAST_MEMBER_CNT: i32 = 28;
pub const IFLA_BR_MCAST_STARTUP_QUERY_CNT: i32 = 29;
pub const IFLA_BR_MCAST_LAST_MEMBER_INTVL: i32 = 30;
pub const IFLA_BR_MCAST_MEMBERSHIP_INTVL: i32 = 31;
pub const IFLA_BR_MCAST_QUERIER_INTVL: i32 = 32;
pub const IFLA_BR_MCAST_QUERY_INTVL: i32 = 33;
pub const IFLA_BR_MCAST_QUERY_RESPONSE_INTVL: i32 = 34;
pub const IFLA_BR_MCAST_STARTUP_QUERY_INTVL: i32 = 35;
pub const IFLA_BR_NF_CALL_IPTABLES: i32 = 36;
pub const IFLA_BR_NF_CALL_IP6TABLES: i32 = 37;
pub const IFLA_BR_NF_CALL_ARPTABLES: i32 = 38;
pub const IFLA_BR_VLAN_DEFAULT_PVID: i32 = 39;
pub const IFLA_BR_PAD: i32 = 40;
pub const IFLA_BR_VLAN_STATS_ENABLED: i32 = 41;
pub const IFLA_BR_MCAST_STATS_ENABLED: i32 = 42;
pub const IFLA_BR_MCAST_IGMP_VERSION: i32 = 43;
pub const IFLA_BR_MCAST_MLD_VERSION: i32 = 44;
pub const IFLA_BR_VLAN_STATS_PER_PORT: i32 = 45;
pub const IFLA_BR_MULTI_BOOLOPT: i32 = 46;
pub const IFLA_BR_MCAST_QUERIER_STATE: i32 = 47;
pub const IFLA_BR_FDB_N_LEARNED: i32 = 48;
pub const IFLA_BR_FDB_MAX_LEARNED: i32 = 49;
pub const IFLA_BR_STP_MODE: i32 = 50;
pub const __IFLA_BR_MAX: i32 = 51;
pub const IFLA_BR_MAX: u32 = (__IFLA_BR_MAX - 1);

pub const BR_STP_MODE_AUTO: i32 = 0;
pub const BR_STP_MODE_USER: i32 = 1;
pub const BR_STP_MODE_KERNEL: i32 = 2;
pub const __BR_STP_MODE_MAX: i32 = 3;
pub const BR_STP_MODE_MAX: u32 = (__BR_STP_MODE_MAX - 1);

#[repr(C)]
pub struct ifla_bridge_id {
    pub prio: [__u8; 2],
    pub addr: [__u8; 6],
}

pub const BRIDGE_MODE_UNSPEC: i32 = 0;
pub const BRIDGE_MODE_HAIRPIN: i32 = 1;

pub const IFLA_BRPORT_UNSPEC: i32 = 0;
pub const IFLA_BRPORT_STATE: i32 = 1;
pub const IFLA_BRPORT_PRIORITY: i32 = 2;
pub const IFLA_BRPORT_COST: i32 = 3;
pub const IFLA_BRPORT_MODE: i32 = 4;
pub const IFLA_BRPORT_GUARD: i32 = 5;
pub const IFLA_BRPORT_PROTECT: i32 = 6;
pub const IFLA_BRPORT_FAST_LEAVE: i32 = 7;
pub const IFLA_BRPORT_LEARNING: i32 = 8;
pub const IFLA_BRPORT_UNICAST_FLOOD: i32 = 9;
pub const IFLA_BRPORT_PROXYARP: i32 = 10;
pub const IFLA_BRPORT_LEARNING_SYNC: i32 = 11;
pub const IFLA_BRPORT_PROXYARP_WIFI: i32 = 12;
pub const IFLA_BRPORT_ROOT_ID: i32 = 13;
pub const IFLA_BRPORT_BRIDGE_ID: i32 = 14;
pub const IFLA_BRPORT_DESIGNATED_PORT: i32 = 15;
pub const IFLA_BRPORT_DESIGNATED_COST: i32 = 16;
pub const IFLA_BRPORT_ID: i32 = 17;
pub const IFLA_BRPORT_NO: i32 = 18;
pub const IFLA_BRPORT_TOPOLOGY_CHANGE_ACK: i32 = 19;
pub const IFLA_BRPORT_CONFIG_PENDING: i32 = 20;
pub const IFLA_BRPORT_MESSAGE_AGE_TIMER: i32 = 21;
pub const IFLA_BRPORT_FORWARD_DELAY_TIMER: i32 = 22;
pub const IFLA_BRPORT_HOLD_TIMER: i32 = 23;
pub const IFLA_BRPORT_FLUSH: i32 = 24;
pub const IFLA_BRPORT_MULTICAST_ROUTER: i32 = 25;
pub const IFLA_BRPORT_PAD: i32 = 26;
pub const IFLA_BRPORT_MCAST_FLOOD: i32 = 27;
pub const IFLA_BRPORT_MCAST_TO_UCAST: i32 = 28;
pub const IFLA_BRPORT_VLAN_TUNNEL: i32 = 29;
pub const IFLA_BRPORT_BCAST_FLOOD: i32 = 30;
pub const IFLA_BRPORT_GROUP_FWD_MASK: i32 = 31;
pub const IFLA_BRPORT_NEIGH_SUPPRESS: i32 = 32;
pub const IFLA_BRPORT_ISOLATED: i32 = 33;
pub const IFLA_BRPORT_BACKUP_PORT: i32 = 34;
pub const IFLA_BRPORT_MRP_RING_OPEN: i32 = 35;
pub const IFLA_BRPORT_MRP_IN_OPEN: i32 = 36;
pub const IFLA_BRPORT_MCAST_EHT_HOSTS_LIMIT: i32 = 37;
pub const IFLA_BRPORT_MCAST_EHT_HOSTS_CNT: i32 = 38;
pub const IFLA_BRPORT_LOCKED: i32 = 39;
pub const IFLA_BRPORT_MAB: i32 = 40;
pub const IFLA_BRPORT_MCAST_N_GROUPS: i32 = 41;
pub const IFLA_BRPORT_MCAST_MAX_GROUPS: i32 = 42;
pub const IFLA_BRPORT_NEIGH_VLAN_SUPPRESS: i32 = 43;
pub const IFLA_BRPORT_BACKUP_NHID: i32 = 44;
pub const IFLA_BRPORT_NEIGH_FORWARD_GRAT: i32 = 45;
pub const __IFLA_BRPORT_MAX: i32 = 46;
pub const IFLA_BRPORT_MAX: u32 = (__IFLA_BRPORT_MAX - 1);

#[repr(C)]
pub struct ifla_cacheinfo {
    pub max_reasm_len: __u32,
    pub tstamp: __u32,
    pub reachable_time: __u32,
    pub retrans_time: __u32,
}

pub const IFLA_INFO_UNSPEC: i32 = 0;
pub const IFLA_INFO_KIND: i32 = 1;
pub const IFLA_INFO_DATA: i32 = 2;
pub const IFLA_INFO_XSTATS: i32 = 3;
pub const IFLA_INFO_SLAVE_KIND: i32 = 4;
pub const IFLA_INFO_SLAVE_DATA: i32 = 5;
pub const __IFLA_INFO_MAX: i32 = 6;
pub const IFLA_INFO_MAX: u32 = (__IFLA_INFO_MAX - 1);

pub const IFLA_VLAN_UNSPEC: i32 = 0;
pub const IFLA_VLAN_ID: i32 = 1;
pub const IFLA_VLAN_FLAGS: i32 = 2;
pub const IFLA_VLAN_EGRESS_QOS: i32 = 3;
pub const IFLA_VLAN_INGRESS_QOS: i32 = 4;
pub const IFLA_VLAN_PROTOCOL: i32 = 5;
pub const __IFLA_VLAN_MAX: i32 = 6;
pub const IFLA_VLAN_MAX: u32 = (__IFLA_VLAN_MAX - 1);

#[repr(C)]
pub struct ifla_vlan_flags {
    pub flags: __u32,
    pub mask: __u32,
}

pub const IFLA_VLAN_QOS_UNSPEC: i32 = 0;
pub const IFLA_VLAN_QOS_MAPPING: i32 = 1;
pub const __IFLA_VLAN_QOS_MAX: i32 = 2;
pub const IFLA_VLAN_QOS_MAX: u32 = (__IFLA_VLAN_QOS_MAX - 1);

#[repr(C)]
pub struct ifla_vlan_qos_mapping {
    pub from: __u32,
    pub to: __u32,
}

pub const IFLA_MACVLAN_UNSPEC: i32 = 0;
pub const IFLA_MACVLAN_MODE: i32 = 1;
pub const IFLA_MACVLAN_FLAGS: i32 = 2;
pub const IFLA_MACVLAN_MACADDR_MODE: i32 = 3;
pub const IFLA_MACVLAN_MACADDR: i32 = 4;
pub const IFLA_MACVLAN_MACADDR_DATA: i32 = 5;
pub const IFLA_MACVLAN_MACADDR_COUNT: i32 = 6;
pub const IFLA_MACVLAN_BC_QUEUE_LEN: i32 = 7;
pub const IFLA_MACVLAN_BC_QUEUE_LEN_USED: i32 = 8;
pub const IFLA_MACVLAN_BC_CUTOFF: i32 = 9;
pub const __IFLA_MACVLAN_MAX: i32 = 10;
pub const IFLA_MACVLAN_MAX: u32 = (__IFLA_MACVLAN_MAX - 1);

pub const MACVLAN_MODE_PRIVATE: i32 = 1;
pub const MACVLAN_MODE_VEPA: i32 = 2;
pub const MACVLAN_MODE_BRIDGE: i32 = 4;
pub const MACVLAN_MODE_PASSTHRU: i32 = 8;
pub const MACVLAN_MODE_SOURCE: i32 = 16;

pub const MACVLAN_MACADDR_ADD: i32 = 0;
pub const MACVLAN_MACADDR_DEL: i32 = 1;
pub const MACVLAN_MACADDR_FLUSH: i32 = 2;
pub const MACVLAN_MACADDR_SET: i32 = 3;
pub const MACVLAN_FLAG_NOPROMISC: u32 = 1;
pub const MACVLAN_FLAG_NODST: u32 = 2;

pub const IFLA_VRF_UNSPEC: i32 = 0;
pub const IFLA_VRF_TABLE: i32 = 1;
pub const __IFLA_VRF_MAX: i32 = 2;
pub const IFLA_VRF_MAX: u32 = (__IFLA_VRF_MAX - 1);

pub const IFLA_VRF_PORT_UNSPEC: i32 = 0;
pub const IFLA_VRF_PORT_TABLE: i32 = 1;
pub const __IFLA_VRF_PORT_MAX: i32 = 2;
pub const IFLA_VRF_PORT_MAX: u32 = (__IFLA_VRF_PORT_MAX - 1);

pub const IFLA_MACSEC_UNSPEC: i32 = 0;
pub const IFLA_MACSEC_SCI: i32 = 1;
pub const IFLA_MACSEC_PORT: i32 = 2;
pub const IFLA_MACSEC_ICV_LEN: i32 = 3;
pub const IFLA_MACSEC_CIPHER_SUITE: i32 = 4;
pub const IFLA_MACSEC_WINDOW: i32 = 5;
pub const IFLA_MACSEC_ENCODING_SA: i32 = 6;
pub const IFLA_MACSEC_ENCRYPT: i32 = 7;
pub const IFLA_MACSEC_PROTECT: i32 = 8;
pub const IFLA_MACSEC_INC_SCI: i32 = 9;
pub const IFLA_MACSEC_ES: i32 = 10;
pub const IFLA_MACSEC_SCB: i32 = 11;
pub const IFLA_MACSEC_REPLAY_PROTECT: i32 = 12;
pub const IFLA_MACSEC_VALIDATION: i32 = 13;
pub const IFLA_MACSEC_PAD: i32 = 14;
pub const IFLA_MACSEC_OFFLOAD: i32 = 15;
pub const __IFLA_MACSEC_MAX: i32 = 16;
pub const IFLA_MACSEC_MAX: u32 = (__IFLA_MACSEC_MAX - 1);

pub const IFLA_XFRM_UNSPEC: i32 = 0;
pub const IFLA_XFRM_LINK: i32 = 1;
pub const IFLA_XFRM_IF_ID: i32 = 2;
pub const IFLA_XFRM_COLLECT_METADATA: i32 = 3;
pub const __IFLA_XFRM_MAX: i32 = 4;
pub const IFLA_XFRM_MAX: u32 = (__IFLA_XFRM_MAX - 1);

pub const MACSEC_VALIDATE_DISABLED: i32 = 0;
pub const MACSEC_VALIDATE_CHECK: i32 = 1;
pub const MACSEC_VALIDATE_STRICT: i32 = 2;
pub const __MACSEC_VALIDATE_END: i32 = 3;
pub const MACSEC_VALIDATE_MAX: i32 = __MACSEC_VALIDATE_END - 1;

pub const MACSEC_OFFLOAD_OFF: i32 = 0;
pub const MACSEC_OFFLOAD_PHY: i32 = 1;
pub const MACSEC_OFFLOAD_MAC: i32 = 2;
pub const __MACSEC_OFFLOAD_END: i32 = 3;
pub const MACSEC_OFFLOAD_MAX: i32 = __MACSEC_OFFLOAD_END - 1;

pub const IFLA_IPVLAN_UNSPEC: i32 = 0;
pub const IFLA_IPVLAN_MODE: i32 = 1;
pub const IFLA_IPVLAN_FLAGS: i32 = 2;
pub const __IFLA_IPVLAN_MAX: i32 = 3;
pub const IFLA_IPVLAN_MAX: u32 = (__IFLA_IPVLAN_MAX - 1);

pub const IPVLAN_MODE_L2: i32 = 0;
pub const IPVLAN_MODE_L3: i32 = 1;
pub const IPVLAN_MODE_L3S: i32 = 2;
pub const IPVLAN_MODE_MAX: i32 = 3;
pub const IPVLAN_F_PRIVATE: u32 = 0x01;
pub const IPVLAN_F_VEPA: u32 = 0x02;

#[repr(C)]
pub struct tunnel_msg {
    pub family: __u8,
    pub flags: __u8,
    pub reserved2: __u16,
    pub ifindex: __u32,
}

pub const NETKIT_NEXT: i32 = -1;
pub const NETKIT_PASS: i32 = 0;
pub const NETKIT_DROP: i32 = 2;
pub const NETKIT_REDIRECT: i32 = 7;

pub const NETKIT_L2: i32 = 0;
pub const NETKIT_L3: i32 = 1;

pub const NETKIT_DEVICE_PAIR: i32 = 0;
pub const NETKIT_DEVICE_SINGLE: i32 = 1;

pub const NETKIT_SCRUB_NONE: i32 = 0;
pub const NETKIT_SCRUB_DEFAULT: i32 = 1;

pub const IFLA_NETKIT_UNSPEC: i32 = 0;
pub const IFLA_NETKIT_PEER_INFO: i32 = 1;
pub const IFLA_NETKIT_PRIMARY: i32 = 2;
pub const IFLA_NETKIT_POLICY: i32 = 3;
pub const IFLA_NETKIT_PEER_POLICY: i32 = 4;
pub const IFLA_NETKIT_MODE: i32 = 5;
pub const IFLA_NETKIT_SCRUB: i32 = 6;
pub const IFLA_NETKIT_PEER_SCRUB: i32 = 7;
pub const IFLA_NETKIT_HEADROOM: i32 = 8;
pub const IFLA_NETKIT_TAILROOM: i32 = 9;
pub const IFLA_NETKIT_PAIRING: i32 = 10;
pub const __IFLA_NETKIT_MAX: i32 = 11;
pub const IFLA_NETKIT_MAX: u32 = (__IFLA_NETKIT_MAX - 1);
pub const TUNNEL_MSG_FLAG_STATS: u32 = 0x01;
pub const TUNNEL_MSG_VALID_USER_FLAGS: u32 = TUNNEL_MSG_FLAG_STATS;

pub const VNIFILTER_ENTRY_STATS_UNSPEC: i32 = 0;
pub const VNIFILTER_ENTRY_STATS_RX_BYTES: i32 = 1;
pub const VNIFILTER_ENTRY_STATS_RX_PKTS: i32 = 2;
pub const VNIFILTER_ENTRY_STATS_RX_DROPS: i32 = 3;
pub const VNIFILTER_ENTRY_STATS_RX_ERRORS: i32 = 4;
pub const VNIFILTER_ENTRY_STATS_TX_BYTES: i32 = 5;
pub const VNIFILTER_ENTRY_STATS_TX_PKTS: i32 = 6;
pub const VNIFILTER_ENTRY_STATS_TX_DROPS: i32 = 7;
pub const VNIFILTER_ENTRY_STATS_TX_ERRORS: i32 = 8;
pub const VNIFILTER_ENTRY_STATS_PAD: i32 = 9;
pub const __VNIFILTER_ENTRY_STATS_MAX: i32 = 10;
pub const VNIFILTER_ENTRY_STATS_MAX: u32 = (__VNIFILTER_ENTRY_STATS_MAX - 1);

pub const VXLAN_VNIFILTER_ENTRY_UNSPEC: i32 = 0;
pub const VXLAN_VNIFILTER_ENTRY_START: i32 = 1;
pub const VXLAN_VNIFILTER_ENTRY_END: i32 = 2;
pub const VXLAN_VNIFILTER_ENTRY_GROUP: i32 = 3;
pub const VXLAN_VNIFILTER_ENTRY_GROUP6: i32 = 4;
pub const VXLAN_VNIFILTER_ENTRY_STATS: i32 = 5;
pub const __VXLAN_VNIFILTER_ENTRY_MAX: i32 = 6;
pub const VXLAN_VNIFILTER_ENTRY_MAX: u32 = (__VXLAN_VNIFILTER_ENTRY_MAX - 1);

pub const VXLAN_VNIFILTER_UNSPEC: i32 = 0;
pub const VXLAN_VNIFILTER_ENTRY: i32 = 1;
pub const __VXLAN_VNIFILTER_MAX: i32 = 2;
pub const VXLAN_VNIFILTER_MAX: u32 = (__VXLAN_VNIFILTER_MAX - 1);

pub const IFLA_VXLAN_UNSPEC: i32 = 0;
pub const IFLA_VXLAN_ID: i32 = 1;
pub const IFLA_VXLAN_GROUP: i32 = 2;
pub const IFLA_VXLAN_LINK: i32 = 3;
pub const IFLA_VXLAN_LOCAL: i32 = 4;
pub const IFLA_VXLAN_TTL: i32 = 5;
pub const IFLA_VXLAN_TOS: i32 = 6;
pub const IFLA_VXLAN_LEARNING: i32 = 7;
pub const IFLA_VXLAN_AGEING: i32 = 8;
pub const IFLA_VXLAN_LIMIT: i32 = 9;
pub const IFLA_VXLAN_PORT_RANGE: i32 = 10;
pub const IFLA_VXLAN_PROXY: i32 = 11;
pub const IFLA_VXLAN_RSC: i32 = 12;
pub const IFLA_VXLAN_L2MISS: i32 = 13;
pub const IFLA_VXLAN_L3MISS: i32 = 14;
pub const IFLA_VXLAN_PORT: i32 = 15;
pub const IFLA_VXLAN_GROUP6: i32 = 16;
pub const IFLA_VXLAN_LOCAL6: i32 = 17;
pub const IFLA_VXLAN_UDP_CSUM: i32 = 18;
pub const IFLA_VXLAN_UDP_ZERO_CSUM6_TX: i32 = 19;
pub const IFLA_VXLAN_UDP_ZERO_CSUM6_RX: i32 = 20;
pub const IFLA_VXLAN_REMCSUM_TX: i32 = 21;
pub const IFLA_VXLAN_REMCSUM_RX: i32 = 22;
pub const IFLA_VXLAN_GBP: i32 = 23;
pub const IFLA_VXLAN_REMCSUM_NOPARTIAL: i32 = 24;
pub const IFLA_VXLAN_COLLECT_METADATA: i32 = 25;
pub const IFLA_VXLAN_LABEL: i32 = 26;
pub const IFLA_VXLAN_GPE: i32 = 27;
pub const IFLA_VXLAN_TTL_INHERIT: i32 = 28;
pub const IFLA_VXLAN_DF: i32 = 29;
pub const IFLA_VXLAN_VNIFILTER: i32 = 30;
pub const IFLA_VXLAN_LOCALBYPASS: i32 = 31;
pub const IFLA_VXLAN_LABEL_POLICY: i32 = 32;
pub const IFLA_VXLAN_RESERVED_BITS: i32 = 33;
pub const IFLA_VXLAN_MC_ROUTE: i32 = 34;
pub const __IFLA_VXLAN_MAX: i32 = 35;
pub const IFLA_VXLAN_MAX: u32 = (__IFLA_VXLAN_MAX - 1);

#[repr(C)]
pub struct ifla_vxlan_port_range {
    pub low: __be16,
    pub high: __be16,
}

pub const VXLAN_DF_UNSET: i32 = 0;
pub const VXLAN_DF_SET: i32 = 1;
pub const VXLAN_DF_INHERIT: i32 = 2;
pub const __VXLAN_DF_END: i32 = 3;
pub const VXLAN_DF_MAX: i32 = __VXLAN_DF_END - 1;

pub const VXLAN_LABEL_FIXED: i32 = 0;
pub const VXLAN_LABEL_INHERIT: i32 = 1;
pub const __VXLAN_LABEL_END: i32 = 2;
pub const VXLAN_LABEL_MAX: i32 = __VXLAN_LABEL_END - 1;

pub const IFLA_GENEVE_UNSPEC: i32 = 0;
pub const IFLA_GENEVE_ID: i32 = 1;
pub const IFLA_GENEVE_REMOTE: i32 = 2;
pub const IFLA_GENEVE_TTL: i32 = 3;
pub const IFLA_GENEVE_TOS: i32 = 4;
pub const IFLA_GENEVE_PORT: i32 = 5;
pub const IFLA_GENEVE_COLLECT_METADATA: i32 = 6;
pub const IFLA_GENEVE_REMOTE6: i32 = 7;
pub const IFLA_GENEVE_UDP_CSUM: i32 = 8;
pub const IFLA_GENEVE_UDP_ZERO_CSUM6_TX: i32 = 9;
pub const IFLA_GENEVE_UDP_ZERO_CSUM6_RX: i32 = 10;
pub const IFLA_GENEVE_LABEL: i32 = 11;
pub const IFLA_GENEVE_TTL_INHERIT: i32 = 12;
pub const IFLA_GENEVE_DF: i32 = 13;
pub const IFLA_GENEVE_INNER_PROTO_INHERIT: i32 = 14;
pub const IFLA_GENEVE_PORT_RANGE: i32 = 15;
pub const IFLA_GENEVE_GRO_HINT: i32 = 16;
pub const IFLA_GENEVE_LOCAL: i32 = 17;
pub const IFLA_GENEVE_LOCAL6: i32 = 18;
pub const __IFLA_GENEVE_MAX: i32 = 19;
pub const IFLA_GENEVE_MAX: u32 = (__IFLA_GENEVE_MAX - 1);

pub const GENEVE_DF_UNSET: i32 = 0;
pub const GENEVE_DF_SET: i32 = 1;
pub const GENEVE_DF_INHERIT: i32 = 2;
pub const __GENEVE_DF_END: i32 = 3;
pub const GENEVE_DF_MAX: i32 = __GENEVE_DF_END - 1;

#[repr(C)]
pub struct ifla_geneve_port_range {
    pub low: __be16,
    pub high: __be16,
}

pub const IFLA_BAREUDP_UNSPEC: i32 = 0;
pub const IFLA_BAREUDP_PORT: i32 = 1;
pub const IFLA_BAREUDP_ETHERTYPE: i32 = 2;
pub const IFLA_BAREUDP_SRCPORT_MIN: i32 = 3;
pub const IFLA_BAREUDP_MULTIPROTO_MODE: i32 = 4;
pub const __IFLA_BAREUDP_MAX: i32 = 5;
pub const IFLA_BAREUDP_MAX: u32 = (__IFLA_BAREUDP_MAX - 1);

pub const IFLA_PPP_UNSPEC: i32 = 0;
pub const IFLA_PPP_DEV_FD: i32 = 1;
pub const __IFLA_PPP_MAX: i32 = 2;
pub const IFLA_PPP_MAX: u32 = (__IFLA_PPP_MAX - 1);

pub const GTP_ROLE_GGSN: i32 = 0;
pub const GTP_ROLE_SGSN: i32 = 1;

pub const IFLA_GTP_UNSPEC: i32 = 0;
pub const IFLA_GTP_FD0: i32 = 1;
pub const IFLA_GTP_FD1: i32 = 2;
pub const IFLA_GTP_PDP_HASHSIZE: i32 = 3;
pub const IFLA_GTP_ROLE: i32 = 4;
pub const IFLA_GTP_CREATE_SOCKETS: i32 = 5;
pub const IFLA_GTP_RESTART_COUNT: i32 = 6;
pub const IFLA_GTP_LOCAL: i32 = 7;
pub const IFLA_GTP_LOCAL6: i32 = 8;
pub const __IFLA_GTP_MAX: i32 = 9;
pub const IFLA_GTP_MAX: u32 = (__IFLA_GTP_MAX - 1);

pub const IFLA_BOND_UNSPEC: i32 = 0;
pub const IFLA_BOND_MODE: i32 = 1;
pub const IFLA_BOND_ACTIVE_SLAVE: i32 = 2;
pub const IFLA_BOND_MIIMON: i32 = 3;
pub const IFLA_BOND_UPDELAY: i32 = 4;
pub const IFLA_BOND_DOWNDELAY: i32 = 5;
pub const IFLA_BOND_USE_CARRIER: i32 = 6;
pub const IFLA_BOND_ARP_INTERVAL: i32 = 7;
pub const IFLA_BOND_ARP_IP_TARGET: i32 = 8;
pub const IFLA_BOND_ARP_VALIDATE: i32 = 9;
pub const IFLA_BOND_ARP_ALL_TARGETS: i32 = 10;
pub const IFLA_BOND_PRIMARY: i32 = 11;
pub const IFLA_BOND_PRIMARY_RESELECT: i32 = 12;
pub const IFLA_BOND_FAIL_OVER_MAC: i32 = 13;
pub const IFLA_BOND_XMIT_HASH_POLICY: i32 = 14;
pub const IFLA_BOND_RESEND_IGMP: i32 = 15;
pub const IFLA_BOND_NUM_PEER_NOTIF: i32 = 16;
pub const IFLA_BOND_ALL_SLAVES_ACTIVE: i32 = 17;
pub const IFLA_BOND_MIN_LINKS: i32 = 18;
pub const IFLA_BOND_LP_INTERVAL: i32 = 19;
pub const IFLA_BOND_PACKETS_PER_SLAVE: i32 = 20;
pub const IFLA_BOND_AD_LACP_RATE: i32 = 21;
pub const IFLA_BOND_AD_SELECT: i32 = 22;
pub const IFLA_BOND_AD_INFO: i32 = 23;
pub const IFLA_BOND_AD_ACTOR_SYS_PRIO: i32 = 24;
pub const IFLA_BOND_AD_USER_PORT_KEY: i32 = 25;
pub const IFLA_BOND_AD_ACTOR_SYSTEM: i32 = 26;
pub const IFLA_BOND_TLB_DYNAMIC_LB: i32 = 27;
pub const IFLA_BOND_PEER_NOTIF_DELAY: i32 = 28;
pub const IFLA_BOND_AD_LACP_ACTIVE: i32 = 29;
pub const IFLA_BOND_MISSED_MAX: i32 = 30;
pub const IFLA_BOND_NS_IP6_TARGET: i32 = 31;
pub const IFLA_BOND_COUPLED_CONTROL: i32 = 32;
pub const IFLA_BOND_BROADCAST_NEIGH: i32 = 33;
pub const IFLA_BOND_LACP_STRICT: i32 = 34;
pub const __IFLA_BOND_MAX: i32 = 35;
pub const IFLA_BOND_MAX: u32 = (__IFLA_BOND_MAX - 1);

pub const IFLA_BOND_AD_INFO_UNSPEC: i32 = 0;
pub const IFLA_BOND_AD_INFO_AGGREGATOR: i32 = 1;
pub const IFLA_BOND_AD_INFO_NUM_PORTS: i32 = 2;
pub const IFLA_BOND_AD_INFO_ACTOR_KEY: i32 = 3;
pub const IFLA_BOND_AD_INFO_PARTNER_KEY: i32 = 4;
pub const IFLA_BOND_AD_INFO_PARTNER_MAC: i32 = 5;
pub const __IFLA_BOND_AD_INFO_MAX: i32 = 6;
pub const IFLA_BOND_AD_INFO_MAX: u32 = (__IFLA_BOND_AD_INFO_MAX - 1);

pub const IFLA_BOND_SLAVE_UNSPEC: i32 = 0;
pub const IFLA_BOND_SLAVE_STATE: i32 = 1;
pub const IFLA_BOND_SLAVE_MII_STATUS: i32 = 2;
pub const IFLA_BOND_SLAVE_LINK_FAILURE_COUNT: i32 = 3;
pub const IFLA_BOND_SLAVE_PERM_HWADDR: i32 = 4;
pub const IFLA_BOND_SLAVE_QUEUE_ID: i32 = 5;
pub const IFLA_BOND_SLAVE_AD_AGGREGATOR_ID: i32 = 6;
pub const IFLA_BOND_SLAVE_AD_ACTOR_OPER_PORT_STATE: i32 = 7;
pub const IFLA_BOND_SLAVE_AD_PARTNER_OPER_PORT_STATE: i32 = 8;
pub const IFLA_BOND_SLAVE_PRIO: i32 = 9;
pub const IFLA_BOND_SLAVE_ACTOR_PORT_PRIO: i32 = 10;
pub const IFLA_BOND_SLAVE_AD_CHURN_ACTOR_STATE: i32 = 11;
pub const IFLA_BOND_SLAVE_AD_CHURN_PARTNER_STATE: i32 = 12;
pub const __IFLA_BOND_SLAVE_MAX: i32 = 13;
pub const IFLA_BOND_SLAVE_MAX: u32 = (__IFLA_BOND_SLAVE_MAX - 1);

pub const IFLA_VF_INFO_UNSPEC: i32 = 0;
pub const IFLA_VF_INFO: i32 = 1;
pub const __IFLA_VF_INFO_MAX: i32 = 2;
pub const IFLA_VF_INFO_MAX: u32 = (__IFLA_VF_INFO_MAX - 1);

pub const IFLA_VF_UNSPEC: i32 = 0;
pub const IFLA_VF_MAC: i32 = 1;
pub const IFLA_VF_VLAN: i32 = 2;
pub const IFLA_VF_TX_RATE: i32 = 3;
pub const IFLA_VF_SPOOFCHK: i32 = 4;
pub const IFLA_VF_LINK_STATE: i32 = 5;
pub const IFLA_VF_RATE: i32 = 6;
pub const IFLA_VF_RSS_QUERY_EN,	/* RSS Redirection Table and Hash Key query: i32 = 7;
pub const IFLA_VF_STATS: i32 = 8;
pub const IFLA_VF_TRUST: i32 = 9;
pub const IFLA_VF_IB_NODE_GUID: i32 = 10;
pub const IFLA_VF_IB_PORT_GUID: i32 = 11;
pub const IFLA_VF_VLAN_LIST: i32 = 12;
pub const IFLA_VF_BROADCAST: i32 = 13;
pub const __IFLA_VF_MAX: i32 = 14;
pub const IFLA_VF_MAX: u32 = (__IFLA_VF_MAX - 1);

#[repr(C)]
pub struct ifla_vf_mac {
    pub vf: __u32,
    pub mac: [__u8; 32],
}

#[repr(C)]
pub struct ifla_vf_broadcast {
    pub broadcast: [__u8; 32],
}

#[repr(C)]
pub struct ifla_vf_vlan {
    pub vf: __u32,
    pub vlan: __u32,
    pub qos: __u32,
}

pub const IFLA_VF_VLAN_INFO_UNSPEC: i32 = 0;
pub const IFLA_VF_VLAN_INFO: i32 = 1;
pub const __IFLA_VF_VLAN_INFO_MAX: i32 = 2;
pub const IFLA_VF_VLAN_INFO_MAX: u32 = (__IFLA_VF_VLAN_INFO_MAX - 1);
pub const MAX_VLAN_LIST_LEN: u32 = 1;

#[repr(C)]
pub struct ifla_vf_vlan_info {
    pub vf: __u32,
    pub vlan: __u32,
    pub qos: __u32,
    pub vlan_proto: __be16,
}

#[repr(C)]
pub struct ifla_vf_tx_rate {
    pub vf: __u32,
    pub rate: __u32,
}

#[repr(C)]
pub struct ifla_vf_rate {
    pub vf: __u32,
    pub min_tx_rate: __u32,
    pub max_tx_rate: __u32,
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

pub const IFLA_VF_LINK_STATE_AUTO: i32 = 0;
pub const IFLA_VF_LINK_STATE_ENABLE: i32 = 1;
pub const IFLA_VF_LINK_STATE_DISABLE: i32 = 2;
pub const __IFLA_VF_LINK_STATE_MAX: i32 = 3;

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

pub const IFLA_VF_STATS_RX_PACKETS: i32 = 0;
pub const IFLA_VF_STATS_TX_PACKETS: i32 = 1;
pub const IFLA_VF_STATS_RX_BYTES: i32 = 2;
pub const IFLA_VF_STATS_TX_BYTES: i32 = 3;
pub const IFLA_VF_STATS_BROADCAST: i32 = 4;
pub const IFLA_VF_STATS_MULTICAST: i32 = 5;
pub const IFLA_VF_STATS_PAD: i32 = 6;
pub const IFLA_VF_STATS_RX_DROPPED: i32 = 7;
pub const IFLA_VF_STATS_TX_DROPPED: i32 = 8;
pub const __IFLA_VF_STATS_MAX: i32 = 9;
pub const IFLA_VF_STATS_MAX: u32 = (__IFLA_VF_STATS_MAX - 1);

#[repr(C)]
pub struct ifla_vf_trust {
    pub vf: __u32,
    pub setting: __u32,
}

pub const IFLA_VF_PORT_UNSPEC: i32 = 0;
pub const IFLA_VF_PORT: i32 = 1;
pub const __IFLA_VF_PORT_MAX: i32 = 2;
pub const IFLA_VF_PORT_MAX: u32 = (__IFLA_VF_PORT_MAX - 1);

pub const IFLA_PORT_UNSPEC: i32 = 0;
pub const IFLA_PORT_VF: i32 = 1;
pub const IFLA_PORT_PROFILE: i32 = 2;
pub const IFLA_PORT_VSI_TYPE: i32 = 3;
pub const IFLA_PORT_INSTANCE_UUID: i32 = 4;
pub const IFLA_PORT_HOST_UUID: i32 = 5;
pub const IFLA_PORT_REQUEST: i32 = 6;
pub const IFLA_PORT_RESPONSE: i32 = 7;
pub const __IFLA_PORT_MAX: i32 = 8;
pub const IFLA_PORT_MAX: u32 = (__IFLA_PORT_MAX - 1);
pub const PORT_PROFILE_MAX: u32 = 40;
pub const PORT_UUID_MAX: u32 = 16;
pub const PORT_SELF_VF: u32 = -1;

pub const PORT_REQUEST_PREASSOCIATE: i32 = 0;
pub const PORT_REQUEST_PREASSOCIATE_RR: i32 = 1;
pub const PORT_REQUEST_ASSOCIATE: i32 = 2;
pub const PORT_REQUEST_DISASSOCIATE: i32 = 3;

pub const PORT_VDP_RESPONSE_SUCCESS: i32 = 0;
pub const PORT_VDP_RESPONSE_INVALID_FORMAT: i32 = 1;
pub const PORT_VDP_RESPONSE_INSUFFICIENT_RESOURCES: i32 = 2;
pub const PORT_VDP_RESPONSE_UNUSED_VTID: i32 = 3;
pub const PORT_VDP_RESPONSE_VTID_VIOLATION: i32 = 4;
pub const PORT_VDP_RESPONSE_VTID_VERSION_VIOALTION: i32 = 5;
pub const PORT_VDP_RESPONSE_OUT_OF_SYNC: i32 = 6;
pub const PORT_PROFILE_RESPONSE_SUCCESS: i32 = 0x100;
pub const PORT_PROFILE_RESPONSE_INPROGRESS: i32 = 257;
pub const PORT_PROFILE_RESPONSE_INVALID: i32 = 258;
pub const PORT_PROFILE_RESPONSE_BADSTATE: i32 = 259;
pub const PORT_PROFILE_RESPONSE_INSUFFICIENT_RESOURCES: i32 = 260;
pub const PORT_PROFILE_RESPONSE_ERROR: i32 = 261;

#[repr(C)]
pub struct ifla_port_vsi {
    pub vsi_mgr_id: __u8,
    pub vsi_type_id: [__u8; 3],
    pub vsi_type_version: __u8,
    pub pad: [__u8; 3],
}

pub const IFLA_IPOIB_UNSPEC: i32 = 0;
pub const IFLA_IPOIB_PKEY: i32 = 1;
pub const IFLA_IPOIB_MODE: i32 = 2;
pub const IFLA_IPOIB_UMCAST: i32 = 3;
pub const __IFLA_IPOIB_MAX: i32 = 4;

pub const IPOIB_MODE_DATAGRAM: i32 = 0;
pub const IPOIB_MODE_CONNECTED: i32 = 1;
pub const IFLA_IPOIB_MAX: u32 = (__IFLA_IPOIB_MAX - 1);

pub const HSR_PROTOCOL_HSR: i32 = 0;
pub const HSR_PROTOCOL_PRP: i32 = 1;
pub const HSR_PROTOCOL_MAX: i32 = 2;

pub const IFLA_HSR_UNSPEC: i32 = 0;
pub const IFLA_HSR_SLAVE1: i32 = 1;
pub const IFLA_HSR_SLAVE2: i32 = 2;
pub const IFLA_HSR_MULTICAST_SPEC: i32 = 3;
pub const IFLA_HSR_SUPERVISION_ADDR: i32 = 4;
pub const IFLA_HSR_SEQ_NR: i32 = 5;
pub const IFLA_HSR_VERSION: i32 = 6;
pub const IFLA_HSR_PROTOCOL,		/* Indicate different protocol than: i32 = 7;
pub const IFLA_HSR_INTERLINK: i32 = 8;
pub const __IFLA_HSR_MAX: i32 = 9;
pub const IFLA_HSR_MAX: u32 = (__IFLA_HSR_MAX - 1);

#[repr(C)]
pub struct if_stats_msg {
    pub family: __u8,
    pub pad1: __u8,
    pub pad2: __u16,
    pub ifindex: __u32,
    pub filter_mask: __u32,
}

pub const IFLA_STATS_UNSPEC: i32 = 0;
pub const IFLA_STATS_LINK_64: i32 = 1;
pub const IFLA_STATS_LINK_XSTATS: i32 = 2;
pub const IFLA_STATS_LINK_XSTATS_SLAVE: i32 = 3;
pub const IFLA_STATS_LINK_OFFLOAD_XSTATS: i32 = 4;
pub const IFLA_STATS_AF_SPEC: i32 = 5;
pub const __IFLA_STATS_MAX: i32 = 6;
pub const IFLA_STATS_MAX: u32 = (__IFLA_STATS_MAX - 1);

pub const IFLA_STATS_GETSET_UNSPEC: i32 = 0;
pub const IFLA_STATS_GET_FILTERS, /* Nest of IFLA_STATS_LINK_xxx, each a u32 with: i32 = 1;
pub const IFLA_STATS_SET_OFFLOAD_XSTATS_L3_STATS: i32 = 2;
pub const __IFLA_STATS_GETSET_MAX: i32 = 3;
pub const IFLA_STATS_GETSET_MAX: u32 = (__IFLA_STATS_GETSET_MAX - 1);

pub const LINK_XSTATS_TYPE_UNSPEC: i32 = 0;
pub const LINK_XSTATS_TYPE_BRIDGE: i32 = 1;
pub const LINK_XSTATS_TYPE_BOND: i32 = 2;
pub const __LINK_XSTATS_TYPE_MAX: i32 = 3;
pub const LINK_XSTATS_TYPE_MAX: u32 = (__LINK_XSTATS_TYPE_MAX - 1);

pub const IFLA_OFFLOAD_XSTATS_UNSPEC: i32 = 0;
pub const IFLA_OFFLOAD_XSTATS_CPU_HIT: i32 = 1;
pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO: i32 = 2;
pub const IFLA_OFFLOAD_XSTATS_L3_STATS: i32 = 3;
pub const __IFLA_OFFLOAD_XSTATS_MAX: i32 = 4;
pub const IFLA_OFFLOAD_XSTATS_MAX: u32 = (__IFLA_OFFLOAD_XSTATS_MAX - 1);

pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO_UNSPEC: i32 = 0;
pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO_REQUEST: i32 = 1;
pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO_USED: i32 = 2;
pub const __IFLA_OFFLOAD_XSTATS_HW_S_INFO_MAX: i32 = 3;
pub const IFLA_OFFLOAD_XSTATS_HW_S_INFO_MAX: u32 = ;
pub const XDP_FLAGS_UPDATE_IF_NOEXIST: u32 = (1u32 << 0);
pub const XDP_FLAGS_SKB_MODE: u32 = (1u32 << 1);
pub const XDP_FLAGS_DRV_MODE: u32 = (1u32 << 2);
pub const XDP_FLAGS_HW_MODE: u32 = (1u32 << 3);
pub const XDP_FLAGS_REPLACE: u32 = (1u32 << 4);
pub const XDP_FLAGS_MODES: u32 = (XDP_FLAGS_SKB_MODE | ;
pub const XDP_FLAGS_MASK: u32 = (XDP_FLAGS_UPDATE_IF_NOEXIST | ;

pub const XDP_ATTACHED_NONE: i32 = 0;
pub const XDP_ATTACHED_DRV: i32 = 1;
pub const XDP_ATTACHED_SKB: i32 = 2;
pub const XDP_ATTACHED_HW: i32 = 3;
pub const XDP_ATTACHED_MULTI: i32 = 4;

pub const IFLA_XDP_UNSPEC: i32 = 0;
pub const IFLA_XDP_FD: i32 = 1;
pub const IFLA_XDP_ATTACHED: i32 = 2;
pub const IFLA_XDP_FLAGS: i32 = 3;
pub const IFLA_XDP_PROG_ID: i32 = 4;
pub const IFLA_XDP_DRV_PROG_ID: i32 = 5;
pub const IFLA_XDP_SKB_PROG_ID: i32 = 6;
pub const IFLA_XDP_HW_PROG_ID: i32 = 7;
pub const IFLA_XDP_EXPECTED_FD: i32 = 8;
pub const __IFLA_XDP_MAX: i32 = 9;
pub const IFLA_XDP_MAX: u32 = (__IFLA_XDP_MAX - 1);

pub const IFLA_EVENT_NONE: i32 = 0;
pub const IFLA_EVENT_REBOOT: i32 = 1;
pub const IFLA_EVENT_FEATURES: i32 = 2;
pub const IFLA_EVENT_BONDING_FAILOVER: i32 = 3;
pub const IFLA_EVENT_NOTIFY_PEERS: i32 = 4;
pub const IFLA_EVENT_IGMP_RESEND: i32 = 5;
pub const IFLA_EVENT_BONDING_OPTIONS: i32 = 6;

pub const IFLA_TUN_UNSPEC: i32 = 0;
pub const IFLA_TUN_OWNER: i32 = 1;
pub const IFLA_TUN_GROUP: i32 = 2;
pub const IFLA_TUN_TYPE: i32 = 3;
pub const IFLA_TUN_PI: i32 = 4;
pub const IFLA_TUN_VNET_HDR: i32 = 5;
pub const IFLA_TUN_PERSIST: i32 = 6;
pub const IFLA_TUN_MULTI_QUEUE: i32 = 7;
pub const IFLA_TUN_NUM_QUEUES: i32 = 8;
pub const IFLA_TUN_NUM_DISABLED_QUEUES: i32 = 9;
pub const __IFLA_TUN_MAX: i32 = 10;
pub const IFLA_TUN_MAX: u32 = (__IFLA_TUN_MAX - 1);
pub const RMNET_FLAGS_INGRESS_DEAGGREGATION: u32 = (1u32 << 0);
pub const RMNET_FLAGS_INGRESS_MAP_COMMANDS: u32 = (1u32 << 1);
pub const RMNET_FLAGS_INGRESS_MAP_CKSUMV4: u32 = (1u32 << 2);
pub const RMNET_FLAGS_EGRESS_MAP_CKSUMV4: u32 = (1u32 << 3);
pub const RMNET_FLAGS_INGRESS_MAP_CKSUMV5: u32 = (1u32 << 4);
pub const RMNET_FLAGS_EGRESS_MAP_CKSUMV5: u32 = (1u32 << 5);

pub const IFLA_RMNET_UNSPEC: i32 = 0;
pub const IFLA_RMNET_MUX_ID: i32 = 1;
pub const IFLA_RMNET_FLAGS: i32 = 2;
pub const __IFLA_RMNET_MAX: i32 = 3;
pub const IFLA_RMNET_MAX: u32 = (__IFLA_RMNET_MAX - 1);

#[repr(C)]
pub struct ifla_rmnet_flags {
    pub flags: __u32,
    pub mask: __u32,
}

pub const IFLA_MCTP_UNSPEC: i32 = 0;
pub const IFLA_MCTP_NET: i32 = 1;
pub const IFLA_MCTP_PHYS_BINDING: i32 = 2;
pub const __IFLA_MCTP_MAX: i32 = 3;
pub const IFLA_MCTP_MAX: u32 = (__IFLA_MCTP_MAX - 1);

pub const IFLA_DSA_UNSPEC: i32 = 0;
pub const IFLA_DSA_CONDUIT: i32 = 1;
pub const IFLA_DSA_MASTER: i32 = IFLA_DSA_CONDUIT;
pub const __IFLA_DSA_MAX: i32 = 0;
pub const IFLA_DSA_MAX: u32 = (__IFLA_DSA_MAX - 1);

pub const OVPN_MODE_P2P: i32 = 0;
pub const OVPN_MODE_MP: i32 = 1;

pub const IFLA_OVPN_UNSPEC: i32 = 0;
pub const IFLA_OVPN_MODE: i32 = 1;
pub const __IFLA_OVPN_MAX: i32 = 2;
pub const IFLA_OVPN_MAX: u32 = (__IFLA_OVPN_MAX - 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
