/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * linux/spear_dma.h
 *
 * Copyright (ST) 2012 Rajeev Kumar (rajeevkumar.linux@gmail.com)
 */

// Dependency supplied by the Linux DMA engine interfaces:
// #include <linux/dmaengine.h>

#[repr(C)]
pub struct spear_dma_data {
    pub data: *mut core::ffi::c_void,
    pub addr: dma_addr_t,
    pub max_burst: u32,
    pub addr_width: dma_slave_buswidth,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
