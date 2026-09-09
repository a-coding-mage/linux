/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  MMP Platform DMA Management
 *
 *  Copyright (c) 2011 Marvell Semiconductors Inc.
 */

// Forward declaration of the externally defined DMA slave map type.
pub enum dma_slave_map {}

#[repr(C)]
pub struct mmp_dma_platdata {
    pub dma_channels: core::ffi::c_int,
    pub nb_requestors: core::ffi::c_int,
    pub slave_map_cnt: core::ffi::c_int,
    pub slave_map: *const dma_slave_map,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
