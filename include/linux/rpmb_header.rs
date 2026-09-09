/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2015-2019 Intel Corp. All rights reserved
 * Copyright (C) 2021-2022 Linaro Ltd
 */

// Dependencies supplied by the surrounding Linux compatibility layer:
// `device`, `list_head`, `class_interface`, `u8`, `u16`, `__be16`, and `__be32`.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rpmb_type {
    RPMB_TYPE_EMMC,
    RPMB_TYPE_UFS,
    RPMB_TYPE_NVME,
}

#[repr(C)]
pub struct rpmb_descr {
    pub type_: rpmb_type,
    pub route_frames: Option<unsafe extern "C" fn(
        dev: *mut device,
        req: *mut u8,
        req_len: libc::c_uint,
        resp: *mut u8,
        resp_len: libc::c_uint,
    ) -> libc::c_int>,
    pub dev_id: *mut u8,
    pub dev_id_len: libc::size_t,
    pub reliable_wr_count: u16,
    pub capacity: u16,
}

#[repr(C)]
pub struct rpmb_dev {
    pub dev: device,
    pub id: libc::c_int,
    pub list_node: list_head,
    pub descr: rpmb_descr,
}

// C macro: container_of((x), struct rpmb_dev, dev).
// The surrounding compatibility layer must provide the corresponding
// container-of operation when this interface is used.

#[repr(C)]
pub struct rpmb_frame {
    pub stuff: [u8; 196],
    pub key_mac: [u8; 32],
    pub data: [u8; 256],
    pub nonce: [u8; 16],
    pub write_counter: __be32,
    pub addr: __be16,
    pub block_count: __be16,
    pub result: __be16,
    pub req_resp: __be16,
}

pub const RPMB_PROGRAM_KEY: u32 = 0x1;
pub const RPMB_GET_WRITE_COUNTER: u32 = 0x2;
pub const RPMB_WRITE_DATA: u32 = 0x3;
pub const RPMB_READ_DATA: u32 = 0x4;
pub const RPMB_RESULT_READ: u32 = 0x5;

#[cfg(feature = "CONFIG_RPMB")]
extern "C" {
    pub fn rpmb_dev_get(rdev: *mut rpmb_dev) -> *mut rpmb_dev;
    pub fn rpmb_dev_put(rdev: *mut rpmb_dev);
    pub fn rpmb_dev_find_device(
        data: *const libc::c_void,
        start: *const rpmb_dev,
        r#match: Option<unsafe extern "C" fn(*mut device, *const libc::c_void) -> libc::c_int>,
    ) -> *mut rpmb_dev;
    pub fn rpmb_interface_register(intf: *mut class_interface) -> libc::c_int;
    pub fn rpmb_interface_unregister(intf: *mut class_interface);
    pub fn rpmb_dev_register(dev: *mut device, descr: *mut rpmb_descr) -> *mut rpmb_dev;
    pub fn rpmb_dev_unregister(rdev: *mut rpmb_dev) -> libc::c_int;
    pub fn rpmb_route_frames(
        rdev: *mut rpmb_dev,
        req: *mut u8,
        req_len: libc::c_uint,
        resp: *mut u8,
        resp_len: libc::c_uint,
    ) -> libc::c_int;
}

#[cfg(not(feature = "CONFIG_RPMB"))]
pub unsafe fn rpmb_dev_get(_rdev: *mut rpmb_dev) -> *mut rpmb_dev {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_RPMB"))]
pub unsafe fn rpmb_dev_put(_rdev: *mut rpmb_dev) {}

#[cfg(not(feature = "CONFIG_RPMB"))]
pub unsafe fn rpmb_dev_find_device(
    _data: *const libc::c_void,
    _start: *const rpmb_dev,
    _match: Option<unsafe extern "C" fn(*mut device, *const libc::c_void) -> libc::c_int>,
) -> *mut rpmb_dev {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_RPMB"))]
pub unsafe fn rpmb_interface_register(_intf: *mut class_interface) -> libc::c_int {
    -95
}

#[cfg(not(feature = "CONFIG_RPMB"))]
pub unsafe fn rpmb_interface_unregister(_intf: *mut class_interface) {}

#[cfg(not(feature = "CONFIG_RPMB"))]
pub unsafe fn rpmb_dev_register(_dev: *mut device, _descr: *mut rpmb_descr) -> *mut rpmb_dev {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_RPMB"))]
pub unsafe fn rpmb_dev_unregister(_dev: *mut rpmb_dev) -> libc::c_int {
    0
}

#[cfg(not(feature = "CONFIG_RPMB"))]
pub unsafe fn rpmb_route_frames(
    _rdev: *mut rpmb_dev,
    _req: *mut u8,
    _req_len: libc::c_uint,
    _resp: *mut u8,
    _resp_len: libc::c_uint,
) -> libc::c_int {
    -95
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
