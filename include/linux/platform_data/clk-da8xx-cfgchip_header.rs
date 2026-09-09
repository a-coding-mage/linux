// SPDX-License-Identifier: GPL-2.0
/*
 * clk-da8xx-cfgchip - TI DaVinci DA8xx CFGCHIP clock driver
 *
 * Copyright (C) 2018 David Lechner <david@lechnology.com>
 */

// Dependency supplied by the Linux regmap subsystem.
pub struct regmap;

/**
 * da8xx_cfgchip_clk_platform_data
 * @cfgchip: CFGCHIP syscon regmap
 */
#[repr(C)]
pub struct da8xx_cfgchip_clk_platform_data {
    pub cfgchip: *mut regmap,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
