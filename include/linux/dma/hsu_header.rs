/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Driver for the High Speed UART DMA
 *
 * Copyright (C) 2015 Intel Corporation
 */

// Translated from hsu.h. C includes and forward declarations are represented
// by the external Rust types and symbols referenced below.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hsu_dma {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hsu_dma_chip {
    /* struct device of the DMA controller */
    pub dev: *mut device,
    /* irq line */
    pub irq: ::core::ffi::c_int,
    /* memory mapped I/O space */
    pub regs: *mut ::core::ffi::c_void,
    /* I/O space length */
    pub length: u32,
    /* offset of the I/O space where registers are located */
    pub offset: u32,
    /* struct hsu_dma that is filed by ->probe() */
    pub hsu: *mut hsu_dma,
}

#[cfg(feature = "CONFIG_HSU_DMA")]
extern "C" {
    pub fn hsu_dma_get_status(
        chip: *mut hsu_dma_chip,
        nr: u16,
        status: *mut u32,
    ) -> ::core::ffi::c_int;
    pub fn hsu_dma_do_irq(
        chip: *mut hsu_dma_chip,
        nr: u16,
        status: u32,
    ) -> ::core::ffi::c_int;
    pub fn hsu_dma_probe(chip: *mut hsu_dma_chip) -> ::core::ffi::c_int;
    pub fn hsu_dma_remove(chip: *mut hsu_dma_chip) -> ::core::ffi::c_int;
}

// When CONFIG_HSU_DMA is disabled, the C header supplies these inline
// fallbacks instead of the external functions above.
#[cfg(not(feature = "CONFIG_HSU_DMA"))]
#[inline]
pub unsafe fn hsu_dma_get_status(
    _chip: *mut hsu_dma_chip,
    _nr: u16,
    _status: *mut u32,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_HSU_DMA"))]
#[inline]
pub unsafe fn hsu_dma_do_irq(
    _chip: *mut hsu_dma_chip,
    _nr: u16,
    _status: u32,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_HSU_DMA"))]
#[inline]
pub unsafe fn hsu_dma_probe(_chip: *mut hsu_dma_chip) -> ::core::ffi::c_int {
    -19 /* -ENODEV */
}

#[cfg(not(feature = "CONFIG_HSU_DMA"))]
#[inline]
pub unsafe fn hsu_dma_remove(_chip: *mut hsu_dma_chip) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
