// SPDX-License-Identifier: GPL-2.0+
/*
 * Host Side support for RNDIS Networking Links
 * Copyright (C) 2005 by David Brownell
 */

// Dependency: linux/rndis.h

/*
 * CONTROL uses CDC "encapsulated commands" with funky notifications.
 *  - control-out:  SEND_ENCAPSULATED
 *  - interrupt-in:  RESPONSE_AVAILABLE
 *  - control-in:  GET_ENCAPSULATED
 *
 * We'll try to ignore the RESPONSE_AVAILABLE notifications.
 *
 * REVISIT some RNDIS implementations seem to have curious issues still
 * to be resolved.
 */
#[repr(C, packed)]
pub struct rndis_msg_hdr {
    pub msg_type: u32, // RNDIS_MSG_*
    pub msg_len: u32,
    // followed by data that varies between messages
    pub request_id: u32,
    pub status: u32,
    // ... and more
}

/* MS-Windows uses this strange size, but RNDIS spec says 1024 minimum */
pub const CONTROL_BUFFER_SIZE: usize = 1025;

/* RNDIS defines an (absurdly huge) 10 second control timeout,
 * but ActiveSync seems to use a more usual 5 second timeout
 * (which matches the USB 2.0 spec).
 */
pub const RNDIS_CONTROL_TIMEOUT_MS: i32 = 5 * 1000;

#[repr(C, packed)]
pub struct rndis_data_hdr {
    pub msg_type: u32, // RNDIS_MSG_PACKET
    pub msg_len: u32, // rndis_data_hdr + data_len + pad
    pub data_offset: u32, // 36 -- right after header
    pub data_len: u32, // ... real packet size
    pub oob_data_offset: u32, // zero
    pub oob_data_len: u32, // zero
    pub num_oob: u32, // zero
    pub packet_data_offset: u32, // zero
    pub packet_data_len: u32, // zero
    pub vc_handle: u32, // zero
    pub reserved: u32, // zero
}

#[repr(C, packed)]
pub struct rndis_init {
    pub msg_type: u32, // RNDIS_MSG_INIT
    pub msg_len: u32, // 24
    pub request_id: u32,
    pub major_version: u32, // of rndis (1.0)
    pub minor_version: u32,
    pub max_transfer_size: u32,
}

#[repr(C, packed)]
pub struct rndis_init_c {
    pub msg_type: u32, // RNDIS_MSG_INIT_C
    pub msg_len: u32,
    pub request_id: u32,
    pub status: u32,
    pub major_version: u32, // of rndis (1.0)
    pub minor_version: u32,
    pub device_flags: u32,
    pub medium: u32, // zero == 802.3
    pub max_packets_per_message: u32,
    pub max_transfer_size: u32,
    pub packet_alignment: u32, // max 7; (1<<n) bytes
    pub af_list_offset: u32, // zero
    pub af_list_size: u32, // zero
}

#[repr(C, packed)]
pub struct rndis_halt { pub msg_type: u32, pub msg_len: u32, pub request_id: u32 }

#[repr(C, packed)]
pub struct rndis_query {
    pub msg_type: u32, pub msg_len: u32, pub request_id: u32,
    pub oid: u32, pub len: u32, pub offset: u32, // zero: handle
    pub handle: u32,
}

#[repr(C, packed)]
pub struct rndis_query_c {
    pub msg_type: u32, pub msg_len: u32, pub request_id: u32,
    pub status: u32, pub len: u32, pub offset: u32,
}

#[repr(C, packed)]
pub struct rndis_set {
    pub msg_type: u32, pub msg_len: u32, pub request_id: u32,
    pub oid: u32, pub len: u32, pub offset: u32, pub handle: u32,
}

#[repr(C, packed)]
pub struct rndis_set_c { pub msg_type: u32, pub msg_len: u32, pub request_id: u32, pub status: u32 }

#[repr(C, packed)]
pub struct rndis_reset { pub msg_type: u32, pub msg_len: u32, pub reserved: u32 }

#[repr(C, packed)]
pub struct rndis_reset_c { pub msg_type: u32, pub msg_len: u32, pub status: u32, pub addressing_lost: u32 }

#[repr(C, packed)]
pub struct rndis_indicate {
    pub msg_type: u32, pub msg_len: u32, pub status: u32, pub length: u32,
    pub offset: u32, pub diag_status: u32, pub error_offset: u32, pub message: u32,
}

#[repr(C, packed)]
pub struct rndis_keepalive { pub msg_type: u32, pub msg_len: u32, pub request_id: u32 }

#[repr(C, packed)]
pub struct rndis_keepalive_c { pub msg_type: u32, pub msg_len: u32, pub request_id: u32, pub status: u32 }

/* default filter used with RNDIS devices */
pub const RNDIS_DEFAULT_FILTER: u32 = RNDIS_PACKET_TYPE_DIRECTED
    | RNDIS_PACKET_TYPE_BROADCAST
    | RNDIS_PACKET_TYPE_ALL_MULTICAST
    | RNDIS_PACKET_TYPE_PROMISCUOUS;

/* Flags to require specific physical medium type for generic_rndis_bind() */
pub const FLAG_RNDIS_PHYM_NOT_WIRELESS: i32 = 0x0001;
pub const FLAG_RNDIS_PHYM_WIRELESS: i32 = 0x0002;

/* Flags for driver_info::data */
pub const RNDIS_DRIVER_DATA_POLL_STATUS: i32 = 1; // poll status before control
pub const RNDIS_DRIVER_DATA_DST_MAC_FIXUP: i32 = 2; // device ignores configured MAC address

extern "C" {
    pub fn rndis_status(dev: *mut usbnet, urb: *mut urb);
    pub fn rndis_command(dev: *mut usbnet, buf: *mut rndis_msg_hdr, buflen: i32) -> i32;
    pub fn generic_rndis_bind(dev: *mut usbnet, intf: *mut usb_interface, flags: i32) -> i32;
    pub fn rndis_unbind(dev: *mut usbnet, intf: *mut usb_interface);
    pub fn rndis_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> i32;
    pub fn rndis_tx_fixup(dev: *mut usbnet, skb: *mut sk_buff, flags: gfp_t) -> *mut sk_buff;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
