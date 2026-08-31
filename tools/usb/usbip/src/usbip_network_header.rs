/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2005-2007 Takahiro Hirofuchi
 */

/* C header guard and include directives omitted.  This translation expects
 * external definitions corresponding to sys/types.h, stdint.h, config.h,
 * SYSFS_BUS_ID_SIZE, usbip_usb_device, and usbip_usb_interface.
 */

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    pub static mut usbip_port: c_int;
    pub static mut usbip_port_string: *mut c_char;
    pub fn usbip_setup_port_number(arg: *mut c_char);
}

/* ---------------------------------------------------------------------- */
/* Common header for all the kinds of PDUs. */
#[repr(C, packed)]
pub struct op_common {
    pub version: u16,

    pub code: u16,

    /* status codes defined in usbip_common.h */
    pub status: u32, /* op_code status (for reply) */
}

pub const OP_REQUEST: u16 = 0x80u16 << 8;
pub const OP_REPLY: u16 = 0x00u16 << 8;

/* ---------------------------------------------------------------------- */
/* Dummy Code */
pub const OP_UNSPEC: u16 = 0x00;
pub const OP_REQ_UNSPEC: u16 = OP_UNSPEC;
pub const OP_REP_UNSPEC: u16 = OP_UNSPEC;

/* ---------------------------------------------------------------------- */
/* Retrieve USB device information. (still not used) */
pub const OP_DEVINFO: u16 = 0x02;
pub const OP_REQ_DEVINFO: u16 = OP_REQUEST | OP_DEVINFO;
pub const OP_REP_DEVINFO: u16 = OP_REPLY | OP_DEVINFO;

#[repr(C, packed)]
pub struct op_devinfo_request {
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],
}

#[repr(C, packed)]
pub struct op_devinfo_reply {
    pub udev: usbip_usb_device,
    pub uinf: [usbip_usb_interface; 0],
}

/* ---------------------------------------------------------------------- */
/* Import a remote USB device. */
pub const OP_IMPORT: u16 = 0x03;
pub const OP_REQ_IMPORT: u16 = OP_REQUEST | OP_IMPORT;
pub const OP_REP_IMPORT: u16 = OP_REPLY | OP_IMPORT;

#[repr(C, packed)]
pub struct op_import_request {
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],
}

#[repr(C, packed)]
pub struct op_import_reply {
    pub udev: usbip_usb_device,
    // pub uinf: [usbip_usb_interface; 0],
}

#[inline]
pub unsafe fn PACK_OP_IMPORT_REQUEST(_pack: c_int, _request: *mut op_import_request) {}

#[inline]
pub unsafe fn PACK_OP_IMPORT_REPLY(pack: c_int, reply: *mut op_import_reply) {
    unsafe {
        usbip_net_pack_usb_device(pack, core::ptr::addr_of_mut!((*reply).udev));
    }
}

/* ---------------------------------------------------------------------- */
/* Export a USB device to a remote host. */
pub const OP_EXPORT: u16 = 0x06;
pub const OP_REQ_EXPORT: u16 = OP_REQUEST | OP_EXPORT;
pub const OP_REP_EXPORT: u16 = OP_REPLY | OP_EXPORT;

#[repr(C, packed)]
pub struct op_export_request {
    pub udev: usbip_usb_device,
}

#[repr(C, packed)]
pub struct op_export_reply {
    pub returncode: c_int,
}

#[inline]
pub unsafe fn PACK_OP_EXPORT_REQUEST(pack: c_int, request: *mut op_export_request) {
    unsafe {
        usbip_net_pack_usb_device(pack, core::ptr::addr_of_mut!((*request).udev));
    }
}

#[inline]
pub unsafe fn PACK_OP_EXPORT_REPLY(_pack: c_int, _reply: *mut op_export_reply) {}

