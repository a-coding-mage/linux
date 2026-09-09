// SPDX-License-Identifier: GPL-2.0
/*
 * phy-da8xx-usb - TI DaVinci DA8xx USB PHY driver
 *
 * Copyright (C) 2018 David Lechner <david@lechnology.com>
 */

// Dependency corresponding to <linux/regmap.h>.
use crate::regmap::regmap;

/**
 * da8xx_usb_phy_platform_data
 * @cfgchip: CFGCHIP syscon regmap
 */
#[repr(C)]
pub struct da8xx_usb_phy_platform_data {
    pub cfgchip: *mut regmap,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
