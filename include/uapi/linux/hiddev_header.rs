/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  Copyright (c) 1999-2000 Vojtech Pavlik
 *
 *  Sponsored by SuSE
 */
/*
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

// Dependency supplied by the surrounding UAPI translation.

/* The event structure itself */
#[repr(C)]
pub struct hiddev_event {
    pub hid: __u32,
    pub value: i32,
}

#[repr(C)]
pub struct hiddev_devinfo {
    pub bustype: __u32,
    pub busnum: __u32,
    pub devnum: __u32,
    pub ifnum: __u32,
    pub vendor: i16,
    pub product: i16,
    pub version: i16,
    pub num_applications: __u32,
}

#[repr(C)]
pub struct hiddev_collection_info {
    pub index: __u32,
    pub type_: __u32,
    pub usage: __u32,
    pub level: __u32,
}

pub const HID_STRING_SIZE: usize = 256;
#[repr(C)]
pub struct hiddev_string_descriptor {
    pub index: i32,
    pub value: [u8; HID_STRING_SIZE],
}

#[repr(C)]
pub struct hiddev_report_info {
    pub report_type: __u32,
    pub report_id: __u32,
    pub num_fields: __u32,
}

pub const HID_REPORT_ID_UNKNOWN: __u32 = 0xffffffff;
pub const HID_REPORT_ID_FIRST: __u32 = 0x00000100;
pub const HID_REPORT_ID_NEXT: __u32 = 0x00000200;
pub const HID_REPORT_ID_MASK: __u32 = 0x000000ff;
pub const HID_REPORT_ID_MAX: __u32 = 0x000000ff;

pub const HID_REPORT_TYPE_INPUT: __u32 = 1;
pub const HID_REPORT_TYPE_OUTPUT: __u32 = 2;
pub const HID_REPORT_TYPE_FEATURE: __u32 = 3;
pub const HID_REPORT_TYPE_MIN: __u32 = 1;
pub const HID_REPORT_TYPE_MAX: __u32 = 3;

#[repr(C)]
pub struct hiddev_field_info {
    pub report_type: __u32,
    pub report_id: __u32,
    pub field_index: __u32,
    pub maxusage: __u32,
    pub flags: __u32,
    pub physical: __u32,
    pub logical: __u32,
    pub application: __u32,
    pub logical_minimum: i32,
    pub logical_maximum: i32,
    pub physical_minimum: i32,
    pub physical_maximum: i32,
    pub unit_exponent: __u32,
    pub unit: __u32,
}

pub const HID_FIELD_CONSTANT: __u32 = 0x001;
pub const HID_FIELD_VARIABLE: __u32 = 0x002;
pub const HID_FIELD_RELATIVE: __u32 = 0x004;
pub const HID_FIELD_WRAP: __u32 = 0x008;
pub const HID_FIELD_NONLINEAR: __u32 = 0x010;
pub const HID_FIELD_NO_PREFERRED: __u32 = 0x020;
pub const HID_FIELD_NULL_STATE: __u32 = 0x040;
pub const HID_FIELD_VOLATILE: __u32 = 0x080;
pub const HID_FIELD_BUFFERED_BYTE: __u32 = 0x100;

#[repr(C)]
pub struct hiddev_usage_ref {
    pub report_type: __u32,
    pub report_id: __u32,
    pub field_index: __u32,
    pub usage_index: __u32,
    pub usage_code: __u32,
    pub value: i32,
}

pub const HID_MAX_MULTI_USAGES: usize = 1024;
#[repr(C)]
pub struct hiddev_usage_ref_multi {
    pub uref: hiddev_usage_ref,
    pub num_values: __u32,
    pub values: [i32; HID_MAX_MULTI_USAGES],
}

pub const HID_FIELD_INDEX_NONE: __u32 = 0xffffffff;
pub const HID_VERSION: u32 = 0x010004;

// IOCTL encodings are provided by the surrounding UAPI translation.
pub const HIDIOCGVERSION: _ = _IOR('H' as u8, 0x01, core::mem::size_of::<i32>());
pub const HIDIOCAPPLICATION: _ = _IO('H' as u8, 0x02);
pub const HIDIOCGDEVINFO: _ = _IOR('H' as u8, 0x03, core::mem::size_of::<hiddev_devinfo>());
pub const HIDIOCGSTRING: _ = _IOR('H' as u8, 0x04, core::mem::size_of::<hiddev_string_descriptor>());
pub const HIDIOCINITREPORT: _ = _IO('H' as u8, 0x05);
macro_rules! HIDIOCGNAME { ($len:expr) => { _IOC(_IOC_READ, 'H' as u8, 0x06, $len) }; }
pub const HIDIOCGREPORT: _ = _IOW('H' as u8, 0x07, core::mem::size_of::<hiddev_report_info>());
pub const HIDIOCSREPORT: _ = _IOW('H' as u8, 0x08, core::mem::size_of::<hiddev_report_info>());
pub const HIDIOCGREPORTINFO: _ = _IOWR('H' as u8, 0x09, core::mem::size_of::<hiddev_report_info>());
pub const HIDIOCGFIELDINFO: _ = _IOWR('H' as u8, 0x0A, core::mem::size_of::<hiddev_field_info>());
pub const HIDIOCGUSAGE: _ = _IOWR('H' as u8, 0x0B, core::mem::size_of::<hiddev_usage_ref>());
pub const HIDIOCSUSAGE: _ = _IOW('H' as u8, 0x0C, core::mem::size_of::<hiddev_usage_ref>());
pub const HIDIOCGUCODE: _ = _IOWR('H' as u8, 0x0D, core::mem::size_of::<hiddev_usage_ref>());
pub const HIDIOCGFLAG: _ = _IOR('H' as u8, 0x0E, core::mem::size_of::<i32>());
pub const HIDIOCSFLAG: _ = _IOW('H' as u8, 0x0F, core::mem::size_of::<i32>());
pub const HIDIOCGCOLLECTIONINDEX: _ = _IOW('H' as u8, 0x10, core::mem::size_of::<hiddev_usage_ref>());
pub const HIDIOCGCOLLECTIONINFO: _ = _IOWR('H' as u8, 0x11, core::mem::size_of::<hiddev_collection_info>());
macro_rules! HIDIOCGPHYS { ($len:expr) => { _IOC(_IOC_READ, 'H' as u8, 0x12, $len) }; }
pub const HIDIOCGUSAGES: _ = _IOWR('H' as u8, 0x13, core::mem::size_of::<hiddev_usage_ref_multi>());
pub const HIDIOCSUSAGES: _ = _IOW('H' as u8, 0x14, core::mem::size_of::<hiddev_usage_ref_multi>());

pub const HIDDEV_FLAG_UREF: __u32 = 0x1;
pub const HIDDEV_FLAG_REPORT: __u32 = 0x2;
pub const HIDDEV_FLAGS: __u32 = 0x3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
