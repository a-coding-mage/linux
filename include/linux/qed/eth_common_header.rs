/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017 QLogic Corporation.
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

pub const ETH_HSI_VER_MAJOR: u32 = 3;
pub const ETH_HSI_VER_MINOR: u32 = 11;
pub const ETH_HSI_VER_NO_PKT_LEN_TUNN: u32 = 5;
pub const ETH_PINNED_CONN_MAX_NUM: u32 = 32;
pub const ETH_CACHE_LINE_SIZE: u32 = 64;
pub const ETH_RX_CQE_GAP: u32 = 32;
pub const ETH_MAX_RAMROD_PER_CON: u32 = 8;
pub const ETH_TX_BD_PAGE_SIZE_BYTES: u32 = 4096;
pub const ETH_RX_BD_PAGE_SIZE_BYTES: u32 = 4096;
pub const ETH_RX_CQE_PAGE_SIZE_BYTES: u32 = 4096;
pub const ETH_RX_NUM_NEXT_PAGE_BDS: u32 = 2;
pub const ETH_MAX_TUNN_LSO_INNER_IPV4_OFFSET: u32 = 253;
pub const ETH_MAX_TUNN_LSO_INNER_IPV6_OFFSET: u32 = 251;
pub const ETH_TX_MIN_BDS_PER_NON_LSO_PKT: u32 = 1;
pub const ETH_TX_MAX_BDS_PER_NON_LSO_PACKET: u32 = 18;
pub const ETH_TX_MAX_BDS_PER_LSO_PACKET: u32 = 255;
pub const ETH_TX_MAX_LSO_HDR_NBD: u32 = 4;
pub const ETH_TX_MIN_BDS_PER_LSO_PKT: u32 = 3;
pub const ETH_TX_MIN_BDS_PER_TUNN_IPV6_WITH_EXT_PKT: u32 = 3;
pub const ETH_TX_MIN_BDS_PER_IPV6_WITH_EXT_PKT: u32 = 2;
pub const ETH_TX_MIN_BDS_PER_PKT_W_LOOPBACK_MODE: u32 = 2;
pub const ETH_TX_MIN_BDS_PER_PKT_W_VPORT_FORWARDING: u32 = 4;
pub const ETH_TX_MAX_NON_LSO_PKT_LEN: u32 = 9700 - (4 + 4 + 12 + 8);
pub const ETH_TX_MAX_LSO_HDR_BYTES: u32 = 510;
pub const ETH_TX_LSO_WINDOW_BDS_NUM: u32 = 18 - 1;
pub const ETH_TX_LSO_WINDOW_MIN_LEN: u32 = 9700;
pub const ETH_TX_MAX_LSO_PAYLOAD_LEN: u32 = 0xFE000;
pub const ETH_TX_NUM_SAME_AS_LAST_ENTRIES: u32 = 320;
pub const ETH_TX_INACTIVE_SAME_AS_LAST: u32 = 0xFFFF;
pub const ETH_NUM_STATISTIC_COUNTERS: u32 = MAX_NUM_VPORTS;
pub const ETH_NUM_STATISTIC_COUNTERS_DOUBLE_VF_ZONE: u32 = ETH_NUM_STATISTIC_COUNTERS - MAX_NUM_VFS / 2;
pub const ETH_NUM_STATISTIC_COUNTERS_QUAD_VF_ZONE: u32 = ETH_NUM_STATISTIC_COUNTERS - 3 * MAX_NUM_VFS / 4;
pub const ETH_RX_MAX_BUFF_PER_PKT: u32 = 5;
pub const ETH_RX_BD_THRESHOLD: u32 = 16;
pub const ETH_NUM_MAC_FILTERS: u32 = 512;
pub const ETH_NUM_VLAN_FILTERS: u32 = 512;
pub const ETH_MULTICAST_BIN_FROM_MAC_SEED: u32 = 0;
pub const ETH_MULTICAST_MAC_BINS: u32 = 256;
pub const ETH_MULTICAST_MAC_BINS_IN_REGS: u32 = ETH_MULTICAST_MAC_BINS / 32;
pub const ETH_FILTER_RULES_COUNT: u32 = 10;
pub const ETH_RSS_IND_TABLE_ENTRIES_NUM: u32 = 128;
pub const ETH_RSS_IND_TABLE_MASK_SIZE_REGS: u32 = ETH_RSS_IND_TABLE_ENTRIES_NUM / 32;
pub const ETH_RSS_KEY_SIZE_REGS: u32 = 10;
pub const ETH_RSS_ENGINE_NUM_K2: u32 = 207;
pub const ETH_RSS_ENGINE_NUM_BB: u32 = 127;
pub const ETH_TPA_MAX_AGGS_NUM: u32 = 64;
pub const ETH_TPA_CQE_START_BW_LEN_LIST_SIZE: usize = 2;
pub const ETH_TPA_CQE_CONT_LEN_LIST_SIZE: usize = 6;
pub const ETH_TPA_CQE_END_LEN_LIST_SIZE: usize = 4;
pub const ETH_CTL_FRAME_ETH_TYPE_NUM: u32 = 4;
pub const ETH_GFT_TRASHCAN_VPORT: u32 = 0x1FF;

