/* SPDX-License-Identifier: GPL-2.0-only */
/* Source-level Rust translation of virtchnl.h. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub const ETH_ALEN: usize = 6;
pub const BIT: fn(u32) -> u32 = |n| 1u32 << n;
pub const BIT_ULL: fn(u32) -> u64 = |n| 1u64 << n;

#[repr(i32)]
pub enum virtchnl_status_code { VIRTCHNL_STATUS_SUCCESS=0, VIRTCHNL_STATUS_ERR_PARAM=-5, VIRTCHNL_STATUS_ERR_NO_MEMORY=-18, VIRTCHNL_STATUS_ERR_OPCODE_MISMATCH=-38, VIRTCHNL_STATUS_ERR_CQP_COMPL_ERROR=-39, VIRTCHNL_STATUS_ERR_INVALID_VF_ID=-40, VIRTCHNL_STATUS_ERR_ADMIN_QUEUE_ERROR=-53, VIRTCHNL_STATUS_ERR_NOT_SUPPORTED=-64 }
pub const VIRTCHNL_ERR_PARAM: virtchnl_status_code = virtchnl_status_code::VIRTCHNL_STATUS_ERR_PARAM;
pub const VIRTCHNL_STATUS_NOT_SUPPORTED: virtchnl_status_code = virtchnl_status_code::VIRTCHNL_STATUS_ERR_NOT_SUPPORTED;
pub const VIRTCHNL_LINK_SPEED_2_5GB_SHIFT:u32=0; pub const VIRTCHNL_LINK_SPEED_100MB_SHIFT:u32=1; pub const VIRTCHNL_LINK_SPEED_1000MB_SHIFT:u32=2; pub const VIRTCHNL_LINK_SPEED_10GB_SHIFT:u32=3; pub const VIRTCHNL_LINK_SPEED_40GB_SHIFT:u32=4; pub const VIRTCHNL_LINK_SPEED_20GB_SHIFT:u32=5; pub const VIRTCHNL_LINK_SPEED_25GB_SHIFT:u32=6; pub const VIRTCHNL_LINK_SPEED_5GB_SHIFT:u32=7;
#[repr(u32)] pub enum virtchnl_link_speed { VIRTCHNL_LINK_SPEED_UNKNOWN=0, VIRTCHNL_LINK_SPEED_100MB=1<<1, VIRTCHNL_LINK_SPEED_1GB=1<<2, VIRTCHNL_LINK_SPEED_10GB=1<<3, VIRTCHNL_LINK_SPEED_40GB=1<<4, VIRTCHNL_LINK_SPEED_20GB=1<<5, VIRTCHNL_LINK_SPEED_25GB=1<<6, VIRTCHNL_LINK_SPEED_2_5GB=1, VIRTCHNL_LINK_SPEED_5GB=1<<7 }
#[repr(u32)] pub enum virtchnl_rx_hsplit { VIRTCHNL_RX_HSPLIT_NO_SPLIT=0, VIRTCHNL_RX_HSPLIT_SPLIT_L2=1, VIRTCHNL_RX_HSPLIT_SPLIT_IP=2, VIRTCHNL_RX_HSPLIT_SPLIT_TCP_UDP=4, VIRTCHNL_RX_HSPLIT_SPLIT_SCTP=8 }
#[repr(u32)] pub enum virtchnl_bw_limit_type { VIRTCHNL_BW_SHAPER=0 }
#[repr(u32)] pub enum virtchnl_ops { VIRTCHNL_OP_UNKNOWN=0,VIRTCHNL_OP_VERSION=1,VIRTCHNL_OP_RESET_VF,VIRTCHNL_OP_GET_VF_RESOURCES,VIRTCHNL_OP_CONFIG_TX_QUEUE,VIRTCHNL_OP_CONFIG_RX_QUEUE,VIRTCHNL_OP_CONFIG_VSI_QUEUES,VIRTCHNL_OP_CONFIG_IRQ_MAP,VIRTCHNL_OP_ENABLE_QUEUES,VIRTCHNL_OP_DISABLE_QUEUES,VIRTCHNL_OP_ADD_ETH_ADDR,VIRTCHNL_OP_DEL_ETH_ADDR,VIRTCHNL_OP_ADD_VLAN,VIRTCHNL_OP_DEL_VLAN,VIRTCHNL_OP_CONFIG_PROMISCUOUS_MODE,VIRTCHNL_OP_GET_STATS,VIRTCHNL_OP_RSVD,VIRTCHNL_OP_EVENT,VIRTCHNL_OP_CONFIG_RSS_HFUNC=18,VIRTCHNL_OP_IWARP=20,VIRTCHNL_OP_CONFIG_IWARP_IRQ_MAP,VIRTCHNL_OP_RELEASE_IWARP_IRQ_MAP,VIRTCHNL_OP_CONFIG_RSS_KEY,VIRTCHNL_OP_CONFIG_RSS_LUT,VIRTCHNL_OP_GET_RSS_HASHCFG_CAPS,VIRTCHNL_OP_SET_RSS_HASHCFG,VIRTCHNL_OP_ENABLE_VLAN_STRIPPING,VIRTCHNL_OP_DISABLE_VLAN_STRIPPING,VIRTCHNL_OP_REQUEST_QUEUES,VIRTCHNL_OP_ENABLE_CHANNELS,VIRTCHNL_OP_DISABLE_CHANNELS,VIRTCHNL_OP_ADD_CLOUD_FILTER,VIRTCHNL_OP_DEL_CLOUD_FILTER,VIRTCHNL_OP_GET_SUPPORTED_RXDIDS=44,VIRTCHNL_OP_ADD_RSS_CFG,VIRTCHNL_OP_DEL_RSS_CFG,VIRTCHNL_OP_ADD_FDIR_FILTER,VIRTCHNL_OP_DEL_FDIR_FILTER,VIRTCHNL_OP_GET_OFFLOAD_VLAN_V2_CAPS=51,VIRTCHNL_OP_ADD_VLAN_V2,VIRTCHNL_OP_DEL_VLAN_V2,VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2,VIRTCHNL_OP_DISABLE_VLAN_STRIPPING_V2,VIRTCHNL_OP_ENABLE_VLAN_INSERTION_V2,VIRTCHNL_OP_DISABLE_VLAN_INSERTION_V2,VIRTCHNL_OP_1588_PTP_GET_CAPS=60,VIRTCHNL_OP_1588_PTP_GET_TIME,VIRTCHNL_OP_GET_QOS_CAPS=66,VIRTCHNL_OP_CONFIG_QUEUE_BW=112,VIRTCHNL_OP_CONFIG_QUANTA,VIRTCHNL_OP_MAX }

pub const VIRTCHNL_VERSION_MAJOR:u32=1; pub const VIRTCHNL_VERSION_MINOR:u32=1; pub const VIRTCHNL_VERSION_MINOR_NO_VF_CAPS:u32=0;
#[repr(C)] #[derive(Copy,Clone)] pub struct virtchnl_version_info { pub major:u32,pub minor:u32 }
#[repr(u32)] pub enum virtchnl_vsi_type { VIRTCHNL_VSI_TYPE_INVALID=0,VIRTCHNL_VSI_SRIOV=6 }
#[repr(C)] pub struct virtchnl_vsi_resource { pub vsi_id:u16,pub num_queue_pairs:u16,pub vsi_type:i32,pub qset_handle:u16,pub default_mac_addr:[u8;6] }
pub const VIRTCHNL_VF_OFFLOAD_L2:u32=1<<0; pub const VIRTCHNL_VF_OFFLOAD_RDMA:u32=1<<1; pub const VIRTCHNL_VF_CAP_RDMA:u32=1<<1; pub const VIRTCHNL_VF_OFFLOAD_RSS_AQ:u32=1<<3; pub const VIRTCHNL_VF_OFFLOAD_RSS_REG:u32=1<<4; pub const VIRTCHNL_VF_OFFLOAD_WB_ON_ITR:u32=1<<5; pub const VIRTCHNL_VF_OFFLOAD_REQ_QUEUES:u32=1<<6; pub const VIRTCHNL_VF_CAP_ADV_LINK_SPEED:u32=1<<7; pub const VIRTCHNL_VF_OFFLOAD_CRC:u32=1<<10; pub const VIRTCHNL_VF_OFFLOAD_TC_U32:u32=1<<11; pub const VIRTCHNL_VF_OFFLOAD_VLAN_V2:u32=1<<15; pub const VIRTCHNL_VF_OFFLOAD_VLAN:u32=1<<16; pub const VIRTCHNL_VF_OFFLOAD_RX_POLLING:u32=1<<17; pub const VIRTCHNL_VF_OFFLOAD_RSS_PCTYPE_V2:u32=1<<18; pub const VIRTCHNL_VF_OFFLOAD_RSS_PF:u32=1<<19; pub const VIRTCHNL_VF_OFFLOAD_ENCAP:u32=1<<20; pub const VIRTCHNL_VF_OFFLOAD_ENCAP_CSUM:u32=1<<21; pub const VIRTCHNL_VF_OFFLOAD_RX_ENCAP_CSUM:u32=1<<22; pub const VIRTCHNL_VF_OFFLOAD_ADQ:u32=1<<23; pub const VIRTCHNL_VF_OFFLOAD_USO:u32=1<<25; pub const VIRTCHNL_VF_OFFLOAD_RX_FLEX_DESC:u32=1<<26; pub const VIRTCHNL_VF_OFFLOAD_ADV_RSS_PF:u32=1<<27; pub const VIRTCHNL_VF_OFFLOAD_FDIR_PF:u32=1<<28; pub const VIRTCHNL_VF_OFFLOAD_QOS:u32=1<<29; pub const VIRTCHNL_VF_CAP_PTP:u32=1<<31;
pub const VF_BASE_MODE_OFFLOADS:u32=VIRTCHNL_VF_OFFLOAD_L2|VIRTCHNL_VF_OFFLOAD_VLAN|VIRTCHNL_VF_OFFLOAD_RSS_PF;
#[repr(C)] pub struct virtchnl_vf_resource { pub num_vsis:u16,pub num_queue_pairs:u16,pub max_vectors:u16,pub max_mtu:u16,pub vf_cap_flags:u32,pub rss_key_size:u32,pub rss_lut_size:u32,pub vsi_res:[virtchnl_vsi_resource;0] }
#[repr(C)] pub struct virtchnl_txq_info { pub vsi_id:u16,pub queue_id:u16,pub ring_len:u16,pub headwb_enabled:u16,pub dma_ring_addr:u64,pub dma_headwb_addr:u64 }
#[repr(u32)] pub enum virtchnl_rx_desc_ids { VIRTCHNL_RXDID_0_16B_BASE=0,VIRTCHNL_RXDID_1_32B_BASE,VIRTCHNL_RXDID_2_FLEX_SQ_NIC,VIRTCHNL_RXDID_3_FLEX_SQ_SW,VIRTCHNL_RXDID_4_FLEX_SQ_NIC_VEB,VIRTCHNL_RXDID_5_FLEX_SQ_NIC_ACL,VIRTCHNL_RXDID_6_FLEX_SQ_NIC_2,VIRTCHNL_RXDID_7_HW_RSVD,VIRTCHNL_RXDID_16_COMMS_GENERIC=16,VIRTCHNL_RXDID_17_COMMS_AUX_VLAN,VIRTCHNL_RXDID_18_COMMS_AUX_IPV4,VIRTCHNL_RXDID_19_COMMS_AUX_IPV6,VIRTCHNL_RXDID_20_COMMS_AUX_FLOW,VIRTCHNL_RXDID_21_COMMS_AUX_TCP }
#[repr(C)] pub struct virtchnl_rxq_info { pub vsi_id:u16,pub queue_id:u16,pub ring_len:u32,pub hdr_size:u16,pub splithdr_enabled:u16,pub databuffer_size:u32,pub max_pkt_size:u32,pub crc_disable:u8,pub rxdid:u8,pub flags:u8,pub pad1:u8,pub dma_ring_addr:u64,pub rx_split_pos:i32,pub pad2:u32 }
#[repr(C)] pub struct virtchnl_queue_pair_info { pub txq:virtchnl_txq_info,pub rxq:virtchnl_rxq_info }
#[repr(C)] pub struct virtchnl_vsi_queue_config_info { pub vsi_id:u16,pub num_queue_pairs:u16,pub pad:u32,pub qpair:[virtchnl_queue_pair_info;0] }
#[repr(C)] pub struct virtchnl_vf_res_request { pub num_queue_pairs:u16 }
#[repr(C)] pub struct virtchnl_vector_map { pub vsi_id:u16,pub vector_id:u16,pub rxq_map:u16,pub txq_map:u16,pub rxitr_idx:u16,pub txitr_idx:u16 }
#[repr(C)] pub struct virtchnl_irq_map_info { pub num_vectors:u16,pub vecmap:[virtchnl_vector_map;0] }
#[repr(C)] pub struct virtchnl_queue_select { pub vsi_id:u16,pub pad:u16,pub rx_queues:u32,pub tx_queues:u32 }
#[repr(C)] pub struct virtchnl_ether_addr { pub addr:[u8;6],pub r#type:u8,pub pad:u8 }
pub const VIRTCHNL_ETHER_ADDR_LEGACY:u8=0; pub const VIRTCHNL_ETHER_ADDR_PRIMARY:u8=1; pub const VIRTCHNL_ETHER_ADDR_EXTRA:u8=2; pub const VIRTCHNL_ETHER_ADDR_TYPE_MASK:u8=3;
#[repr(C)] pub struct virtchnl_ether_addr_list { pub vsi_id:u16,pub num_elements:u16,pub list:[virtchnl_ether_addr;0] }
#[repr(C)] pub struct virtchnl_vlan_filter_list { pub vsi_id:u16,pub num_elements:u16,pub vlan_id:[u16;0] }
#[repr(C)] pub struct virtchnl_vlan_supported_caps { pub outer:u32,pub inner:u32 }
#[repr(C)] pub struct virtchnl_vlan_filtering_caps { pub filtering_support:virtchnl_vlan_supported_caps,pub ethertype_init:u32,pub max_filters:u16,pub pad:[u8;2] }
#[repr(C)] pub struct virtchnl_vlan_offload_caps { pub stripping_support:virtchnl_vlan_supported_caps,pub insertion_support:virtchnl_vlan_supported_caps,pub ethertype_init:u32,pub ethertype_match:u8,pub pad:[u8;3] }
#[repr(C)] pub struct virtchnl_vlan_caps { pub filtering:virtchnl_vlan_filtering_caps,pub offloads:virtchnl_vlan_offload_caps }
#[repr(C)] pub struct virtchnl_vlan { pub tci:u16,pub tci_mask:u16,pub tpid:u16,pub pad:[u8;2] }
#[repr(C)] pub struct virtchnl_vlan_filter { pub inner:virtchnl_vlan,pub outer:virtchnl_vlan,pub pad:[u8;16] }
#[repr(C)] pub struct virtchnl_vlan_filter_list_v2 { pub vport_id:u16,pub num_elements:u16,pub pad:[u8;4],pub filters:[virtchnl_vlan_filter;0] }
#[repr(C)] pub struct virtchnl_vlan_setting { pub outer_ethertype_setting:u32,pub inner_ethertype_setting:u32,pub vport_id:u16,pub pad:[u8;6] }
#[repr(u32)] pub enum virtchnl_vlan_support { VIRTCHNL_VLAN_UNSUPPORTED=0,VIRTCHNL_VLAN_ETHERTYPE_8100=1,VIRTCHNL_VLAN_ETHERTYPE_88A8=2,VIRTCHNL_VLAN_ETHERTYPE_9100=4,VIRTCHNL_VLAN_TAG_LOCATION_L2TAG1=1<<8,VIRTCHNL_VLAN_TAG_LOCATION_L2TAG2=1<<9,VIRTCHNL_VLAN_TAG_LOCATION_L2TAG2_2=1<<10,VIRTCHNL_VLAN_PRIO=1<<24,VIRTCHNL_VLAN_FILTER_MASK=1<<28,VIRTCHNL_VLAN_ETHERTYPE_AND=1<<29,VIRTCHNL_VLAN_ETHERTYPE_XOR=1<<30,VIRTCHNL_VLAN_TOGGLE=1<<31 }
#[repr(C)] pub struct virtchnl_promisc_info { pub vsi_id:u16,pub flags:u16 }
pub const FLAG_VF_UNICAST_PROMISC:u32=1; pub const FLAG_VF_MULTICAST_PROMISC:u32=2;
#[repr(C)] pub struct virtchnl_rss_key { pub vsi_id:u16,pub key_len:u16,pub key:[u8;0] }
#[repr(C)] pub struct virtchnl_rss_lut { pub vsi_id:u16,pub lut_entries:u16,pub lut:[u8;0] }
#[repr(C)] pub struct virtchnl_rss_hashcfg { pub hashcfg:u64 }
#[repr(C)] pub struct virtchnl_rss_hfunc { pub vsi_id:u16,pub rss_algorithm:u16,pub reserved:u32 }
#[repr(C)] pub struct virtchnl_channel_info { pub count:u16,pub offset:u16,pub pad:u32,pub max_tx_rate:u64 }
#[repr(C)] pub struct virtchnl_tc_info { pub num_tc:u32,pub pad:u32,pub list:[virtchnl_channel_info;0] }
#[repr(C)] pub struct virtchnl_l4_spec { pub src_mac:[u8;6],pub dst_mac:[u8;6],pub vlan_id:u16,pub pad:u16,pub src_ip:[u32;4],pub dst_ip:[u32;4],pub src_port:u16,pub dst_port:u16 }
#[repr(C)] pub union virtchnl_flow_spec { pub tcp_spec:virtchnl_l4_spec,pub buffer:[u8;128] }
#[repr(u32)] pub enum virtchnl_action { VIRTCHNL_ACTION_DROP=0,VIRTCHNL_ACTION_TC_REDIRECT,VIRTCHNL_ACTION_PASSTHRU,VIRTCHNL_ACTION_QUEUE,VIRTCHNL_ACTION_Q_REGION,VIRTCHNL_ACTION_MARK,VIRTCHNL_ACTION_COUNT }
#[repr(C)] pub struct virtchnl_filter { pub data:virtchnl_flow_spec,pub mask:virtchnl_flow_spec,pub flow_type:i32,pub action:i32,pub action_meta:u32,pub field_flags:u8,pub pad:[u8;3] }
#[repr(C)] pub union virtchnl_event_data { pub link_event:virtchnl_link_event,pub link_event_adv:virtchnl_link_event_adv }
#[repr(C)] pub struct virtchnl_link_event { pub link_speed:virtchnl_link_speed,pub link_status:bool,pub pad:[u8;3] }
#[repr(C)] pub struct virtchnl_link_event_adv { pub link_speed:u32,pub link_status:u8,pub pad:[u8;3] }
#[repr(C)] pub struct virtchnl_pf_event { pub event:i32,pub event_data:virtchnl_event_data,pub severity:i32 }
#[repr(C)] pub struct virtchnl_rdma_qv_info { pub v_idx:u32,pub ceq_idx:u16,pub aeq_idx:u16,pub itr_idx:u8,pub pad:[u8;3] }
#[repr(C)] pub struct virtchnl_rdma_qvlist_info { pub num_vectors:u32,pub qv_info:[virtchnl_rdma_qv_info;0] }
#[repr(C)] pub struct virtchnl_ptp_caps { pub caps:u32,pub rsvd:[u8;44] }
#[repr(C)] pub struct virtchnl_phc_time { pub time:u64,pub rsvd:[u8;8] }
#[repr(C)] pub struct virtchnl_shaper_bw { pub committed:u32,pub peak:u32 }
#[repr(C)] pub union virtchnl_qos_cap_elem_union { pub shaper:virtchnl_shaper_bw,pub pad2:[u8;32] }
#[repr(C)] pub struct virtchnl_qos_cap_elem { pub tc_num:u8,pub tc_prio:u8,pub arbiter:u8,pub weight:u8,pub r#type:virtchnl_bw_limit_type,pub act:virtchnl_qos_cap_elem_union }
#[repr(C)] pub struct virtchnl_qos_cap_list { pub vsi_id:u16,pub num_elem:u16,pub cap:[virtchnl_qos_cap_elem;0] }
#[repr(C)] pub struct virtchnl_queue_bw { pub queue_id:u16,pub tc:u8,pub pad:u8,pub shaper:virtchnl_shaper_bw }
#[repr(C)] pub struct virtchnl_queues_bw_cfg { pub vsi_id:u16,pub num_queues:u16,pub cfg:[virtchnl_queue_bw;0] }
#[repr(u32)] pub enum virtchnl_queue_type { VIRTCHNL_QUEUE_TYPE_TX=0,VIRTCHNL_QUEUE_TYPE_RX=1 }
#[repr(C)] pub struct virtchnl_queue_chunk { pub r#type:i32,pub start_queue_id:u16,pub num_queues:u16 }
#[repr(C)] pub struct virtchnl_quanta_cfg { pub quanta_size:u16,pub pad:u16,pub queue_select:virtchnl_queue_chunk }
pub const VIRTCHNL_RDMA_INVALID_QUEUE_IDX:u16=0xffff; pub const VIRTCHNL_MAX_NUM_PROTO_HDRS:usize=32; pub const VIRTCHNL_MAX_SIZE_RAW_PACKET:usize=1024; pub const PROTO_HDR_SHIFT:u32=5;
pub const VIRTCHNL_1588_PTP_CAP_RX_TSTAMP:u32=1<<1; pub const VIRTCHNL_1588_PTP_CAP_READ_PHC:u32=1<<2;

#[repr(u32)] pub enum virtchnl_rss_algorithm { VIRTCHNL_RSS_ALG_TOEPLITZ_ASYMMETRIC=0,VIRTCHNL_RSS_ALG_R_ASYMMETRIC,VIRTCHNL_RSS_ALG_TOEPLITZ_SYMMETRIC,VIRTCHNL_RSS_ALG_XOR_SYMMETRIC }
#[repr(C)] pub struct virtchnl_proto_hdr { pub r#type:i32,pub field_selector:u32,pub buffer:[u8;64] }
#[repr(C)] pub union virtchnl_proto_hdrs_union { pub proto_hdr:[virtchnl_proto_hdr;32],pub raw:virtchnl_raw_packet }
#[repr(C)] pub struct virtchnl_raw_packet { pub pkt_len:u16,pub spec:[u8;1024],pub mask:[u8;1024] }
#[repr(C)] pub struct virtchnl_proto_hdrs { pub tunnel_level:u8,pub pad:[u8;3],pub count:u32,pub data:virtchnl_proto_hdrs_union }
#[repr(C)] pub struct virtchnl_rss_cfg { pub proto_hdrs:virtchnl_proto_hdrs,pub rss_algorithm:i32,pub reserved:[u8;128] }
#[repr(C)] pub union virtchnl_filter_action_conf { pub queue:virtchnl_action_queue,pub count:virtchnl_action_count,pub mark_id:u32,pub reserve:[u8;32] }
#[repr(C)] pub struct virtchnl_action_queue { pub index:u16,pub region:u8 }
#[repr(C)] pub struct virtchnl_action_count { pub shared:u8,pub id:u32 }
#[repr(C)] pub struct virtchnl_filter_action { pub r#type:i32,pub act_conf:virtchnl_filter_action_conf }
#[repr(C)] pub struct virtchnl_filter_action_set { pub count:u32,pub actions:[virtchnl_filter_action;8] }
#[repr(C)] pub struct virtchnl_fdir_rule { pub proto_hdrs:virtchnl_proto_hdrs,pub action_set:virtchnl_filter_action_set }
#[repr(C)] pub struct virtchnl_fdir_add { pub vsi_id:u16,pub validate_only:u16,pub flow_id:u32,pub rule_cfg:virtchnl_fdir_rule,pub status:i32 }
#[repr(C)] pub struct virtchnl_fdir_del { pub vsi_id:u16,pub pad:u16,pub flow_id:u32,pub status:i32 }
#[repr(u32)] pub enum virtchnl_fdir_prgm_status { VIRTCHNL_FDIR_SUCCESS=0,VIRTCHNL_FDIR_FAILURE_RULE_NORESOURCE,VIRTCHNL_FDIR_FAILURE_RULE_EXIST,VIRTCHNL_FDIR_FAILURE_RULE_CONFLICT,VIRTCHNL_FDIR_FAILURE_RULE_NONEXIST,VIRTCHNL_FDIR_FAILURE_RULE_INVALID,VIRTCHNL_FDIR_FAILURE_RULE_TIMEOUT,VIRTCHNL_FDIR_FAILURE_QUERY_INVALID }
#[repr(u32)] pub enum virtchnl_event_codes { VIRTCHNL_EVENT_UNKNOWN=0,VIRTCHNL_EVENT_LINK_CHANGE,VIRTCHNL_EVENT_RESET_IMPENDING,VIRTCHNL_EVENT_PF_DRIVER_CLOSE }
pub const PF_EVENT_SEVERITY_INFO:i32=0; pub const PF_EVENT_SEVERITY_CERTAIN_DOOM:i32=255;
#[repr(u32)] pub enum virtchnl_vfr_states { VIRTCHNL_VFR_INPROGRESS=0,VIRTCHNL_VFR_COMPLETED,VIRTCHNL_VFR_VFACTIVE }
pub const VIRTCHNL_MAX_NUM_ACTIONS:usize=8; pub const VIRTCHNL_ABITER_STRICT:u8=0; pub const VIRTCHNL_ABITER_ETS:u8=2; pub const VIRTCHNL_STRICT_WEIGHT:u8=1;
pub const virtchnl_vf_resource_LEGACY_SIZEOF:usize=36; pub const virtchnl_vsi_queue_config_info_LEGACY_SIZEOF:usize=72; pub const virtchnl_irq_map_info_LEGACY_SIZEOF:usize=14; pub const virtchnl_ether_addr_list_LEGACY_SIZEOF:usize=12; pub const virtchnl_vlan_filter_list_LEGACY_SIZEOF:usize=6; pub const virtchnl_vlan_filter_list_v2_LEGACY_SIZEOF:usize=40; pub const virtchnl_tc_info_LEGACY_SIZEOF:usize=24; pub const virtchnl_rdma_qvlist_info_LEGACY_SIZEOF:usize=16; pub const virtchnl_qos_cap_list_LEGACY_SIZEOF:usize=44; pub const virtchnl_queues_bw_cfg_LEGACY_SIZEOF:usize=16; pub const virtchnl_rss_key_LEGACY_SIZEOF:usize=6; pub const virtchnl_rss_lut_LEGACY_SIZEOF:usize=6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
