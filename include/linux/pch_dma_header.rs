/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010 Intel Corporation
 */

// Dependency intent: declarations from <linux/dmaengine.h> are supplied by
// other translated files.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pch_dma_width {
    PCH_DMA_WIDTH_1_BYTE = 0,
    PCH_DMA_WIDTH_2_BYTES = 1,
    PCH_DMA_WIDTH_4_BYTES = 2,
}

#[repr(C)]
pub struct pch_dma_slave {
    pub dma_dev: *mut device,
    pub chan_id: ::core::ffi::c_uint,
    pub tx_reg: dma_addr_t,
    pub rx_reg: dma_addr_t,
    pub width: pch_dma_width,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