/* Bitfield masks and shifts retained from the C header. */
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_L2_HDR_SIZE_W_MASK: u16 = 0xF;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_L2_HDR_SIZE_W_SHIFT: u16 = 0;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_ETH_TYPE_MASK: u16 = 0x3;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_ETH_TYPE_SHIFT: u16 = 4;
pub const ETH_TX_DATA_2ND_BD_DST_PORT_MODE_MASK: u16 = 0x3;
pub const ETH_TX_DATA_2ND_BD_DST_PORT_MODE_SHIFT: u16 = 6;
pub const ETH_TX_DATA_2ND_BD_START_BD_MASK: u16 = 0x1;
pub const ETH_TX_DATA_2ND_BD_START_BD_SHIFT: u16 = 8;
pub const ETH_TX_DATA_2ND_BD_TUNN_TYPE_MASK: u16 = 0x3;
pub const ETH_TX_DATA_2ND_BD_TUNN_TYPE_SHIFT: u16 = 9;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_IPV6_MASK: u16 = 0x1;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_IPV6_SHIFT: u16 = 11;
pub const ETH_TX_DATA_2ND_BD_IPV6_EXT_MASK: u16 = 0x1;
pub const ETH_TX_DATA_2ND_BD_IPV6_EXT_SHIFT: u16 = 12;
pub const ETH_TX_DATA_2ND_BD_TUNN_IPV6_EXT_MASK: u16 = 0x1;
pub const ETH_TX_DATA_2ND_BD_TUNN_IPV6_EXT_SHIFT: u16 = 13;
pub const ETH_TX_DATA_2ND_BD_L4_UDP_MASK: u16 = 0x1;
pub const ETH_TX_DATA_2ND_BD_L4_UDP_SHIFT: u16 = 14;
pub const ETH_TX_DATA_2ND_BD_L4_PSEUDO_CSUM_MODE_MASK: u16 = 0x1;
pub const ETH_TX_DATA_2ND_BD_L4_PSEUDO_CSUM_MODE_SHIFT: u16 = 15;
pub const ETH_TX_DATA_2ND_BD_L4_HDR_START_OFFSET_W_MASK: u16 = 0x1FFF;
pub const ETH_TX_DATA_2ND_BD_L4_HDR_START_OFFSET_W_SHIFT: u16 = 0;
pub const ETH_TX_DATA_2ND_BD_RESERVED0_MASK: u16 = 0x7;
pub const ETH_TX_DATA_2ND_BD_RESERVED0_SHIFT: u16 = 13;
pub const ETH_PMD_FLOW_FLAGS_VALID_MASK: u8 = 1; pub const ETH_PMD_FLOW_FLAGS_VALID_SHIFT: u8 = 0;
pub const ETH_PMD_FLOW_FLAGS_TOGGLE_MASK: u8 = 1; pub const ETH_PMD_FLOW_FLAGS_TOGGLE_SHIFT: u8 = 1;
pub const ETH_PMD_FLOW_FLAGS_RESERVED_MASK: u8 = 0x3F; pub const ETH_PMD_FLOW_FLAGS_RESERVED_SHIFT: u8 = 2;
pub const ETH_DB_DATA_DEST_MASK: u8 = 0x3; pub const ETH_DB_DATA_DEST_SHIFT: u8 = 0;
pub const ETH_DB_DATA_AGG_CMD_MASK: u8 = 0x3; pub const ETH_DB_DATA_AGG_CMD_SHIFT: u8 = 2;
pub const ETH_DB_DATA_BYPASS_EN_MASK: u8 = 1; pub const ETH_DB_DATA_BYPASS_EN_SHIFT: u8 = 4;
pub const ETH_DB_DATA_RESERVED_MASK: u8 = 1; pub const ETH_DB_DATA_RESERVED_SHIFT: u8 = 5;
pub const ETH_DB_DATA_AGG_VAL_SEL_MASK: u8 = 0x3; pub const ETH_DB_DATA_AGG_VAL_SEL_SHIFT: u8 = 6;
pub const ETH_TUNNEL_PARSING_FLAGS_TYPE_MASK: u8 = 0x3; pub const ETH_TUNNEL_PARSING_FLAGS_TYPE_SHIFT: u8 = 0;
pub const ETH_TUNNEL_PARSING_FLAGS_TENNANT_ID_EXIST_MASK: u8 = 1; pub const ETH_TUNNEL_PARSING_FLAGS_TENNANT_ID_EXIST_SHIFT: u8 = 2;
pub const ETH_TUNNEL_PARSING_FLAGS_NEXT_PROTOCOL_MASK: u8 = 0x3; pub const ETH_TUNNEL_PARSING_FLAGS_NEXT_PROTOCOL_SHIFT: u8 = 3;
pub const ETH_TUNNEL_PARSING_FLAGS_FIRSTHDRIPMATCH_MASK: u8 = 1; pub const ETH_TUNNEL_PARSING_FLAGS_FIRSTHDRIPMATCH_SHIFT: u8 = 5;
pub const ETH_TUNNEL_PARSING_FLAGS_IPV4_FRAGMENT_MASK: u8 = 1; pub const ETH_TUNNEL_PARSING_FLAGS_IPV4_FRAGMENT_SHIFT: u8 = 6;
pub const ETH_TUNNEL_PARSING_FLAGS_IPV4_OPTIONS_MASK: u8 = 1; pub const ETH_TUNNEL_PARSING_FLAGS_IPV4_OPTIONS_SHIFT: u8 = 7;
pub const ETH_TX_DATA_3RD_BD_TCP_HDR_LEN_DW_MASK: u16 = 0xF; pub const ETH_TX_DATA_3RD_BD_TCP_HDR_LEN_DW_SHIFT: u16 = 0;
pub const ETH_TX_DATA_3RD_BD_HDR_NBD_MASK: u16 = 0xF; pub const ETH_TX_DATA_3RD_BD_HDR_NBD_SHIFT: u16 = 4;
pub const ETH_TX_DATA_3RD_BD_START_BD_MASK: u16 = 1; pub const ETH_TX_DATA_3RD_BD_START_BD_SHIFT: u16 = 8;
pub const ETH_TX_DATA_3RD_BD_RESERVED0_MASK: u16 = 0x7F; pub const ETH_TX_DATA_3RD_BD_RESERVED0_SHIFT: u16 = 9;
pub const ETH_TX_DATA_4TH_BD_DST_VPORT_ID_VALID_MASK: u16 = 1; pub const ETH_TX_DATA_4TH_BD_DST_VPORT_ID_VALID_SHIFT: u16 = 0;
pub const ETH_TX_DATA_4TH_BD_RESERVED1_MASK: u16 = 0x7F; pub const ETH_TX_DATA_4TH_BD_RESERVED1_SHIFT: u16 = 1;
pub const ETH_TX_DATA_4TH_BD_START_BD_MASK: u16 = 1; pub const ETH_TX_DATA_4TH_BD_START_BD_SHIFT: u16 = 8;
pub const ETH_TX_DATA_4TH_BD_RESERVED2_MASK: u16 = 0x7F; pub const ETH_TX_DATA_4TH_BD_RESERVED2_SHIFT: u16 = 9;
pub const ETH_TX_DATA_BD_RESERVED1_MASK: u16 = 0xFF; pub const ETH_TX_DATA_BD_RESERVED1_SHIFT: u16 = 0;
pub const ETH_TX_DATA_BD_START_BD_MASK: u16 = 1; pub const ETH_TX_DATA_BD_START_BD_SHIFT: u16 = 8;
pub const ETH_TX_DATA_BD_RESERVED2_MASK: u16 = 0x7F; pub const ETH_TX_DATA_BD_RESERVED2_SHIFT: u16 = 9;
pub const ETH_FAST_PATH_RX_REG_CQE_RSS_HASH_TYPE_MASK: u8 = 0x7; pub const ETH_FAST_PATH_RX_REG_CQE_RSS_HASH_TYPE_SHIFT: u8 = 0;
pub const ETH_FAST_PATH_RX_REG_CQE_TC_MASK: u8 = 0xF; pub const ETH_FAST_PATH_RX_REG_CQE_TC_SHIFT: u8 = 3;
pub const ETH_FAST_PATH_RX_REG_CQE_RESERVED0_MASK: u8 = 1; pub const ETH_FAST_PATH_RX_REG_CQE_RESERVED0_SHIFT: u8 = 7;
pub const ETH_FAST_PATH_RX_TPA_START_CQE_RSS_HASH_TYPE_MASK: u8 = 0x7; pub const ETH_FAST_PATH_RX_TPA_START_CQE_RSS_HASH_TYPE_SHIFT: u8 = 0;
pub const ETH_FAST_PATH_RX_TPA_START_CQE_TC_MASK: u8 = 0xF; pub const ETH_FAST_PATH_RX_TPA_START_CQE_TC_SHIFT: u8 = 3;
pub const ETH_FAST_PATH_RX_TPA_START_CQE_RESERVED0_MASK: u8 = 1; pub const ETH_FAST_PATH_RX_TPA_START_CQE_RESERVED0_SHIFT: u8 = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dst_port_mode { DST_PORT_PHY, DST_PORT_LOOPBACK, DST_PORT_PHY_LOOPBACK, DST_PORT_DROP, MAX_DST_PORT_MODE }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum eth_addr_type { BROADCAST_ADDRESS, MULTICAST_ADDRESS, UNICAST_ADDRESS, UNKNOWN_ADDRESS, MAX_ETH_ADDR_TYPE }

