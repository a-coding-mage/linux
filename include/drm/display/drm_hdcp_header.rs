/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2017 Google, Inc.
 *
 * Authors:
 * Sean Paul <seanpaul@chromium.org>
 */

/* Translated from drm_hdcp.h. Linux __be16 is represented by u16 here. */

pub const DRM_HDCP_CHECK_PERIOD_MS: u32 = 128 * 16;
pub const DRM_HDCP2_CHECK_PERIOD_MS: u32 = 500;

pub const DRM_HDCP_AN_LEN: usize = 8;
pub const DRM_HDCP_BSTATUS_LEN: usize = 2;
pub const DRM_HDCP_KSV_LEN: usize = 5;
pub const DRM_HDCP_RI_LEN: usize = 2;
pub const DRM_HDCP_V_PRIME_PART_LEN: usize = 4;
pub const DRM_HDCP_V_PRIME_NUM_PARTS: usize = 5;
macro_rules! DRM_HDCP_NUM_DOWNSTREAM { ($x:expr) => (($x) & 0x7f) }
macro_rules! DRM_HDCP_MAX_CASCADE_EXCEEDED { ($x:expr) => (($x) & (1 << 3)) }
macro_rules! DRM_HDCP_MAX_DEVICE_EXCEEDED { ($x:expr) => (($x) & (1 << 7)) }

pub const DRM_HDCP_DDC_ADDR: u8 = 0x3A;
pub const DRM_HDCP_SHA1_TERMINATOR: u8 = 0x80;
pub const DRM_HDCP_DDC_BKSV: u8 = 0x00;
pub const DRM_HDCP_DDC_RI_PRIME: u8 = 0x08;
pub const DRM_HDCP_DDC_AKSV: u8 = 0x10;
pub const DRM_HDCP_DDC_AN: u8 = 0x18;
macro_rules! DRM_HDCP_DDC_V_PRIME { ($h:expr) => (0x20 + ($h) * 4) }
pub const DRM_HDCP_DDC_BCAPS: u8 = 0x40;
pub const DRM_HDCP_DDC_BCAPS_REPEATER_PRESENT: u8 = 1 << 6;
pub const DRM_HDCP_DDC_BCAPS_KSV_FIFO_READY: u8 = 1 << 5;
pub const DRM_HDCP_DDC_BSTATUS: u8 = 0x41;
pub const DRM_HDCP_DDC_KSV_FIFO: u8 = 0x43;

pub const DRM_HDCP_1_4_SRM_ID: u8 = 0x8;
pub const DRM_HDCP_1_4_VRL_LENGTH_SIZE: usize = 3;
pub const DRM_HDCP_1_4_DCP_SIG_SIZE: usize = 40;
pub const HDCP_STREAM_TYPE0: u8 = 0x00;
pub const HDCP_STREAM_TYPE1: u8 = 0x01;

pub const HDCP_2_2_NULL_MSG: u8 = 1;
pub const HDCP_2_2_AKE_INIT: u8 = 2;
pub const HDCP_2_2_AKE_SEND_CERT: u8 = 3;
pub const HDCP_2_2_AKE_NO_STORED_KM: u8 = 4;
pub const HDCP_2_2_AKE_STORED_KM: u8 = 5;
pub const HDCP_2_2_AKE_SEND_HPRIME: u8 = 7;
pub const HDCP_2_2_AKE_SEND_PAIRING_INFO: u8 = 8;
pub const HDCP_2_2_LC_INIT: u8 = 9;
pub const HDCP_2_2_LC_SEND_LPRIME: u8 = 10;
pub const HDCP_2_2_SKE_SEND_EKS: u8 = 11;
pub const HDCP_2_2_REP_SEND_RECVID_LIST: u8 = 12;
pub const HDCP_2_2_REP_SEND_ACK: u8 = 15;
pub const HDCP_2_2_REP_STREAM_MANAGE: u8 = 16;
pub const HDCP_2_2_REP_STREAM_READY: u8 = 17;

