/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Remote processor messaging
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 * Copyright (C) 2011 Google, Inc.
 * All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not defined here.

pub struct rpmsg_device;
pub struct rpmsg_endpoint;
pub struct rpmsg_device_ops;
pub struct rpmsg_endpoint_ops;

#[repr(C)]
pub struct rpmsg_channel_info {
    pub name: [core::ffi::c_char; RPMSG_NAME_SIZE as usize],
    pub src: u32,
    pub dst: u32,
}

#[repr(C)]
pub struct rpmsg_device {
    pub dev: device,
    pub id: rpmsg_device_id,
    pub src: u32,
    pub dst: u32,
    pub ept: *mut rpmsg_endpoint,
    pub announce: bool,
    pub little_endian: bool,
    pub ops: *const rpmsg_device_ops,
}

pub type rpmsg_rx_cb_t = Option<unsafe extern "C" fn(*mut rpmsg_device, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32) -> i32>;
pub type rpmsg_flowcontrol_cb_t = Option<unsafe extern "C" fn(*mut rpmsg_device, *mut core::ffi::c_void, bool) -> i32>;

#[repr(C)]
pub struct rpmsg_endpoint {
    pub rpdev: *mut rpmsg_device,
    pub refcount: kref,
    pub cb: rpmsg_rx_cb_t,
    pub flow_cb: rpmsg_flowcontrol_cb_t,
    pub cb_lock: mutex,
    pub addr: u32,
    pub priv_: *mut core::ffi::c_void,
    pub ops: *const rpmsg_endpoint_ops,
}

#[repr(C)]
pub struct rpmsg_driver {
    pub drv: device_driver,
    pub id_table: *const rpmsg_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut rpmsg_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut rpmsg_device)>,
    pub callback: Option<unsafe extern "C" fn(*mut rpmsg_device, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32) -> i32>,
    pub flowcontrol: Option<unsafe extern "C" fn(*mut rpmsg_device, *mut core::ffi::c_void, bool) -> i32>,
}

#[inline]
pub unsafe fn rpmsg16_to_cpu(rpdev: *mut rpmsg_device, val: __rpmsg16) -> u16 {
    if rpdev.is_null() {
        __rpmsg16_to_cpu(rpmsg_is_little_endian(), val)
    } else {
        __rpmsg16_to_cpu((*rpdev).little_endian, val)
    }
}

#[inline]
pub unsafe fn cpu_to_rpmsg16(rpdev: *mut rpmsg_device, val: u16) -> __rpmsg16 {
    if rpdev.is_null() {
        __cpu_to_rpmsg16(rpmsg_is_little_endian(), val)
    } else {
        __cpu_to_rpmsg16((*rpdev).little_endian, val)
    }
}

#[inline]
pub unsafe fn rpmsg32_to_cpu(rpdev: *mut rpmsg_device, val: __rpmsg32) -> u32 {
    if rpdev.is_null() {
        __rpmsg32_to_cpu(rpmsg_is_little_endian(), val)
    } else {
        __rpmsg32_to_cpu((*rpdev).little_endian, val)
    }
}

#[inline]
pub unsafe fn cpu_to_rpmsg32(rpdev: *mut rpmsg_device, val: u32) -> __rpmsg32 {
    if rpdev.is_null() {
        __cpu_to_rpmsg32(rpmsg_is_little_endian(), val)
    } else {
        __cpu_to_rpmsg32((*rpdev).little_endian, val)
    }
}

#[inline]
pub unsafe fn rpmsg64_to_cpu(rpdev: *mut rpmsg_device, val: __rpmsg64) -> u64 {
    if rpdev.is_null() {
        __rpmsg64_to_cpu(rpmsg_is_little_endian(), val)
    } else {
        __rpmsg64_to_cpu((*rpdev).little_endian, val)
    }
}

#[inline]
pub unsafe fn cpu_to_rpmsg64(rpdev: *mut rpmsg_device, val: u64) -> __rpmsg64 {
    if rpdev.is_null() {
        __cpu_to_rpmsg64(rpmsg_is_little_endian(), val)
    } else {
        __cpu_to_rpmsg64((*rpdev).little_endian, val)
    }
}