/* ---------------------------------------------------------------------- */
/* un-Export a USB device from a remote host. */
pub const OP_UNEXPORT: u16 = 0x07;
pub const OP_REQ_UNEXPORT: u16 = OP_REQUEST | OP_UNEXPORT;
pub const OP_REP_UNEXPORT: u16 = OP_REPLY | OP_UNEXPORT;

#[repr(C, packed)]
pub struct op_unexport_request {
    pub udev: usbip_usb_device,
}

#[repr(C, packed)]
pub struct op_unexport_reply {
    pub returncode: c_int,
}

#[inline]
pub unsafe fn PACK_OP_UNEXPORT_REQUEST(pack: c_int, request: *mut op_unexport_request) {
    unsafe {
        usbip_net_pack_usb_device(pack, core::ptr::addr_of_mut!((*request).udev));
    }
}

#[inline]
pub unsafe fn PACK_OP_UNEXPORT_REPLY(_pack: c_int, _reply: *mut op_unexport_reply) {}

/* ---------------------------------------------------------------------- */
/* Negotiate IPSec encryption key. (still not used) */
pub const OP_CRYPKEY: u16 = 0x04;
pub const OP_REQ_CRYPKEY: u16 = OP_REQUEST | OP_CRYPKEY;
pub const OP_REP_CRYPKEY: u16 = OP_REPLY | OP_CRYPKEY;

#[repr(C, packed)]
pub struct op_crypkey_request {
    /* 128bit key */
    pub key: [u32; 4],
}

#[repr(C, packed)]
pub struct op_crypkey_reply {
    pub __reserved: u32,
}

/* ---------------------------------------------------------------------- */
/* Retrieve the list of exported USB devices. */
pub const OP_DEVLIST: u16 = 0x05;
pub const OP_REQ_DEVLIST: u16 = OP_REQUEST | OP_DEVLIST;
pub const OP_REP_DEVLIST: u16 = OP_REPLY | OP_DEVLIST;

#[repr(C, packed)]
pub struct op_devlist_request {}

#[repr(C, packed)]
pub struct op_devlist_reply {
    pub ndev: u32,
    /* followed by reply_extra[] */
}

#[repr(C, packed)]
pub struct op_devlist_reply_extra {
    pub udev: usbip_usb_device,
    pub uinf: [usbip_usb_interface; 0],
}

#[inline]
pub unsafe fn PACK_OP_DEVLIST_REQUEST(_pack: c_int, _request: *mut op_devlist_request) {}

#[inline]
pub unsafe fn PACK_OP_DEVLIST_REPLY(pack: c_int, reply: *mut op_devlist_reply) {
    unsafe {
        (*reply).ndev = usbip_net_pack_uint32_t(pack, (*reply).ndev);
    }
}

unsafe extern "C" {
    pub fn usbip_net_pack_uint32_t(pack: c_int, num: u32) -> u32;
    pub fn usbip_net_pack_uint16_t(pack: c_int, num: u16) -> u16;
    pub fn usbip_net_pack_usb_device(pack: c_int, udev: *mut usbip_usb_device);
    pub fn usbip_net_pack_usb_interface(pack: c_int, uinf: *mut usbip_usb_interface);

    pub fn usbip_net_recv(sockfd: c_int, buff: *mut c_void, bufflen: size_t) -> ssize_t;
    pub fn usbip_net_send(sockfd: c_int, buff: *mut c_void, bufflen: size_t) -> ssize_t;
    pub fn usbip_net_send_op_common(sockfd: c_int, code: u32, status: u32) -> c_int;
    pub fn usbip_net_recv_op_common(sockfd: c_int, code: *mut u16, status: *mut c_int) -> c_int;
    pub fn usbip_net_set_reuseaddr(sockfd: c_int) -> c_int;
    pub fn usbip_net_set_nodelay(sockfd: c_int) -> c_int;
    pub fn usbip_net_set_keepalive(sockfd: c_int) -> c_int;
    pub fn usbip_net_set_v6only(sockfd: c_int) -> c_int;
    pub fn usbip_net_tcp_connect(hostname: *mut c_char, port: *mut c_char) -> c_int;
}
