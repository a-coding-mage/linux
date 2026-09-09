/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2023 Intel Corporation */

/* This is an interface definition file where existing enums and their values
 * must remain unchanged over time, so we specify explicit values for all enums.
 */

/* Transmit descriptor ID flags */
pub const VIRTCHNL2_TXDID_DATA: u32 = 1 << 0;
pub const VIRTCHNL2_TXDID_CTX: u32 = 1 << 1;
pub const VIRTCHNL2_TXDID_FLEX_TSO_CTX: u32 = 1 << 5;
pub const VIRTCHNL2_TXDID_FLEX_L2TAG1_L2TAG2: u32 = 1 << 7;
pub const VIRTCHNL2_TXDID_FLEX_FLOW_SCHED: u32 = 1 << 12;
pub const VIRTCHNL2_TXDID_DESC_DONE: u32 = 1 << 15;

/* Receive descriptor IDs */
pub const VIRTCHNL2_RXDID_1_32B_BASE: u32 = 1;
pub const VIRTCHNL2_RXDID_2_FLEX_SPLITQ: u32 = 2;
pub const VIRTCHNL2_RXDID_2_FLEX_SQ_NIC: u32 = VIRTCHNL2_RXDID_2_FLEX_SPLITQ;
pub const VIRTCHNL2_RXDID_7_HW_RSVD: u32 = 7;

/* Receive descriptor ID bitmasks */
pub const VIRTCHNL2_RXDID_1_32B_BASE_M: u64 = 1u64 << VIRTCHNL2_RXDID_1_32B_BASE;
pub const VIRTCHNL2_RXDID_2_FLEX_SPLITQ_M: u64 = 1u64 << VIRTCHNL2_RXDID_2_FLEX_SPLITQ;
pub const VIRTCHNL2_RXDID_2_FLEX_SQ_NIC_M: u64 = 1u64 << VIRTCHNL2_RXDID_2_FLEX_SQ_NIC;
pub const VIRTCHNL2_RXDID_7_HW_RSVD_M: u64 = 1u64 << VIRTCHNL2_RXDID_7_HW_RSVD;

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_RXDID_M: u64 = 0x0f;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_UMBCAST_M: u64 = 0xc0;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_PTYPE_M: u64 = 0x3ff;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_RAW_CSUM_INV_S: u32 = 12;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_RAW_CSUM_INV_M: u64 = 1u64 << VIRTCHNL2_RX_FLEX_DESC_ADV_RAW_CSUM_INV_S;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_FF0_M: u64 = 0xe000;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_LEN_PBUF_M: u64 = 0x3fff;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_GEN_S: u32 = 14;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_GEN_M: u64 = 1u64 << VIRTCHNL2_RX_FLEX_DESC_ADV_GEN_S;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_BUFQ_ID_S: u32 = 15;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_BUFQ_ID_M: u64 = 1u64 << VIRTCHNL2_RX_FLEX_DESC_ADV_BUFQ_ID_S;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_LEN_HDR_M: u64 = 0x3ff;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_RSC_S: u32 = 10;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_RSC_M: u64 = 1u64 << VIRTCHNL2_RX_FLEX_DESC_ADV_RSC_S;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_SPH_S: u32 = 11;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_SPH_M: u64 = 1u64 << VIRTCHNL2_RX_FLEX_DESC_ADV_SPH_S;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_FF1_S: u32 = 12;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_FF1_M: u64 = 0x7000;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_MISS_S: u32 = 15;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_MISS_M: u64 = 1u64 << VIRTCHNL2_RX_FLEX_DESC_ADV_MISS_S;

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_DD_M: u8 = 1 << 0;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_EOF_M: u8 = 1 << 1;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_HBO_M: u8 = 1 << 2;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_L3L4P_M: u8 = 1 << 3;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XSUM_IPE_M: u8 = 1 << 4;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XSUM_L4E_M: u8 = 1 << 5;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XSUM_EIPE_M: u8 = 1 << 6;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XSUM_EUDPE_M: u8 = 1 << 7;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_LPBK_M: u8 = 1 << 0;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_IPV6EXADD_M: u8 = 1 << 1;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_RXE_M: u8 = 1 << 2;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_CRCP_M: u8 = 1 << 3;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_RSS_VALID_M: u8 = 1 << 4;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_L2TAG1P_M: u8 = 1 << 5;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XTRMD0_VALID_M: u8 = 1 << 6;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XTRMD1_VALID_M: u8 = 1 << 7;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_RSVD_M: u8 = 0x03;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_ATRAEFAIL_M: u8 = 1 << 2;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_L2TAG2P_M: u8 = 1 << 3;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_XTRMD2_VALID_M: u8 = 1 << 4;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_XTRMD3_VALID_M: u8 = 1 << 5;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_XTRMD4_VALID_M: u8 = 1 << 6;
pub const VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_XTRMD5_VALID_M: u8 = 1 << 7;

