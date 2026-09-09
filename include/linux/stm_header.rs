// SPDX-License-Identifier: GPL-2.0
/*
 * System Trace Module (STM) infrastructure apis
 * Copyright (C) 2014 Intel Corporation.
 */

use core::ffi::c_char;

// Dependency supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stp_policy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stm_source_device {
    _private: [u8; 0],
}

pub type phys_addr_t = usize;

/**
 * enum stp_packet_type - STP packets that an STM driver sends
 */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum stp_packet_type {
    STP_PACKET_DATA = 0,
    STP_PACKET_FLAG,
    STP_PACKET_USER,
    STP_PACKET_MERR,
    STP_PACKET_GERR,
    STP_PACKET_TRIG,
    STP_PACKET_XSYNC,
}

/**
 * enum stp_packet_flags - STP packet modifiers
 */
pub const STP_PACKET_MARKED: u32 = 0x1;
pub const STP_PACKET_TIMESTAMPED: u32 = 0x2;

/**
 * enum stm_source_type - STM source driver
 * @STM_USER: any STM trace source
 * @STM_FTRACE: ftrace STM source
 */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum stm_source_type {
    STM_USER = 0,
    STM_FTRACE,
}

/**
 * struct stm_data - STM device description and callbacks
 *
 * Fill out this structure before calling stm_register_device() to create an
 * STM device and stm_unregister_device() to destroy it.
 */
#[repr(C)]
pub struct stm_data {
    pub name: *const c_char,
    pub stm: *mut stm_device,
    pub sw_start: u32,
    pub sw_end: u32,
    pub sw_nchannels: u32,
    pub sw_mmiosz: u32,
    pub hw_override: u32,
    pub packet: Option<unsafe extern "C" fn(
        *mut stm_data,
        u32,
        u32,
        u32,
        u32,
        u32,
        *const u8,
    ) -> isize>,
    pub mmio_addr: Option<unsafe extern "C" fn(*mut stm_data, u32, u32, u32) -> phys_addr_t>,
    pub link: Option<unsafe extern "C" fn(*mut stm_data, u32, u32) -> i32>,
    pub unlink: Option<unsafe extern "C" fn(*mut stm_data, u32, u32)>,
    pub set_options: Option<unsafe extern "C" fn(*mut stm_data, u32, u32, u32, u64) -> i64>,
}

extern "C" {
    pub fn stm_register_device(
        parent: *mut device,
        stm_data: *mut stm_data,
        owner: *mut module,
    ) -> i32;
    pub fn stm_unregister_device(stm_data: *mut stm_data);
}

/**
 * struct stm_source_data - STM source device description and callbacks
 * @name: device name, will be used for policy lookup
 * @src: internal structure, only used by stm class code
 * @nr_chans: number of channels to allocate
 * @type: type of STM source driver represented by stm_source_type
 * @link: called when this source gets linked to an STM device
 * @unlink: called when this source is about to get unlinked from its STM
 */
#[repr(C)]
pub struct stm_source_data {
    pub name: *const c_char,
    pub src: *mut stm_source_device,
    pub percpu: u32,
    pub nr_chans: u32,
    pub type_: u32,
    pub link: Option<unsafe extern "C" fn(*mut stm_source_data) -> i32>,
    pub unlink: Option<unsafe extern "C" fn(*mut stm_source_data)>,
}

extern "C" {
    pub fn stm_source_register_device(
        parent: *mut device,
        data: *mut stm_source_data,
    ) -> i32;
    pub fn stm_source_unregister_device(data: *mut stm_source_data);
    pub fn stm_source_write(
        data: *mut stm_source_data,
        chan: u32,
        buf: *const c_char,
        count: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
