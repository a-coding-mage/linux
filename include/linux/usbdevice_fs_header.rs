// SPDX-License-Identifier: GPL-2.0+
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

// Dependency supplied by the corresponding UAPI header:
// use the externally provided `usbdevfs_iso_packet_desc` type.

// The following declarations are conditional on CONFIG_COMPAT in the C header.
#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct usbdevfs_ctrltransfer32 {
    pub bRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
    pub timeout: u32, // in milliseconds
    pub data: compat_caddr_t,
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct usbdevfs_bulktransfer32 {
    pub ep: compat_uint_t,
    pub len: compat_uint_t,
    pub timeout: compat_uint_t, // in milliseconds
    pub data: compat_caddr_t,
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct usbdevfs_disconnectsignal32 {
    pub signr: compat_int_t,
    pub context: compat_caddr_t,
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct usbdevfs_urb32 {
    pub type_: u8,
    pub endpoint: u8,
    pub status: compat_int_t,
    pub flags: compat_uint_t,
    pub buffer: compat_caddr_t,
    pub buffer_length: compat_int_t,
    pub actual_length: compat_int_t,
    pub start_frame: compat_int_t,
    pub number_of_packets: compat_int_t,
    pub error_count: compat_int_t,
    pub signr: compat_uint_t,
    pub usercontext: compat_caddr_t, // unused
    pub iso_frame_desc: [usbdevfs_iso_packet_desc; 0],
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct usbdevfs_ioctl32 {
    pub ifno: i32,
    pub ioctl_code: i32,
    pub data: compat_caddr_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