pub const VIRTCHNL2_RX_FLEX_DESC_PTYPE_M: u16 = 0x03ff;
pub const VIRTCHNL2_RX_FLEX_DESC_PKT_LEN_M: u16 = 0x3fff;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_DD_M: u16 = 1 << 0;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_EOF_M: u16 = 1 << 1;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_HBO_M: u16 = 1 << 2;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_L3L4P_M: u16 = 1 << 3;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_XSUM_IPE_M: u16 = 1 << 4;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_XSUM_L4E_M: u16 = 1 << 5;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_XSUM_EIPE_M: u16 = 1 << 6;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_XSUM_EUDPE_M: u16 = 1 << 7;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_LPBK_M: u16 = 1 << 8;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_IPV6EXADD_M: u16 = 1 << 9;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_RXE_M: u16 = 1 << 10;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_CRCP_M: u16 = 1 << 11;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_RSS_VALID_M: u16 = 1 << 12;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_L2TAG1P_M: u16 = 1 << 13;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_XTRMD0_VALID_M: u16 = 1 << 14;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS0_XTRMD1_VALID_M: u16 = 1 << 15;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS1_CPM_M: u16 = 0x000f;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS1_NAT_M: u16 = 1 << 4;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS1_CRYPTO_M: u16 = 1 << 5;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS1_L2TAG2P_M: u16 = 1 << 11;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS1_XTRMD2_VALID_M: u16 = 1 << 12;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS1_XTRMD3_VALID_M: u16 = 1 << 13;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS1_XTRMD4_VALID_M: u16 = 1 << 14;
pub const VIRTCHNL2_RX_FLEX_DESC_STATUS1_XTRMD5_VALID_M: u16 = 1 << 15;
pub const VIRTCHNL2_RX_FLEX_TSTAMP_VALID: u8 = 1;

pub const VIRTCHNL2_RX_BASE_DESC_QW1_LEN_PBUF_M: u64 = 0x003c000000000000;
pub const VIRTCHNL2_RX_BASE_DESC_QW1_PTYPE_M: u64 = 0x0000003fc0000000;
pub const VIRTCHNL2_RX_BASE_DESC_QW1_ERROR_M: u64 = 0x0000000007f80000;
pub const VIRTCHNL2_RX_BASE_DESC_QW1_STATUS_M: u64 = 0x000000000007ffff;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_DD_M: u32 = 1 << 0;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_EOF_M: u32 = 1 << 1;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_L2TAG1P_M: u32 = 1 << 2;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_L3L4P_M: u32 = 1 << 3;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_CRCP_M: u32 = 1 << 4;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_RSVD_M: u32 = 0xe0;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_EXT_UDP_0_M: u32 = 1 << 8;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_UMBCAST_M: u32 = 0x600;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_FLM_M: u32 = 1 << 11;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_FLTSTAT_M: u32 = 0x3000;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_LPBK_M: u32 = 1 << 14;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_IPV6EXADD_M: u32 = 1 << 15;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_RSVD1_M: u32 = 0x30000;
pub const VIRTCHNL2_RX_BASE_DESC_STATUS_INT_UDP_0_M: u32 = 1 << 18;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_RXE_M: u32 = 1;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_ATRAEFAIL_M: u32 = 1 << 1;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_HBO_M: u32 = 1 << 2;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_L3L4E_M: u32 = 0x38;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_IPE_M: u32 = 1 << 3;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_L4E_M: u32 = 1 << 4;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_EIPE_M: u32 = 1 << 5;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_OVERSIZE_M: u32 = 1 << 6;
pub const VIRTCHNL2_RX_BASE_DESC_ERROR_PPRS_M: u32 = 1 << 7;
pub const VIRTCHNL2_RX_BASE_DESC_FLTSTAT_RSS_HASH_M: u32 = 0x3000;

#[repr(C)]
pub struct virtchnl2_splitq_rx_buf_desc {
    pub qword0: virtchnl2_splitq_rx_buf_desc_qword0,
    pub pkt_addr: __le64,
    pub hdr_addr: __le64,
    pub rsvd2: __le64,
}
#[repr(C)]
pub struct virtchnl2_splitq_rx_buf_desc_qword0 { pub buf_id: __le16, pub rsvd0: __le16, pub rsvd1: __le32 }

