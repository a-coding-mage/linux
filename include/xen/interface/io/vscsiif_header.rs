/* SPDX-License-Identifier: MIT */
/******************************************************************************
 * vscsiif.h
 *
 * Based on the blkif.h code.
 *
 * Copyright(c) FUJITSU Limited 2008.
 ******************************************************************************/

// Dependency intent from the C header: "ring.h" and "../grant_table.h".

/*
 * Feature and Parameter Negotiation
 * =================================
 * The two halves of a Xen pvSCSI driver utilize nodes within the XenStore to
 * communicate capabilities and to negotiate operating parameters.  This
 * section enumerates these nodes which reside in the respective front and
 * backend portions of the XenStore, following the XenBus convention.
 *
 * XenStore format and backend/frontend protocol are documented in the
 * original header; no executable Rust declarations are required for them.
 */

/* Requests from the frontend to the backend */

/* Request actions. */
pub const VSCSIIF_ACT_SCSI_CDB: u8 = 1;
pub const VSCSIIF_ACT_SCSI_ABORT: u8 = 2;
pub const VSCSIIF_ACT_SCSI_RESET: u8 = 3;
/* Deprecated; retained to avoid usage of the value 4 for other actions. */
pub const VSCSIIF_ACT_SCSI_SG_PRESET: u8 = 4;

/* Maximum scatter/gather segments per request. */
pub const VSCSIIF_SG_TABLESIZE: usize = 26;

/* Based on Linux kernel 2.6.18, still valid. */
pub const VSCSIIF_MAX_COMMAND_SIZE: usize = 16;
pub const VSCSIIF_SENSE_BUFFERSIZE: usize = 96;
pub const VSCSIIF_PAGE_SIZE: usize = 4096;

#[repr(C)]
pub struct scsiif_request_segment {
    pub gref: grant_ref_t,
    pub offset: u16,
    pub length: u16,
}

pub const VSCSIIF_SG_PER_PAGE: usize =
    VSCSIIF_PAGE_SIZE / core::mem::size_of::<scsiif_request_segment>();

#[repr(C)]
pub struct vscsiif_request {
    pub rqid: u16, // private guest value, echoed in resp
    pub act: u8, // command between backend and frontend
    pub cmd_len: u8, // valid CDB bytes
    pub cmnd: [u8; VSCSIIF_MAX_COMMAND_SIZE], // the CDB
    pub timeout_per_command: u16, // deprecated
    pub channel: u16,
    pub id: u16,
    pub lun: u16, // (virtual) device specification
    pub ref_rqid: u16, // command abort reference
    pub sc_data_direction: u8, // DMA_TO_DEVICE(1), DMA_FROM_DEVICE(2), DMA_NONE(3)
    pub nr_segments: u8, // Number of pieces of scatter-gather
    pub seg: [scsiif_request_segment; VSCSIIF_SG_TABLESIZE],
    pub reserved: [u32; 3],
}

/* Flag in nr_segments: SG elements via grant page. */
pub const VSCSIIF_SG_GRANT: u8 = 0x80;

#[repr(C)]
pub struct vscsiif_response {
    pub rqid: u16, // identifies request
    pub padding: u8,
    pub sense_len: u8,
    pub sense_buffer: [u8; VSCSIIF_SENSE_BUFFERSIZE],
    pub rslt: i32,
    pub residual_len: u32, // request bufflen - return the value from physical device
    pub reserved: [u32; 36],
}

/* SCSI I/O status from vscsiif_response->rslt */
#[inline]
pub const fn XEN_VSCSIIF_RSLT_STATUS(x: i32) -> i32 { x & 0x00ff }

/* Host I/O status from vscsiif_response->rslt */
#[inline]
pub const fn XEN_VSCSIIF_RSLT_HOST(x: i32) -> i32 { (x & 0x00ff0000) >> 16 }
pub const XEN_VSCSIIF_RSLT_HOST_OK: i32 = 0;
pub const XEN_VSCSIIF_RSLT_HOST_NO_CONNECT: i32 = 1;
pub const XEN_VSCSIIF_RSLT_HOST_BUS_BUSY: i32 = 2;
pub const XEN_VSCSIIF_RSLT_HOST_TIME_OUT: i32 = 3;
pub const XEN_VSCSIIF_RSLT_HOST_BAD_TARGET: i32 = 4;
pub const XEN_VSCSIIF_RSLT_HOST_ABORT: i32 = 5;
pub const XEN_VSCSIIF_RSLT_HOST_PARITY: i32 = 6;
pub const XEN_VSCSIIF_RSLT_HOST_ERROR: i32 = 7;
pub const XEN_VSCSIIF_RSLT_HOST_RESET: i32 = 8;
pub const XEN_VSCSIIF_RSLT_HOST_BAD_INTR: i32 = 9;
pub const XEN_VSCSIIF_RSLT_HOST_PASSTHROUGH: i32 = 10;
pub const XEN_VSCSIIF_RSLT_HOST_SOFT_ERROR: i32 = 11;
pub const XEN_VSCSIIF_RSLT_HOST_IMM_RETRY: i32 = 12;
pub const XEN_VSCSIIF_RSLT_HOST_REQUEUE: i32 = 13;
pub const XEN_VSCSIIF_RSLT_HOST_TRANSPORT_DISRUPTED: i32 = 14;
pub const XEN_VSCSIIF_RSLT_HOST_TRANSPORT_FAILFAST: i32 = 15;
pub const XEN_VSCSIIF_RSLT_HOST_TARGET_FAILURE: i32 = 16;
pub const XEN_VSCSIIF_RSLT_HOST_NEXUS_FAILURE: i32 = 17;
pub const XEN_VSCSIIF_RSLT_HOST_ALLOC_FAILURE: i32 = 18;
pub const XEN_VSCSIIF_RSLT_HOST_MEDIUM_ERROR: i32 = 19;
pub const XEN_VSCSIIF_RSLT_HOST_TRANSPORT_MARGINAL: i32 = 20;

/* Result values of reset operations */
pub const XEN_VSCSIIF_RSLT_RESET_SUCCESS: i32 = 0x2002;
pub const XEN_VSCSIIF_RSLT_RESET_FAILED: i32 = 0x2003;

// DEFINE_RING_TYPES(vscsiif, struct vscsiif_request, struct vscsiif_response)
// is supplied by the external Xen ring implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
