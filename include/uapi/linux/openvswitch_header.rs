/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of uapi/linux/openvswitch.h. */

#[repr(C)]
pub struct ovs_header { pub dp_ifindex: i32 }

pub const OVS_DATAPATH_FAMILY: &str = "ovs_datapath";
pub const OVS_DATAPATH_MCGROUP: &str = "ovs_datapath";
pub const OVS_DATAPATH_VERSION: u32 = 2;
pub const OVS_DP_VER_FEATURES: u32 = 2;

#[repr(u32)]
pub enum ovs_datapath_cmd { OVS_DP_CMD_UNSPEC, OVS_DP_CMD_NEW, OVS_DP_CMD_DEL, OVS_DP_CMD_GET, OVS_DP_CMD_SET }
#[repr(u32)]
pub enum ovs_datapath_attr { OVS_DP_ATTR_UNSPEC, OVS_DP_ATTR_NAME, OVS_DP_ATTR_UPCALL_PID, OVS_DP_ATTR_STATS, OVS_DP_ATTR_MEGAFLOW_STATS, OVS_DP_ATTR_USER_FEATURES, OVS_DP_ATTR_PAD, OVS_DP_ATTR_MASKS_CACHE_SIZE, OVS_DP_ATTR_PER_CPU_PIDS, OVS_DP_ATTR_IFINDEX, __OVS_DP_ATTR_MAX }
pub const OVS_DP_ATTR_MAX: u32 = __OVS_DP_ATTR_MAX as u32 - 1;

#[repr(C)]
pub struct ovs_dp_stats { pub n_hit: u64, pub n_missed: u64, pub n_lost: u64, pub n_flows: u64 }
#[repr(C)]
pub struct ovs_dp_megaflow_stats { pub n_mask_hit: u64, pub n_masks: u32, pub pad0: u32, pub n_cache_hit: u64, pub pad1: u64 }
#[repr(C)]
pub struct ovs_vport_stats { pub rx_packets:u64, pub tx_packets:u64, pub rx_bytes:u64, pub tx_bytes:u64, pub rx_errors:u64, pub tx_errors:u64, pub rx_dropped:u64, pub tx_dropped:u64 }
pub const OVS_DP_F_UNALIGNED: u32 = 1 << 0;
pub const OVS_DP_F_VPORT_PIDS: u32 = 1 << 1;
pub const OVS_DP_F_TC_RECIRC_SHARING: u32 = 1 << 2;
pub const OVS_DP_F_DISPATCH_UPCALL_PER_CPU: u32 = 1 << 3;
pub const OVSP_LOCAL: u32 = 0;

pub const OVS_PACKET_FAMILY: &str = "ovs_packet";
pub const OVS_PACKET_VERSION: u32 = 0x1;
#[repr(u32)]
pub enum ovs_packet_cmd { OVS_PACKET_CMD_UNSPEC, OVS_PACKET_CMD_MISS, OVS_PACKET_CMD_ACTION, OVS_PACKET_CMD_EXECUTE }
#[repr(u32)]
pub enum ovs_packet_attr { OVS_PACKET_ATTR_UNSPEC, OVS_PACKET_ATTR_PACKET, OVS_PACKET_ATTR_KEY, OVS_PACKET_ATTR_ACTIONS, OVS_PACKET_ATTR_USERDATA, OVS_PACKET_ATTR_EGRESS_TUN_KEY, OVS_PACKET_ATTR_UNUSED1, OVS_PACKET_ATTR_UNUSED2, OVS_PACKET_ATTR_PROBE, OVS_PACKET_ATTR_MRU, OVS_PACKET_ATTR_LEN, OVS_PACKET_ATTR_HASH, OVS_PACKET_ATTR_UPCALL_PID, __OVS_PACKET_ATTR_MAX }
pub const OVS_PACKET_ATTR_MAX: u32 = __OVS_PACKET_ATTR_MAX as u32 - 1;

