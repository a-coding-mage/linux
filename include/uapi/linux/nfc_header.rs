/*
 * Copyright (C) 2011 Instituto Nokia de Tecnologia
 *
 * Authors:
 *    Lauro Ramos Venancio <lauro.venancio@openbossa.org>
 *    Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 */

// Dependencies: linux/types.h and linux/socket.h provide the kernel types used below.

pub const NFC_GENL_NAME: &str = "nfc";
pub const NFC_GENL_VERSION: u32 = 1;
pub const NFC_GENL_MCAST_EVENT_NAME: &str = "events";

#[repr(u32)]
pub enum nfc_commands {
    NFC_CMD_UNSPEC,
    NFC_CMD_GET_DEVICE,
    NFC_CMD_DEV_UP,
    NFC_CMD_DEV_DOWN,
    NFC_CMD_DEP_LINK_UP,
    NFC_CMD_DEP_LINK_DOWN,
    NFC_CMD_START_POLL,
    NFC_CMD_STOP_POLL,
    NFC_CMD_GET_TARGET,
    NFC_EVENT_TARGETS_FOUND,
    NFC_EVENT_DEVICE_ADDED,
    NFC_EVENT_DEVICE_REMOVED,
    NFC_EVENT_TARGET_LOST,
    NFC_EVENT_TM_ACTIVATED,
    NFC_EVENT_TM_DEACTIVATED,
    NFC_CMD_LLC_GET_PARAMS,
    NFC_CMD_LLC_SET_PARAMS,
    NFC_CMD_ENABLE_SE,
    NFC_CMD_DISABLE_SE,
    NFC_CMD_LLC_SDREQ,
    NFC_EVENT_LLC_SDRES,
    NFC_CMD_FW_DOWNLOAD,
    NFC_EVENT_SE_ADDED,
    NFC_EVENT_SE_REMOVED,
    NFC_EVENT_SE_CONNECTIVITY,
    NFC_EVENT_SE_TRANSACTION,
    NFC_CMD_GET_SE,
    NFC_CMD_SE_IO,
    NFC_CMD_ACTIVATE_TARGET,
    NFC_CMD_VENDOR,
    NFC_CMD_DEACTIVATE_TARGET,
    // private: internal use only
    __NFC_CMD_AFTER_LAST,
}
pub const NFC_CMD_MAX: u32 = __NFC_CMD_AFTER_LAST as u32 - 1;

#[repr(u32)]
pub enum nfc_attrs {
    NFC_ATTR_UNSPEC,
    NFC_ATTR_DEVICE_INDEX,
    NFC_ATTR_DEVICE_NAME,
    NFC_ATTR_PROTOCOLS,
    NFC_ATTR_TARGET_INDEX,
    NFC_ATTR_TARGET_SENS_RES,
    NFC_ATTR_TARGET_SEL_RES,
    NFC_ATTR_TARGET_NFCID1,
    NFC_ATTR_TARGET_SENSB_RES,
    NFC_ATTR_TARGET_SENSF_RES,
    NFC_ATTR_COMM_MODE,
    NFC_ATTR_RF_MODE,
    NFC_ATTR_DEVICE_POWERED,
    NFC_ATTR_IM_PROTOCOLS,
    NFC_ATTR_TM_PROTOCOLS,
    NFC_ATTR_LLC_PARAM_LTO,
    NFC_ATTR_LLC_PARAM_RW,
    NFC_ATTR_LLC_PARAM_MIUX,
    NFC_ATTR_SE,
    NFC_ATTR_LLC_SDP,
    NFC_ATTR_FIRMWARE_NAME,
    NFC_ATTR_SE_INDEX,
    NFC_ATTR_SE_TYPE,
    NFC_ATTR_SE_AID,
    NFC_ATTR_FIRMWARE_DOWNLOAD_STATUS,
    NFC_ATTR_SE_APDU,
    NFC_ATTR_TARGET_ISO15693_DSFID,
    NFC_ATTR_TARGET_ISO15693_UID,
    NFC_ATTR_SE_PARAMS,
    NFC_ATTR_VENDOR_ID,
    NFC_ATTR_VENDOR_SUBCMD,
    NFC_ATTR_VENDOR_DATA,
    NFC_ATTR_TARGET_ATS,
    // private: internal use only
    __NFC_ATTR_AFTER_LAST,
}
pub const NFC_ATTR_MAX: u32 = __NFC_ATTR_AFTER_LAST as u32 - 1;

#[repr(u32)]
pub enum nfc_sdp_attr {
    NFC_SDP_ATTR_UNSPEC,
    NFC_SDP_ATTR_URI,
    NFC_SDP_ATTR_SAP,
    // private: internal use only
    __NFC_SDP_ATTR_AFTER_LAST,
}
pub const NFC_SDP_ATTR_MAX: u32 = __NFC_SDP_ATTR_AFTER_LAST as u32 - 1;

