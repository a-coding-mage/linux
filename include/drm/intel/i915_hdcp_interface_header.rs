/* SPDX-License-Identifier: (GPL-2.0+) */
/*
 * Copyright © 2017-2019 Intel Corporation
 *
 * Authors:
 * Ramalingam C <ramalingam.c@intel.com>
 */

// C header dependencies are supplied by other translation units.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdcp_port_type { HDCP_PORT_TYPE_INVALID, HDCP_PORT_TYPE_INTEGRATED, HDCP_PORT_TYPE_LSPCON, HDCP_PORT_TYPE_CPDP }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdcp_wired_protocol { HDCP_PROTOCOL_INVALID, HDCP_PROTOCOL_HDMI, HDCP_PROTOCOL_DP }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdcp_ddi {
    HDCP_DDI_INVALID_PORT = 0x0,
    HDCP_DDI_B = 1, HDCP_DDI_C, HDCP_DDI_D, HDCP_DDI_E, HDCP_DDI_F,
    HDCP_DDI_A = 7, HDCP_DDI_RANGE_END = 7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdcp_transcoder {
    HDCP_INVALID_TRANSCODER = 0x00,
    HDCP_TRANSCODER_EDP, HDCP_TRANSCODER_DSI0, HDCP_TRANSCODER_DSI1,
    HDCP_TRANSCODER_A = 0x10, HDCP_TRANSCODER_B, HDCP_TRANSCODER_C, HDCP_TRANSCODER_D,
}

#[repr(C)]
pub struct hdcp_port_data {
    pub hdcp_ddi: hdcp_ddi,
    pub hdcp_transcoder: hdcp_transcoder,
    pub port_type: u8,
    pub protocol: u8,
    pub k: u16,
    pub seq_num_m: u32,
    pub streams: *mut hdcp2_streamid_type,
}

#[repr(C)]
pub struct i915_hdcp_ops {
    pub owner: *mut module,
    pub initiate_hdcp2_session: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_ake_init) -> i32>,
    pub verify_receiver_cert_prepare_km: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_ake_send_cert, *mut bool, *mut hdcp2_ake_no_stored_km, *mut usize) -> i32>,
    pub verify_hprime: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_ake_send_hprime) -> i32>,
    pub store_pairing_info: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_ake_send_pairing_info) -> i32>,
    pub initiate_locality_check: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_lc_init) -> i32>,
    pub verify_lprime: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_lc_send_lprime) -> i32>,
    pub get_session_key: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_ske_send_eks) -> i32>,
    pub repeater_check_flow_prepare_ack: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_rep_send_receiverid_list, *mut hdcp2_rep_send_ack) -> i32>,
    pub verify_mprime: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data, *mut hdcp2_rep_stream_ready) -> i32>,
    pub enable_hdcp_authentication: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data) -> i32>,
    pub close_hdcp_session: Option<unsafe extern "C" fn(*mut device, *mut hdcp_port_data) -> i32>,
}