pub const HDCP_2_2_RTX_LEN: usize = 8;
pub const HDCP_2_2_RRX_LEN: usize = 8;
pub const HDCP_2_2_K_PUB_RX_MOD_N_LEN: usize = 128;
pub const HDCP_2_2_K_PUB_RX_EXP_E_LEN: usize = 3;
pub const HDCP_2_2_K_PUB_RX_LEN: usize = HDCP_2_2_K_PUB_RX_MOD_N_LEN + HDCP_2_2_K_PUB_RX_EXP_E_LEN;
pub const HDCP_2_2_DCP_LLC_SIG_LEN: usize = 384;
pub const HDCP_2_2_E_KPUB_KM_LEN: usize = 128;
pub const HDCP_2_2_E_KH_KM_M_LEN: usize = 16 + 16;
pub const HDCP_2_2_H_PRIME_LEN: usize = 32;
pub const HDCP_2_2_E_KH_KM_LEN: usize = 16;
pub const HDCP_2_2_RN_LEN: usize = 8;
pub const HDCP_2_2_L_PRIME_LEN: usize = 32;
pub const HDCP_2_2_E_DKEY_KS_LEN: usize = 16;
pub const HDCP_2_2_RIV_LEN: usize = 8;
pub const HDCP_2_2_SEQ_NUM_LEN: usize = 3;
pub const HDCP_2_2_V_PRIME_HALF_LEN: usize = HDCP_2_2_L_PRIME_LEN / 2;
pub const HDCP_2_2_RECEIVER_ID_LEN: usize = DRM_HDCP_KSV_LEN;
pub const HDCP_2_2_MAX_DEVICE_COUNT: usize = 31;
pub const HDCP_2_2_RECEIVER_IDS_MAX_LEN: usize = HDCP_2_2_RECEIVER_ID_LEN * HDCP_2_2_MAX_DEVICE_COUNT;
pub const HDCP_2_2_MPRIME_LEN: usize = 32;
pub const HDCP_2_2_MAX_CONTENT_STREAMS_CNT: usize = 4;
pub const HDCP_2_2_TXCAP_MASK_LEN: usize = 2;
pub const HDCP_2_2_RXCAPS_LEN: usize = 3;
macro_rules! HDCP_2_2_RX_REPEATER { ($x:expr) => (($x) & (1 << 0)) }
macro_rules! HDCP_2_2_DP_HDCP_CAPABLE { ($x:expr) => (($x) & (1 << 1)) }
pub const HDCP_2_2_RXINFO_LEN: usize = 2;
macro_rules! HDCP_2_2_HDCP1_DEVICE_CONNECTED { ($x:expr) => (($x) & (1 << 0)) }
macro_rules! HDCP_2_2_HDCP_2_0_REP_CONNECTED { ($x:expr) => (($x) & (1 << 1)) }
macro_rules! HDCP_2_2_MAX_CASCADE_EXCEEDED { ($x:expr) => (($x) & (1 << 2)) }
macro_rules! HDCP_2_2_MAX_DEVS_EXCEEDED { ($x:expr) => (($x) & (1 << 3)) }
macro_rules! HDCP_2_2_DEV_COUNT_LO { ($x:expr) => (((($x) & (0xF << 4)) >> 4)) }
macro_rules! HDCP_2_2_DEV_COUNT_HI { ($x:expr) => (($x) & (1 << 0)) }
macro_rules! HDCP_2_2_DEPTH { ($x:expr) => (((($x) & (0x7 << 1)) >> 1)) }

