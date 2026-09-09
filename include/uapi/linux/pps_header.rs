/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * PPS API header
 *
 * Copyright (C) 2005-2009   Rodolfo Giometti <giometti@linux.it>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation; either version 2 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 */

// Dependency intent: Linux integer types and ioctl encoding are represented
// by their fixed-width Rust equivalents and the local Linux ioctl encoding.

pub const PPS_VERSION: &str = "5.3.6";
pub const PPS_MAX_SOURCES: i32 = 256; // should be enough...

/* Implementation note: the logical states ``assert'' and ``clear''
 * are implemented in terms of the chip register, i.e. ``assert''
 * means the bit is set.  */

pub const PPS_API_VERS_1: i32 = 1;
pub const PPS_API_VERS: i32 = PPS_API_VERS_1; // we use API version 1
pub const PPS_MAX_NAME_LEN: i32 = 32;

#[repr(C)]
pub struct pps_ktime {
    pub sec: i64,
    pub nsec: i32,
    pub flags: u32,
}

#[repr(C, packed(4))]
pub struct pps_ktime_compat {
    pub sec: i64,
    pub nsec: i32,
    pub flags: u32,
}

pub const PPS_TIME_INVALID: i32 = 1 << 0; // used to specify timeout==NULL

#[repr(C)]
pub struct pps_kinfo {
    pub assert_sequence: u32,
    pub clear_sequence: u32,
    pub assert_tu: pps_ktime,
    pub clear_tu: pps_ktime,
    pub current_mode: i32,
}

#[repr(C)]
pub struct pps_kinfo_compat {
    pub assert_sequence: u32,
    pub clear_sequence: u32,
    pub assert_tu: pps_ktime_compat,
    pub clear_tu: pps_ktime_compat,
    pub current_mode: i32,
}

#[repr(C)]
pub struct pps_kparams {
    pub api_version: i32,
    pub mode: i32,
    pub assert_off_tu: pps_ktime,
    pub clear_off_tu: pps_ktime,
}

pub const PPS_CAPTUREASSERT: i32 = 0x01;
pub const PPS_CAPTURECLEAR: i32 = 0x02;
pub const PPS_CAPTUREBOTH: i32 = 0x03;
pub const PPS_OFFSETASSERT: i32 = 0x10;
pub const PPS_OFFSETCLEAR: i32 = 0x20;
pub const PPS_CANWAIT: i32 = 0x100;
pub const PPS_CANPOLL: i32 = 0x200;
pub const PPS_ECHOASSERT: i32 = 0x40;
pub const PPS_ECHOCLEAR: i32 = 0x80;
pub const PPS_TSFMT_TSPEC: i32 = 0x1000;
pub const PPS_TSFMT_NTPFP: i32 = 0x2000;

pub const PPS_KC_HARDPPS: i32 = 0;
pub const PPS_KC_HARDPPS_PLL: i32 = 1;
pub const PPS_KC_HARDPPS_FLL: i32 = 2;

#[repr(C)]
pub struct pps_fdata {
    pub info: pps_kinfo,
    pub timeout: pps_ktime,
}

#[repr(C)]
pub struct pps_fdata_compat {
    pub info: pps_kinfo_compat,
    pub timeout: pps_ktime_compat,
}

#[repr(C)]
pub struct pps_bind_args {
    pub tsformat: i32,
    pub edge: i32,
    pub consumer: i32,
}

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT)
        | ((size as u32) << IOC_SIZESHIFT)
}

pub const PPS_GETPARAMS: u32 = ioc(IOC_READ, b'p' as u32, 0xa1, core::mem::size_of::<pps_kparams>());
pub const PPS_SETPARAMS: u32 = ioc(IOC_WRITE, b'p' as u32, 0xa2, core::mem::size_of::<pps_kparams>());
pub const PPS_GETCAP: u32 = ioc(IOC_READ, b'p' as u32, 0xa3, core::mem::size_of::<i32>());
pub const PPS_FETCH: u32 = ioc(IOC_READ | IOC_WRITE, b'p' as u32, 0xa4, core::mem::size_of::<pps_fdata>());
pub const PPS_KC_BIND: u32 = ioc(IOC_WRITE, b'p' as u32, 0xa5, core::mem::size_of::<pps_bind_args>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