pub const NFC_DEVICE_NAME_MAXSIZE: usize = 8;
pub const NFC_NFCID1_MAXSIZE: usize = 10;
pub const NFC_NFCID2_MAXSIZE: usize = 8;
pub const NFC_NFCID3_MAXSIZE: usize = 10;
pub const NFC_SENSB_RES_MAXSIZE: usize = 12;
pub const NFC_SENSF_RES_MAXSIZE: usize = 18;
pub const NFC_ATR_REQ_MAXSIZE: usize = 64;
pub const NFC_ATR_RES_MAXSIZE: usize = 64;
pub const NFC_ATR_REQ_GB_MAXSIZE: usize = 48;
pub const NFC_ATR_RES_GB_MAXSIZE: usize = 47;
pub const NFC_GB_MAXSIZE: usize = 48;
pub const NFC_FIRMWARE_NAME_MAXSIZE: usize = 32;
pub const NFC_ISO15693_UID_MAXSIZE: usize = 8;
pub const NFC_ATS_MAXSIZE: usize = 20;

pub const NFC_PROTO_JEWEL: u32 = 1;
pub const NFC_PROTO_MIFARE: u32 = 2;
pub const NFC_PROTO_FELICA: u32 = 3;
pub const NFC_PROTO_ISO14443: u32 = 4;
pub const NFC_PROTO_NFC_DEP: u32 = 5;
pub const NFC_PROTO_ISO14443_B: u32 = 6;
pub const NFC_PROTO_ISO15693: u32 = 7;
pub const NFC_PROTO_MAX: u32 = 8;

pub const NFC_COMM_ACTIVE: u32 = 0;
pub const NFC_COMM_PASSIVE: u32 = 1;
pub const NFC_RF_INITIATOR: u32 = 0;
pub const NFC_RF_TARGET: u32 = 1;
pub const NFC_RF_NONE: u32 = 2;

pub const NFC_PROTO_JEWEL_MASK: u32 = 1 << NFC_PROTO_JEWEL;
pub const NFC_PROTO_MIFARE_MASK: u32 = 1 << NFC_PROTO_MIFARE;
pub const NFC_PROTO_FELICA_MASK: u32 = 1 << NFC_PROTO_FELICA;
pub const NFC_PROTO_ISO14443_MASK: u32 = 1 << NFC_PROTO_ISO14443;
pub const NFC_PROTO_NFC_DEP_MASK: u32 = 1 << NFC_PROTO_NFC_DEP;
pub const NFC_PROTO_ISO14443_B_MASK: u32 = 1 << NFC_PROTO_ISO14443_B;
pub const NFC_PROTO_ISO15693_MASK: u32 = 1 << NFC_PROTO_ISO15693;

pub const NFC_SE_UICC: u32 = 0x1;
pub const NFC_SE_EMBEDDED: u32 = 0x2;
pub const NFC_SE_DISABLED: u32 = 0x0;
pub const NFC_SE_ENABLED: u32 = 0x1;

#[repr(C)]
pub struct sockaddr_nfc {
    pub sa_family: __kernel_sa_family_t,
    pub dev_idx: __u32,
    pub target_idx: __u32,
    pub nfc_protocol: __u32,
}

pub const NFC_LLCP_MAX_SERVICE_NAME: usize = 63;

#[repr(C)]
pub struct sockaddr_nfc_llcp {
    pub sa_family: __kernel_sa_family_t,
    pub dev_idx: __u32,
    pub target_idx: __u32,
    pub nfc_protocol: __u32,
    pub dsap: __u8, // Destination SAP, if known
    pub ssap: __u8, // Source SAP to be bound to
    pub service_name: [core::ffi::c_char; NFC_LLCP_MAX_SERVICE_NAME], // Service name URI
    pub service_name_len: __kernel_size_t,
}

pub const NFC_SOCKPROTO_RAW: u32 = 0;
pub const NFC_SOCKPROTO_LLCP: u32 = 1;
pub const NFC_SOCKPROTO_MAX: u32 = 2;
pub const NFC_HEADER_SIZE: usize = 1;

pub const NFC_RAW_HEADER_SIZE: usize = 2;
pub const NFC_DIRECTION_RX: u32 = 0x00;
pub const NFC_DIRECTION_TX: u32 = 0x01;
pub const RAW_PAYLOAD_LLCP: u32 = 0;
pub const RAW_PAYLOAD_NCI: u32 = 1;
pub const RAW_PAYLOAD_HCI: u32 = 2;
pub const RAW_PAYLOAD_DIGITAL: u32 = 3;
pub const RAW_PAYLOAD_PROPRIETARY: u32 = 4;

pub const NFC_LLCP_RW: u32 = 0;
pub const NFC_LLCP_MIUX: u32 = 1;
pub const NFC_LLCP_REMOTE_MIU: u32 = 2;
pub const NFC_LLCP_REMOTE_LTO: u32 = 3;
pub const NFC_LLCP_REMOTE_RW: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