#[repr(C, packed)]
pub struct hdcp2_cert_rx { pub receiver_id: [u8; HDCP_2_2_RECEIVER_ID_LEN], pub kpub_rx: [u8; HDCP_2_2_K_PUB_RX_LEN], pub reserved: [u8; 2], pub dcp_signature: [u8; HDCP_2_2_DCP_LLC_SIG_LEN] }
#[repr(C, packed)]
pub struct hdcp2_streamid_type { pub stream_id: u8, pub stream_type: u8 }
#[repr(C, packed)]
pub struct hdcp2_tx_caps { pub version: u8, pub tx_cap_mask: [u8; HDCP_2_2_TXCAP_MASK_LEN] }
#[repr(C, packed)]
pub struct hdcp2_ake_init { pub msg_id: u8, pub r_tx: [u8; HDCP_2_2_RTX_LEN], pub tx_caps: hdcp2_tx_caps }
#[repr(C, packed)]
pub struct hdcp2_ake_send_cert { pub msg_id: u8, pub cert_rx: hdcp2_cert_rx, pub r_rx: [u8; HDCP_2_2_RRX_LEN], pub rx_caps: [u8; HDCP_2_2_RXCAPS_LEN] }
#[repr(C, packed)]
pub struct hdcp2_ake_no_stored_km { pub msg_id: u8, pub e_kpub_km: [u8; HDCP_2_2_E_KPUB_KM_LEN] }
#[repr(C, packed)]
pub struct hdcp2_ake_stored_km { pub msg_id: u8, pub e_kh_km_m: [u8; HDCP_2_2_E_KH_KM_M_LEN] }
#[repr(C, packed)]
pub struct hdcp2_ake_send_hprime { pub msg_id: u8, pub h_prime: [u8; HDCP_2_2_H_PRIME_LEN] }
#[repr(C, packed)]
pub struct hdcp2_ake_send_pairing_info { pub msg_id: u8, pub e_kh_km: [u8; HDCP_2_2_E_KH_KM_LEN] }
#[repr(C, packed)]
pub struct hdcp2_lc_init { pub msg_id: u8, pub r_n: [u8; HDCP_2_2_RN_LEN] }
#[repr(C, packed)]
pub struct hdcp2_lc_send_lprime { pub msg_id: u8, pub l_prime: [u8; HDCP_2_2_L_PRIME_LEN] }
#[repr(C, packed)]
pub struct hdcp2_ske_send_eks { pub msg_id: u8, pub e_dkey_ks: [u8; HDCP_2_2_E_DKEY_KS_LEN], pub riv: [u8; HDCP_2_2_RIV_LEN] }
#[repr(C, packed)]
pub struct hdcp2_rep_send_receiverid_list { pub msg_id: u8, pub rx_info: [u8; HDCP_2_2_RXINFO_LEN], pub seq_num_v: [u8; HDCP_2_2_SEQ_NUM_LEN], pub v_prime: [u8; HDCP_2_2_V_PRIME_HALF_LEN], pub receiver_ids: [u8; HDCP_2_2_RECEIVER_IDS_MAX_LEN] }
#[repr(C, packed)]
pub struct hdcp2_rep_send_ack { pub msg_id: u8, pub v: [u8; HDCP_2_2_V_PRIME_HALF_LEN] }
#[repr(C, packed)]
pub struct hdcp2_rep_stream_manage { pub msg_id: u8, pub seq_num_m: [u8; HDCP_2_2_SEQ_NUM_LEN], pub k: u16, pub streams: [hdcp2_streamid_type; HDCP_2_2_MAX_CONTENT_STREAMS_CNT] }
#[repr(C, packed)]
pub struct hdcp2_rep_stream_ready { pub msg_id: u8, pub m_prime: [u8; HDCP_2_2_MPRIME_LEN] }

