// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2011-2017, The Linux Foundation */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub static slimbus_bus: bus_type;
}

#[inline]
pub unsafe fn slim_get_devicedata(dev: *const slim_device) -> *mut c_void {
    dev_get_drvdata(&(*dev).dev as *const device)
}

#[inline]
pub unsafe fn slim_set_devicedata(dev: *mut slim_device, data: *mut c_void) {
    dev_set_drvdata(&mut (*dev).dev as *mut device, data);
}

#[repr(C, packed)]
pub struct slim_eaddr {
    pub instance: u8,
    pub dev_index: u8,
    pub prod_code: u16,
    pub manf_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum slim_device_status {
    SLIM_DEVICE_STATUS_DOWN = 0,
    SLIM_DEVICE_STATUS_UP,
    SLIM_DEVICE_STATUS_RESERVED,
}

#[repr(C)]
pub struct slim_controller;

#[repr(C)]
pub struct slim_device {
    pub dev: device,
    pub e_addr: slim_eaddr,
    pub ctrl: *mut slim_controller,
    pub status: slim_device_status,
    pub laddr: u8,
    pub is_laddr_valid: bool,
    pub stream_list: list_head,
    pub stream_list_lock: spinlock_t,
}

#[repr(C)]
pub struct slim_driver {
    pub probe: Option<unsafe extern "C" fn(*mut slim_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut slim_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut slim_device)>,
    pub device_status: Option<unsafe extern "C" fn(*mut slim_device, slim_device_status) -> i32>,
    pub driver: device_driver,
    pub id_table: *const slim_device_id,
}

#[repr(C)]
pub struct slim_val_inf {
    pub start_offset: u16,
    pub num_bytes: u8,
    pub rbuf: *mut u8,
    pub wbuf: *const u8,
    pub comp: *mut completion,
}

pub const SLIM_DEVICE_MAX_CHANNELS: u32 = 256;
pub const SLIM_DEVICE_MAX_PORTS: u32 = 32;

#[repr(C)]
pub struct slim_stream_config {
    pub rate: ::core::ffi::c_uint,
    pub bps: ::core::ffi::c_uint,
    pub ch_count: ::core::ffi::c_uint,
    pub chs: *mut ::core::ffi::c_uint,
    pub port_mask: ::core::ffi::c_ulong,
    pub direction: ::core::ffi::c_int,
}

extern "C" {
    pub fn __slim_driver_register(drv: *mut slim_driver, owner: *mut module) -> i32;
    pub fn slim_driver_unregister(drv: *mut slim_driver);
    pub fn dev_get_drvdata(dev: *const device) -> *mut ::core::ffi::c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut ::core::ffi::c_void);
    pub fn of_slim_get_device(ctrl: *mut slim_controller, np: *mut device_node) -> *mut slim_device;
    pub fn slim_get_device(ctrl: *mut slim_controller, e_addr: *mut slim_eaddr) -> *mut slim_device;
    pub fn slim_get_logical_addr(sbdev: *mut slim_device) -> i32;
    pub fn slim_xfer_msg(sbdev: *mut slim_device, msg: *mut slim_val_inf, mc: u8) -> i32;
    pub fn slim_readb(sdev: *mut slim_device, addr: u32) -> i32;
    pub fn slim_writeb(sdev: *mut slim_device, addr: u32, value: u8) -> i32;
    pub fn slim_read(sdev: *mut slim_device, addr: u32, count: usize, val: *mut u8) -> i32;
    pub fn slim_write(sdev: *mut slim_device, addr: u32, count: usize, val: *mut u8) -> i32;
    pub fn slim_stream_allocate(dev: *mut slim_device, sname: *const u8) -> *mut slim_stream_runtime;
    pub fn slim_stream_prepare(stream: *mut slim_stream_runtime, c: *mut slim_stream_config) -> i32;
    pub fn slim_stream_enable(stream: *mut slim_stream_runtime) -> i32;
    pub fn slim_stream_disable(stream: *mut slim_stream_runtime) -> i32;
    pub fn slim_stream_unprepare(stream: *mut slim_stream_runtime) -> i32;
    pub fn slim_stream_free(stream: *mut slim_stream_runtime) -> i32;
}

#[repr(C)]
pub struct slim_stream_runtime;

pub const SLIM_MSG_MC_REQUEST_INFORMATION: u8 = 0x20;
pub const SLIM_MSG_MC_REQUEST_CLEAR_INFORMATION: u8 = 0x21;
pub const SLIM_MSG_MC_REPLY_INFORMATION: u8 = 0x24;
pub const SLIM_MSG_MC_CLEAR_INFORMATION: u8 = 0x28;
pub const SLIM_MSG_MC_REPORT_INFORMATION: u8 = 0x29;
pub const SLIM_MSG_MC_REQUEST_VALUE: u8 = 0x60;
pub const SLIM_MSG_MC_REQUEST_CHANGE_VALUE: u8 = 0x61;
pub const SLIM_MSG_MC_REPLY_VALUE: u8 = 0x64;
pub const SLIM_MSG_MC_CHANGE_VALUE: u8 = 0x68;

// The C container_of/container_of_const macros are provided by the kernel support layer.
// slim_driver_register and module_slim_driver retain their C macro intent here.

pub type c_void = ::core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
