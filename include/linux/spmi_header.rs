/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2012-2013, The Linux Foundation. All rights reserved. */

// Translated from the Linux SPMI header. The included kernel types and helpers
// are supplied by other translation units.

pub const SPMI_MAX_SLAVE_ID: u32 = 16;

pub const SPMI_CMD_EXT_WRITE: u8 = 0x00;
pub const SPMI_CMD_RESET: u8 = 0x10;
pub const SPMI_CMD_SLEEP: u8 = 0x11;
pub const SPMI_CMD_SHUTDOWN: u8 = 0x12;
pub const SPMI_CMD_WAKEUP: u8 = 0x13;
pub const SPMI_CMD_AUTHENTICATE: u8 = 0x14;
pub const SPMI_CMD_MSTR_READ: u8 = 0x15;
pub const SPMI_CMD_MSTR_WRITE: u8 = 0x16;
pub const SPMI_CMD_TRANSFER_BUS_OWNERSHIP: u8 = 0x1A;
pub const SPMI_CMD_DDB_MASTER_READ: u8 = 0x1B;
pub const SPMI_CMD_DDB_SLAVE_READ: u8 = 0x1C;
pub const SPMI_CMD_EXT_READ: u8 = 0x20;
pub const SPMI_CMD_EXT_WRITEL: u8 = 0x30;
pub const SPMI_CMD_EXT_READL: u8 = 0x38;
pub const SPMI_CMD_WRITE: u8 = 0x40;
pub const SPMI_CMD_READ: u8 = 0x60;
pub const SPMI_CMD_ZERO_WRITE: u8 = 0x80;

#[repr(C)]
pub struct spmi_device {
    pub dev: device,
    pub ctrl: *mut spmi_controller,
    pub usid: u8,
}

pub unsafe fn to_spmi_device(d: *mut device) -> *mut spmi_device {
    container_of!(d, spmi_device, dev)
}

pub unsafe fn spmi_device_get_drvdata(sdev: *const spmi_device) -> *mut core::ffi::c_void {
    dev_get_drvdata(&(*sdev).dev)
}

pub unsafe fn spmi_device_set_drvdata(sdev: *mut spmi_device, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*sdev).dev, data);
}

unsafe extern "C" {
    pub fn spmi_device_alloc(ctrl: *mut spmi_controller) -> *mut spmi_device;
}

pub unsafe fn spmi_device_put(sdev: *mut spmi_device) {
    if !sdev.is_null() {
        put_device(&mut (*sdev).dev);
    }
}

unsafe extern "C" {
    pub fn spmi_device_add(sdev: *mut spmi_device) -> i32;
    pub fn spmi_device_remove(sdev: *mut spmi_device);
}

#[repr(C)]
pub struct spmi_controller {
    pub dev: device,
    pub nr: u32,
    pub cmd: Option<unsafe extern "C" fn(*mut spmi_controller, u8, u8) -> i32>,
    pub read_cmd: Option<unsafe extern "C" fn(*mut spmi_controller, u8, u8, u16, *mut u8, usize) -> i32>,
    pub write_cmd: Option<unsafe extern "C" fn(*mut spmi_controller, u8, u8, u16, *const u8, usize) -> i32>,
    pub priv_: [u8; 0],
}

pub unsafe fn to_spmi_controller(d: *mut device) -> *mut spmi_controller {
    container_of!(d, spmi_controller, dev)
}

pub unsafe fn spmi_controller_get_drvdata(ctrl: *const spmi_controller) -> *mut core::ffi::c_void {
    dev_get_drvdata(&(*ctrl).dev)
}

pub unsafe fn spmi_controller_set_drvdata(ctrl: *mut spmi_controller, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*ctrl).dev, data);
}

unsafe extern "C" {
    pub fn spmi_controller_alloc(parent: *mut device, size: usize) -> *mut spmi_controller;
    pub fn spmi_controller_add(ctrl: *mut spmi_controller) -> i32;
    pub fn spmi_controller_remove(ctrl: *mut spmi_controller);
    pub fn devm_spmi_controller_alloc(parent: *mut device, size: usize) -> *mut spmi_controller;
    pub fn devm_spmi_controller_add(parent: *mut device, ctrl: *mut spmi_controller) -> i32;
}

pub unsafe fn spmi_controller_put(ctrl: *mut spmi_controller) {
    if !ctrl.is_null() {
        put_device(&mut (*ctrl).dev);
    }
}

#[repr(C)]
pub struct spmi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spmi_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut spmi_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut spmi_device)>,
}

pub unsafe fn to_spmi_driver(d: *mut device_driver) -> *mut spmi_driver {
    container_of!(d, spmi_driver, driver)
}

// spmi_driver_register(sdrv) expands to __spmi_driver_register(sdrv, THIS_MODULE).
unsafe extern "C" {
    pub fn __spmi_driver_register(sdrv: *mut spmi_driver, owner: *mut module) -> i32;
}

pub unsafe fn spmi_driver_unregister(sdrv: *mut spmi_driver) {
    if !sdrv.is_null() {
        driver_unregister(&mut (*sdrv).driver);
    }
}

pub enum device_node {}

unsafe extern "C" {
    pub fn spmi_find_device_by_of_node(np: *mut device_node) -> *mut spmi_device;
    pub fn spmi_register_read(sdev: *mut spmi_device, addr: u8, buf: *mut u8) -> i32;
    pub fn spmi_ext_register_read(sdev: *mut spmi_device, addr: u8, buf: *mut u8, len: usize) -> i32;
    pub fn spmi_ext_register_readl(sdev: *mut spmi_device, addr: u16, buf: *mut u8, len: usize) -> i32;
    pub fn spmi_register_write(sdev: *mut spmi_device, addr: u8, data: u8) -> i32;
    pub fn spmi_register_zero_write(sdev: *mut spmi_device, data: u8) -> i32;
    pub fn spmi_ext_register_write(sdev: *mut spmi_device, addr: u8, buf: *const u8, len: usize) -> i32;
    pub fn spmi_ext_register_writel(sdev: *mut spmi_device, addr: u16, buf: *const u8, len: usize) -> i32;
    pub fn spmi_command_reset(sdev: *mut spmi_device) -> i32;
    pub fn spmi_command_sleep(sdev: *mut spmi_device) -> i32;
    pub fn spmi_command_wakeup(sdev: *mut spmi_device) -> i32;
    pub fn spmi_command_shutdown(sdev: *mut spmi_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