#[repr(C)]
pub struct i915_hdcp_arbiter { pub hdcp_dev: *mut device, pub ops: *const i915_hdcp_ops, pub mutex: mutex }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fw_hdcp_status {
    FW_HDCP_STATUS_SUCCESS = 0x0000,
    FW_HDCP_STATUS_INTERNAL_ERROR = 0x1000, FW_HDCP_STATUS_UNKNOWN_ERROR, FW_HDCP_STATUS_INCORRECT_API_VERSION,
    FW_HDCP_STATUS_INVALID_FUNCTION, FW_HDCP_STATUS_INVALID_BUFFER_LENGTH, FW_HDCP_STATUS_INVALID_PARAMS,
    FW_HDCP_STATUS_AUTHENTICATION_FAILED,
    FW_HDCP_INVALID_SESSION_STATE = 0x6000, FW_HDCP_SRM_FRAGMENT_UNEXPECTED, FW_HDCP_SRM_INVALID_LENGTH,
    FW_HDCP_SRM_FRAGMENT_OFFSET_INVALID, FW_HDCP_SRM_VERIFICATION_FAILED, FW_HDCP_SRM_VERSION_TOO_OLD,
    FW_HDCP_RX_CERT_VERIFICATION_FAILED, FW_HDCP_RX_REVOKED, FW_HDCP_H_VERIFICATION_FAILED,
    FW_HDCP_REPEATER_CHECK_UNEXPECTED, FW_HDCP_TOPOLOGY_MAX_EXCEEDED, FW_HDCP_V_VERIFICATION_FAILED,
    FW_HDCP_L_VERIFICATION_FAILED, FW_HDCP_STREAM_KEY_ALLOC_FAILED, FW_HDCP_BASE_KEY_RESET_FAILED,
    FW_HDCP_NONCE_GENERATION_FAILED, FW_HDCP_STATUS_INVALID_E_KEY_STATE, FW_HDCP_STATUS_INVALID_CS_ICV,
    FW_HDCP_STATUS_INVALID_KB_KEY_STATE, FW_HDCP_STATUS_INVALID_PAVP_MODE_ICV, FW_HDCP_STATUS_INVALID_PAVP_MODE,
    FW_HDCP_STATUS_LC_MAX_ATTEMPTS, FW_HDCP_STATUS_MISMATCH_IN_M,
    FW_HDCP_STATUS_RX_PROV_NOT_ALLOWED, FW_HDCP_STATUS_RX_PROV_WRONG_SUBJECT, FW_HDCP_RX_NEEDS_PROVISIONING,
    FW_HDCP_BKSV_ICV_AUTH_FAILED = 0x6020, FW_HDCP_STATUS_INVALID_STREAM_ID, FW_HDCP_STATUS_CHAIN_NOT_INITIALIZED,
    FW_HDCP_FAIL_NOT_EXPECTED, FW_HDCP_FAIL_HDCP_OFF, FW_HDCP_FAIL_INVALID_PAVP_MEMORY_MODE, FW_HDCP_FAIL_AES_ECB_FAILURE,
    FW_HDCP_FEATURE_NOT_SUPPORTED, FW_HDCP_DMA_READ_ERROR, FW_HDCP_DMA_WRITE_ERROR,
    FW_HDCP_FAIL_INVALID_PACKET_SIZE = 0x6030, FW_HDCP_H264_PARSING_ERROR, FW_HDCP_HDCP2_ERRATA_VIDEO_VIOLATION,
    FW_HDCP_HDCP2_ERRATA_AUDIO_VIOLATION, FW_HDCP_TX_ACTIVE_ERROR, FW_HDCP_MODE_CHANGE_ERROR,
    FW_HDCP_STREAM_TYPE_ERROR, FW_HDCP_STREAM_MANAGE_NOT_POSSIBLE, FW_HDCP_STATUS_PORT_INVALID_COMMAND,
    FW_HDCP_STATUS_UNSUPPORTED_PROTOCOL, FW_HDCP_STATUS_INVALID_PORT_INDEX, FW_HDCP_STATUS_TX_AUTH_NEEDED,
    FW_HDCP_STATUS_NOT_INTEGRATED_PORT, FW_HDCP_STATUS_SESSION_MAX_REACHED,
    FW_HDCP_STATUS_NOT_HDCP_CAPABLE = 0x6041, FW_HDCP_STATUS_INVALID_STREAM_COUNT,
}

