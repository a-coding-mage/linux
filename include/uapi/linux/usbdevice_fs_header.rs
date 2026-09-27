/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*****************************************************************************/
/*
 *	usbdevice_fs.h  --  USB device file system.
 *
 *	Copyright (C) 2000
 *          Thomas Sailer (sailer@ife.ee.ethz.ch)
 *
 *	This program is free software; you can redistribute it and/or modify
 *	it under the terms of the GNU General Public License as published by
 *	the Free Software Foundation; either version 2 of the License, or
 *	(at your option) any later version.
 */

/* C dependencies: linux/types.h and linux/magic.h. */

#[repr(C)]
pub struct usbdevfs_ctrltransfer {
    pub bRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __u16,
    pub wIndex: __u16,
    pub wLength: __u16,
    pub timeout: __u32,
    pub data: *mut kernel::ffi::c_void,
}

#[repr(C)]
pub struct usbdevfs_bulktransfer {
    pub ep: kernel::ffi::c_uint,
    pub len: kernel::ffi::c_uint,
    pub timeout: kernel::ffi::c_uint,
    pub data: *mut kernel::ffi::c_void,
}

#[repr(C)]
pub struct usbdevfs_setinterface {
    pub interface: kernel::ffi::c_uint,
    pub altsetting: kernel::ffi::c_uint,
}

#[repr(C)]
pub struct usbdevfs_disconnectsignal {
    pub signr: kernel::ffi::c_uint,
    pub context: *mut kernel::ffi::c_void,
}

pub const USBDEVFS_MAXDRIVERNAME: usize = 255;

#[repr(C)]
pub struct usbdevfs_getdriver {
    pub interface: kernel::ffi::c_uint,
    pub driver: [kernel::ffi::c_char; USBDEVFS_MAXDRIVERNAME + 1],
}

#[repr(C)]
pub struct usbdevfs_connectinfo {
    pub devnum: kernel::ffi::c_uint,
    pub slow: kernel::ffi::c_uchar,
}

#[repr(C)]
pub struct usbdevfs_conninfo_ex {
    pub size: __u32,
    pub busnum: __u32,
    pub devnum: __u32,
    pub speed: __u32,
    pub num_ports: __u8,
    pub ports: [__u8; 7],
}

pub const USBDEVFS_URB_SHORT_NOT_OK: u32 = 0x01;
pub const USBDEVFS_URB_ISO_ASAP: u32 = 0x02;
pub const USBDEVFS_URB_BULK_CONTINUATION: u32 = 0x04;
pub const USBDEVFS_URB_NO_FSBR: u32 = 0x20;
pub const USBDEVFS_URB_ZERO_PACKET: u32 = 0x40;
pub const USBDEVFS_URB_NO_INTERRUPT: u32 = 0x80;

pub const USBDEVFS_URB_TYPE_ISO: u32 = 0;
pub const USBDEVFS_URB_TYPE_INTERRUPT: u32 = 1;
pub const USBDEVFS_URB_TYPE_CONTROL: u32 = 2;
pub const USBDEVFS_URB_TYPE_BULK: u32 = 3;

#[repr(C)]
pub struct usbdevfs_iso_packet_desc {
    pub length: kernel::ffi::c_uint,
    pub actual_length: kernel::ffi::c_uint,
    pub status: kernel::ffi::c_uint,
}

#[repr(C)]
pub union usbdevfs_urb__bindgen_ty_1 {
    pub number_of_packets: kernel::ffi::c_int,
    pub stream_id: kernel::ffi::c_uint,
}

#[repr(C)]
pub struct usbdevfs_urb {
    pub type_: kernel::ffi::c_uchar,
    pub endpoint: kernel::ffi::c_uchar,
    pub status: kernel::ffi::c_int,
    pub flags: kernel::ffi::c_uint,
    pub buffer: *mut kernel::ffi::c_void,
    pub buffer_length: kernel::ffi::c_int,
    pub actual_length: kernel::ffi::c_int,
    pub start_frame: kernel::ffi::c_int,
    pub __bindgen_anon_1: usbdevfs_urb__bindgen_ty_1,
    pub error_count: kernel::ffi::c_int,
    pub signr: kernel::ffi::c_uint,
    pub usercontext: *mut kernel::ffi::c_void,
    pub iso_frame_desc: [usbdevfs_iso_packet_desc; 0],
}

#[repr(C)]
pub struct usbdevfs_ioctl {
    pub ifno: kernel::ffi::c_int,
    pub ioctl_code: kernel::ffi::c_int,
    pub data: *mut kernel::ffi::c_void,
}

#[repr(C)]
pub struct usbdevfs_hub_portinfo {
    pub nports: kernel::ffi::c_char,
    pub port: [kernel::ffi::c_char; 127],
}

pub const USBDEVFS_CAP_ZERO_PACKET: u32 = 0x01;
pub const USBDEVFS_CAP_BULK_CONTINUATION: u32 = 0x02;
pub const USBDEVFS_CAP_NO_PACKET_SIZE_LIM: u32 = 0x04;
pub const USBDEVFS_CAP_BULK_SCATTER_GATHER: u32 = 0x08;
pub const USBDEVFS_CAP_REAP_AFTER_DISCONNECT: u32 = 0x10;
pub const USBDEVFS_CAP_MMAP: u32 = 0x20;
pub const USBDEVFS_CAP_DROP_PRIVILEGES: u32 = 0x40;
pub const USBDEVFS_CAP_CONNINFO_EX: u32 = 0x80;
pub const USBDEVFS_CAP_SUSPEND: u32 = 0x100;

