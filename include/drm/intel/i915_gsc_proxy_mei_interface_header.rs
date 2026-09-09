/* SPDX-License-Identifier: MIT */
/*
 * Copyright (c) 2022-2023 Intel Corporation
 */

// Dependency intent: the original header includes <linux/types.h>.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

/**
 * struct i915_gsc_proxy_component_ops - ops for GSC Proxy services.
 * @owner: Module providing the ops
 * @send: sends a proxy message from GSC FW to ME FW
 * @recv: receives a proxy message for GSC FW from ME FW
 */
#[repr(C)]
pub struct i915_gsc_proxy_component_ops {
    pub owner: *mut module,

    /**
     * @send: Sends a proxy message to ME FW.
     * @dev: device struct corresponding to the mei device
     * @buf: message buffer to send
     * @size: size of the message
     * Return: bytes sent on success, negative errno value on failure
     */
    pub send: Option<unsafe extern "C" fn(dev: *mut device, buf: *const core::ffi::c_void, size: usize) -> i32>,

    /**
     * @recv: Receives a proxy message from ME FW.
     * @dev: device struct corresponding to the mei device
     * @buf: message buffer to contain the received message
     * @size: size of the buffer
     * Return: bytes received on success, negative errno value on failure
     */
    pub recv: Option<unsafe extern "C" fn(dev: *mut device, buf: *mut core::ffi::c_void, size: usize) -> i32>,
}

/**
 * struct i915_gsc_proxy_component - Used for communication between i915 and
 * MEI drivers for GSC proxy services
 * @mei_dev: device that provide the GSC proxy service.
 * @ops: Ops implemented by GSC proxy driver, used by i915 driver.
 */
#[repr(C)]
pub struct i915_gsc_proxy_component {
    pub mei_dev: *mut device,
    pub ops: *const i915_gsc_proxy_component_ops,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
