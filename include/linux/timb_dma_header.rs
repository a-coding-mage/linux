/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * timb_dma.h timberdale FPGA DMA driver defines
 * Copyright (c) 2010 Intel Corporation
 */

/* Supports:
 * Timberdale FPGA DMA engine
 */

/// Description of each individual DMA channel for the timberdale DMA driver.
///
/// `rx`: true if this channel handles data in the direction to the CPU.
/// `bytes_per_line`: Number of bytes per line, specific for channels handling
/// video data. For other channels this shall be left to 0.
/// `descriptors`: Number of descriptors to allocate for this channel.
/// `descriptor_elements`: Number of elements in each descriptor.
#[repr(C)]
pub struct timb_dma_platform_data_channel {
    pub rx: bool,
    pub bytes_per_line: u32,
    pub descriptors: u32,
    pub descriptor_elements: u32,
}

/// Platform data of the timberdale DMA driver.
///
/// `nr_channels`: Number of defined channels in the channels array.
/// `channels`: Definition of each channel.
#[repr(C)]
pub struct timb_dma_platform_data {
    pub nr_channels: u32,
    pub channels: [timb_dma_platform_data_channel; 32],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
