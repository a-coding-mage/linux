/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ACPI helpers for DMA request / controller
 *
 * Based on of_dma.h
 *
 * Copyright (C) 2013, Intel Corporation
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_dma_spec {
    pub chan_id: core::ffi::c_int,
    pub slave_id: core::ffi::c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct acpi_dma {
    pub dma_controllers: list_head,
    pub dev: *mut device,
    pub acpi_dma_xlate:
        Option<unsafe extern "C" fn(*mut acpi_dma_spec, *mut acpi_dma) -> *mut dma_chan>,
    pub data: *mut core::ffi::c_void,
    pub base_request_line: u16,
    pub end_request_line: u16,
}

/* Used with acpi_dma_simple_xlate() */
#[repr(C)]
pub struct acpi_dma_filter_info {
    pub dma_cap: dma_cap_mask_t,
    pub filter_fn: dma_filter_fn,
}

#[cfg(feature = "CONFIG_DMA_ACPI")]
extern "C" {
    pub fn acpi_dma_controller_register(
        dev: *mut device,
        acpi_dma_xlate: Option<unsafe extern "C" fn(*mut acpi_dma_spec, *mut acpi_dma) -> *mut dma_chan>,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn acpi_dma_controller_free(dev: *mut device) -> core::ffi::c_int;
    pub fn devm_acpi_dma_controller_register(
        dev: *mut device,
        acpi_dma_xlate: Option<unsafe extern "C" fn(*mut acpi_dma_spec, *mut acpi_dma) -> *mut dma_chan>,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn acpi_dma_request_slave_chan_by_index(
        dev: *mut device,
        index: usize,
    ) -> *mut dma_chan;
    pub fn acpi_dma_request_slave_chan_by_name(
        dev: *mut device,
        name: *const core::ffi::c_char,
    ) -> *mut dma_chan;

    pub fn acpi_dma_simple_xlate(
        dma_spec: *mut acpi_dma_spec,
        adma: *mut acpi_dma,
    ) -> *mut dma_chan;
}

#[cfg(not(feature = "CONFIG_DMA_ACPI"))]
pub unsafe fn acpi_dma_controller_register(
    _dev: *mut device,
    _acpi_dma_xlate: Option<unsafe extern "C" fn(*mut acpi_dma_spec, *mut acpi_dma) -> *mut dma_chan>,
    _data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_DMA_ACPI"))]
pub unsafe fn acpi_dma_controller_free(_dev: *mut device) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_DMA_ACPI"))]
pub unsafe fn devm_acpi_dma_controller_register(
    _dev: *mut device,
    _acpi_dma_xlate: Option<unsafe extern "C" fn(*mut acpi_dma_spec, *mut acpi_dma) -> *mut dma_chan>,
    _data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_DMA_ACPI"))]
pub unsafe fn acpi_dma_request_slave_chan_by_index(
    _dev: *mut device,
    _index: usize,
) -> *mut dma_chan {
    (-ENODEV as isize) as *mut dma_chan
}

#[cfg(not(feature = "CONFIG_DMA_ACPI"))]
pub unsafe fn acpi_dma_request_slave_chan_by_name(
    _dev: *mut device,
    _name: *const core::ffi::c_char,
) -> *mut dma_chan {
    (-ENODEV as isize) as *mut dma_chan
}

#[cfg(not(feature = "CONFIG_DMA_ACPI"))]
pub const acpi_dma_simple_xlate: Option<unsafe extern "C" fn(*mut acpi_dma_spec, *mut acpi_dma) -> *mut dma_chan> = None;

pub use acpi_dma_request_slave_chan_by_index as acpi_dma_request_slave_channel;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
