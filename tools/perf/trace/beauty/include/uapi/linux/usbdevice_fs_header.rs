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
 *
 *	This program is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *	GNU General Public License for more details.
 *
 *	You should have received a copy of the GNU General Public License
 *	along with this program; if not, write to the Free Software
 *	Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 *
 *  History:
 *   0.1  04.01.2000  Created
 */

/*****************************************************************************/

/* Original C dependencies: <linux/types.h>, <linux/magic.h>. */

/* --------------------------------------------------------------------- */

/* usbdevfs ioctl codes */

#[repr(C)]
pub struct usbdevfs_ctrltransfer {
    pub bRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __u16,
    pub wIndex: __u16,
    pub wLength: __u16,
    pub timeout: __u32, /* in milliseconds */
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct usbdevfs_bulktransfer {
    pub ep: core::ffi::c_uint,
    pub len: core::ffi::c_uint,
    pub timeout: core::ffi::c_uint, /* in milliseconds */
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct usbdevfs_setinterface {
    pub interface: core::ffi::c_uint,
    pub altsetting: core::ffi::c_uint,
}

#[repr(C)]
pub struct usbdevfs_disconnectsignal {
    pub signr: core::ffi::c_uint,
    pub context: *mut core::ffi::c_void,
}

pub const USBDEVFS_MAXDRIVERNAME: usize = 255;

#[repr(C)]
pub struct usbdevfs_getdriver {
    pub interface: core::ffi::c_uint,
    pub driver: [core::ffi::c_char; USBDEVFS_MAXDRIVERNAME + 1],
}

#[repr(C)]
pub struct usbdevfs_connectinfo {
    pub devnum: core::ffi::c_uint,
    pub slow: core::ffi::c_uchar,
}

#[repr(C)]
pub struct usbdevfs_conninfo_ex {
    pub size: __u32,      /* Size of the structure from the kernel's */
                         /* point of view. Can be used by userspace */
                         /* to determine how much data can be       */
                         /* used/trusted.                           */
    pub busnum: __u32,    /* USB bus number, as enumerated by the    */
                         /* kernel, the device is connected to.     */
    pub devnum: __u32,    /* Device address on the bus.              */
    pub speed: __u32,     /* USB_SPEED_* constants from ch9.h        */
    pub num_ports: __u8,  /* Number of ports the device is connected */
                         /* to on the way to the root hub. It may   */
                         /* be bigger than size of 'ports' array so */
                         /* userspace can detect overflows.         */
    pub ports: [__u8; 7], /* List of ports on the way from the root  */
                         /* hub to the device. Current limit in     */
                         /* USB specification is 7 tiers (root hub, */
                         /* 5 intermediate hubs, device), which     */
                         /* gives at most 6 port entries.           */
}

pub const USBDEVFS_URB_SHORT_NOT_OK: u32 = 0x01;
pub const USBDEVFS_URB_ISO_ASAP: u32 = 0x02;
pub const USBDEVFS_URB_BULK_CONTINUATION: u32 = 0x04;
pub const USBDEVFS_URB_NO_FSBR: u32 = 0x20; /* Not used */
pub const USBDEVFS_URB_ZERO_PACKET: u32 = 0x40;
pub const USBDEVFS_URB_NO_INTERRUPT: u32 = 0x80;

pub const USBDEVFS_URB_TYPE_ISO: u32 = 0;
pub const USBDEVFS_URB_TYPE_INTERRUPT: u32 = 1;
pub const USBDEVFS_URB_TYPE_CONTROL: u32 = 2;
pub const USBDEVFS_URB_TYPE_BULK: u32 = 3;

#[repr(C)]
pub struct usbdevfs_iso_packet_desc {
    pub length: core::ffi::c_uint,
    pub actual_length: core::ffi::c_uint,
    pub status: core::ffi::c_uint,
}

#[repr(C)]
pub union usbdevfs_urb__bindgen_ty_1 {
    pub number_of_packets: core::ffi::c_int, /* Only used for isoc urbs */
    pub stream_id: core::ffi::c_uint,        /* Only used with bulk streams */
}

#[repr(C)]
pub struct usbdevfs_urb {
    pub type_: core::ffi::c_uchar,
    pub endpoint: core::ffi::c_uchar,
    pub status: core::ffi::c_int,
    pub flags: core::ffi::c_uint,
    pub buffer: *mut core::ffi::c_void,
    pub buffer_length: core::ffi::c_int,
    pub actual_length: core::ffi::c_int,
    pub start_frame: core::ffi::c_int,
    pub u: usbdevfs_urb__bindgen_ty_1,
    pub error_count: core::ffi::c_int,
    pub signr: core::ffi::c_uint, /* signal to be sent on completion,
                                     or 0 if none should be sent. */
    pub usercontext: *mut core::ffi::c_void,
    /* C flexible array member: struct usbdevfs_iso_packet_desc iso_frame_desc[]; */
    pub iso_frame_desc: [usbdevfs_iso_packet_desc; 0],
}

/* ioctls for talking directly to drivers */
#[repr(C)]
pub struct usbdevfs_ioctl {
    pub ifno: core::ffi::c_int,       /* interface 0..N ; negative numbers reserved */
    pub ioctl_code: core::ffi::c_int, /* MUST encode size + direction of data so the
                                      * macros in <asm/ioctl.h> give correct values */
    pub data: *mut core::ffi::c_void, /* param buffer (in, or out) */
}

/* You can do most things with hubs just through control messages,
 * except find out what device connects to what port. */
#[repr(C)]
pub struct usbdevfs_hub_portinfo {
    pub nports: core::ffi::c_char,        /* number of downstream ports in this hub */
    pub port: [core::ffi::c_char; 127],   /* e.g. port 3 connects to device 27 */
}

/* System and bus capability flags */
pub const USBDEVFS_CAP_ZERO_PACKET: u32 = 0x01;
pub const USBDEVFS_CAP_BULK_CONTINUATION: u32 = 0x02;
pub const USBDEVFS_CAP_NO_PACKET_SIZE_LIM: u32 = 0x04;
pub const USBDEVFS_CAP_BULK_SCATTER_GATHER: u32 = 0x08;
pub const USBDEVFS_CAP_REAP_AFTER_DISCONNECT: u32 = 0x10;
pub const USBDEVFS_CAP_MMAP: u32 = 0x20;
pub const USBDEVFS_CAP_DROP_PRIVILEGES: u32 = 0x40;
pub const USBDEVFS_CAP_CONNINFO_EX: u32 = 0x80;
pub const USBDEVFS_CAP_SUSPEND: u32 = 0x100;

/* USBDEVFS_DISCONNECT_CLAIM flags & struct */

/* disconnect-and-claim if the driver matches the driver field */
pub const USBDEVFS_DISCONNECT_CLAIM_IF_DRIVER: u32 = 0x01;
/* disconnect-and-claim except when the driver matches the driver field */
pub const USBDEVFS_DISCONNECT_CLAIM_EXCEPT_DRIVER: u32 = 0x02;

#[repr(C)]
pub struct usbdevfs_disconnect_claim {
    pub interface: core::ffi::c_uint,
    pub flags: core::ffi::c_uint,
    pub driver: [core::ffi::c_char; USBDEVFS_MAXDRIVERNAME + 1],
}

#[repr(C)]
pub struct usbdevfs_streams {
    pub num_streams: core::ffi::c_uint, /* Not used by USBDEVFS_FREE_STREAMS */
    pub num_eps: core::ffi::c_uint,
    /* C flexible array member: unsigned char eps[]; */
    pub eps: [core::ffi::c_uchar; 0],
}

/*
 * USB_SPEED_* values returned by USBDEVFS_GET_SPEED are defined in
 * linux/usb/ch9.h
 */

/* The following ioctl constants depend on Linux _IO/_IOR/_IOW/_IOWR/_IOC macros
 * and external 32-bit compatibility structs supplied outside this header. */
pub const USBDEVFS_CONTROL: core::ffi::c_ulong = _IOWR::<usbdevfs_ctrltransfer>(b'U', 0);
pub const USBDEVFS_CONTROL32: core::ffi::c_ulong = _IOWR::<usbdevfs_ctrltransfer32>(b'U', 0);
pub const USBDEVFS_BULK: core::ffi::c_ulong = _IOWR::<usbdevfs_bulktransfer>(b'U', 2);
pub const USBDEVFS_BULK32: core::ffi::c_ulong = _IOWR::<usbdevfs_bulktransfer32>(b'U', 2);
pub const USBDEVFS_RESETEP: core::ffi::c_ulong = _IOR::<core::ffi::c_uint>(b'U', 3);
pub const USBDEVFS_SETINTERFACE: core::ffi::c_ulong = _IOR::<usbdevfs_setinterface>(b'U', 4);
pub const USBDEVFS_SETCONFIGURATION: core::ffi::c_ulong = _IOR::<core::ffi::c_uint>(b'U', 5);
pub const USBDEVFS_GETDRIVER: core::ffi::c_ulong = _IOW::<usbdevfs_getdriver>(b'U', 8);
pub const USBDEVFS_SUBMITURB: core::ffi::c_ulong = _IOR::<usbdevfs_urb>(b'U', 10);
pub const USBDEVFS_SUBMITURB32: core::ffi::c_ulong = _IOR::<usbdevfs_urb32>(b'U', 10);
pub const USBDEVFS_DISCARDURB: core::ffi::c_ulong = _IO(b'U', 11);
pub const USBDEVFS_REAPURB: core::ffi::c_ulong = _IOW::<*mut core::ffi::c_void>(b'U', 12);
pub const USBDEVFS_REAPURB32: core::ffi::c_ulong = _IOW::<__u32>(b'U', 12);
pub const USBDEVFS_REAPURBNDELAY: core::ffi::c_ulong = _IOW::<*mut core::ffi::c_void>(b'U', 13);
pub const USBDEVFS_REAPURBNDELAY32: core::ffi::c_ulong = _IOW::<__u32>(b'U', 13);
pub const USBDEVFS_DISCSIGNAL: core::ffi::c_ulong = _IOR::<usbdevfs_disconnectsignal>(b'U', 14);
pub const USBDEVFS_DISCSIGNAL32: core::ffi::c_ulong = _IOR::<usbdevfs_disconnectsignal32>(b'U', 14);
pub const USBDEVFS_CLAIMINTERFACE: core::ffi::c_ulong = _IOR::<core::ffi::c_uint>(b'U', 15);
pub const USBDEVFS_RELEASEINTERFACE: core::ffi::c_ulong = _IOR::<core::ffi::c_uint>(b'U', 16);
pub const USBDEVFS_CONNECTINFO: core::ffi::c_ulong = _IOW::<usbdevfs_connectinfo>(b'U', 17);
pub const USBDEVFS_IOCTL: core::ffi::c_ulong = _IOWR::<usbdevfs_ioctl>(b'U', 18);
pub const USBDEVFS_IOCTL32: core::ffi::c_ulong = _IOWR::<usbdevfs_ioctl32>(b'U', 18);
pub const USBDEVFS_HUB_PORTINFO: core::ffi::c_ulong = _IOR::<usbdevfs_hub_portinfo>(b'U', 19);
pub const USBDEVFS_RESET: core::ffi::c_ulong = _IO(b'U', 20);
pub const USBDEVFS_CLEAR_HALT: core::ffi::c_ulong = _IOR::<core::ffi::c_uint>(b'U', 21);
pub const USBDEVFS_DISCONNECT: core::ffi::c_ulong = _IO(b'U', 22);
pub const USBDEVFS_CONNECT: core::ffi::c_ulong = _IO(b'U', 23);
pub const USBDEVFS_CLAIM_PORT: core::ffi::c_ulong = _IOR::<core::ffi::c_uint>(b'U', 24);
pub const USBDEVFS_RELEASE_PORT: core::ffi::c_ulong = _IOR::<core::ffi::c_uint>(b'U', 25);
pub const USBDEVFS_GET_CAPABILITIES: core::ffi::c_ulong = _IOR::<__u32>(b'U', 26);
pub const USBDEVFS_DISCONNECT_CLAIM: core::ffi::c_ulong = _IOR::<usbdevfs_disconnect_claim>(b'U', 27);
pub const USBDEVFS_ALLOC_STREAMS: core::ffi::c_ulong = _IOR::<usbdevfs_streams>(b'U', 28);
pub const USBDEVFS_FREE_STREAMS: core::ffi::c_ulong = _IOR::<usbdevfs_streams>(b'U', 29);
pub const USBDEVFS_DROP_PRIVILEGES: core::ffi::c_ulong = _IOW::<__u32>(b'U', 30);
pub const USBDEVFS_GET_SPEED: core::ffi::c_ulong = _IO(b'U', 31);
/*
 * Returns struct usbdevfs_conninfo_ex; length is variable to allow
 * extending size of the data returned.
 */
pub const fn USBDEVFS_CONNINFO_EX(len: core::ffi::c_uint) -> core::ffi::c_ulong {
    _IOC(_IOC_READ, b'U', 32, len)
}
pub const USBDEVFS_FORBID_SUSPEND: core::ffi::c_ulong = _IO(b'U', 33);
pub const USBDEVFS_ALLOW_SUSPEND: core::ffi::c_ulong = _IO(b'U', 34);
pub const USBDEVFS_WAIT_FOR_RESUME: core::ffi::c_ulong = _IO(b'U', 35);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