pub const HDCP_API_VERSION: u32 = 0x00010000;
pub const HDCP_M_LEN: usize = 16;
pub const HDCP_KH_LEN: usize = 16;
pub const WIRED_CMD_BUF_LEN_INITIATE_HDCP2_SESSION_IN: usize = 4 + 1;
pub const WIRED_CMD_BUF_LEN_INITIATE_HDCP2_SESSION_OUT: usize = 4 + 8 + 3;
pub const WIRED_CMD_BUF_LEN_VERIFY_RECEIVER_CERT_IN: usize = 4 + 522 + 8 + 3;
pub const WIRED_CMD_BUF_LEN_VERIFY_RECEIVER_CERT_MIN_OUT: usize = 4 + 1 + 3 + 16 + 16;
pub const WIRED_CMD_BUF_LEN_VERIFY_RECEIVER_CERT_MAX_OUT: usize = 4 + 1 + 3 + 128;
pub const WIRED_CMD_BUF_LEN_AKE_SEND_HPRIME_IN: usize = 4 + 32;
pub const WIRED_CMD_BUF_LEN_AKE_SEND_HPRIME_OUT: usize = 4;
pub const WIRED_CMD_BUF_LEN_SEND_PAIRING_INFO_IN: usize = 4 + 16;
pub const WIRED_CMD_BUF_LEN_SEND_PAIRING_INFO_OUT: usize = 4;
pub const WIRED_CMD_BUF_LEN_CLOSE_SESSION_IN: usize = 4;
pub const WIRED_CMD_BUF_LEN_CLOSE_SESSION_OUT: usize = 4;
pub const WIRED_CMD_BUF_LEN_INIT_LOCALITY_CHECK_IN: usize = 4;
pub const WIRED_CMD_BUF_LEN_INIT_LOCALITY_CHECK_OUT: usize = 4 + 8;
pub const WIRED_CMD_BUF_LEN_VALIDATE_LOCALITY_IN: usize = 4 + 32;
pub const WIRED_CMD_BUF_LEN_VALIDATE_LOCALITY_OUT: usize = 4;
pub const WIRED_CMD_BUF_LEN_GET_SESSION_KEY_IN: usize = 4;
pub const WIRED_CMD_BUF_LEN_GET_SESSION_KEY_OUT: usize = 4 + 16 + 8;
pub const WIRED_CMD_BUF_LEN_ENABLE_AUTH_IN: usize = 4 + 1;
pub const WIRED_CMD_BUF_LEN_ENABLE_AUTH_OUT: usize = 4;
pub const WIRED_CMD_BUF_LEN_VERIFY_REPEATER_IN: usize = 4 + 2 + 3 + 16 + 155;
pub const WIRED_CMD_BUF_LEN_VERIFY_REPEATER_OUT: usize = 4 + 1 + 16;
pub const WIRED_CMD_BUF_LEN_REPEATER_AUTH_STREAM_REQ_MIN_IN: usize = 4 + 3 + 32 + 2 + 2;
pub const WIRED_CMD_BUF_LEN_REPEATER_AUTH_STREAM_REQ_OUT: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdcp_command_id {
    _WIDI_COMMAND_BASE = 0x00030000, WIDI_INITIATE_HDCP2_SESSION, HDCP_GET_SRM_STATUS, HDCP_SEND_SRM_FRAGMENT,
    _WIRED_COMMAND_BASE = 0x00031000, WIRED_INITIATE_HDCP2_SESSION, WIRED_VERIFY_RECEIVER_CERT,
    WIRED_AKE_SEND_HPRIME, WIRED_AKE_SEND_PAIRING_INFO, WIRED_INIT_LOCALITY_CHECK, WIRED_VALIDATE_LOCALITY,
    WIRED_GET_SESSION_KEY, WIRED_ENABLE_AUTH, WIRED_VERIFY_REPEATER, WIRED_REPEATER_AUTH_STREAM_REQ,
    WIRED_CLOSE_SESSION, _WIRED_COMMANDS_COUNT,
}

#[repr(C)]
pub union encrypted_buff {
    pub e_kpub_km: [u8; HDCP_2_2_E_KPUB_KM_LEN],
    pub e_kh_km_m: [u8; HDCP_2_2_E_KH_KM_M_LEN],
    pub packed: encrypted_buff_packed,
}
#[repr(C, packed)]
pub struct encrypted_buff_packed { pub e_kh_km: [u8; HDCP_KH_LEN], pub m: [u8; HDCP_M_LEN] }

