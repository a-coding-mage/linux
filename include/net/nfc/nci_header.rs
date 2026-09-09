/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of nci.h. */

// Dependency constants from <net/nfc/nfc.h> are referenced by name below.

pub const NCI_MAX_NUM_MAPPING_CONFIGS: usize = 10;
pub const NCI_MAX_NUM_RF_CONFIGS: usize = 10;
pub const NCI_MAX_NUM_CONN: usize = 10;
pub const NCI_MAX_PARAM_LEN: usize = 251;
pub const NCI_MAX_PAYLOAD_SIZE: usize = 255;
pub const NCI_MAX_PACKET_SIZE: usize = 258;
pub const NCI_MAX_LARGE_PARAMS_NCI_V2: u8 = 15;
pub const NCI_VER_2_MASK: u8 = 0x20;

pub const NCI_STATUS_OK: u8 = 0x00; pub const NCI_STATUS_REJECTED: u8 = 0x01;
pub const NCI_STATUS_RF_FRAME_CORRUPTED: u8 = 0x02; pub const NCI_STATUS_FAILED: u8 = 0x03;
pub const NCI_STATUS_NOT_INITIALIZED: u8 = 0x04; pub const NCI_STATUS_SYNTAX_ERROR: u8 = 0x05;
pub const NCI_STATUS_SEMANTIC_ERROR: u8 = 0x06; pub const NCI_STATUS_UNKNOWN_GID: u8 = 0x07;
pub const NCI_STATUS_UNKNOWN_OID: u8 = 0x08; pub const NCI_STATUS_INVALID_PARAM: u8 = 0x09;
pub const NCI_STATUS_MESSAGE_SIZE_EXCEEDED: u8 = 0x0a;
pub const NCI_STATUS_DISCOVERY_ALREADY_STARTED: u8 = 0xa0;
pub const NCI_STATUS_DISCOVERY_TARGET_ACTIVATION_FAILED: u8 = 0xa1;
pub const NCI_STATUS_DISCOVERY_TEAR_DOWN: u8 = 0xa2;
pub const NCI_STATUS_RF_TRANSMISSION_ERROR: u8 = 0xb0; pub const NCI_STATUS_RF_PROTOCOL_ERROR: u8 = 0xb1;
pub const NCI_STATUS_RF_TIMEOUT_ERROR: u8 = 0xb2;
pub const NCI_STATUS_NFCEE_INTERFACE_ACTIVATION_FAILED: u8 = 0xc0;
pub const NCI_STATUS_NFCEE_TRANSMISSION_ERROR: u8 = 0xc1; pub const NCI_STATUS_NFCEE_PROTOCOL_ERROR: u8 = 0xc2;
pub const NCI_STATUS_NFCEE_TIMEOUT_ERROR: u8 = 0xc3;

pub const NCI_NFCEE_INTERFACE_APDU: u8 = 0; pub const NCI_NFCEE_INTERFACE_HCI_ACCESS: u8 = 1;
pub const NCI_NFCEE_INTERFACE_TYPE3_CMD_SET: u8 = 2; pub const NCI_NFCEE_INTERFACE_TRANSPARENT: u8 = 3;
pub const NCI_DESTINATION_NFCC_LOOPBACK: u8 = 1; pub const NCI_DESTINATION_REMOTE_NFC_ENDPOINT: u8 = 2;
pub const NCI_DESTINATION_NFCEE: u8 = 3;
pub const NCI_DESTINATION_SPECIFIC_PARAM_RF_TYPE: u8 = 0; pub const NCI_DESTINATION_SPECIFIC_PARAM_NFCEE_TYPE: u8 = 1;
pub const NCI_NFCEE_DISCOVERY_ACTION_DISABLE: u8 = 0; pub const NCI_NFCEE_DISCOVERY_ACTION_ENABLE: u8 = 1;