pub const USBDEVFS_DISCONNECT_CLAIM_IF_DRIVER: u32 = 0x01;
pub const USBDEVFS_DISCONNECT_CLAIM_EXCEPT_DRIVER: u32 = 0x02;

#[repr(C)]
pub struct usbdevfs_disconnect_claim {
    pub interface: kernel::ffi::c_uint,
    pub flags: kernel::ffi::c_uint,
    pub driver: [kernel::ffi::c_char; USBDEVFS_MAXDRIVERNAME + 1],
}

#[repr(C)]
pub struct usbdevfs_streams {
    pub num_streams: kernel::ffi::c_uint,
    pub num_eps: kernel::ffi::c_uint,
    pub eps: [kernel::ffi::c_uchar; 0],
}

/* USB_SPEED_* values returned by USBDEVFS_GET_SPEED are defined in linux/usb/ch9.h. */

pub const USBDEVFS_CONTROL: u32 = _IOWR!('U', 0, usbdevfs_ctrltransfer);
pub const USBDEVFS_CONTROL32: u32 = _IOWR!('U', 0, usbdevfs_ctrltransfer32);
pub const USBDEVFS_BULK: u32 = _IOWR!('U', 2, usbdevfs_bulktransfer);
pub const USBDEVFS_BULK32: u32 = _IOWR!('U', 2, usbdevfs_bulktransfer32);
pub const USBDEVFS_RESETEP: u32 = _IOR!('U', 3, kernel::ffi::c_uint);
pub const USBDEVFS_SETINTERFACE: u32 = _IOR!('U', 4, usbdevfs_setinterface);
pub const USBDEVFS_SETCONFIGURATION: u32 = _IOR!('U', 5, kernel::ffi::c_uint);
pub const USBDEVFS_GETDRIVER: u32 = _IOW!('U', 8, usbdevfs_getdriver);
pub const USBDEVFS_SUBMITURB: u32 = _IOR!('U', 10, usbdevfs_urb);
pub const USBDEVFS_SUBMITURB32: u32 = _IOR!('U', 10, usbdevfs_urb32);
pub const USBDEVFS_DISCARDURB: u32 = _IO!('U', 11);
pub const USBDEVFS_REAPURB: u32 = _IOW!('U', 12, *mut kernel::ffi::c_void);
pub const USBDEVFS_REAPURB32: u32 = _IOW!('U', 12, __u32);
pub const USBDEVFS_REAPURBNDELAY: u32 = _IOW!('U', 13, *mut kernel::ffi::c_void);
pub const USBDEVFS_REAPURBNDELAY32: u32 = _IOW!('U', 13, __u32);
pub const USBDEVFS_DISCSIGNAL: u32 = _IOR!('U', 14, usbdevfs_disconnectsignal);
pub const USBDEVFS_DISCSIGNAL32: u32 = _IOR!('U', 14, usbdevfs_disconnectsignal32);
pub const USBDEVFS_CLAIMINTERFACE: u32 = _IOR!('U', 15, kernel::ffi::c_uint);
pub const USBDEVFS_RELEASEINTERFACE: u32 = _IOR!('U', 16, kernel::ffi::c_uint);
pub const USBDEVFS_CONNECTINFO: u32 = _IOW!('U', 17, usbdevfs_connectinfo);
pub const USBDEVFS_IOCTL: u32 = _IOWR!('U', 18, usbdevfs_ioctl);
pub const USBDEVFS_IOCTL32: u32 = _IOWR!('U', 18, usbdevfs_ioctl32);
pub const USBDEVFS_HUB_PORTINFO: u32 = _IOR!('U', 19, usbdevfs_hub_portinfo);
pub const USBDEVFS_RESET: u32 = _IO!('U', 20);
pub const USBDEVFS_CLEAR_HALT: u32 = _IOR!('U', 21, kernel::ffi::c_uint);
pub const USBDEVFS_DISCONNECT: u32 = _IO!('U', 22);
pub const USBDEVFS_CONNECT: u32 = _IO!('U', 23);
pub const USBDEVFS_CLAIM_PORT: u32 = _IOR!('U', 24, kernel::ffi::c_uint);
pub const USBDEVFS_RELEASE_PORT: u32 = _IOR!('U', 25, kernel::ffi::c_uint);
pub const USBDEVFS_GET_CAPABILITIES: u32 = _IOR!('U', 26, __u32);
pub const USBDEVFS_DISCONNECT_CLAIM: u32 = _IOR!('U', 27, usbdevfs_disconnect_claim);
pub const USBDEVFS_ALLOC_STREAMS: u32 = _IOR!('U', 28, usbdevfs_streams);
pub const USBDEVFS_FREE_STREAMS: u32 = _IOR!('U', 29, usbdevfs_streams);
pub const USBDEVFS_DROP_PRIVILEGES: u32 = _IOW!('U', 30, __u32);
pub const USBDEVFS_GET_SPEED: u32 = _IO!('U', 31);

/* Returns struct usbdevfs_conninfo_ex; length is variable to allow extending size. */
macro_rules! USBDEVFS_CONNINFO_EX {
    ($len:expr) => { _IOC!(_IOC_READ, 'U', 32, $len) };
}

pub const USBDEVFS_FORBID_SUSPEND: u32 = _IO!('U', 33);
pub const USBDEVFS_ALLOW_SUSPEND: u32 = _IO!('U', 34);
pub const USBDEVFS_WAIT_FOR_RESUME: u32 = _IO!('U', 35);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