pub const HDCP_2_2_CERT_TIMEOUT_MS: u32 = 100;
pub const HDCP_2_2_DP_CERT_READ_TIMEOUT_MS: u32 = 110;
pub const HDCP_2_2_HPRIME_NO_PAIRED_TIMEOUT_MS: u32 = 1000;
pub const HDCP_2_2_HPRIME_PAIRED_TIMEOUT_MS: u32 = 200;
pub const HDCP_2_2_DP_HPRIME_READ_TIMEOUT_MS: u32 = 7;
pub const HDCP_2_2_PAIRING_TIMEOUT_MS: u32 = 200;
pub const HDCP_2_2_DP_PAIRING_READ_TIMEOUT_MS: u32 = 5;
pub const HDCP_2_2_HDMI_LPRIME_TIMEOUT_MS: u32 = 20;
pub const HDCP_2_2_DP_LPRIME_TIMEOUT_MS: u32 = 16;
pub const HDCP_2_2_RECVID_LIST_TIMEOUT_MS: u32 = 3000;
pub const HDCP_2_2_STREAM_READY_TIMEOUT_MS: u32 = 100;
pub const HDCP_2_2_HDMI_REG_VER_OFFSET: u8 = 0x50;
pub const HDCP_2_2_HDMI_REG_WR_MSG_OFFSET: u8 = 0x60;
pub const HDCP_2_2_HDMI_REG_RXSTATUS_OFFSET: u8 = 0x70;
pub const HDCP_2_2_HDMI_REG_RD_MSG_OFFSET: u8 = 0x80;
pub const HDCP_2_2_HDMI_REG_DBG_OFFSET: u8 = 0xC0;
pub const HDCP_2_2_HDMI_SUPPORT_MASK: u8 = 1 << 2;
pub const HDCP_2_2_RX_CAPS_VERSION_VAL: u8 = 0x02;
pub const HDCP_2_2_SEQ_NUM_MAX: u32 = 0xFFFFFF;
pub const HDCP_2_2_DELAY_BEFORE_ENCRYPTION_EN: u32 = 200;
pub const HDCP_2_2_HDMI_RXSTATUS_LEN: usize = 2;
macro_rules! HDCP_2_2_HDMI_RXSTATUS_MSG_SZ_HI { ($x:expr) => (($x) & 0x3) }
macro_rules! HDCP_2_2_HDMI_RXSTATUS_READY { ($x:expr) => (($x) & (1 << 2)) }
macro_rules! HDCP_2_2_HDMI_RXSTATUS_REAUTH_REQ { ($x:expr) => (($x) & (1 << 3)) }

pub fn drm_hdcp_be24_to_cpu(seq_num: &[u8; HDCP_2_2_SEQ_NUM_LEN]) -> u32 {
    (seq_num[2] as u32) | ((seq_num[1] as u32) << 8) | ((seq_num[0] as u32) << 16)
}

pub fn drm_hdcp_cpu_to_be24(seq_num: &mut [u8; HDCP_2_2_SEQ_NUM_LEN], val: u32) {
    seq_num[0] = (val >> 16) as u8;
    seq_num[1] = (val >> 8) as u8;
    seq_num[2] = val as u8;
}

pub const DRM_HDCP_SRM_GEN1_MAX_BYTES: usize = 5 * 1024;
pub const DRM_HDCP_SRM_ID_MASK: u8 = 0xF << 4;
pub const DRM_HDCP_2_SRM_ID: u8 = 0x9;
pub const DRM_HDCP_2_INDICATOR: u8 = 0x1;
pub const DRM_HDCP_2_INDICATOR_MASK: u8 = 0xF;
pub const DRM_HDCP_2_VRL_LENGTH_SIZE: usize = 3;
pub const DRM_HDCP_2_DCP_SIG_SIZE: usize = 384;
pub const DRM_HDCP_2_NO_OF_DEV_PLUS_RESERVED_SZ: usize = 4;
macro_rules! DRM_HDCP_2_KSV_COUNT_2_LSBITS { ($byte:expr) => (((($byte) & 0xC0) >> 6)) }
#[repr(C, packed)]
pub struct hdcp_srm_header { pub srm_id: u8, pub reserved: u8, pub srm_version: u16, pub srm_gen_no: u8 }
pub const DRM_MODE_HDCP_CONTENT_TYPE0: u8 = 0;
pub const DRM_MODE_HDCP_CONTENT_TYPE1: u8 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
