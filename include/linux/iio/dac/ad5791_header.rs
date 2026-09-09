/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AD5791 SPI DAC driver
 *
 * Copyright 2011 Analog Devices Inc.
 */

// Translated from the C header; the original SPI_AD5791_H_ header guard is
// omitted because Rust items are scoped by their containing module.

/**
 * struct ad5791_platform_data - platform specific information
 * @vref_pos_mv: Vdd Positive Analog Supply Volatge (mV)
 * @vref_neg_mv: Vdd Negative Analog Supply Volatge (mV)
 * @use_rbuf_gain2: ext. amplifier connected in gain of two configuration
 */
#[repr(C)]
pub struct ad5791_platform_data {
    pub vref_pos_mv: u16,
    pub vref_neg_mv: u16,
    pub use_rbuf_gain2: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