pub const NCI_NFC_A_PASSIVE_POLL_MODE: u8 = 0; pub const NCI_NFC_B_PASSIVE_POLL_MODE: u8 = 1;
pub const NCI_NFC_F_PASSIVE_POLL_MODE: u8 = 2; pub const NCI_NFC_A_ACTIVE_POLL_MODE: u8 = 3;
pub const NCI_NFC_F_ACTIVE_POLL_MODE: u8 = 5; pub const NCI_NFC_V_PASSIVE_POLL_MODE: u8 = 6;
pub const NCI_NFC_A_PASSIVE_LISTEN_MODE: u8 = 0x80; pub const NCI_NFC_B_PASSIVE_LISTEN_MODE: u8 = 0x81;
pub const NCI_NFC_F_PASSIVE_LISTEN_MODE: u8 = 0x82; pub const NCI_NFC_A_ACTIVE_LISTEN_MODE: u8 = 0x83;
pub const NCI_NFC_F_ACTIVE_LISTEN_MODE: u8 = 0x85; pub const NCI_RF_TECH_MODE_LISTEN_MASK: u8 = 0x80;
pub const NCI_NFC_RF_TECHNOLOGY_A: u8 = 0; pub const NCI_NFC_RF_TECHNOLOGY_B: u8 = 1;
pub const NCI_NFC_RF_TECHNOLOGY_F: u8 = 2; pub const NCI_NFC_RF_TECHNOLOGY_V: u8 = 3;
pub const NCI_NFC_BIT_RATE_106: u8 = 0; pub const NCI_NFC_BIT_RATE_212: u8 = 1;
pub const NCI_NFC_BIT_RATE_424: u8 = 2; pub const NCI_NFC_BIT_RATE_848: u8 = 3;
pub const NCI_NFC_BIT_RATE_1695: u8 = 4; pub const NCI_NFC_BIT_RATE_3390: u8 = 5;
pub const NCI_NFC_BIT_RATE_6780: u8 = 6; pub const NCI_NFC_BIT_RATE_26: u8 = 0x20;
pub const NCI_RF_PROTOCOL_UNKNOWN: u8 = 0; pub const NCI_RF_PROTOCOL_T1T: u8 = 1;
pub const NCI_RF_PROTOCOL_T2T: u8 = 2; pub const NCI_RF_PROTOCOL_T3T: u8 = 3;
pub const NCI_RF_PROTOCOL_ISO_DEP: u8 = 4; pub const NCI_RF_PROTOCOL_NFC_DEP: u8 = 5;
pub const NCI_RF_PROTOCOL_T5T: u8 = 6;
pub const NCI_RF_INTERFACE_NFCEE_DIRECT: u8 = 0; pub const NCI_RF_INTERFACE_FRAME: u8 = 1;
pub const NCI_RF_INTERFACE_ISO_DEP: u8 = 2; pub const NCI_RF_INTERFACE_NFC_DEP: u8 = 3;

pub const NCI_PN_ATR_REQ_GEN_BYTES: u8 = 0x29; pub const NCI_LN_ATR_RES_GEN_BYTES: u8 = 0x61;
pub const NCI_LA_SEL_INFO: u8 = 0x32; pub const NCI_LF_PROTOCOL_TYPE: u8 = 0x50; pub const NCI_LF_CON_BITR_F: u8 = 0x54;
pub const NCI_LA_SEL_INFO_ISO_DEP_MASK: u8 = 0x20; pub const NCI_LA_SEL_INFO_NFC_DEP_MASK: u8 = 0x40;
pub const NCI_LF_PROTOCOL_TYPE_NFC_DEP_MASK: u8 = 0x02; pub const NCI_LF_CON_BITR_F_212: u8 = 0x02; pub const NCI_LF_CON_BITR_F_424: u8 = 0x04;
pub const NCI_FEATURE_DISABLE: u8 = 0; pub const NCI_RESET_TYPE_KEEP_CONFIG: u8 = 0; pub const NCI_RESET_TYPE_RESET_CONFIG: u8 = 1;
pub const NCI_STATIC_RF_CONN_ID: u8 = 0; pub const NCI_DATA_FLOW_CONTROL_NOT_USED: u8 = 0xff;
pub const NCI_DISC_MAP_MODE_POLL: u8 = 1; pub const NCI_DISC_MAP_MODE_LISTEN: u8 = 2;
pub const NCI_DISCOVER_NTF_TYPE_LAST: u8 = 0; pub const NCI_DISCOVER_NTF_TYPE_LAST_NFCC: u8 = 1; pub const NCI_DISCOVER_NTF_TYPE_MORE: u8 = 2;
pub const NCI_DEACTIVATE_TYPE_IDLE_MODE: u8 = 0; pub const NCI_DEACTIVATE_TYPE_SLEEP_MODE: u8 = 1;
pub const NCI_DEACTIVATE_TYPE_SLEEP_AF_MODE: u8 = 2; pub const NCI_DEACTIVATE_TYPE_DISCOVERY: u8 = 3;
pub const NCI_MT_DATA_PKT: u8 = 0; pub const NCI_MT_CMD_PKT: u8 = 1; pub const NCI_MT_RSP_PKT: u8 = 2; pub const NCI_MT_NTF_PKT: u8 = 3;

