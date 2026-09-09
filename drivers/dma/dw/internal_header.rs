/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Driver for the Synopsys DesignWare DMA Controller
 *
 * Copyright (C) 2013 Intel Corporation
 */

// Translated from the C header. Kernel dependencies are supplied externally.

extern "C" {
    pub fn do_dma_probe(chip: *mut dw_dma_chip) -> ::core::ffi::c_int;
    pub fn do_dma_remove(chip: *mut dw_dma_chip) -> ::core::ffi::c_int;

    pub fn do_dw_dma_on(dw: *mut dw_dma);
    pub fn do_dw_dma_off(dw: *mut dw_dma);

    pub fn do_dw_dma_disable(chip: *mut dw_dma_chip) -> ::core::ffi::c_int;
    pub fn do_dw_dma_enable(chip: *mut dw_dma_chip) -> ::core::ffi::c_int;

    pub fn dw_dma_filter(chan: *mut dma_chan, param: *mut ::core::ffi::c_void) -> bool;
}

// CONFIG_ACPI controls whether these are external hooks or empty inline stubs.
#[cfg(feature = "CONFIG_ACPI")]
extern "C" {
    pub fn dw_dma_acpi_controller_register(dw: *mut dw_dma);
    pub fn dw_dma_acpi_controller_free(dw: *mut dw_dma);
}

#[cfg(not(feature = "CONFIG_ACPI"))]
#[inline]
pub unsafe fn dw_dma_acpi_controller_register(_dw: *mut dw_dma) {}

#[cfg(not(feature = "CONFIG_ACPI"))]
#[inline]
pub unsafe fn dw_dma_acpi_controller_free(_dw: *mut dw_dma) {}

pub enum platform_device {}

// CONFIG_OF controls whether device-tree helpers are external hooks or stubs.
#[cfg(feature = "CONFIG_OF")]
extern "C" {
    pub fn dw_dma_parse_dt(pdev: *mut platform_device) -> *mut dw_dma_platform_data;
    pub fn dw_dma_of_controller_register(dw: *mut dw_dma);
    pub fn dw_dma_of_controller_free(dw: *mut dw_dma);
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn dw_dma_parse_dt(_pdev: *mut platform_device) -> *mut dw_dma_platform_data {
    ::core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn dw_dma_of_controller_register(_dw: *mut dw_dma) {}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn dw_dma_of_controller_free(_dw: *mut dw_dma) {}

#[repr(C)]
pub struct dw_dma_chip_pdata {
    pub pdata: *const dw_dma_platform_data,
    pub probe: Option<unsafe extern "C" fn(chip: *mut dw_dma_chip) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(chip: *mut dw_dma_chip) -> ::core::ffi::c_int>,
    pub chip: *mut dw_dma_chip,
    pub m_master: u8,
    pub p_master: u8,
}

extern "C" {
    pub fn dw_dma_probe(chip: *mut dw_dma_chip) -> ::core::ffi::c_int;
    pub fn dw_dma_remove(chip: *mut dw_dma_chip) -> ::core::ffi::c_int;
    pub fn idma32_dma_probe(chip: *mut dw_dma_chip) -> ::core::ffi::c_int;
    pub fn idma32_dma_remove(chip: *mut dw_dma_chip) -> ::core::ffi::c_int;
}

#[allow(non_upper_case_globals)]
pub static dw_dma_chip_pdata: dw_dma_chip_pdata = dw_dma_chip_pdata {
    pdata: ::core::ptr::null(),
    probe: Some(dw_dma_probe),
    remove: Some(dw_dma_remove),
    chip: ::core::ptr::null_mut(),
    m_master: 0,
    p_master: 1,
};

#[allow(non_upper_case_globals)]
pub static idma32_pdata: dw_dma_platform_data = dw_dma_platform_data {
    nr_channels: 8,
    chan_allocation_order: CHAN_ALLOCATION_ASCENDING,
    chan_priority: CHAN_PRIORITY_ASCENDING,
    block_size: 131071,
    nr_masters: 1,
    data_width: [4],
    multi_block: [1, 1, 1, 1, 1, 1, 1, 1],
};

#[allow(non_upper_case_globals)]
pub static idma32_chip_pdata: dw_dma_chip_pdata = dw_dma_chip_pdata {
    pdata: &idma32_pdata,
    probe: Some(idma32_dma_probe),
    remove: Some(idma32_dma_remove),
    chip: ::core::ptr::null_mut(),
    m_master: 0,
    p_master: 0,
};

#[allow(non_upper_case_globals)]
pub static xbar_pdata: dw_dma_platform_data = dw_dma_platform_data {
    nr_channels: 8,
    chan_allocation_order: CHAN_ALLOCATION_ASCENDING,
    chan_priority: CHAN_PRIORITY_ASCENDING,
    block_size: 131071,
    nr_masters: 1,
    data_width: [4],
    quirks: DW_DMA_QUIRK_XBAR_PRESENT,
};

#[allow(non_upper_case_globals)]
pub static xbar_chip_pdata: dw_dma_chip_pdata = dw_dma_chip_pdata {
    pdata: &xbar_pdata,
    probe: Some(idma32_dma_probe),
    remove: Some(idma32_dma_remove),
    chip: ::core::ptr::null_mut(),
    m_master: 0,
    p_master: 0,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