pub const OVS_VPORT_FAMILY: &str = "ovs_vport";
pub const OVS_VPORT_MCGROUP: &str = "ovs_vport";
pub const OVS_VPORT_VERSION: u32 = 0x1;
#[repr(u32)]
pub enum ovs_vport_cmd { OVS_VPORT_CMD_UNSPEC, OVS_VPORT_CMD_NEW, OVS_VPORT_CMD_DEL, OVS_VPORT_CMD_GET, OVS_VPORT_CMD_SET }
#[repr(u32)]
pub enum ovs_vport_type { OVS_VPORT_TYPE_UNSPEC, OVS_VPORT_TYPE_NETDEV, OVS_VPORT_TYPE_INTERNAL, OVS_VPORT_TYPE_GRE, OVS_VPORT_TYPE_VXLAN, OVS_VPORT_TYPE_GENEVE, __OVS_VPORT_TYPE_MAX }
pub const OVS_VPORT_TYPE_MAX: u32 = __OVS_VPORT_TYPE_MAX as u32 - 1;
#[repr(u32)]
pub enum ovs_vport_attr { OVS_VPORT_ATTR_UNSPEC, OVS_VPORT_ATTR_PORT_NO, OVS_VPORT_ATTR_TYPE, OVS_VPORT_ATTR_NAME, OVS_VPORT_ATTR_OPTIONS, OVS_VPORT_ATTR_UPCALL_PID, OVS_VPORT_ATTR_STATS, OVS_VPORT_ATTR_PAD, OVS_VPORT_ATTR_IFINDEX, OVS_VPORT_ATTR_NETNSID, OVS_VPORT_ATTR_UPCALL_STATS, __OVS_VPORT_ATTR_MAX }
pub const OVS_VPORT_ATTR_MAX: u32 = __OVS_VPORT_ATTR_MAX as u32 - 1;
#[repr(u32)]
pub enum ovs_vport_upcall_attr { OVS_VPORT_UPCALL_ATTR_SUCCESS, OVS_VPORT_UPCALL_ATTR_FAIL, __OVS_VPORT_UPCALL_ATTR_MAX }
pub const OVS_VPORT_UPCALL_ATTR_MAX: u32 = __OVS_VPORT_UPCALL_ATTR_MAX as u32 - 1;

#[repr(u32)]
pub enum ovs_vxlan_ext { OVS_VXLAN_EXT_UNSPEC, OVS_VXLAN_EXT_GBP, __OVS_VXLAN_EXT_MAX }
pub const OVS_VXLAN_EXT_MAX: u32 = __OVS_VXLAN_EXT_MAX as u32 - 1;
#[repr(u32)]
pub enum ovs_tunnel_attr { OVS_TUNNEL_ATTR_UNSPEC, OVS_TUNNEL_ATTR_DST_PORT, OVS_TUNNEL_ATTR_EXTENSION, __OVS_TUNNEL_ATTR_MAX }
pub const OVS_TUNNEL_ATTR_MAX: u32 = __OVS_TUNNEL_ATTR_MAX as u32 - 1;

