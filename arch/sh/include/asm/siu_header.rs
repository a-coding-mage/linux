/* SPDX-License-Identifier: GPL-2.0
 *
 * platform header for the SIU ASoC driver
 *
 * Copyright (C) 2009-2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

// Forward declaration of the externally defined device type.
pub enum device {}

#[repr(C)]
pub struct siu_platform {
    pub dma_slave_tx_a: ::core::ffi::c_uint,
    pub dma_slave_rx_a: ::core::ffi::c_uint,
    pub dma_slave_tx_b: ::core::ffi::c_uint,
    pub dma_slave_rx_b: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
