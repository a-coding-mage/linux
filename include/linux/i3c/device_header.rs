/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Cadence Design Systems Inc.
 *
 * Author: Boris Brezillon <boris.brezillon@bootlin.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not implemented here.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i3c_error_code {
    I3C_ERROR_UNKNOWN = 0,
    I3C_ERROR_M0 = 1,
    I3C_ERROR_M1 = 2,
    I3C_ERROR_M2 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i3c_xfer_mode {
    I3C_HDR_DDR = 0,
    I3C_HDR_TSP = 1,
    I3C_HDR_TSL = 2,
    I3C_SDR = 31,
}

#[repr(C)]
pub union i3c_xfer_data {
    pub r#in: *mut core::ffi::c_void,
    pub out: *const core::ffi::c_void,
}

#[repr(C)]
pub union i3c_xfer_rnw_cmd {
    pub rnw: u8,
    pub cmd: u8,
}

#[repr(C)]
pub struct i3c_xfer {
    pub rnw_cmd: i3c_xfer_rnw_cmd,
    pub len: u16,
    pub actual_len: u16,
    pub data: i3c_xfer_data,
    pub err: i3c_error_code,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i3c_dcr {
    I3C_DCR_GENERIC_DEVICE = 0,
}

#[inline]
pub const fn I3C_PID_MANUF_ID(pid: u64) -> u64 { (pid & (((1u64 << 15) - 1) << 33)) >> 33 }
#[inline]
pub const fn I3C_PID_RND_LOWER_32BITS(pid: u64) -> bool { (pid & (1u64 << 32)) != 0 }
#[inline]
pub const fn I3C_PID_RND_VAL(pid: u64) -> u64 { pid & 0xffff_ffff }
#[inline]
pub const fn I3C_PID_PART_ID(pid: u64) -> u64 { (pid & (0xffffu64 << 16)) >> 16 }
#[inline]
pub const fn I3C_PID_INSTANCE_ID(pid: u64) -> u64 { (pid & (0xfu64 << 12)) >> 12 }
#[inline]
pub const fn I3C_PID_EXTRA_INFO(pid: u64) -> u64 { pid & 0xfff }

#[inline]
pub const fn I3C_BCR_DEVICE_ROLE(bcr: u8) -> u8 { bcr & (0x3 << 6) }
pub const I3C_BCR_I3C_SLAVE: u8 = 0 << 6;
pub const I3C_BCR_I3C_MASTER: u8 = 1 << 6;
pub const I3C_BCR_HDR_CAP: u8 = 1 << 5;
pub const I3C_BCR_BRIDGE: u8 = 1 << 4;
pub const I3C_BCR_OFFLINE_CAP: u8 = 1 << 3;
pub const I3C_BCR_IBI_PAYLOAD: u8 = 1 << 2;
pub const I3C_BCR_IBI_REQ_CAP: u8 = 1 << 1;
pub const I3C_BCR_MAX_DATA_SPEED_LIM: u8 = 1;

#[repr(C)]
pub struct i3c_device_info {
    pub pid: u64,
    pub bcr: u8,
    pub dcr: u8,
    pub static_addr: u8,
    pub dyn_addr: u8,
    pub hdr_cap: u8,
    pub max_read_ds: u8,
    pub max_write_ds: u8,
    pub max_ibi_len: u8,
    pub max_read_turnaround: u32,
    pub max_read_len: u16,
    pub max_write_len: u16,
}

#[repr(C)]
pub struct i3c_device;

pub const I3C_MATCH_MANUF_AND_PART: u32 = I3C_MATCH_MANUF | I3C_MATCH_PART;

// C initialiser macros, represented as direct constructors for the external ID type.
#[inline]
pub const fn I3C_DEVICE(_manufid: u16, _partid: u16, _drvdata: usize) -> i3c_device_id {
    i3c_device_id { match_flags: I3C_MATCH_MANUF_AND_PART, manuf_id: _manufid, part_id: _partid, extra_info: 0, data: _drvdata }
}
#[inline]
pub const fn I3C_DEVICE_EXTRA_INFO(_manufid: u16, _partid: u16, _info: u16, _drvdata: usize) -> i3c_device_id {
    i3c_device_id { match_flags: I3C_MATCH_MANUF_AND_PART | I3C_MATCH_EXTRA_INFO, manuf_id: _manufid, part_id: _partid, extra_info: _info, data: _drvdata }
}
#[inline]
pub const fn I3C_CLASS(_dcr: u8, _drvdata: usize) -> i3c_device_id {
    i3c_device_id { match_flags: I3C_MATCH_DCR, manuf_id: 0, part_id: 0, extra_info: _dcr as u16, data: _drvdata }
}

#[repr(C)]
pub struct i3c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i3c_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut i3c_device)>,
    pub id_table: *const i3c_device_id,
}

unsafe extern "C" {
    pub fn i3cdev_to_dev(i3cdev: *mut i3c_device) -> *mut device;
    pub fn i3c_device_match_id(i3cdev: *mut i3c_device, id_table: *const i3c_device_id) -> *const i3c_device_id;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    pub fn i3c_driver_register_with_owner(drv: *mut i3c_driver, owner: *mut module) -> i32;
    pub fn i3c_driver_unregister(drv: *mut i3c_driver);
    pub fn i3c_device_do_xfers(dev: *mut i3c_device, xfers: *mut i3c_xfer, nxfers: i32, mode: i3c_xfer_mode) -> i32;
    pub fn i3c_device_get_supported_xfer_mode(dev: *mut i3c_device) -> u32;
    pub fn i3c_device_do_setdasa(dev: *mut i3c_device) -> i32;
    pub fn i3c_device_get_info(dev: *const i3c_device, info: *mut i3c_device_info);
    pub fn i3c_device_request_ibi(dev: *mut i3c_device, setup: *const i3c_ibi_setup) -> i32;
    pub fn i3c_device_free_ibi(dev: *mut i3c_device);
    pub fn i3c_device_enable_ibi(dev: *mut i3c_device) -> i32;
    pub fn i3c_device_disable_ibi(dev: *mut i3c_device) -> i32;
}

#[inline]
pub unsafe fn i3cdev_set_drvdata(i3cdev: *mut i3c_device, data: *mut core::ffi::c_void) {
    dev_set_drvdata(i3cdev_to_dev(i3cdev), data);
}

#[inline]
pub unsafe fn i3cdev_get_drvdata(i3cdev: *mut i3c_device) -> *mut core::ffi::c_void {
    dev_get_drvdata(i3cdev_to_dev(i3cdev))
}

#[inline]
pub unsafe fn i3c_driver_register(drv: *mut i3c_driver) -> i32 {
    i3c_driver_register_with_owner(drv, core::ptr::null_mut())
}

#[inline]
pub unsafe fn i3c_device_do_xfers_disabled(_: *mut i3c_device, _: *mut i3c_xfer, _: i32, _: i3c_xfer_mode) -> i32 { -95 }
#[inline]
pub unsafe fn i3c_device_get_supported_xfer_mode_disabled(_: *mut i3c_device) -> u32 { 0 }

// When CONFIG_I3C is disabled, the C header supplies these fallbacks.
pub const I3C_NOT_SUPPORTED: i32 = -95;

#[repr(C)]
pub struct i3c_ibi_payload {
    pub len: core::ffi::c_uint,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct i3c_ibi_setup {
    pub max_payload_len: core::ffi::c_uint,
    pub num_slots: core::ffi::c_uint,
    pub handler: Option<unsafe extern "C" fn(*mut i3c_device, *const i3c_ibi_payload)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