#[repr(C)]
pub struct virtchnl2_singleq_rx_buf_desc { pub pkt_addr: __le64, pub hdr_addr: __le64, pub rsvd1: __le64, pub rsvd2: __le64 }

#[repr(C)]
pub struct virtchnl2_singleq_base_rx_desc {
    pub qword0: virtchnl2_singleq_base_rx_desc_qword0,
    pub qword1: virtchnl2_singleq_base_rx_desc_qword1,
    pub qword2: virtchnl2_singleq_base_rx_desc_qword2,
    pub qword3: virtchnl2_singleq_base_rx_desc_qword3,
}
#[repr(C)] pub struct virtchnl2_singleq_base_rx_desc_qword0 { pub lo_dword: virtchnl2_singleq_base_rx_desc_lo_dword, pub hi_dword: virtchnl2_singleq_base_rx_desc_hi_dword }
#[repr(C)] pub struct virtchnl2_singleq_base_rx_desc_lo_dword { pub mirroring_status: __le16, pub l2tag1: __le16 }
#[repr(C)] pub union virtchnl2_singleq_base_rx_desc_hi_dword { pub rss: __le32, pub fd_id: __le32 }
#[repr(C)] pub struct virtchnl2_singleq_base_rx_desc_qword1 { pub status_error_ptype_len: __le64 }
#[repr(C)] pub struct virtchnl2_singleq_base_rx_desc_qword2 { pub ext_status: __le16, pub rsvd: __le16, pub l2tag2_1: __le16, pub l2tag2_2: __le16 }
#[repr(C)] pub struct virtchnl2_singleq_base_rx_desc_qword3 { pub reserved: __le32, pub fd_id: __le32 }

#[repr(C)]
pub struct virtchnl2_rx_flex_desc_nic {
    pub rxdid: u8, pub mir_id_umb_cast: u8, pub ptype_flex_flags0: __le16, pub pkt_len: __le16,
    pub hdr_len_sph_flex_flags1: __le16, pub status_error0: __le16, pub l2tag1: __le16,
    pub rss_hash: __le32, pub status_error1: __le16, pub flexi_flags2: u8, pub ts_low: u8,
    pub l2tag2_1st: __le16, pub l2tag2_2nd: __le16, pub flow_id: __le32, pub flex_ts: virtchnl2_rx_flex_desc_nic_flex_ts,
}
#[repr(C)] pub union virtchnl2_rx_flex_desc_nic_flex_ts { pub flex: virtchnl2_rx_flex_desc_nic_flex, pub ts_high: __le32 }
#[repr(C)] pub struct virtchnl2_rx_flex_desc_nic_flex { pub rsvd: __le16, pub flow_id_ipv6: __le16 }

#[repr(C)]
pub struct virtchnl2_rx_flex_desc_adv_nic_3 {
    pub rxdid_ucast: u8, pub status_err0_qw0: u8, pub ptype_err_fflags0: __le16, pub pktlen_gen_bufq_id: __le16,
    pub hdrlen_flags: __le16, pub status_err0_qw1: u8, pub status_err1: u8, pub fflags1: u8, pub ts_low: u8,
    pub buf_id: __le16, pub misc: virtchnl2_rx_flex_desc_adv_nic_3_misc, pub hash1: __le16,
    pub ff2_mirrid_hash2: virtchnl2_rx_flex_desc_adv_nic_3_ff2_mirrid_hash2, pub hash3: u8,
    pub l2tag2: __le16, pub fmd4: __le16, pub l2tag1: __le16, pub fmd6: __le16, pub ts_high: __le32,
}
#[repr(C)] pub union virtchnl2_rx_flex_desc_adv_nic_3_misc { pub raw_cs: __le16, pub l2tag1: __le16, pub rscseglen: __le16 }
#[repr(C)] pub union virtchnl2_rx_flex_desc_adv_nic_3_ff2_mirrid_hash2 { pub fflags2: u8, pub mirrorid: u8, pub hash2: u8 }

#[repr(C)]
pub union virtchnl2_rx_desc {
    pub base_wb: virtchnl2_singleq_base_rx_desc,
    pub flex_nic_wb: virtchnl2_rx_flex_desc_nic,
    pub flex_adv_nic_3_wb: virtchnl2_rx_flex_desc_adv_nic_3,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