pub const OVS_FLOW_FAMILY: &str = "ovs_flow";
pub const OVS_FLOW_MCGROUP: &str = "ovs_flow";
pub const OVS_FLOW_VERSION: u32 = 0x1;
#[repr(u32)]
pub enum ovs_flow_cmd { OVS_FLOW_CMD_UNSPEC, OVS_FLOW_CMD_NEW, OVS_FLOW_CMD_DEL, OVS_FLOW_CMD_GET, OVS_FLOW_CMD_SET }
#[repr(C)] pub struct ovs_flow_stats { pub n_packets:u64, pub n_bytes:u64 }
#[repr(u32)]
pub enum ovs_key_attr { OVS_KEY_ATTR_UNSPEC, OVS_KEY_ATTR_ENCAP, OVS_KEY_ATTR_PRIORITY, OVS_KEY_ATTR_IN_PORT, OVS_KEY_ATTR_ETHERNET, OVS_KEY_ATTR_VLAN, OVS_KEY_ATTR_ETHERTYPE, OVS_KEY_ATTR_IPV4, OVS_KEY_ATTR_IPV6, OVS_KEY_ATTR_TCP, OVS_KEY_ATTR_UDP, OVS_KEY_ATTR_ICMP, OVS_KEY_ATTR_ICMPV6, OVS_KEY_ATTR_ARP, OVS_KEY_ATTR_ND, OVS_KEY_ATTR_SKB_MARK, OVS_KEY_ATTR_TUNNEL, OVS_KEY_ATTR_SCTP, OVS_KEY_ATTR_TCP_FLAGS, OVS_KEY_ATTR_DP_HASH, OVS_KEY_ATTR_RECIRC_ID, OVS_KEY_ATTR_MPLS, OVS_KEY_ATTR_CT_STATE, OVS_KEY_ATTR_CT_ZONE, OVS_KEY_ATTR_CT_MARK, OVS_KEY_ATTR_CT_LABELS, OVS_KEY_ATTR_CT_ORIG_TUPLE_IPV4, OVS_KEY_ATTR_CT_ORIG_TUPLE_IPV6, OVS_KEY_ATTR_NSH, OVS_KEY_ATTR_PACKET_TYPE, OVS_KEY_ATTR_ND_EXTENSIONS, OVS_KEY_ATTR_TUNNEL_INFO, OVS_KEY_ATTR_IPV6_EXTHDRS, __OVS_KEY_ATTR_MAX }
pub const OVS_KEY_ATTR_MAX: u32 = __OVS_KEY_ATTR_MAX as u32 - 1;
#[repr(u32)]
pub enum ovs_tunnel_key_attr { OVS_TUNNEL_KEY_ATTR_ID, OVS_TUNNEL_KEY_ATTR_IPV4_SRC, OVS_TUNNEL_KEY_ATTR_IPV4_DST, OVS_TUNNEL_KEY_ATTR_TOS, OVS_TUNNEL_KEY_ATTR_TTL, OVS_TUNNEL_KEY_ATTR_DONT_FRAGMENT, OVS_TUNNEL_KEY_ATTR_CSUM, OVS_TUNNEL_KEY_ATTR_OAM, OVS_TUNNEL_KEY_ATTR_GENEVE_OPTS, OVS_TUNNEL_KEY_ATTR_TP_SRC, OVS_TUNNEL_KEY_ATTR_TP_DST, OVS_TUNNEL_KEY_ATTR_VXLAN_OPTS, OVS_TUNNEL_KEY_ATTR_IPV6_SRC, OVS_TUNNEL_KEY_ATTR_IPV6_DST, OVS_TUNNEL_KEY_ATTR_PAD, OVS_TUNNEL_KEY_ATTR_ERSPAN_OPTS, OVS_TUNNEL_KEY_ATTR_IPV4_INFO_BRIDGE, __OVS_TUNNEL_KEY_ATTR_MAX }
pub const OVS_TUNNEL_KEY_ATTR_MAX: u32 = __OVS_TUNNEL_KEY_ATTR_MAX as u32 - 1;
#[repr(u32)] pub enum ovs_frag_type { OVS_FRAG_TYPE_NONE, OVS_FRAG_TYPE_FIRST, OVS_FRAG_TYPE_LATER, __OVS_FRAG_TYPE_MAX }
pub const OVS_FRAG_TYPE_MAX: u32 = __OVS_FRAG_TYPE_MAX as u32 - 1;

