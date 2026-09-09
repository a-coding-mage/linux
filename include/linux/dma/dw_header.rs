/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Driver for the Synopsys DesignWare DMA Controller
 *
 * Copyright (C) 2007 Atmel Corporation
 * Copyright (C) 2010-2011 ST Microelectronics
 * Copyright (C) 2014 Intel Corporation
 */

/* C dependencies: linux/clk.h, linux/device.h, linux/dmaengine.h,
 * and linux/platform_data/dma-dw.h. */

use core::ffi::c_void;

#[repr(C)]
pub struct dw_dma;

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct clk;

#[repr(C)]
pub struct dw_dma_platform_data;

/**
 * struct dw_dma_chip - representation of DesignWare DMA controller hardware
 * @dev:        struct device of the DMA controller
 * @id:         instance ID
 * @irq:        irq line
 * @regs:       memory mapped I/O space
 * @clk:        hclk clock
 * @dw:         struct dw_dma that is filed by dw_dma_probe()
 * @pdata:      pointer to platform data
 */
#[repr(C)]
pub struct dw_dma_chip {
    pub dev: *mut device,
    pub id: core::ffi::c_int,
    pub irq: core::ffi::c_int,
    pub regs: *mut c_void,
    pub clk: *mut clk,
    pub dw: *mut dw_dma,
    pub pdata: *const dw_dma_platform_data,
}

/* Export to the platform drivers. */
#[cfg(feature = "CONFIG_DW_DMAC_CORE")]
extern "C" {
    pub fn dw_dma_probe(chip: *mut dw_dma_chip) -> core::ffi::c_int;
    pub fn dw_dma_remove(chip: *mut dw_dma_chip) -> core::ffi::c_int;
    pub fn idma32_dma_probe(chip: *mut dw_dma_chip) -> core::ffi::c_int;
    pub fn idma32_dma_remove(chip: *mut dw_dma_chip) -> core::ffi::c_int;
}

/* CONFIG_DW_DMAC_CORE is disabled: ENODEV is supplied by the kernel
 * environment, as in the original header. */
#[cfg(not(feature = "CONFIG_DW_DMAC_CORE"))]
#[inline]
pub unsafe fn dw_dma_probe(_chip: *mut dw_dma_chip) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_DW_DMAC_CORE"))]
#[inline]
pub unsafe fn dw_dma_remove(_chip: *mut dw_dma_chip) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_DW_DMAC_CORE"))]
#[inline]
pub unsafe fn idma32_dma_probe(_chip: *mut dw_dma_chip) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_DW_DMAC_CORE"))]
#[inline]
pub unsafe fn idma32_dma_remove(_chip: *mut dw_dma_chip) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
