/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependency declarations from icp_qat_fw.h are expected to be supplied externally.

#[repr(C)]
pub struct icp_qat_fw_req_hdr_pke_cd_pars {
    pub content_desc_addr: __u64,
    pub content_desc_resrvd: __u32,
    pub func_id: __u32,
}

#[repr(C)]
pub struct icp_qat_fw_req_pke_mid {
    pub opaque: __u64,
    pub src_data_addr: __u64,
    pub dest_data_addr: __u64,
}

#[repr(C)]
pub struct icp_qat_fw_req_pke_hdr {
    pub resrvd1: __u8,
    pub resrvd2: __u8,
    pub service_type: __u8,
    pub hdr_flags: __u8,
    pub comn_req_flags: __u16,
    pub resrvd4: __u16,
    pub cd_pars: icp_qat_fw_req_hdr_pke_cd_pars,
}

#[repr(C)]
pub struct icp_qat_fw_pke_request {
    pub pke_hdr: icp_qat_fw_req_pke_hdr,
    pub pke_mid: icp_qat_fw_req_pke_mid,
    pub output_param_count: __u8,
    pub input_param_count: __u8,
    pub resrvd1: __u16,
    pub resrvd2: __u32,
    pub next_req_adr: __u64,
}

#[repr(C)]
pub struct icp_qat_fw_resp_pke_hdr {
    pub resrvd1: __u8,
    pub resrvd2: __u8,
    pub response_type: __u8,
    pub hdr_flags: __u8,
    pub comn_resp_flags: __u16,
    pub resrvd4: __u16,
}

#[repr(C)]
pub struct icp_qat_fw_pke_resp {
    pub pke_resp_hdr: icp_qat_fw_resp_pke_hdr,
    pub opaque: __u64,
    pub src_data_addr: __u64,
    pub dest_data_addr: __u64,
}

pub const ICP_QAT_FW_PKE_HDR_VALID_FLAG_BITPOS: u32 = 7;
pub const ICP_QAT_FW_PKE_HDR_VALID_FLAG_MASK: u32 = 0x1;

#[macro_export]
macro_rules! ICP_QAT_FW_PKE_RESP_PKE_STAT_GET {
    ($status_word:expr) => {
        QAT_FIELD_GET(
            (($status_word >> ICP_QAT_FW_COMN_ONE_BYTE_SHIFT)
                & ICP_QAT_FW_COMN_SINGLE_BYTE_MASK),
            QAT_COMN_RESP_PKE_STATUS_BITPOS,
            QAT_COMN_RESP_PKE_STATUS_MASK,
        )
    };
}

#[macro_export]
macro_rules! ICP_QAT_FW_PKE_HDR_VALID_FLAG_SET {
    ($hdr_t:expr, $val:expr) => {
        QAT_FIELD_SET(
            ($hdr_t.hdr_flags),
            ($val),
            ICP_QAT_FW_PKE_HDR_VALID_FLAG_BITPOS,
            ICP_QAT_FW_PKE_HDR_VALID_FLAG_MASK,
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