#[inline] pub unsafe fn nci_mt(hdr: *const u8) -> u8 { ((*hdr >> 5) & 7) }
#[inline] pub unsafe fn nci_mt_set(hdr: *mut u8, mt: u8) { *hdr |= (mt & 7) << 5; }
pub const NCI_PBF_LAST: u8 = 0; pub const NCI_PBF_CONT: u8 = 1;
#[inline] pub unsafe fn nci_pbf(hdr: *const u8) -> u8 { (*hdr >> 4) & 1 }
#[inline] pub unsafe fn nci_pbf_set(hdr: *mut u8, pbf: u8) { *hdr |= (pbf & 1) << 4; }
#[inline] pub const fn nci_opcode_pack(gid: u8, oid: u8) -> u16 { (((gid as u16 & 0x0f) << 8) | (oid as u16 & 0x3f)) }
#[inline] pub unsafe fn nci_opcode(hdr: *const u8) -> u16 { nci_opcode_pack(*hdr, *hdr.add(1)) }
#[inline] pub const fn nci_opcode_gid(op: u16) -> u8 { ((op & 0x0f00) >> 8) as u8 }
#[inline] pub const fn nci_opcode_oid(op: u16) -> u8 { (op & 0x003f) as u8 }
#[inline] pub unsafe fn nci_plen(hdr: *const u8) -> u8 { *hdr.add(2) }
#[inline] pub unsafe fn nci_conn_id(hdr: *const u8) -> u8 { *hdr & 0x0f }

pub const NCI_GID_CORE: u8 = 0; pub const NCI_GID_RF_MGMT: u8 = 1; pub const NCI_GID_NFCEE_MGMT: u8 = 2; pub const NCI_GID_PROPRIETARY: u8 = 0xf;
pub const NCI_SPI_HDR_LEN: usize = 4; pub const NCI_SPI_CRC_LEN: usize = 2; pub const NCI_CTRL_HDR_SIZE: usize = 3; pub const NCI_DATA_HDR_SIZE: usize = 3;

#[repr(C, packed)] pub struct nci_ctrl_hdr { pub gid: u8, pub oid: u8, pub plen: u8 }
#[repr(C, packed)] pub struct nci_data_hdr { pub conn_id: u8, pub rfu: u8, pub plen: u8 }

macro_rules! opcode { ($name:ident, $gid:expr, $oid:expr) => { pub const $name: u16 = nci_opcode_pack($gid, $oid); }; }
opcode!(NCI_OP_CORE_RESET_CMD, NCI_GID_CORE, 0); opcode!(NCI_OP_CORE_INIT_CMD, NCI_GID_CORE, 1); opcode!(NCI_OP_CORE_SET_CONFIG_CMD, NCI_GID_CORE, 2);
opcode!(NCI_OP_CORE_CONN_CREATE_CMD, NCI_GID_CORE, 4); opcode!(NCI_OP_CORE_CONN_CLOSE_CMD, NCI_GID_CORE, 5); opcode!(NCI_OP_CORE_GET_CONFIG_CMD, NCI_GID_CORE, 3);
opcode!(NCI_OP_RF_DISCOVER_MAP_CMD, NCI_GID_RF_MGMT, 0); opcode!(NCI_OP_RF_DISCOVER_CMD, NCI_GID_RF_MGMT, 3); opcode!(NCI_OP_RF_DISCOVER_SELECT_CMD, NCI_GID_RF_MGMT, 4); opcode!(NCI_OP_RF_DEACTIVATE_CMD, NCI_GID_RF_MGMT, 6);
opcode!(NCI_OP_NFCEE_DISCOVER_CMD, NCI_GID_NFCEE_MGMT, 0); opcode!(NCI_OP_NFCEE_MODE_SET_CMD, NCI_GID_NFCEE_MGMT, 1);

