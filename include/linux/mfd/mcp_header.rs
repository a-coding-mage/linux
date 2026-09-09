/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/drivers/mfd/mcp.h
 *
 *  Copyright (C) 2001 Russell King, All Rights Reserved.
 */

// Dependency supplied by <linux/device.h>.

use core::ffi::c_void;

pub struct mcp_ops;

#[repr(C)]
pub struct mcp {
    pub owner: *mut module,
    pub ops: *mut mcp_ops,
    pub lock: spinlock_t,
    pub use_count: i32,
    pub sclk_rate: u32,
    pub rw_timeout: u32,
    pub attached_device: device,
}

#[repr(C)]
pub struct mcp_ops {
    pub set_telecom_divisor: Option<unsafe extern "C" fn(*mut mcp, u32)>,
    pub set_audio_divisor: Option<unsafe extern "C" fn(*mut mcp, u32)>,
    pub reg_write: Option<unsafe extern "C" fn(*mut mcp, u32, u32)>,
    pub reg_read: Option<unsafe extern "C" fn(*mut mcp, u32) -> u32>,
    pub enable: Option<unsafe extern "C" fn(*mut mcp)>,
    pub disable: Option<unsafe extern "C" fn(*mut mcp)>,
}

unsafe extern "C" {
    pub fn mcp_set_telecom_divisor(mcp: *mut mcp, divisor: u32);
    pub fn mcp_set_audio_divisor(mcp: *mut mcp, divisor: u32);
    pub fn mcp_reg_write(mcp: *mut mcp, reg: u32, val: u32);
    pub fn mcp_reg_read(mcp: *mut mcp, reg: u32) -> u32;
    pub fn mcp_enable(mcp: *mut mcp);
    pub fn mcp_disable(mcp: *mut mcp);
}

#[inline]
pub unsafe fn mcp_get_sclk_rate(mcp: *mut mcp) -> u32 {
    unsafe { (*mcp).sclk_rate }
}

unsafe extern "C" {
    pub fn mcp_host_alloc(device: *mut device, size: usize) -> *mut mcp;
    pub fn mcp_host_add(mcp: *mut mcp, data: *mut c_void) -> i32;
    pub fn mcp_host_del(mcp: *mut mcp);
    pub fn mcp_host_free(mcp: *mut mcp);
}

#[repr(C)]
pub struct mcp_driver {
    pub drv: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut mcp) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut mcp)>,
}

unsafe extern "C" {
    pub fn mcp_driver_register(driver: *mut mcp_driver) -> i32;
    pub fn mcp_driver_unregister(driver: *mut mcp_driver);
}

#[inline]
pub unsafe fn mcp_get_drvdata(mcp: *mut mcp) -> *mut c_void {
    unsafe { dev_get_drvdata(&mut (*mcp).attached_device) }
}

#[inline]
pub unsafe fn mcp_set_drvdata(mcp: *mut mcp, data: *mut c_void) {
    unsafe { dev_set_drvdata(&mut (*mcp).attached_device, data) }
}

#[inline]
pub unsafe fn mcp_priv(mcp: *mut mcp) -> *mut c_void {
    unsafe { mcp.add(1).cast() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