// The C header selects these declarations when CONFIG_RPMSG is enabled.
#[cfg(feature = "CONFIG_RPMSG")]
extern "C" {
    pub fn rpmsg_register_device_override(rpdev: *mut rpmsg_device, driver_override: *const core::ffi::c_char) -> i32;
    pub fn rpmsg_register_device(rpdev: *mut rpmsg_device) -> i32;
    pub fn rpmsg_unregister_device(parent: *mut device, chinfo: *mut rpmsg_channel_info) -> i32;
    pub fn __register_rpmsg_driver(drv: *mut rpmsg_driver, owner: *mut module) -> i32;
    pub fn unregister_rpmsg_driver(drv: *mut rpmsg_driver);
    pub fn rpmsg_destroy_ept(ept: *mut rpmsg_endpoint);
    pub fn rpmsg_create_ept(rpdev: *mut rpmsg_device, cb: rpmsg_rx_cb_t, priv_: *mut core::ffi::c_void, chinfo: rpmsg_channel_info) -> *mut rpmsg_endpoint;
    pub fn rpmsg_send(ept: *mut rpmsg_endpoint, data: *const core::ffi::c_void, len: i32) -> i32;
    pub fn rpmsg_sendto(ept: *mut rpmsg_endpoint, data: *const core::ffi::c_void, len: i32, dst: u32) -> i32;
    pub fn rpmsg_trysend(ept: *mut rpmsg_endpoint, data: *const core::ffi::c_void, len: i32) -> i32;
    pub fn rpmsg_trysendto(ept: *mut rpmsg_endpoint, data: *const core::ffi::c_void, len: i32, dst: u32) -> i32;
    pub fn rpmsg_poll(ept: *mut rpmsg_endpoint, filp: *mut file, wait: *mut poll_table) -> __poll_t;
    pub fn rpmsg_get_mtu(ept: *mut rpmsg_endpoint) -> isize;
    pub fn rpmsg_set_flow_control(ept: *mut rpmsg_endpoint, pause: bool, dst: u32) -> i32;
}

// When CONFIG_RPMSG is disabled, the C inline functions warn and return -ENXIO
// (or NULL for rpmsg_create_ept); these stubs preserve that interface.
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_register_device_override(_: *mut rpmsg_device, _: *const core::ffi::c_char) -> i32 { -ENXIO }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_register_device(_: *mut rpmsg_device) -> i32 { -ENXIO }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_unregister_device(_: *mut device, _: *mut rpmsg_channel_info) -> i32 { WARN_ON(1); -ENXIO }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn __register_rpmsg_driver(_: *mut rpmsg_driver, _: *mut module) -> i32 { WARN_ON(1); -ENXIO }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn unregister_rpmsg_driver(_: *mut rpmsg_driver) { WARN_ON(1); }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_destroy_ept(_: *mut rpmsg_endpoint) { WARN_ON(1); }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_create_ept(_: *mut rpmsg_device, _: rpmsg_rx_cb_t, _: *mut core::ffi::c_void, _: rpmsg_channel_info) -> *mut rpmsg_endpoint { WARN_ON(1); core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_send(_: *mut rpmsg_endpoint, _: *const core::ffi::c_void, _: i32) -> i32 { WARN_ON(1); -ENXIO }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_sendto(_: *mut rpmsg_endpoint, _: *const core::ffi::c_void, _: i32, _: u32) -> i32 { WARN_ON(1); -ENXIO }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_trysend(_: *mut rpmsg_endpoint, _: *const core::ffi::c_void, _: i32) -> i32 { WARN_ON(1); -ENXIO }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_trysendto(_: *mut rpmsg_endpoint, _: *const core::ffi::c_void, _: i32, _: u32) -> i32 { WARN_ON(1); -ENXIO }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_poll(_: *mut rpmsg_endpoint, _: *mut file, _: *mut poll_table) -> __poll_t { WARN_ON(1); 0 }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_get_mtu(_: *mut rpmsg_endpoint) -> isize { WARN_ON(1); -ENXIO as isize }
#[cfg(not(feature = "CONFIG_RPMSG"))]
pub unsafe fn rpmsg_set_flow_control(_: *mut rpmsg_endpoint, _: bool, _: u32) -> i32 { WARN_ON(1); -ENXIO }

// register_rpmsg_driver(drv) expands to __register_rpmsg_driver(drv, THIS_MODULE).
// module_rpmsg_driver(...) expands to module_driver(..., register_rpmsg_driver,
// unregister_rpmsg_driver).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