#[repr(C, packed)] pub struct nci_core_reset_cmd { pub reset_type: u8 }
#[repr(C)] pub struct nci_core_init_v2_cmd { pub feature1: u8, pub feature2: u8 }
#[repr(C, packed)] pub struct set_config_param { pub id: u8, pub len: u8, pub val: [u8; NCI_MAX_PARAM_LEN] }
#[repr(C, packed)] pub struct nci_core_set_config_cmd { pub num_params: u8, pub param: set_config_param }
pub const DEST_SPEC_PARAMS_ID_INDEX: usize = 0; pub const DEST_SPEC_PARAMS_PROTOCOL_INDEX: usize = 1;
#[repr(C, packed)] pub struct dest_spec_params { pub id: u8, pub protocol: u8 }
#[repr(C, packed)] pub struct core_conn_create_dest_spec_params { pub type_: u8, pub length: u8, pub value: [u8; 0] }
#[repr(C, packed)] pub struct nci_core_conn_create_cmd { pub destination_type: u8, pub number_destination_params: u8, pub params: [core_conn_create_dest_spec_params; 0] }
#[repr(C, packed)] pub struct disc_map_config { pub rf_protocol: u8, pub mode: u8, pub rf_interface: u8 }
#[repr(C, packed)] pub struct nci_rf_disc_map_cmd { pub num_mapping_configs: u8, pub mapping_configs: [disc_map_config; NCI_MAX_NUM_MAPPING_CONFIGS] }
#[repr(C, packed)] pub struct disc_config { pub rf_tech_and_mode: u8, pub frequency: u8 }
#[repr(C, packed)] pub struct nci_rf_disc_cmd { pub num_disc_configs: u8, pub disc_configs: [disc_config; NCI_MAX_NUM_RF_CONFIGS] }
#[repr(C, packed)] pub struct nci_rf_discover_select_cmd { pub rf_discovery_id: u8, pub rf_protocol: u8, pub rf_interface: u8 }
#[repr(C, packed)] pub struct nci_rf_deactivate_cmd { pub type_: u8 }
#[repr(C, packed)] pub struct nci_nfcee_discover_cmd { pub discovery_action: u8 }
pub const NCI_NFCEE_DISABLE: u8 = 0; pub const NCI_NFCEE_ENABLE: u8 = 1;
#[repr(C, packed)] pub struct nci_nfcee_mode_set_cmd { pub nfcee_id: u8, pub nfcee_mode: u8 }