#[repr(C)] pub struct ovs_key_ethernet { pub eth_src:[u8;6], pub eth_dst:[u8;6] }
#[repr(C)] pub struct ovs_key_mpls { pub mpls_lse:u32 }
#[repr(C)] pub struct ovs_key_ipv4 { pub ipv4_src:u32, pub ipv4_dst:u32, pub ipv4_proto:u8, pub ipv4_tos:u8, pub ipv4_ttl:u8, pub ipv4_frag:u8 }
#[repr(C)] pub struct ovs_key_ipv6 { pub ipv6_src:[u32;4], pub ipv6_dst:[u32;4], pub ipv6_label:u32, pub ipv6_proto:u8, pub ipv6_tclass:u8, pub ipv6_hlimit:u8, pub ipv6_frag:u8 }
#[repr(C)] pub struct ovs_key_ipv6_exthdrs { pub hdrs:u16 }
#[repr(C)] pub struct ovs_key_tcp { pub tcp_src:u16, pub tcp_dst:u16 }
#[repr(C)] pub struct ovs_key_udp { pub udp_src:u16, pub udp_dst:u16 }
#[repr(C)] pub struct ovs_key_sctp { pub sctp_src:u16, pub sctp_dst:u16 }
#[repr(C)] pub struct ovs_key_icmp { pub icmp_type:u8, pub icmp_code:u8 }
#[repr(C)] pub struct ovs_key_icmpv6 { pub icmpv6_type:u8, pub icmpv6_code:u8 }
#[repr(C)] pub struct ovs_key_arp { pub arp_sip:u32, pub arp_tip:u32, pub arp_op:u16, pub arp_sha:[u8;6], pub arp_tha:[u8;6] }
#[repr(C)] pub struct ovs_key_nd { pub nd_target:[u32;4], pub nd_sll:[u8;6], pub nd_tll:[u8;6] }
pub const OVS_CT_LABELS_LEN_32: usize = 4;
pub const OVS_CT_LABELS_LEN: usize = OVS_CT_LABELS_LEN_32 * core::mem::size_of::<u32>();
#[repr(C)] pub union ovs_key_ct_labels { pub ct_labels:[u8;OVS_CT_LABELS_LEN], pub ct_labels_32:[u32;OVS_CT_LABELS_LEN_32] }
pub const OVS_CS_F_NEW:u32=0x01; pub const OVS_CS_F_ESTABLISHED:u32=0x02; pub const OVS_CS_F_RELATED:u32=0x04; pub const OVS_CS_F_REPLY_DIR:u32=0x08; pub const OVS_CS_F_INVALID:u32=0x10; pub const OVS_CS_F_TRACKED:u32=0x20; pub const OVS_CS_F_SRC_NAT:u32=0x40; pub const OVS_CS_F_DST_NAT:u32=0x80; pub const OVS_CS_F_NAT_MASK:u32=OVS_CS_F_SRC_NAT|OVS_CS_F_DST_NAT;
#[repr(C)] pub struct ovs_key_ct_tuple_ipv4 { pub ipv4_src:u32,pub ipv4_dst:u32,pub src_port:u16,pub dst_port:u16,pub ipv4_proto:u8 }
#[repr(C)] pub struct ovs_key_ct_tuple_ipv6 { pub ipv6_src:[u32;4],pub ipv6_dst:[u32;4],pub src_port:u16,pub dst_port:u16,pub ipv6_proto:u8 }
#[repr(u32)] pub enum ovs_nsh_key_attr { OVS_NSH_KEY_ATTR_UNSPEC, OVS_NSH_KEY_ATTR_BASE, OVS_NSH_KEY_ATTR_MD1, OVS_NSH_KEY_ATTR_MD2, __OVS_NSH_KEY_ATTR_MAX }
pub const OVS_NSH_KEY_ATTR_MAX:u32=__OVS_NSH_KEY_ATTR_MAX as u32-1;
#[repr(C)] pub struct ovs_nsh_key_base { pub flags:u8,pub ttl:u8,pub mdtype:u8,pub np:u8,pub path_hdr:u32 }
pub const NSH_MD1_CONTEXT_SIZE:usize=4;
#[repr(C)] pub struct ovs_nsh_key_md1 { pub context:[u32;NSH_MD1_CONTEXT_SIZE] }

