/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2022, Advanced Micro Devices, Inc.
 */

// Dependency: linux/dmaengine.h

/**
 * struct xdma_chan_info - DMA channel information
 *	This information is used to match channel when request dma channel
 * @dir: Channel transfer direction
 */
#[repr(C)]
pub struct xdma_chan_info {
    pub dir: dma_transfer_direction,
}

#[macro_export]
macro_rules! XDMA_FILTER_PARAM {
    ($chan_info:expr) => {
        ($chan_info as *const _ as *mut core::ffi::c_void)
    };
}

pub struct dma_slave_map;

/**
 * struct xdma_platdata - platform specific data for XDMA engine
 * @max_dma_channels: Maximum dma channels in each direction
 */
#[repr(C)]
pub struct xdma_platdata {
    pub max_dma_channels: u32,
    pub device_map_cnt: u32,
    pub device_map: *mut dma_slave_map,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