opcode!(NCI_OP_CORE_RESET_RSP, NCI_GID_CORE, 0); opcode!(NCI_OP_CORE_INIT_RSP, NCI_GID_CORE, 1); opcode!(NCI_OP_CORE_SET_CONFIG_RSP, NCI_GID_CORE, 2); opcode!(NCI_OP_CORE_CONN_CREATE_RSP, NCI_GID_CORE, 4); opcode!(NCI_OP_CORE_CONN_CLOSE_RSP, NCI_GID_CORE, 5);
#[repr(C, packed)] pub struct nci_core_reset_rsp { pub status: u8, pub nci_ver: u8, pub config_status: u8 }
#[repr(C, packed)] pub struct nci_core_init_rsp_1 { pub status: u8, pub nfcc_features: u32, pub num_supported_rf_interfaces: u8, pub supported_rf_interfaces: [u8; 0] }
#[repr(C, packed)] pub struct nci_core_init_rsp_2 { pub max_logical_connections: u8, pub max_routing_table_size: u16, pub max_ctrl_pkt_payload_len: u8, pub max_size_for_large_params: u16, pub manufact_id: u8, pub manufact_specific_info: u32 }
#[repr(C, packed)] pub struct nci_core_init_rsp_nci_ver2 { pub status: u8, pub nfcc_features: u32, pub max_logical_connections: u8, pub max_routing_table_size: u16, pub max_ctrl_pkt_payload_len: u8, pub max_data_pkt_hci_payload_len: u8, pub number_of_hci_credit: u8, pub max_nfc_v_frame_size: u16, pub num_supported_rf_interfaces: u8, pub supported_rf_interfaces: [u8; 0] }
#[repr(C, packed)] pub struct nci_core_set_config_rsp { pub status: u8, pub num_params: u8, pub params_id: [u8; 0] }
#[repr(C, packed)] pub struct nci_core_conn_create_rsp { pub status: u8, pub max_ctrl_pkt_payload_len: u8, pub credits_cnt: u8, pub conn_id: u8 }
opcode!(NCI_OP_RF_DISCOVER_MAP_RSP, NCI_GID_RF_MGMT, 0); opcode!(NCI_OP_RF_DISCOVER_RSP, NCI_GID_RF_MGMT, 3); opcode!(NCI_OP_RF_DISCOVER_SELECT_RSP, NCI_GID_RF_MGMT, 4); opcode!(NCI_OP_RF_DEACTIVATE_RSP, NCI_GID_RF_MGMT, 6); opcode!(NCI_OP_NFCEE_DISCOVER_RSP, NCI_GID_NFCEE_MGMT, 0); opcode!(NCI_OP_NFCEE_MODE_SET_RSP, NCI_GID_NFCEE_MGMT, 1); opcode!(NCI_OP_CORE_GET_CONFIG_RSP, NCI_GID_CORE, 3);
#[repr(C, packed)] pub struct nci_nfcee_discover_rsp { pub status: u8, pub num_nfcee: u8 }

opcode!(NCI_OP_CORE_RESET_NTF, NCI_GID_CORE, 0); opcode!(NCI_OP_CORE_CONN_CREDITS_NTF, NCI_GID_CORE, 6); opcode!(NCI_OP_CORE_GENERIC_ERROR_NTF, NCI_GID_CORE, 7); opcode!(NCI_OP_CORE_INTF_ERROR_NTF, NCI_GID_CORE, 8);
#[repr(C, packed)] pub struct nci_core_reset_ntf { pub reset_trigger: u8, pub config_status: u8, pub nci_ver: u8, pub manufact_id: u8, pub manufacturer_specific_len: u8, pub manufact_specific_info: u32 }
#[repr(C, packed)] pub struct conn_credit_entry { pub conn_id: u8, pub credits: u8 }
#[repr(C, packed)] pub struct nci_core_conn_credit_ntf { pub num_entries: u8, pub conn_entries: [conn_credit_entry; NCI_MAX_NUM_CONN] }
#[repr(C, packed)] pub struct nci_core_intf_error_ntf { pub status: u8, pub conn_id: u8 }

// NFC size constants below are supplied by <net/nfc/nfc.h>.
#[repr(C, packed)] pub struct rf_tech_specific_params_nfca_poll { pub sens_res: u16, pub nfcid1_len: u8, pub nfcid1: [u8; NFC_NFCID1_MAXSIZE], pub sel_res_len: u8, pub sel_res: u8 }
#[repr(C, packed)] pub struct rf_tech_specific_params_nfcb_poll { pub sensb_res_len: u8, pub sensb_res: [u8; NFC_SENSB_RES_MAXSIZE] }
#[repr(C, packed)] pub struct rf_tech_specific_params_nfcf_poll { pub bit_rate: u8, pub sensf_res_len: u8, pub sensf_res: [u8; NFC_SENSF_RES_MAXSIZE] }
#[repr(C, packed)] pub struct rf_tech_specific_params_nfcv_poll { pub res_flags: u8, pub dsfid: u8, pub uid: [u8; NFC_ISO15693_UID_MAXSIZE] }
#[repr(C, packed)] pub struct rf_tech_specific_params_nfcf_listen { pub local_nfcid2_len: u8, pub local_nfcid2: [u8; NFC_NFCID2_MAXSIZE] }
#[repr(C)] pub union rf_tech_specific_params { pub nfca_poll: rf_tech_specific_params_nfca_poll, pub nfcb_poll: rf_tech_specific_params_nfcb_poll, pub nfcf_poll: rf_tech_specific_params_nfcf_poll, pub nfcv_poll: rf_tech_specific_params_nfcv_poll, pub nfcf_listen: rf_tech_specific_params_nfcf_listen }
#[repr(C, packed)] pub struct nci_rf_discover_ntf { pub rf_discovery_id: u8, pub rf_protocol: u8, pub rf_tech_and_mode: u8, pub rf_tech_specific_params_len: u8, pub rf_tech_specific_params: rf_tech_specific_params, pub ntf_type: u8 }

