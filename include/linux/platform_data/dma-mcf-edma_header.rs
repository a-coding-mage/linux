/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Freescale eDMA platform data, ColdFire SoC's family.
 *
 * Copyright (c) 2017 Angelo Dureghello <angelo@sysam.it>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct dma_slave_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

extern "C" {
    pub fn mcf_edma_filter_fn(chan: *mut dma_chan, param: *mut c_void) -> bool;
}

pub unsafe fn MCF_EDMA_FILTER_PARAM(ch: *mut dma_chan) -> *mut c_void {
    ch as *mut c_void
}

/**
 * struct mcf_edma_platform_data - platform specific data for eDMA engine
 *
 * @dma_channels:    The number of eDMA channels.
 * @slave_map:       Slave device map
 * @slavecnt:        Number of entries in @slave_map
 */
#[repr(C)]
pub struct mcf_edma_platform_data {
    pub dma_channels: i32,
    pub slave_map: *const dma_slave_map,
    pub slavecnt: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
