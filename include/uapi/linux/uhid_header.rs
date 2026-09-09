/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 * User-space I/O driver support for HID subsystem
 * Copyright (c) 2012 David Herrmann
 */

/* Public header for user-space communication. Structures are packed for ABI compatibility. */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum uhid_event_type {
    __UHID_LEGACY_CREATE = 0,
    UHID_DESTROY,
    UHID_START,
    UHID_STOP,
    UHID_OPEN,
    UHID_CLOSE,
    UHID_OUTPUT,
    __UHID_LEGACY_OUTPUT_EV,
    __UHID_LEGACY_INPUT,
    UHID_GET_REPORT,
    UHID_GET_REPORT_REPLY,
    UHID_CREATE2,
    UHID_INPUT2,
    UHID_SET_REPORT,
    UHID_SET_REPORT_REPLY,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_create2_req {
    pub name: [u8; 128],
    pub phys: [u8; 64],
    pub uniq: [u8; 64],
    pub rd_size: u16,
    pub bus: u16,
    pub vendor: u32,
    pub product: u32,
    pub version: u32,
    pub country: u32,
    pub rd_data: [u8; HID_MAX_DESCRIPTOR_SIZE],
}

pub const UHID_DEV_NUMBERED_FEATURE_REPORTS: u64 = 1u64 << 0;
pub const UHID_DEV_NUMBERED_OUTPUT_REPORTS: u64 = 1u64 << 1;
pub const UHID_DEV_NUMBERED_INPUT_REPORTS: u64 = 1u64 << 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_start_req {
    pub dev_flags: u64,
}

pub const UHID_DATA_MAX: usize = 4096;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum uhid_report_type {
    UHID_FEATURE_REPORT = 0,
    UHID_OUTPUT_REPORT,
    UHID_INPUT_REPORT,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_input2_req {
    pub size: u16,
    pub data: [u8; UHID_DATA_MAX],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_output_req {
    pub data: [u8; UHID_DATA_MAX],
    pub size: u16,
    pub rtype: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_get_report_req {
    pub id: u32,
    pub rnum: u8,
    pub rtype: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_get_report_reply_req {
    pub id: u32,
    pub err: u16,
    pub size: u16,
    pub data: [u8; UHID_DATA_MAX],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_set_report_req {
    pub id: u32,
    pub rnum: u8,
    pub rtype: u8,
    pub size: u16,
    pub data: [u8; UHID_DATA_MAX],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_set_report_reply_req {
    pub id: u32,
    pub err: u16,
}

/* Compat Layer: obsolete commands retained for backwards compatibility. */
pub const UHID_CREATE: u32 = uhid_event_type::__UHID_LEGACY_CREATE as u32;
pub const UHID_OUTPUT_EV: u32 = uhid_event_type::__UHID_LEGACY_OUTPUT_EV as u32;
pub const UHID_INPUT: u32 = uhid_event_type::__UHID_LEGACY_INPUT as u32;
pub const UHID_FEATURE: u32 = uhid_event_type::UHID_GET_REPORT as u32;
pub const UHID_FEATURE_ANSWER: u32 = uhid_event_type::UHID_GET_REPORT_REPLY as u32;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_create_req {
    pub name: [u8; 128],
    pub phys: [u8; 64],
    pub uniq: [u8; 64],
    pub rd_data: *mut u8,
    pub rd_size: u16,
    pub bus: u16,
    pub vendor: u32,
    pub product: u32,
    pub version: u32,
    pub country: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_input_req {
    pub data: [u8; UHID_DATA_MAX],
    pub size: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_output_ev_req {
    pub r#type: u16,
    pub code: u16,
    pub value: i32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_feature_req {
    pub id: u32,
    pub rnum: u8,
    pub rtype: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_feature_answer_req {
    pub id: u32,
    pub err: u16,
    pub size: u16,
    pub data: [u8; UHID_DATA_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union uhid_event_union {
    pub create: uhid_create_req,
    pub input: uhid_input_req,
    pub output: uhid_output_req,
    pub output_ev: uhid_output_ev_req,
    pub feature: uhid_feature_req,
    pub get_report: uhid_get_report_req,
    pub feature_answer: uhid_feature_answer_req,
    pub get_report_reply: uhid_get_report_reply_req,
    pub create2: uhid_create2_req,
    pub input2: uhid_input2_req,
    pub set_report: uhid_set_report_req,
    pub set_report_reply: uhid_set_report_reply_req,
    pub start: uhid_start_req,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uhid_event {
    pub r#type: u32,
    pub u: uhid_event_union,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
