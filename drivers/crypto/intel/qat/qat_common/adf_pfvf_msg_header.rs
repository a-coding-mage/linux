/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2015 - 2021 Intel Corporation */

/* PF<->VF Gen2 and Gen4 messaging declarations. */

/* PFVF message common bits */
pub const ADF_PFVF_INT: u32 = 1u32 << 0;
pub const ADF_PFVF_MSGORIGIN_SYSTEM: u32 = 1u32 << 1;

/* Different generations have different CSR layouts, use this struct to
 * abstract these differences away. */
#[repr(C)]
pub struct pfvf_message {
    pub type_: u8,
    pub data: u32,
}

/* PF->VF messages */
#[repr(C)]
pub enum pf2vf_msgtype {
    ADF_PF2VF_MSGTYPE_RESTARTING = 0x01,
    ADF_PF2VF_MSGTYPE_VERSION_RESP = 0x02,
    ADF_PF2VF_MSGTYPE_BLKMSG_RESP = 0x03,
    ADF_PF2VF_MSGTYPE_FATAL_ERROR = 0x04,
    ADF_PF2VF_MSGTYPE_RESTARTED = 0x05,
    /* Values from 0x10 are Gen4 specific, message type is only 4 bits in Gen2 devices. */
    ADF_PF2VF_MSGTYPE_RP_RESET_RESP = 0x10,
}

/* VF->PF messages */
#[repr(C)]
pub enum vf2pf_msgtype {
    ADF_VF2PF_MSGTYPE_INIT = 0x03,
    ADF_VF2PF_MSGTYPE_SHUTDOWN = 0x04,
    ADF_VF2PF_MSGTYPE_VERSION_REQ = 0x05,
    ADF_VF2PF_MSGTYPE_COMPAT_VER_REQ = 0x06,
    ADF_VF2PF_MSGTYPE_LARGE_BLOCK_REQ = 0x07,
    ADF_VF2PF_MSGTYPE_MEDIUM_BLOCK_REQ = 0x08,
    ADF_VF2PF_MSGTYPE_SMALL_BLOCK_REQ = 0x09,
    ADF_VF2PF_MSGTYPE_RESTARTING_COMPLETE = 0x0a,
    /* Values from 0x10 are Gen4 specific, message type is only 4 bits in Gen2 devices. */
    ADF_VF2PF_MSGTYPE_RP_RESET = 0x10,
}

/* VF/PF compatibility version. */
#[repr(C)]
pub enum pfvf_compatibility_version {
    /* Support for extended capabilities */
    ADF_PFVF_COMPAT_CAPABILITIES = 0x02,
    /* In-use pattern cleared by receiver */
    ADF_PFVF_COMPAT_FAST_ACK = 0x03,
    /* Ring to service mapping support for non-standard mappings */
    ADF_PFVF_COMPAT_RING_TO_SVC_MAP = 0x04,
    /* Fallback compat */
    ADF_PFVF_COMPAT_FALLBACK = 0x05,
    /* Reference to the latest version */
    ADF_PFVF_COMPAT_THIS_VERSION = 0x05,
}

/* PF->VF Version Response */
pub const ADF_PF2VF_VERSION_RESP_VERS_MASK: u32 = 0xff;
pub const ADF_PF2VF_VERSION_RESP_RESULT_MASK: u32 = 0x300;

#[repr(C)]
pub enum pf2vf_compat_response {
    ADF_PF2VF_VF_COMPATIBLE = 0x01,
    ADF_PF2VF_VF_INCOMPATIBLE = 0x02,
    ADF_PF2VF_VF_COMPAT_UNKNOWN = 0x03,
}

#[repr(C)]
pub enum ring_reset_result {
    RPRESET_SUCCESS = 0x00,
    RPRESET_NOT_SUPPORTED = 0x01,
    RPRESET_INVAL_BANK = 0x02,
    RPRESET_TIMEOUT = 0x03,
}

pub const ADF_VF2PF_RNG_RESET_RP_MASK: u32 = 0x3;
pub const ADF_VF2PF_RNG_RESET_RSVD_MASK: u32 = 0x03ff_fffc;

/* PF->VF Block Responses */
pub const ADF_PF2VF_BLKMSG_RESP_TYPE_MASK: u32 = 0x3;
pub const ADF_PF2VF_BLKMSG_RESP_DATA_MASK: u32 = 0x3fc;

#[repr(C)]
pub enum pf2vf_blkmsg_resp_type {
    ADF_PF2VF_BLKMSG_RESP_TYPE_DATA = 0x00,
    ADF_PF2VF_BLKMSG_RESP_TYPE_CRC = 0x01,
    ADF_PF2VF_BLKMSG_RESP_TYPE_ERROR = 0x02,
}