#[repr(u32)] pub enum ovs_flow_attr { OVS_FLOW_ATTR_UNSPEC, OVS_FLOW_ATTR_KEY, OVS_FLOW_ATTR_ACTIONS, OVS_FLOW_ATTR_STATS, OVS_FLOW_ATTR_TCP_FLAGS, OVS_FLOW_ATTR_USED, OVS_FLOW_ATTR_CLEAR, OVS_FLOW_ATTR_MASK, OVS_FLOW_ATTR_PROBE, OVS_FLOW_ATTR_UFID, OVS_FLOW_ATTR_UFID_FLAGS, OVS_FLOW_ATTR_PAD, __OVS_FLOW_ATTR_MAX }
pub const OVS_FLOW_ATTR_MAX:u32=__OVS_FLOW_ATTR_MAX as u32-1;
pub const OVS_UFID_F_OMIT_KEY:u32=1<<0; pub const OVS_UFID_F_OMIT_MASK:u32=1<<1; pub const OVS_UFID_F_OMIT_ACTIONS:u32=1<<2;
#[repr(u32)] pub enum ovs_sample_attr { OVS_SAMPLE_ATTR_UNSPEC, OVS_SAMPLE_ATTR_PROBABILITY, OVS_SAMPLE_ATTR_ACTIONS, __OVS_SAMPLE_ATTR_MAX, #[cfg(feature="__KERNEL__")] OVS_SAMPLE_ATTR_ARG }
pub const OVS_SAMPLE_ATTR_MAX:u32=__OVS_SAMPLE_ATTR_MAX as u32-1;
#[cfg(feature="__KERNEL__")]
#[repr(C)] pub struct sample_arg { pub exec: bool, pub probability:u32 }
#[repr(u32)] pub enum ovs_userspace_attr { OVS_USERSPACE_ATTR_UNSPEC, OVS_USERSPACE_ATTR_PID, OVS_USERSPACE_ATTR_USERDATA, OVS_USERSPACE_ATTR_EGRESS_TUN_PORT, OVS_USERSPACE_ATTR_ACTIONS, __OVS_USERSPACE_ATTR_MAX }
pub const OVS_USERSPACE_ATTR_MAX:u32=__OVS_USERSPACE_ATTR_MAX as u32-1;
#[repr(C)] pub struct ovs_action_trunc { pub max_len:u32 }
#[repr(C)] pub struct ovs_action_push_mpls { pub mpls_lse:u32,pub mpls_ethertype:u16 }
#[repr(C)] pub struct ovs_action_add_mpls { pub mpls_lse:u32,pub mpls_ethertype:u16,pub tun_flags:u16 }
pub const OVS_MPLS_L3_TUNNEL_FLAG_MASK:u32=1<<0;
#[repr(C)] pub struct ovs_action_push_vlan { pub vlan_tpid:u16,pub vlan_tci:u16 }
#[repr(u32)] pub enum ovs_hash_alg { OVS_HASH_ALG_L4, OVS_HASH_ALG_SYM_L4 }
#[repr(C)] pub struct ovs_action_hash { pub hash_alg:u32,pub hash_basis:u32 }
#[repr(u32)] pub enum ovs_ct_attr { OVS_CT_ATTR_UNSPEC, OVS_CT_ATTR_COMMIT, OVS_CT_ATTR_ZONE, OVS_CT_ATTR_MARK, OVS_CT_ATTR_LABELS, OVS_CT_ATTR_HELPER, OVS_CT_ATTR_NAT, OVS_CT_ATTR_FORCE_COMMIT, OVS_CT_ATTR_EVENTMASK, OVS_CT_ATTR_TIMEOUT, __OVS_CT_ATTR_MAX }
pub const OVS_CT_ATTR_MAX:u32=__OVS_CT_ATTR_MAX as u32-1;
#[repr(u32)] pub enum ovs_nat_attr { OVS_NAT_ATTR_UNSPEC, OVS_NAT_ATTR_SRC, OVS_NAT_ATTR_DST, OVS_NAT_ATTR_IP_MIN, OVS_NAT_ATTR_IP_MAX, OVS_NAT_ATTR_PROTO_MIN, OVS_NAT_ATTR_PROTO_MAX, OVS_NAT_ATTR_PERSISTENT, OVS_NAT_ATTR_PROTO_HASH, OVS_NAT_ATTR_PROTO_RANDOM, __OVS_NAT_ATTR_MAX }
pub const OVS_NAT_ATTR_MAX:u32=__OVS_NAT_ATTR_MAX as u32-1;
#[repr(C)] pub struct ovs_action_push_eth { pub addresses:ovs_key_ethernet }
#[repr(u32)] pub enum ovs_check_pkt_len_attr { OVS_CHECK_PKT_LEN_ATTR_UNSPEC, OVS_CHECK_PKT_LEN_ATTR_PKT_LEN, OVS_CHECK_PKT_LEN_ATTR_ACTIONS_IF_GREATER, OVS_CHECK_PKT_LEN_ATTR_ACTIONS_IF_LESS_EQUAL, __OVS_CHECK_PKT_LEN_ATTR_MAX, #[cfg(feature="__KERNEL__")] OVS_CHECK_PKT_LEN_ATTR_ARG }
pub const OVS_CHECK_PKT_LEN_ATTR_MAX:u32=__OVS_CHECK_PKT_LEN_ATTR_MAX as u32-1;
#[cfg(feature="__KERNEL__")]
#[repr(C)] pub struct check_pkt_len_arg { pub pkt_len:u16, pub exec_for_greater:bool, pub exec_for_lesser_equal:bool }
pub const OVS_PSAMPLE_COOKIE_MAX_SIZE:usize=16;
#[repr(u32)] pub enum ovs_psample_attr { OVS_PSAMPLE_ATTR_GROUP=1, OVS_PSAMPLE_ATTR_COOKIE, __OVS_PSAMPLE_ATTR_MAX }
pub const OVS_PSAMPLE_ATTR_MAX:u32=__OVS_PSAMPLE_ATTR_MAX as u32-1;
#[repr(u32)] pub enum ovs_action_attr { OVS_ACTION_ATTR_UNSPEC, OVS_ACTION_ATTR_OUTPUT, OVS_ACTION_ATTR_USERSPACE, OVS_ACTION_ATTR_SET, OVS_ACTION_ATTR_PUSH_VLAN, OVS_ACTION_ATTR_POP_VLAN, OVS_ACTION_ATTR_SAMPLE, OVS_ACTION_ATTR_RECIRC, OVS_ACTION_ATTR_HASH, OVS_ACTION_ATTR_PUSH_MPLS, OVS_ACTION_ATTR_POP_MPLS, OVS_ACTION_ATTR_SET_MASKED, OVS_ACTION_ATTR_CT, OVS_ACTION_ATTR_TRUNC, OVS_ACTION_ATTR_PUSH_ETH, OVS_ACTION_ATTR_POP_ETH, OVS_ACTION_ATTR_CT_CLEAR, OVS_ACTION_ATTR_PUSH_NSH, OVS_ACTION_ATTR_POP_NSH, OVS_ACTION_ATTR_METER, OVS_ACTION_ATTR_CLONE, OVS_ACTION_ATTR_CHECK_PKT_LEN, OVS_ACTION_ATTR_ADD_MPLS, OVS_ACTION_ATTR_DEC_TTL, OVS_ACTION_ATTR_DROP, OVS_ACTION_ATTR_PSAMPLE, __OVS_ACTION_ATTR_MAX, #[cfg(feature="__KERNEL__")] OVS_ACTION_ATTR_SET_TO_MASKED }
pub const OVS_ACTION_ATTR_MAX:u32=__OVS_ACTION_ATTR_MAX as u32-1;

