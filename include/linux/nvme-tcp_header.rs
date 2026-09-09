/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NVMe over Fabrics TCP protocol header.
 * Copyright (c) 2018 LightBits Labs. All rights reserved.
 */

// Dependency: <linux/nvme.h>

pub const NVME_TCP_DISC_PORT: u32 = 8009;
pub const NVME_TCP_ADMIN_CCSZ: usize = SZ_8K;
pub const NVME_TCP_DIGEST_LENGTH: u32 = 4;
pub const NVME_TCP_MIN_MAXH2CDATA: u32 = 4096;
pub const NVME_TCP_MIN_C2HTERM_PLEN: u32 = 24;
pub const NVME_TCP_MAX_C2HTERM_PLEN: u32 = 152;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nvme_tcp_pfv {
    NVME_TCP_PFV_1_0 = 0x0,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nvme_tcp_tls_cipher {
    NVME_TCP_TLS_CIPHER_INVALID = 0,
    NVME_TCP_TLS_CIPHER_SHA256 = 1,
    NVME_TCP_TLS_CIPHER_SHA384 = 2,
}

pub type nvme_tcp_fatal_error_status = u16;
pub const NVME_TCP_FES_INVALID_PDU_HDR: nvme_tcp_fatal_error_status = 0x01;
pub const NVME_TCP_FES_PDU_SEQ_ERR: nvme_tcp_fatal_error_status = 0x02;
pub const NVME_TCP_FES_HDR_DIGEST_ERR: nvme_tcp_fatal_error_status = 0x03;
pub const NVME_TCP_FES_DATA_OUT_OF_RANGE: nvme_tcp_fatal_error_status = 0x04;
pub const NVME_TCP_FES_R2T_LIMIT_EXCEEDED: nvme_tcp_fatal_error_status = 0x05;
pub const NVME_TCP_FES_DATA_LIMIT_EXCEEDED: nvme_tcp_fatal_error_status = 0x05;
pub const NVME_TCP_FES_UNSUPPORTED_PARAM: nvme_tcp_fatal_error_status = 0x06;

pub type nvme_tcp_digest_option = u32;
pub const NVME_TCP_HDR_DIGEST_ENABLE: nvme_tcp_digest_option = 1 << 0;
pub const NVME_TCP_DATA_DIGEST_ENABLE: nvme_tcp_digest_option = 1 << 1;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nvme_tcp_pdu_type {
    nvme_tcp_icreq = 0x0,
    nvme_tcp_icresp = 0x1,
    nvme_tcp_h2c_term = 0x2,
    nvme_tcp_c2h_term = 0x3,
    nvme_tcp_cmd = 0x4,
    nvme_tcp_rsp = 0x5,
    nvme_tcp_h2c_data = 0x6,
    nvme_tcp_c2h_data = 0x7,
    nvme_tcp_r2t = 0x9,
}

pub type nvme_tcp_pdu_flags = u8;
pub const NVME_TCP_F_HDGST: nvme_tcp_pdu_flags = 1 << 0;
pub const NVME_TCP_F_DDGST: nvme_tcp_pdu_flags = 1 << 1;
pub const NVME_TCP_F_DATA_LAST: nvme_tcp_pdu_flags = 1 << 2;
pub const NVME_TCP_F_DATA_SUCCESS: nvme_tcp_pdu_flags = 1 << 3;

#[repr(C)]
pub struct nvme_tcp_hdr {
    pub r#type: __u8,
    pub flags: __u8,
    pub hlen: __u8,
    pub pdo: __u8,
    pub plen: __le32,
}

#[repr(C)]
pub struct nvme_tcp_icreq_pdu {
    pub hdr: nvme_tcp_hdr,
    pub pfv: __le16,
    pub hpda: __u8,
    pub digest: __u8,
    pub maxr2t: __le32,
    pub rsvd2: [__u8; 112],
}

#[repr(C)]
pub struct nvme_tcp_icresp_pdu {
    pub hdr: nvme_tcp_hdr,
    pub pfv: __le16,
    pub cpda: __u8,
    pub digest: __u8,
    pub maxdata: __le32,
    pub rsvd: [__u8; 112],
}

#[repr(C)]
pub struct nvme_tcp_term_pdu {
    pub hdr: nvme_tcp_hdr,
    pub fes: __le16,
    pub feil: __le16,
    pub feiu: __le16,
    pub rsvd: [__u8; 10],
}

#[repr(C)]
pub struct nvme_tcp_cmd_pdu {
    pub hdr: nvme_tcp_hdr,
    pub cmd: nvme_command,
}

#[repr(C)]
pub struct nvme_tcp_rsp_pdu {
    pub hdr: nvme_tcp_hdr,
    pub cqe: nvme_completion,
}

#[repr(C)]
pub struct nvme_tcp_r2t_pdu {
    pub hdr: nvme_tcp_hdr,
    pub command_id: __u16,
    pub ttag: __u16,
    pub r2t_offset: __le32,
    pub r2t_length: __le32,
    pub rsvd: [__u8; 4],
}

#[repr(C)]
pub struct nvme_tcp_data_pdu {
    pub hdr: nvme_tcp_hdr,
    pub command_id: __u16,
    pub ttag: __u16,
    pub data_offset: __le32,
    pub data_length: __le32,
    pub rsvd: [__u8; 4],
}

#[repr(C)]
pub union nvme_tcp_pdu {
    pub icreq: nvme_tcp_icreq_pdu,
    pub icresp: nvme_tcp_icresp_pdu,
    pub cmd: nvme_tcp_cmd_pdu,
    pub rsp: nvme_tcp_rsp_pdu,
    pub r2t: nvme_tcp_r2t_pdu,
    pub data: nvme_tcp_data_pdu,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
