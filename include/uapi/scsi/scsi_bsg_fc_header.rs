/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  FC Transport BSG Interface
 *
 *  Copyright (C) 2008   James Smart, Emulex Corporation
 */

/* This file is intended to be included by both kernel and user space. */
/* FC Transport SGIO v4 BSG Message Support */

/* Default BSG request timeout (in seconds); HZ is supplied externally. */
pub const FC_DEFAULT_BSG_TIMEOUT: u32 = 10 * HZ;

/* Request Message Codes supported by the FC Transport. */
pub const FC_BSG_CLS_MASK: u32 = 0xF0000000;
pub const FC_BSG_HST_MASK: u32 = 0x80000000;
pub const FC_BSG_RPT_MASK: u32 = 0x40000000;

pub const FC_BSG_HST_ADD_RPORT: u32 = FC_BSG_HST_MASK | 0x00000001;
pub const FC_BSG_HST_DEL_RPORT: u32 = FC_BSG_HST_MASK | 0x00000002;
pub const FC_BSG_HST_ELS_NOLOGIN: u32 = FC_BSG_HST_MASK | 0x00000003;
pub const FC_BSG_HST_CT: u32 = FC_BSG_HST_MASK | 0x00000004;
pub const FC_BSG_HST_VENDOR: u32 = FC_BSG_HST_MASK | 0x000000FF;

pub const FC_BSG_RPT_ELS: u32 = FC_BSG_RPT_MASK | 0x00000001;
pub const FC_BSG_RPT_CT: u32 = FC_BSG_RPT_MASK | 0x00000002;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_add_rport {
    pub reserved: __u8,
    pub port_id: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_del_rport {
    pub reserved: __u8,
    pub port_id: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_els {
    pub command_code: __u8,
    pub port_id: [__u8; 3],
}

pub const FC_CTELS_STATUS_OK: u32 = 0x00000000;
pub const FC_CTELS_STATUS_REJECT: u32 = 0x00000001;
pub const FC_CTELS_STATUS_P_RJT: u32 = 0x00000002;
pub const FC_CTELS_STATUS_F_RJT: u32 = 0x00000003;
pub const FC_CTELS_STATUS_P_BSY: u32 = 0x00000004;
pub const FC_CTELS_STATUS_F_BSY: u32 = 0x00000006;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_ctels_reply_rjt_data {
    pub action: __u8,
    pub reason_code: __u8,
    pub reason_explanation: __u8,
    pub vendor_unique: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_ctels_reply {
    pub status: __u32,
    pub rjt_data: fc_bsg_ctels_reply_rjt_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_ct {
    pub reserved: __u8,
    pub port_id: [__u8; 3],
    pub preamble_word0: __u32,
    pub preamble_word1: __u32,
    pub preamble_word2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_vendor {
    pub vendor_id: __u64,
    pub vendor_cmd: [__u32; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_vendor_reply {
    pub vendor_rsp: [__u32; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_rport_els {
    pub els_code: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_rport_ct {
    pub preamble_word0: __u32,
    pub preamble_word1: __u32,
    pub preamble_word2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fc_bsg_request_rqst_data {
    pub h_addrport: fc_bsg_host_add_rport,
    pub h_delrport: fc_bsg_host_del_rport,
    pub h_els: fc_bsg_host_els,
    pub h_ct: fc_bsg_host_ct,
    pub h_vendor: fc_bsg_host_vendor,
    pub r_els: fc_bsg_rport_els,
    pub r_ct: fc_bsg_rport_ct,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fc_bsg_request {
    pub msgcode: __u32,
    pub rqst_data: fc_bsg_request_rqst_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fc_bsg_reply_reply_data {
    pub vendor_reply: fc_bsg_host_vendor_reply,
    pub ctels_reply: fc_bsg_ctels_reply,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_reply {
    pub result: __u32,
    pub reply_payload_rcv_len: __u32,
    pub reply_data: fc_bsg_reply_reply_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