pub const OVS_METER_FAMILY:&str="ovs_meter"; pub const OVS_METER_MCGROUP:&str="ovs_meter"; pub const OVS_METER_VERSION:u32=0x1;
#[repr(u32)] pub enum ovs_meter_cmd { OVS_METER_CMD_UNSPEC, OVS_METER_CMD_FEATURES, OVS_METER_CMD_SET, OVS_METER_CMD_DEL, OVS_METER_CMD_GET }
#[repr(u32)] pub enum ovs_meter_attr { OVS_METER_ATTR_UNSPEC, OVS_METER_ATTR_ID, OVS_METER_ATTR_KBPS, OVS_METER_ATTR_STATS, OVS_METER_ATTR_BANDS, OVS_METER_ATTR_USED, OVS_METER_ATTR_CLEAR, OVS_METER_ATTR_MAX_METERS, OVS_METER_ATTR_MAX_BANDS, OVS_METER_ATTR_PAD, __OVS_METER_ATTR_MAX }
pub const OVS_METER_ATTR_MAX:u32=__OVS_METER_ATTR_MAX as u32-1;
#[repr(u32)] pub enum ovs_band_attr { OVS_BAND_ATTR_UNSPEC, OVS_BAND_ATTR_TYPE, OVS_BAND_ATTR_RATE, OVS_BAND_ATTR_BURST, OVS_BAND_ATTR_STATS, __OVS_BAND_ATTR_MAX }
pub const OVS_BAND_ATTR_MAX:u32=__OVS_BAND_ATTR_MAX as u32-1;
#[repr(u32)] pub enum ovs_meter_band_type { OVS_METER_BAND_TYPE_UNSPEC, OVS_METER_BAND_TYPE_DROP, __OVS_METER_BAND_TYPE_MAX }
pub const OVS_METER_BAND_TYPE_MAX:u32=__OVS_METER_BAND_TYPE_MAX as u32-1;
pub const OVS_CT_LIMIT_FAMILY:&str="ovs_ct_limit"; pub const OVS_CT_LIMIT_MCGROUP:&str="ovs_ct_limit"; pub const OVS_CT_LIMIT_VERSION:u32=0x1;
#[repr(u32)] pub enum ovs_ct_limit_cmd { OVS_CT_LIMIT_CMD_UNSPEC, OVS_CT_LIMIT_CMD_SET, OVS_CT_LIMIT_CMD_DEL, OVS_CT_LIMIT_CMD_GET }
#[repr(u32)] pub enum ovs_ct_limit_attr { OVS_CT_LIMIT_ATTR_UNSPEC, OVS_CT_LIMIT_ATTR_ZONE_LIMIT, __OVS_CT_LIMIT_ATTR_MAX }
pub const OVS_CT_LIMIT_ATTR_MAX:u32=__OVS_CT_LIMIT_ATTR_MAX as u32-1;
pub const OVS_ZONE_LIMIT_DEFAULT_ZONE:i32=-1;
#[repr(C)] pub struct ovs_zone_limit { pub zone_id:i32,pub limit:u32,pub count:u32 }
#[repr(u32)] pub enum ovs_dec_ttl_attr { OVS_DEC_TTL_ATTR_UNSPEC, OVS_DEC_TTL_ATTR_ACTION, __OVS_DEC_TTL_ATTR_MAX }
pub const OVS_DEC_TTL_ATTR_MAX:u32=__OVS_DEC_TTL_ATTR_MAX as u32-1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
