/*
 * HP Human Interface Loop Master Link Controller driver.
 *
 * Copyright (c) 2001 Brian S. Julin
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions, and the following disclaimer,
 *    without modification.
 * 2. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL").
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 *
 * References:
 * HP-HIL Technical Reference Manual.  Hewlett Packard Product No. 45918A
 */

use core::mem::ManuallyDrop;

// Dependencies supplied by the Linux translation environment:
// hil_packet, suseconds_t, list_head, rwlock_t, semaphore, serio, and
// tasklet_struct.

#[repr(i32)]
pub enum hilse_act {
    HILSE_OUT = 0,
    HILSE_CTS,
    HILSE_OUT_LAST,
    HILSE_OUT_DISC,
    HILSE_FUNC,
    HILSE_IN = 0x100,
    HILSE_EXPECT,
    HILSE_EXPECT_LAST,
    HILSE_EXPECT_DISC,
}

pub type hilse_func = unsafe extern "C" fn(mlc: *mut hil_mlc, arg: i32) -> i32;

#[repr(C)]
pub union hilse_node_object {
    pub func: Option<hilse_func>,
    pub packet: ManuallyDrop<hil_packet>,
}

#[repr(C)]
pub struct hilse_node {
    pub act: hilse_act,
    pub object: hilse_node_object,
    pub arg: i32,
    pub good: i32,
    pub bad: i32,
    pub ugly: i32,
}

pub type hil_mlc_cts = unsafe extern "C" fn(mlc: *mut hil_mlc) -> i32;
pub type hil_mlc_out = unsafe extern "C" fn(mlc: *mut hil_mlc) -> i32;
pub type hil_mlc_in = unsafe extern "C" fn(mlc: *mut hil_mlc, timeout: suseconds_t) -> i32;

#[repr(C)]
pub struct hil_mlc_devinfo {
    pub idd: [u8; 16],
    pub rsc: [u8; 16],
    pub exd: [u8; 16],
    pub rnm: [u8; 16],
}

#[repr(C)]
pub struct hil_mlc_serio_map {
    pub mlc: *mut hil_mlc,
    pub di_revmap: i32,
    pub didx: i32,
}

pub const HIL_MLC_DEVMEM: usize = 16;

#[repr(C)]
pub struct hil_mlc {
    pub list: list_head,
    pub lock: rwlock_t,
    pub priv_: *mut core::ffi::c_void,
    pub seidx: i32,
    pub istarted: i32,
    pub ostarted: i32,
    pub cts: Option<hil_mlc_cts>,
    pub csem: semaphore,
    pub out: Option<hil_mlc_out>,
    pub osem: semaphore,
    pub opacket: hil_packet,
    pub in_: Option<hil_mlc_in>,
    pub isem: semaphore,
    pub ipacket: [hil_packet; 16],
    pub imatch: hil_packet,
    pub icount: i32,
    pub instart: core::ffi::c_ulong,
    pub intimeout: core::ffi::c_ulong,
    pub ddi: i32,
    pub lcv: i32,
    pub lcv_time: time64_t,
    pub di_map: [i32; 7],
    pub di: [hil_mlc_devinfo; HIL_MLC_DEVMEM],
    pub serio: [*mut serio; HIL_MLC_DEVMEM],
    pub serio_map: [hil_mlc_serio_map; HIL_MLC_DEVMEM],
    pub serio_opacket: [hil_packet; HIL_MLC_DEVMEM],
    pub serio_oidx: [i32; HIL_MLC_DEVMEM],
    pub di_scratch: hil_mlc_devinfo,
    pub opercnt: i32,
    pub tasklet: *mut tasklet_struct,
}

unsafe extern "C" {
    pub fn hil_mlc_register(mlc: *mut hil_mlc) -> i32;
    pub fn hil_mlc_unregister(mlc: *mut hil_mlc) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