/* PF->VF Block Error Code */
#[repr(C)]
pub enum pf2vf_blkmsg_error {
    ADF_PF2VF_INVALID_BLOCK_TYPE = 0x00,
    ADF_PF2VF_INVALID_BYTE_NUM_REQ = 0x01,
    ADF_PF2VF_PAYLOAD_TRUNCATED = 0x02,
    ADF_PF2VF_UNSPECIFIED_ERROR = 0x03,
}

/* VF->PF Block Requests */
pub const ADF_VF2PF_LARGE_BLOCK_TYPE_MASK: u32 = 0x3;
pub const ADF_VF2PF_LARGE_BLOCK_BYTE_MASK: u32 = 0x1fc;
pub const ADF_VF2PF_MEDIUM_BLOCK_TYPE_MASK: u32 = 0x7;
pub const ADF_VF2PF_MEDIUM_BLOCK_BYTE_MASK: u32 = 0x1f8;
pub const ADF_VF2PF_SMALL_BLOCK_TYPE_MASK: u32 = 0xf;
pub const ADF_VF2PF_SMALL_BLOCK_BYTE_MASK: u32 = 0x1f0;
pub const ADF_VF2PF_BLOCK_CRC_REQ_MASK: u32 = 1u32 << 9;

/* PF->VF Block Request Types
 *  0..15 - 32 byte message
 * 16..23 - 64 byte message
 * 24..27 - 128 byte message
 */
#[repr(C)]
pub enum vf2pf_blkmsg_req_type {
    ADF_VF2PF_BLKMSG_REQ_CAP_SUMMARY = 0x02,
    ADF_VF2PF_BLKMSG_REQ_RING_SVC_MAP = 0x03,
}

pub const ADF_VF2PF_SMALL_BLOCK_TYPE_MAX: u32 = 0xf;
pub const ADF_VF2PF_MEDIUM_BLOCK_TYPE_MAX: u32 = 0x7 + ADF_VF2PF_SMALL_BLOCK_TYPE_MAX + 1;
pub const ADF_VF2PF_LARGE_BLOCK_TYPE_MAX: u32 = 0x3 + ADF_VF2PF_MEDIUM_BLOCK_TYPE_MAX;
pub const ADF_VF2PF_SMALL_BLOCK_BYTE_MAX: u32 = 0x1f0;
pub const ADF_VF2PF_MEDIUM_BLOCK_BYTE_MAX: u32 = 0x1f8;
pub const ADF_VF2PF_LARGE_BLOCK_BYTE_MAX: u32 = 0x1fc;

#[repr(C, packed)]
pub struct pfvf_blkmsg_header {
    pub version: u8,
    pub payload_size: u8,
}

pub const ADF_PFVF_BLKMSG_HEADER_SIZE: usize = core::mem::size_of::<pfvf_blkmsg_header>();
pub const ADF_PFVF_BLKMSG_MSG_MAX_SIZE: usize = 128;

#[macro_export]
macro_rules! ADF_PFVF_BLKMSG_PAYLOAD_SIZE {
    ($blkmsg:ty) => { core::mem::size_of::<$blkmsg>() - $crate::ADF_PFVF_BLKMSG_HEADER_SIZE };
}

#[macro_export]
macro_rules! ADF_PFVF_BLKMSG_MSG_SIZE {
    ($blkmsg:expr) => { $crate::ADF_PFVF_BLKMSG_HEADER_SIZE + ($blkmsg).hdr.payload_size as usize };
}

/* PF->VF Block message header bytes */
pub const ADF_PFVF_BLKMSG_VER_BYTE: u32 = 0;
pub const ADF_PFVF_BLKMSG_LEN_BYTE: u32 = 1;

/* PF/VF Capabilities message values */
#[repr(C)]
pub enum blkmsg_capabilities_versions {
    ADF_PFVF_CAPABILITIES_V1_VERSION = 0x01,
    ADF_PFVF_CAPABILITIES_V2_VERSION = 0x02,
    ADF_PFVF_CAPABILITIES_V3_VERSION = 0x03,
}

#[repr(C, packed)]
pub struct capabilities_v1 {
    pub hdr: pfvf_blkmsg_header,
    pub ext_dc_caps: u32,
}

#[repr(C, packed)]
pub struct capabilities_v2 {
    pub hdr: pfvf_blkmsg_header,
    pub ext_dc_caps: u32,
    pub capabilities: u32,
}

#[repr(C, packed)]
pub struct capabilities_v3 {
    pub hdr: pfvf_blkmsg_header,
    pub ext_dc_caps: u32,
    pub capabilities: u32,
    pub frequency: u32,
}

/* PF/VF Ring to service mapping values */
#[repr(C)]
pub enum blkmsg_ring_to_svc_versions {
    ADF_PFVF_RING_TO_SVC_VERSION = 0x01,
}

#[repr(C, packed)]
pub struct ring_to_svc_map_v1 {
    pub hdr: pfvf_blkmsg_header,
    pub map: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
