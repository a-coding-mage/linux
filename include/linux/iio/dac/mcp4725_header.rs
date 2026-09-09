/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * MCP4725 DAC driver
 *
 * Copyright (C) 2012 Peter Meerwald <pmeerw@pmeerw.net>
 */

/**
 * struct mcp4725_platform_data - MCP4725/6 DAC specific data.
 * @use_vref: Whether an external reference voltage on Vref pin should be used.
 *            Additional vref-supply must be specified when used.
 * @vref_buffered: Controls buffering of the external reference voltage.
 *
 * Vref related settings are available only on MCP4756. See
 * Documentation/devicetree/bindings/iio/dac/microchip,mcp4725.yaml for more information.
 */
#[repr(C)]
pub struct mcp4725_platform_data {
    pub use_vref: bool,
    pub vref_buffered: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
