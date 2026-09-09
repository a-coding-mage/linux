/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * COMEDI ISA DMA support functions
 * Copyright (c) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
 */

// Dependency supplied by the surrounding translation unit:
// linux/types.h

use core::ffi::c_void;

pub struct comedi_device;
pub struct device;

/*
 * These are used to avoid issues when <asm/dma.h> and the DMA_MODE_
 * defines are not available.
 */
pub const COMEDI_ISADMA_READ: i32 = 0;
pub const COMEDI_ISADMA_WRITE: i32 = 1;

pub type dma_addr_t = usize;

/**
 * struct comedi_isadma_desc - cookie for ISA DMA
 * @virt_addr: virtual address of buffer
 * @hw_addr: hardware (bus) address of buffer
 * @chan: DMA channel
 * @maxsize: allocated size of buffer (in bytes)
 * @size: transfer size (in bytes)
 * @mode: DMA_MODE_READ or DMA_MODE_WRITE
 */
#[repr(C)]
pub struct comedi_isadma_desc {
    pub virt_addr: *mut c_void,
    pub hw_addr: dma_addr_t,
    pub chan: u32,
    pub maxsize: u32,
    pub size: u32,
    pub mode: i8,
}

/**
 * struct comedi_isadma - ISA DMA data
 * @dev: device to allocate non-coherent memory for
 * @desc: cookie for each DMA buffer
 * @n_desc: the number of cookies
 * @cur_dma: the current cookie in use
 * @chan: the first DMA channel requested
 * @chan2: the second DMA channel requested
 */
#[repr(C)]
pub struct comedi_isadma {
    pub dev: *mut device,
    pub n_desc: i32,
    pub cur_dma: i32,
    pub chan: u32,
    pub chan2: u32,
    // Flexible array member: __counted_by(n_desc).
    pub desc: [comedi_isadma_desc; 0],
}

// IS_ENABLED(CONFIG_ISA_DMA_API) is represented by the cfg feature below.
#[cfg(feature = "CONFIG_ISA_DMA_API")]
unsafe extern "C" {
    pub fn comedi_isadma_program(desc: *mut comedi_isadma_desc);
    pub fn comedi_isadma_disable(dma_chan: u32) -> u32;
    pub fn comedi_isadma_disable_on_sample(dma_chan: u32, size: u32) -> u32;
    pub fn comedi_isadma_poll(dma: *mut comedi_isadma) -> u32;
    pub fn comedi_isadma_set_mode(desc: *mut comedi_isadma_desc, dma_dir: i8);

    pub fn comedi_isadma_alloc(
        dev: *mut comedi_device,
        n_desc: i32,
        dma_chan1: u32,
        dma_chan2: u32,
        maxsize: u32,
        dma_dir: i8,
    ) -> *mut comedi_isadma;
    pub fn comedi_isadma_free(dma: *mut comedi_isadma);
}

#[cfg(not(feature = "CONFIG_ISA_DMA_API"))]
#[inline]
pub unsafe fn comedi_isadma_program(_desc: *mut comedi_isadma_desc) {}

#[cfg(not(feature = "CONFIG_ISA_DMA_API"))]
#[inline]
pub unsafe fn comedi_isadma_disable(_dma_chan: u32) -> u32 { 0 }

#[cfg(not(feature = "CONFIG_ISA_DMA_API"))]
#[inline]
pub unsafe fn comedi_isadma_disable_on_sample(_dma_chan: u32, _size: u32) -> u32 { 0 }

#[cfg(not(feature = "CONFIG_ISA_DMA_API"))]
#[inline]
pub unsafe fn comedi_isadma_poll(_dma: *mut comedi_isadma) -> u32 { 0 }

#[cfg(not(feature = "CONFIG_ISA_DMA_API"))]
#[inline]
pub unsafe fn comedi_isadma_set_mode(_desc: *mut comedi_isadma_desc, _dma_dir: i8) {}

#[cfg(not(feature = "CONFIG_ISA_DMA_API"))]
#[inline]
pub unsafe fn comedi_isadma_alloc(
    _dev: *mut comedi_device,
    _n_desc: i32,
    _dma_chan1: u32,
    _dma_chan2: u32,
    _maxsize: u32,
    _dma_dir: i8,
) -> *mut comedi_isadma {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_ISA_DMA_API"))]
#[inline]
pub unsafe fn comedi_isadma_free(_dma: *mut comedi_isadma) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