#[repr(C)] pub struct eth_tx_1st_bd_flags { pub bitfields: u8 }
pub const ETH_TX_1ST_BD_FLAGS_START_BD_MASK: u8 = 0x1; pub const ETH_TX_1ST_BD_FLAGS_START_BD_SHIFT: u8 = 0;
pub const ETH_TX_1ST_BD_FLAGS_FORCE_VLAN_MODE_MASK: u8 = 0x1; pub const ETH_TX_1ST_BD_FLAGS_FORCE_VLAN_MODE_SHIFT: u8 = 1;
pub const ETH_TX_1ST_BD_FLAGS_IP_CSUM_MASK: u8 = 0x1; pub const ETH_TX_1ST_BD_FLAGS_IP_CSUM_SHIFT: u8 = 2;
pub const ETH_TX_1ST_BD_FLAGS_L4_CSUM_MASK: u8 = 0x1; pub const ETH_TX_1ST_BD_FLAGS_L4_CSUM_SHIFT: u8 = 3;
pub const ETH_TX_1ST_BD_FLAGS_VLAN_INSERTION_MASK: u8 = 0x1; pub const ETH_TX_1ST_BD_FLAGS_VLAN_INSERTION_SHIFT: u8 = 4;
pub const ETH_TX_1ST_BD_FLAGS_LSO_MASK: u8 = 0x1; pub const ETH_TX_1ST_BD_FLAGS_LSO_SHIFT: u8 = 5;
pub const ETH_TX_1ST_BD_FLAGS_TUNN_IP_CSUM_MASK: u8 = 0x1; pub const ETH_TX_1ST_BD_FLAGS_TUNN_IP_CSUM_SHIFT: u8 = 6;
pub const ETH_TX_1ST_BD_FLAGS_TUNN_L4_CSUM_MASK: u8 = 0x1; pub const ETH_TX_1ST_BD_FLAGS_TUNN_L4_CSUM_SHIFT: u8 = 7;

