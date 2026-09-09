/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm-alpha/err_common.h
 *
 * Copyright (C) 2000 Jeff Wiedemeier (Compaq Computer Corporation)
 *
 * Contains declarations and macros to support Alpha error handling
 * implementations.
 */

/* SCB Vector definitions */
pub const SCB_Q_SYSERR: u64 = 0x620;
pub const SCB_Q_PROCERR: u64 = 0x630;
pub const SCB_Q_SYSMCHK: u64 = 0x660;
pub const SCB_Q_PROCMCHK: u64 = 0x670;
pub const SCB_Q_SYSEVENT: u64 = 0x680;

/* Disposition definitions for logout frame parser */
pub const MCHK_DISPOSITION_UNKNOWN_ERROR: u64 = 0x00;
pub const MCHK_DISPOSITION_REPORT: u64 = 0x01;
pub const MCHK_DISPOSITION_DISMISS: u64 = 0x02;

/* Error Log definitions */
/* Types */
pub const EL_CLASS__TERMINATION: u64 = 0;
pub const EL_TYPE__TERMINATION__TERMINATION: u64 = 0;
pub const EL_CLASS__HEADER: u64 = 5;
pub const EL_TYPE__HEADER__SYSTEM_ERROR_FRAME: u64 = 1;
pub const EL_TYPE__HEADER__SYSTEM_EVENT_FRAME: u64 = 2;
pub const EL_TYPE__HEADER__HALT_FRAME: u64 = 3;
pub const EL_TYPE__HEADER__LOGOUT_FRAME: u64 = 19;
pub const EL_CLASS__GENERAL_NOTIFICATION: u64 = 9;
pub const EL_CLASS__PCI_ERROR_FRAME: u64 = 11;
pub const EL_CLASS__REGATTA_FAMILY: u64 = 12;
pub const EL_TYPE__REGATTA__PROCESSOR_ERROR_FRAME: u64 = 1;
pub const EL_TYPE__REGATTA__SYSTEM_ERROR_FRAME: u64 = 2;
pub const EL_TYPE__REGATTA__ENVIRONMENTAL_FRAME: u64 = 3;
pub const EL_TYPE__REGATTA__TITAN_PCHIP0_EXTENDED: u64 = 8;
pub const EL_TYPE__REGATTA__TITAN_PCHIP1_EXTENDED: u64 = 9;
pub const EL_TYPE__REGATTA__TITAN_MEMORY_EXTENDED: u64 = 10;
pub const EL_TYPE__REGATTA__PROCESSOR_DBL_ERROR_HALT: u64 = 11;
pub const EL_TYPE__REGATTA__SYSTEM_DBL_ERROR_HALT: u64 = 12;
pub const EL_CLASS__PAL: u64 = 14;
pub const EL_TYPE__PAL__LOGOUT_FRAME: u64 = 1;
pub const EL_TYPE__PAL__EV7_PROCESSOR: u64 = 4;
pub const EL_TYPE__PAL__EV7_ZBOX: u64 = 5;
pub const EL_TYPE__PAL__EV7_RBOX: u64 = 6;
pub const EL_TYPE__PAL__EV7_IO: u64 = 7;
pub const EL_TYPE__PAL__ENV__AMBIENT_TEMPERATURE: u64 = 10;
pub const EL_TYPE__PAL__ENV__AIRMOVER_FAN: u64 = 11;
pub const EL_TYPE__PAL__ENV__VOLTAGE: u64 = 12;
pub const EL_TYPE__PAL__ENV__INTRUSION: u64 = 13;
pub const EL_TYPE__PAL__ENV__POWER_SUPPLY: u64 = 14;
pub const EL_TYPE__PAL__ENV__LAN: u64 = 15;
pub const EL_TYPE__PAL__ENV__HOT_PLUG: u64 = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub union ElTimestamp {
    pub b: ElTimestampBytes,
    pub as_int: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElTimestampBytes {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElSubpacket {
    pub length: u16, /* length of header (in bytes) */
    pub class: u16, /* header class and type... */
    pub type_: u16, /* ...determine content */
    pub revision: u16, /* header revision */
    pub by_type: ElSubpacketByType,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ElSubpacketByType {
    pub sys_err: ElSysErr,
    pub sys_event: ElSysEvent,
    pub err_halt: ElErrHalt,
    pub logout_header: ElLogoutHeader,
    pub regatta_frame: ElRegattaFrame,
    pub raw: ElRaw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElSysErr {
    pub frame_length: u32,
    pub frame_packet_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElSysEvent {
    pub timestamp: ElTimestamp,
    pub frame_length: u32,
    pub frame_packet_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElErrHalt {
    pub halt_code: u16,
    pub reserved: u16,
    pub timestamp: ElTimestamp,
    pub frame_length: u32,
    pub frame_packet_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElLogoutHeader {
    pub frame_length: u32,
    pub frame_flags: u32,
    pub cpu_offset: u32,
    pub system_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElRegattaFrame {
    pub cpuid: u64,
    pub data_start: [u64; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElRaw {
    pub data_start: [u64; 1],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