opcode!(NCI_OP_RF_DISCOVER_NTF, NCI_GID_RF_MGMT, 3); opcode!(NCI_OP_RF_INTF_ACTIVATED_NTF, NCI_GID_RF_MGMT, 5);
#[repr(C, packed)] pub struct activation_params_nfca_poll_iso_dep { pub rats_res_len: u8, pub rats_res: [u8; NFC_ATS_MAXSIZE] }
#[repr(C, packed)] pub struct activation_params_nfcb_poll_iso_dep { pub attrib_res_len: u8, pub attrib_res: [u8; 50] }
#[repr(C, packed)] pub struct activation_params_poll_nfc_dep { pub atr_res_len: u8, pub atr_res: [u8; NFC_ATR_RES_MAXSIZE - 2] }
#[repr(C, packed)] pub struct activation_params_listen_nfc_dep { pub atr_req_len: u8, pub atr_req: [u8; NFC_ATR_REQ_MAXSIZE - 2] }
#[repr(C)] pub union activation_params { pub nfca_poll_iso_dep: activation_params_nfca_poll_iso_dep, pub nfcb_poll_iso_dep: activation_params_nfcb_poll_iso_dep, pub poll_nfc_dep: activation_params_poll_nfc_dep, pub listen_nfc_dep: activation_params_listen_nfc_dep }
#[repr(C, packed)] pub struct nci_rf_intf_activated_ntf { pub rf_discovery_id: u8, pub rf_interface: u8, pub rf_protocol: u8, pub activation_rf_tech_and_mode: u8, pub max_data_pkt_payload_size: u8, pub initial_num_credits: u8, pub rf_tech_specific_params_len: u8, pub rf_tech_specific_params: rf_tech_specific_params, pub data_exch_rf_tech_and_mode: u8, pub data_exch_tx_bit_rate: u8, pub data_exch_rx_bit_rate: u8, pub activation_params_len: u8, pub activation_params: activation_params }
opcode!(NCI_OP_RF_DEACTIVATE_NTF, NCI_GID_RF_MGMT, 6); opcode!(NCI_OP_RF_NFCEE_ACTION_NTF, NCI_GID_RF_MGMT, 9);
#[repr(C, packed)] pub struct nci_rf_deactivate_ntf { pub type_: u8, pub reason: u8 }
#[repr(C, packed)] pub struct nci_rf_nfcee_action_ntf { pub nfcee_id: u8, pub trigger: u8, pub supported_data_length: u8, pub supported_data: [u8; 0] }
opcode!(NCI_OP_NFCEE_DISCOVER_NTF, NCI_GID_NFCEE_MGMT, 0);
#[repr(C, packed)] pub struct nci_nfcee_supported_protocol { pub num_protocol: u8, pub supported_protocol: [u8; 0] }
#[repr(C, packed)] pub struct nci_nfcee_information_tlv { pub num_tlv: u8, pub information_tlv: [u8; 0] }
#[repr(C, packed)] pub struct nci_nfcee_discover_ntf { pub nfcee_id: u8, pub nfcee_status: u8, pub supported_protocols: nci_nfcee_supported_protocol, pub information_tlv: nci_nfcee_information_tlv }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