#[repr(C, packed)]
pub struct hdcp_cmd_header { pub api_version: u32, pub command_id: u32, pub status: fw_hdcp_status, pub buffer_len: u32 }
#[repr(C, packed)] pub struct hdcp_cmd_no_data { pub header: hdcp_cmd_header }
#[repr(C, packed)] pub struct hdcp_port_id { pub integrated_port_type: u8, pub physical_port: u8, pub attached_transcoder: u8, pub reserved: u8 }

#[repr(C, packed)]
pub struct wired_cmd_initiate_hdcp2_session_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub protocol: u8 }
#[repr(C, packed)]
pub struct wired_cmd_initiate_hdcp2_session_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub r_tx: [u8; HDCP_2_2_RTX_LEN], pub tx_caps: hdcp2_tx_caps }
#[repr(C, packed)] pub struct wired_cmd_close_session_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id }
#[repr(C, packed)] pub struct wired_cmd_close_session_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id }
#[repr(C, packed)] pub struct wired_cmd_verify_receiver_cert_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub cert_rx: hdcp2_cert_rx, pub r_rx: [u8; HDCP_2_2_RRX_LEN], pub rx_caps: [u8; HDCP_2_2_RXCAPS_LEN] }
#[repr(C, packed)] pub struct wired_cmd_verify_receiver_cert_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub km_stored: u8, pub reserved: [u8; 3], pub ekm_buff: encrypted_buff }
#[repr(C, packed)] pub struct wired_cmd_ake_send_hprime_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub h_prime: [u8; HDCP_2_2_H_PRIME_LEN] }
#[repr(C, packed)] pub struct wired_cmd_ake_send_hprime_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id }
#[repr(C, packed)] pub struct wired_cmd_ake_send_pairing_info_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub e_kh_km: [u8; HDCP_2_2_E_KH_KM_LEN] }
#[repr(C, packed)] pub struct wired_cmd_ake_send_pairing_info_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id }
#[repr(C, packed)] pub struct wired_cmd_init_locality_check_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id }
#[repr(C, packed)] pub struct wired_cmd_init_locality_check_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub r_n: [u8; HDCP_2_2_RN_LEN] }
#[repr(C, packed)] pub struct wired_cmd_validate_locality_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub l_prime: [u8; HDCP_2_2_L_PRIME_LEN] }
#[repr(C, packed)] pub struct wired_cmd_validate_locality_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id }
#[repr(C, packed)] pub struct wired_cmd_get_session_key_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id }
#[repr(C, packed)] pub struct wired_cmd_get_session_key_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub e_dkey_ks: [u8; HDCP_2_2_E_DKEY_KS_LEN], pub r_iv: [u8; HDCP_2_2_RIV_LEN] }
#[repr(C, packed)] pub struct wired_cmd_enable_auth_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub stream_type: u8 }
#[repr(C, packed)] pub struct wired_cmd_enable_auth_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id }
#[repr(C, packed)] pub struct wired_cmd_verify_repeater_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub rx_info: [u8; HDCP_2_2_RXINFO_LEN], pub seq_num_v: [u8; HDCP_2_2_SEQ_NUM_LEN], pub v_prime: [u8; HDCP_2_2_V_PRIME_HALF_LEN], pub receiver_ids: [u8; HDCP_2_2_RECEIVER_IDS_MAX_LEN] }
#[repr(C, packed)] pub struct wired_cmd_verify_repeater_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub content_type_supported: u8, pub v: [u8; HDCP_2_2_V_PRIME_HALF_LEN] }
#[repr(C, packed)] pub struct wired_cmd_repeater_auth_stream_req_in { pub header: hdcp_cmd_header, pub port: hdcp_port_id, pub seq_num_m: [u8; HDCP_2_2_SEQ_NUM_LEN], pub m_prime: [u8; HDCP_2_2_MPRIME_LEN], pub k: __be16, pub streams: [hdcp2_streamid_type; 0] }
#[repr(C, packed)] pub struct wired_cmd_repeater_auth_stream_req_out { pub header: hdcp_cmd_header, pub port: hdcp_port_id }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
