/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Platform data for Cirrus Logic Madera codecs
 *
 * Copyright (C) 2015-2018 Cirrus Logic
 */

// C dependencies:
// linux/regulator/arizona-ldo1.h
// linux/regulator/arizona-micsupp.h
// linux/regulator/machine.h
// linux/types.h
// sound/madera-pdata.h

pub const MADERA_MAX_MICBIAS: usize = 4;
pub const MADERA_MAX_CHILD_MICBIAS: usize = 4;
pub const MADERA_MAX_GPSW: usize = 2;

// Forward declarations from the C header.
pub struct gpio_desc;
pub struct pinctrl_map;

/// Configuration data for Madera devices.
#[repr(C)]
pub struct madera_pdata {
    /// GPIO controlling /RESET (NULL = none).
    pub reset: *mut gpio_desc,

    /// Substruct of pdata for the LDO1 regulator.
    pub ldo1: arizona_ldo1_pdata,
    /// Substruct of pdata for the MICVDD regulator.
    pub micvdd: arizona_micsupp_pdata,

    /// Mode for primary IRQ (defaults to active low).
    pub irq_flags: u32,
    /// Base GPIO number.
    pub gpio_base: i32,

    /// Array of GPIO configurations (see Documentation/driver-api/pin-control.rst).
    pub gpio_configs: *const pinctrl_map,
    /// Number of entries in gpio_configs.
    pub n_gpio_configs: i32,

    /// General purpose switch mode setting.
    pub gpsw: [u32; MADERA_MAX_GPSW],

    /// Substruct of pdata for the ASoC codec driver.
    pub codec: madera_codec_pdata,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
