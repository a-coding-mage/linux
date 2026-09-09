/* SPDX-License-Identifier: GPL-2.0 */
/* Driver for MMIO-Mapped MDIO devices. Some IPs expose internal PHYs or PCS
 * within the MMIO-mapped area
 *
 * Copyright (C) 2023 Maxime Chevallier <maxime.chevallier@bootlin.com>
 */

/* Dependency supplied by linux/phy.h. */
use core::ffi::c_char;

/* Opaque declarations corresponding to the C forward declarations. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mii_bus {
    _private: [u8; 0],
}

/* MII_BUS_ID_SIZE is supplied by linux/phy.h. */
#[repr(C)]
pub struct mdio_regmap_config {
    pub parent: *mut device,
    pub regmap: *mut regmap,
    pub name: [c_char; MII_BUS_ID_SIZE],
    pub valid_addr: u8,
    pub autoscan: bool,
}

extern "C" {
    pub fn devm_mdio_regmap_register(
        dev: *mut device,
        config: *const mdio_regmap_config,
    ) -> *mut mii_bus;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
