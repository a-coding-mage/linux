/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  Copyright (c) 2007 Jiri Kosina
 */
/*
 * This program is free software; you can redistribute it and/or modify it
 * under the terms and conditions of the GNU General Public License,
 * version 2, as published by the Free Software Foundation.
 */

// C dependencies: <linux/hid.h> and <linux/types.h>.

#[repr(C)]
pub struct hidraw_report_descriptor {
    pub size: u32,
    pub value: [u8; HID_MAX_DESCRIPTOR_SIZE],
}

#[repr(C)]
pub struct hidraw_devinfo {
    pub bustype: u32,
    pub vendor: i16,
    pub product: i16,
}

/* ioctl interface */
macro_rules! HIDIOCGRDESCSIZE { () => { _IOR('H', 0x01, core::ffi::c_int) }; }
macro_rules! HIDIOCGRDESC { () => { _IOR('H', 0x02, hidraw_report_descriptor) }; }
macro_rules! HIDIOCGRAWINFO { () => { _IOR('H', 0x03, hidraw_devinfo) }; }
macro_rules! HIDIOCGRAWNAME { ($len:expr) => { _IOC(_IOC_READ, 'H', 0x04, $len) }; }
macro_rules! HIDIOCGRAWPHYS { ($len:expr) => { _IOC(_IOC_READ, 'H', 0x05, $len) }; }
/* The first byte of SFEATURE and GFEATURE is the report number */
macro_rules! HIDIOCSFEATURE { ($len:expr) => { _IOC(_IOC_WRITE | _IOC_READ, 'H', 0x06, $len) }; }
macro_rules! HIDIOCGFEATURE { ($len:expr) => { _IOC(_IOC_WRITE | _IOC_READ, 'H', 0x07, $len) }; }
macro_rules! HIDIOCGRAWUNIQ { ($len:expr) => { _IOC(_IOC_READ, 'H', 0x08, $len) }; }
/* The first byte of SINPUT and GINPUT is the report number */
macro_rules! HIDIOCSINPUT { ($len:expr) => { _IOC(_IOC_WRITE | _IOC_READ, 'H', 0x09, $len) }; }
macro_rules! HIDIOCGINPUT { ($len:expr) => { _IOC(_IOC_WRITE | _IOC_READ, 'H', 0x0A, $len) }; }
/* The first byte of SOUTPUT and GOUTPUT is the report number */
macro_rules! HIDIOCSOUTPUT { ($len:expr) => { _IOC(_IOC_WRITE | _IOC_READ, 'H', 0x0B, $len) }; }
macro_rules! HIDIOCGOUTPUT { ($len:expr) => { _IOC(_IOC_WRITE | _IOC_READ, 'H', 0x0C, $len) }; }
macro_rules! HIDIOCREVOKE { () => { _IOW('H', 0x0D, core::ffi::c_int) }; } /* Revoke device access */

macro_rules! HIDIOCTL_LAST { () => { _IOC_NR(HIDIOCREVOKE!()) }; }

pub const HIDRAW_FIRST_MINOR: i32 = 0;
pub const HIDRAW_MAX_DEVICES: i32 = 64;
/* number of reports to buffer */
pub const HIDRAW_BUFFER_SIZE: i32 = 64;

/* kernel-only API declarations */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