#[repr(C)] pub struct eth_tx_data_1st_bd { pub vlan: __le16, pub nbds: u8, pub bd_flags: eth_tx_1st_bd_flags, pub bitfields: __le16 }
pub const ETH_TX_DATA_1ST_BD_TUNN_FLAG_MASK: u16 = 0x1; pub const ETH_TX_DATA_1ST_BD_TUNN_FLAG_SHIFT: u16 = 0;
pub const ETH_TX_DATA_1ST_BD_RESERVED0_MASK: u16 = 0x1; pub const ETH_TX_DATA_1ST_BD_RESERVED0_SHIFT: u16 = 1;
pub const ETH_TX_DATA_1ST_BD_PKT_LEN_MASK: u16 = 0x3FFF; pub const ETH_TX_DATA_1ST_BD_PKT_LEN_SHIFT: u16 = 2;
#[repr(C)] pub struct eth_tx_data_2nd_bd { pub tunn_ip_size: __le16, pub bitfields1: __le16, pub bitfields2: __le16 }
#[repr(C)] pub struct eth_edpm_fw_data { pub data_1st_bd: eth_tx_data_1st_bd, pub data_2nd_bd: eth_tx_data_2nd_bd, pub reserved: __le32 }
#[repr(C)] pub struct eth_tunnel_parsing_flags { pub flags: u8 }
#[repr(C)] pub struct eth_pmd_flow_flags { pub flags: u8 }
#[repr(C)] pub struct eth_fast_path_rx_reg_cqe { pub type_: u8, pub bitfields: u8, pub pkt_len: __le16, pub pars_flags: parsing_and_err_flags, pub vlan_tag: __le16, pub rss_hash: __le32, pub len_on_first_bd: __le16, pub placement_offset: u8, pub tunnel_pars_flags: eth_tunnel_parsing_flags, pub bd_num: u8, pub reserved: u8, pub reserved2: __le16, pub flow_id_or_resource_id: __le32, pub reserved1: [u8; 7], pub pmd_flags: eth_pmd_flow_flags }
#[repr(C)] pub struct eth_fast_path_rx_tpa_cont_cqe { pub type_: u8, pub tpa_agg_index: u8, pub len_list: [__le16; ETH_TPA_CQE_CONT_LEN_LIST_SIZE], pub reserved: u8, pub reserved1: u8, pub reserved2: [__le16; ETH_TPA_CQE_CONT_LEN_LIST_SIZE], pub reserved3: [u8; 3], pub pmd_flags: eth_pmd_flow_flags }
#[repr(C)] pub struct eth_fast_path_rx_tpa_end_cqe { pub type_: u8, pub tpa_agg_index: u8, pub total_packet_len: __le16, pub num_of_bds: u8, pub end_reason: u8, pub num_of_coalesced_segs: __le16, pub ts_delta: __le32, pub len_list: [__le16; ETH_TPA_CQE_END_LEN_LIST_SIZE], pub reserved3: [__le16; ETH_TPA_CQE_END_LEN_LIST_SIZE], pub reserved1: __le16, pub reserved2: u8, pub pmd_flags: eth_pmd_flow_flags }
#[repr(C)] pub struct eth_fast_path_rx_tpa_start_cqe { pub type_: u8, pub bitfields: u8, pub seg_len: __le16, pub pars_flags: parsing_and_err_flags, pub vlan_tag: __le16, pub rss_hash: __le32, pub len_on_first_bd: __le16, pub placement_offset: u8, pub tunnel_pars_flags: eth_tunnel_parsing_flags, pub tpa_agg_index: u8, pub header_len: u8, pub bw_ext_bd_len_list: [__le16; ETH_TPA_CQE_START_BW_LEN_LIST_SIZE], pub reserved2: __le16, pub flow_id_or_resource_id: __le32, pub reserved: [u8; 3], pub pmd_flags: eth_pmd_flow_flags }
#[repr(C)] pub enum eth_l4_pseudo_checksum_mode { ETH_L4_PSEUDO_CSUM_CORRECT_LENGTH, ETH_L4_PSEUDO_CSUM_ZERO_LENGTH, MAX_ETH_L4_PSEUDO_CHECKSUM_MODE }
#[repr(C)] pub struct eth_rx_bd { pub addr: regpair }
#[repr(C)] pub struct eth_slow_path_rx_cqe { pub type_: u8, pub ramrod_cmd_id: u8, pub error_flag: u8, pub reserved: [u8; 25], pub echo: __le16, pub reserved1: u8, pub pmd_flags: eth_pmd_flow_flags }
#[repr(C)] pub union eth_rx_cqe { pub fast_path_regular: eth_fast_path_rx_reg_cqe, pub fast_path_tpa_start: eth_fast_path_rx_tpa_start_cqe, pub fast_path_tpa_cont: eth_fast_path_rx_tpa_cont_cqe, pub fast_path_tpa_end: eth_fast_path_rx_tpa_end_cqe, pub slow_path: eth_slow_path_rx_cqe }
#[repr(C)] pub enum eth_rx_cqe_type { ETH_RX_CQE_TYPE_UNUSED, ETH_RX_CQE_TYPE_REGULAR, ETH_RX_CQE_TYPE_SLOW_PATH, ETH_RX_CQE_TYPE_TPA_START, ETH_RX_CQE_TYPE_TPA_CONT, ETH_RX_CQE_TYPE_TPA_END, MAX_ETH_RX_CQE_TYPE }
#[repr(C)] pub struct eth_rx_pmd_cqe { pub cqe: eth_rx_cqe, pub reserved: [u8; ETH_RX_CQE_GAP as usize] }
#[repr(C)] pub enum eth_rx_tunn_type { ETH_RX_NO_TUNN, ETH_RX_TUNN_GENEVE, ETH_RX_TUNN_GRE, ETH_RX_TUNN_VXLAN, MAX_ETH_RX_TUNN_TYPE }
#[repr(C)] pub enum eth_tpa_end_reason { ETH_AGG_END_UNUSED, ETH_AGG_END_SP_UPDATE, ETH_AGG_END_MAX_LEN, ETH_AGG_END_LAST_SEG, ETH_AGG_END_TIMEOUT, ETH_AGG_END_NOT_CONSISTENT, ETH_AGG_END_OUT_OF_ORDER, ETH_AGG_END_NON_TPA_SEG, MAX_ETH_TPA_END_REASON }
#[repr(C)] pub struct eth_tx_1st_bd { pub addr: regpair, pub nbytes: __le16, pub data: eth_tx_data_1st_bd }
#[repr(C)] pub struct eth_tx_2nd_bd { pub addr: regpair, pub nbytes: __le16, pub data: eth_tx_data_2nd_bd }
#[repr(C)] pub struct eth_tx_data_3rd_bd { pub lso_mss: __le16, pub bitfields: __le16, pub tunn_l4_hdr_start_offset_w: u8, pub tunn_hdr_size_w: u8 }
#[repr(C)] pub struct eth_tx_3rd_bd { pub addr: regpair, pub nbytes: __le16, pub data: eth_tx_data_3rd_bd }
#[repr(C)] pub struct eth_tx_data_4th_bd { pub dst_vport_id: u8, pub reserved4: u8, pub bitfields: __le16, pub reserved3: __le16 }
#[repr(C)] pub struct eth_tx_4th_bd { pub addr: regpair, pub nbytes: __le16, pub data: eth_tx_data_4th_bd }
#[repr(C)] pub struct eth_tx_data_bd { pub reserved0: __le16, pub bitfields: __le16, pub reserved3: __le16 }
#[repr(C)] pub struct eth_tx_bd { pub addr: regpair, pub nbytes: __le16, pub data: eth_tx_data_bd }
#[repr(C)] pub union eth_tx_bd_types { pub first_bd: eth_tx_1st_bd, pub second_bd: eth_tx_2nd_bd, pub third_bd: eth_tx_3rd_bd, pub fourth_bd: eth_tx_4th_bd, pub reg_bd: eth_tx_bd }
#[repr(C)] pub enum eth_tx_tunn_type { ETH_TX_TUNN_GENEVE, ETH_TX_TUNN_TTAG, ETH_TX_TUNN_GRE, ETH_TX_TUNN_VXLAN, MAX_ETH_TX_TUNN_TYPE }
#[repr(C)] pub struct mstorm_eth_queue_zone { pub rx_producers: eth_rx_prod_data, pub reserved: [__le32; 3] }
#[repr(C)] pub struct xstorm_eth_queue_zone { pub int_coalescing_timeset: coalescing_timeset, pub reserved: [u8; 7] }
#[repr(C)] pub struct eth_db_data { pub params: u8, pub agg_flags: u8, pub bd_prod: __le16 }
#[repr(C)] pub enum rss_hash_type { RSS_HASH_TYPE_DEFAULT = 0, RSS_HASH_TYPE_IPV4 = 1, RSS_HASH_TYPE_TCP_IPV4 = 2, RSS_HASH_TYPE_IPV6 = 3, RSS_HASH_TYPE_TCP_IPV6 = 4, RSS_HASH_TYPE_UDP_IPV4 = 5, RSS_HASH_TYPE_UDP_IPV6 = 6, MAX_RSS_HASH_TYPE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
