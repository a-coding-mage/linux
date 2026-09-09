/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  SCSI Transport Netlink Interface
 *    Used for the posting of outbound SCSI transport events
 *
 *  Copyright (C) 2006   James Smart, Emulex Corporation
 */

// Dependency supplied by the Linux netlink definitions.
pub const SCSI_TRANSPORT_MSG: u16 = NLMSG_MIN_TYPE + 1;

/* SCSI Transport Broadcast Groups */
// leaving groups 0 and 1 unassigned
pub const SCSI_NL_GRP_FC_EVENTS: u32 = 1 << 2; // Group 2
pub const SCSI_NL_GRP_CNT: u32 = 3;

/* SCSI_TRANSPORT_MSG event message header */
#[repr(C, align(8))]
pub struct scsi_nl_hdr {
    pub version: u8,
    pub transport: u8,
    pub magic: u16,
    pub msgtype: u16,
    pub msglen: u16,
}

/* scsi_nl_hdr->version value */
pub const SCSI_NL_VERSION: u16 = 1;

/* scsi_nl_hdr->magic value */
pub const SCSI_NL_MAGIC: u16 = 0xA1B2;

/* scsi_nl_hdr->transport value */
pub const SCSI_NL_TRANSPORT: u8 = 0;
pub const SCSI_NL_TRANSPORT_FC: u8 = 1;
pub const SCSI_NL_MAX_TRANSPORTS: u8 = 2;

/* Transport-based scsi_nl_hdr->msgtype values are defined in each transport */

/*
 * GENERIC SCSI scsi_nl_hdr->msgtype Values
 */
// kernel -> user
pub const SCSI_NL_SHOST_VENDOR: u16 = 0x0001;
// user -> kernel
// SCSI_NL_SHOST_VENDOR msgtype is kernel->user and user->kernel

/*
 * Message Structures :
 */

/* macro to round up message lengths to 8byte boundary */
#[inline]
pub const fn SCSI_NL_MSGALIGN(len: usize) -> usize {
    (len + 7) & !7
}

/*
 * SCSI HOST Vendor Unique messages :
 *   SCSI_NL_SHOST_VENDOR
 *
 * Note: The Vendor Unique message payload will begin directly after
 *       this structure, with the length of the payload per vmsg_datalen.
 *
 * Note: When specifying vendor_id, be sure to read the Vendor Type and ID
 *   formatting requirements specified below
 */
#[repr(C, align(8))]
pub struct scsi_nl_host_vendor_msg {
    pub snlh: scsi_nl_hdr, // must be 1st element !
    pub vendor_id: u64,
    pub host_no: u16,
    pub vmsg_datalen: u16,
}

/*
 * Vendor ID:
 *   If transports post vendor-unique events, they must pass a well-known
 *   32-bit vendor identifier. This identifier consists of 8 bits indicating
 *   the "type" of identifier contained, and 24 bits of id data.
 *
 *   Identifiers for each type:
 *    PCI :  ID data is the 16 bit PCI Registered Vendor ID
 */
pub const SCSI_NL_VID_TYPE_SHIFT: u32 = 56;
pub const SCSI_NL_VID_TYPE_MASK: u64 = 0xFFu64 << SCSI_NL_VID_TYPE_SHIFT;
pub const SCSI_NL_VID_TYPE_PCI: u64 = 0x01u64 << SCSI_NL_VID_TYPE_SHIFT;
pub const SCSI_NL_VID_ID_MASK: u64 = !SCSI_NL_VID_TYPE_MASK;

#[macro_export]
macro_rules! INIT_SCSI_NL_HDR {
    ($hdr:expr, $t:expr, $mtype:expr, $mlen:expr) => {{
        $hdr.version = $crate::SCSI_NL_VERSION as u8;
        $hdr.transport = $t;
        $hdr.magic = $crate::SCSI_NL_MAGIC;
        $hdr.msgtype = $mtype;
        $hdr.msglen = $mlen;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
