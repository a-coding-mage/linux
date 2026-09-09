/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

// Dependency supplied by the Linux DMA engine interfaces.

/**
 * struct qdma_queue_info - DMA queue information. This information is used to
 *                            match queue when DMA channel is requested
 * @dir: Channel transfer direction
 */
#[repr(C)]
pub struct qdma_queue_info {
    pub dir: dma_transfer_direction,
}

#[macro_export]
macro_rules! QDMA_FILTER_PARAM {
    ($qinfo:expr) => {
        ($qinfo as *mut core::ffi::c_void)
    };
}

pub struct dma_slave_map;

/**
 * struct qdma_platdata - Platform specific data for QDMA engine
 * @max_mm_channels: Maximum number of MM DMA channels in each direction
 * @device_map: DMA slave map
 * @irq_index: The index of first IRQ
 * @dma_dev: The device pointer for dma operations
 */
#[repr(C)]
pub struct qdma_platdata {
    pub max_mm_channels: u32,
    pub irq_index: u32,
    pub device_map: *mut dma_slave_map,
    pub dma_dev: *mut device,
}

// External Linux type supplied by the device model interfaces.
pub struct device;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
