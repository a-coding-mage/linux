/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  iSCSI Transport BSG Interface
 *
 *  Copyright (C) 2009   James Smart, Emulex Corporation
 */

/* This file is intended to be included by both kernel and user space. */
/* Dependency: HZ is supplied by the surrounding kernel environment. */

/* iSCSI Transport SGIO v4 BSG Message Support */

/* Default BSG request timeout (in seconds) */
pub const ISCSI_DEFAULT_BSG_TIMEOUT: u32 = 10 * HZ;

/* Request Message Codes supported by the iSCSI Transport */

/* define the class masks for the message codes */
pub const ISCSI_BSG_CLS_MASK: u32 = 0xF0000000; /* find object class */
pub const ISCSI_BSG_HST_MASK: u32 = 0x80000000; /* iscsi host class */

/* iscsi host Message Codes */
pub const ISCSI_BSG_HST_VENDOR: u32 = ISCSI_BSG_HST_MASK | 0x000000FF;

/* iSCSI Host Messages */

/* ISCSI_BSG_HST_VENDOR : */

/* Request:
 * Note: When specifying vendor_id, be sure to read the Vendor Type and ID
 *   formatting requirements specified in scsi_netlink.h
 */
#[repr(C)]
pub struct iscsi_bsg_host_vendor {
    /*
     * Identifies the vendor that the message is formatted for. This
     * should be the recipient of the message.
     */
    pub vendor_id: u64,

    /* start of vendor command area */
    pub vendor_cmd: [u32; 0],
}

/* Response:
 */
#[repr(C)]
pub struct iscsi_bsg_host_vendor_reply {
    /* start of vendor response area */
    pub vendor_rsp: [u32; 0],
}

/* request (CDB) structure of the sg_io_v4 */
#[repr(C, packed)]
pub struct iscsi_bsg_request {
    pub msgcode: u32,
    pub rqst_data: iscsi_bsg_request_rqst_data,
}

#[repr(C)]
pub union iscsi_bsg_request_rqst_data {
    pub h_vendor: iscsi_bsg_host_vendor,
}

/* response (request sense data) structure of the sg_io_v4 */
#[repr(C)]
pub struct iscsi_bsg_reply {
    /*
     * The completion result. Result exists in two forms:
     * if negative, it is an -Exxx system errno value. There will
     * be no further reply information supplied.
     * else, it's the 4-byte scsi error result, with driver, host,
     * msg and status fields. The per-msgcode reply structure
     * will contain valid data.
     */
    pub result: u32,

    /* If there was reply_payload, how much was received ? */
    pub reply_payload_rcv_len: u32,

    pub reply_data: iscsi_bsg_reply_reply_data,
}

#[repr(C)]
pub union iscsi_bsg_reply_reply_data {
    pub vendor_reply: iscsi_bsg_host_vendor_reply,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
