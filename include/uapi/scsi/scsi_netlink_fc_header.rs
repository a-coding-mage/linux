/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  FC Transport Netlink Interface
 *
 *  Copyright (C) 2006   James Smart, Emulex Corporation
 */

/*
 * This file intended to be included by both kernel and user space
 */

/*
 * FC Transport Message Types
 */
/* kernel -> user */
pub const FC_NL_ASYNC_EVENT: u32 = 0x0100;
/* user -> kernel */
/* none */

/*
 * Message Structures :
 */

/* macro to round up message lengths to 8byte boundary */
#[inline]
pub const fn FC_NL_MSGALIGN(len: usize) -> usize {
    (len + 7) & !7
}

/*
 * FC Transport Broadcast Event Message :
 *   FC_NL_ASYNC_EVENT
 *
 * Note: if Vendor Unique message, event_data_flex will be start of
 *       vendor unique payload, and the length of the payload is
 *       per event_datalen
 *
 * Note: When specifying vendor_id, be sure to read the Vendor Type and ID
 *   formatting requirements specified in scsi_netlink.h
 */
#[repr(C)]
pub union fc_nl_event_data {
    pub event_data: u32,
    pub event_data_flex: [u8; 0],
}

#[repr(C, align(8))]
pub struct fc_nl_event {
    pub snlh: scsi_nl_hdr, /* must be 1st element ! */
    pub seconds: u64,
    pub vendor_id: u64,
    pub host_no: u16,
    pub event_datalen: u16,
    pub event_num: u32,
    pub event_code: u32,
    pub event_data: fc_nl_event_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
