/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OF helpers for DMA request / controller
 *
 * Based on of_gpio.h
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - http://www.ti.com/
 */

// Dependencies supplied by the surrounding Linux-compatible Rust translation.

use core::ffi::c_void;

pub struct device_node;
pub struct device;
pub struct dma_chan;
pub struct dma_router;
pub struct of_phandle_args;
pub struct list_head;
pub type dma_cap_mask_t = u64;
pub type dma_filter_fn = unsafe extern "C" fn(*mut dma_chan, *mut c_void) -> bool;

#[repr(C)]
pub struct of_dma {
    pub of_dma_controllers: list_head,
    pub of_node: *mut device_node,
    pub of_dma_xlate: Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut dma_chan>,
    pub of_dma_route_allocate:
        Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut c_void>,
    pub dma_router: *mut dma_router,
    pub of_dma_data: *mut c_void,
}

#[repr(C)]
pub struct of_dma_filter_info {
    pub dma_cap: dma_cap_mask_t,
    pub filter_fn: dma_filter_fn,
}

#[cfg(CONFIG_DMA_OF)]
extern "C" {
    pub fn of_dma_controller_register(
        np: *mut device_node,
        of_dma_xlate: Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut dma_chan>,
        data: *mut c_void,
    ) -> i32;
    pub fn of_dma_controller_free(np: *mut device_node);

    pub fn of_dma_router_register(
        np: *mut device_node,
        of_dma_route_allocate:
            Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut c_void>,
        dma_router: *mut dma_router,
    ) -> i32;

    pub fn of_dma_request_slave_channel(np: *mut device_node, name: *const i8) -> *mut dma_chan;
    pub fn of_dma_simple_xlate(
        dma_spec: *mut of_phandle_args,
        ofdma: *mut of_dma,
    ) -> *mut dma_chan;
    pub fn of_dma_xlate_by_chan_id(
        dma_spec: *mut of_phandle_args,
        ofdma: *mut of_dma,
    ) -> *mut dma_chan;
}

#[cfg(CONFIG_DMA_OF)]
unsafe extern "C" fn __of_dma_controller_free(np: *mut c_void) {
    of_dma_controller_free(np as *mut device_node);
}

#[cfg(CONFIG_DMA_OF)]
pub unsafe fn devm_of_dma_controller_register(
    dev: *mut device,
    np: *mut device_node,
    of_dma_xlate: Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut dma_chan>,
    data: *mut c_void,
) -> i32 {
    let ret = of_dma_controller_register(np, of_dma_xlate, data);
    if ret != 0 {
        return ret;
    }

    devm_add_action_or_reset(dev, Some(__of_dma_controller_free), np as *mut c_void)
}

#[cfg(CONFIG_DMA_OF)]
pub use of_dma_controller_free as of_dma_router_free;

#[cfg(not(CONFIG_DMA_OF))]
pub unsafe fn of_dma_controller_register(
    _np: *mut device_node,
    _of_dma_xlate: Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut dma_chan>,
    _data: *mut c_void,
) -> i32 {
    -ENODEV
}

#[cfg(not(CONFIG_DMA_OF))]
pub unsafe fn of_dma_controller_free(_np: *mut device_node) {}

#[cfg(not(CONFIG_DMA_OF))]
pub unsafe fn devm_of_dma_controller_register(
    _dev: *mut device,
    _np: *mut device_node,
    _of_dma_xlate: Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut dma_chan>,
    _data: *mut c_void,
) -> i32 {
    -ENODEV
}

#[cfg(not(CONFIG_DMA_OF))]
pub unsafe fn of_dma_router_register(
    _np: *mut device_node,
    _of_dma_route_allocate:
        Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut c_void>,
    _dma_router: *mut dma_router,
) -> i32 {
    -ENODEV
}

#[cfg(not(CONFIG_DMA_OF))]
pub use of_dma_controller_free as of_dma_router_free;

#[cfg(not(CONFIG_DMA_OF))]
pub unsafe fn of_dma_request_slave_channel(
    _np: *mut device_node,
    _name: *const i8,
) -> *mut dma_chan {
    ERR_PTR(-ENODEV)
}

#[cfg(not(CONFIG_DMA_OF))]
pub unsafe fn of_dma_simple_xlate(
    _dma_spec: *mut of_phandle_args,
    _ofdma: *mut of_dma,
) -> *mut dma_chan {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_DMA_OF))]
pub const of_dma_xlate_by_chan_id: Option<unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut dma_chan> = None;

extern "C" {
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> i32;
    fn ERR_PTR(error: i32) -> *mut dma_chan;
    static ENODEV: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
