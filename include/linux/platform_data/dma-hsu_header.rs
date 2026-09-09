/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Driver for the High Speed UART DMA
 *
 * Copyright (C) 2015 Intel Corporation
 */

// Forward declaration of the externally defined device type.
pub enum device {}

#[repr(C)]
pub struct hsu_dma_slave {
    pub dma_dev: *mut device,
    pub chan_id: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
