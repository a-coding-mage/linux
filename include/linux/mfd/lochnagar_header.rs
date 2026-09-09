/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Lochnagar internals
 *
 * Copyright (c) 2013-2018 Cirrus Logic, Inc. and
 *                         Cirrus Logic International Semiconductor Ltd.
 *
 * Author: Charles Keepax <ckeepax@opensource.cirrus.com>
 */

// Dependencies supplied by the surrounding kernel bindings.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lochnagar_type {
    LOCHNAGAR1,
    LOCHNAGAR2,
}

/**
 * struct lochnagar - Core data for the Lochnagar audio board driver.
 *
 * @type: The type of Lochnagar device connected.
 * @dev: A pointer to the struct device for the main MFD.
 * @regmap: The devices main register map.
 * @analogue_config_lock: Lock used to protect updates in the analogue
 * configuration as these must not be changed whilst the hardware is processing
 * the last update.
 */
#[repr(C)]
pub struct lochnagar {
    pub r#type: lochnagar_type,
    pub dev: *mut device,
    pub regmap: *mut regmap,

    /* Lock to protect updates to the analogue configuration */
    pub analogue_config_lock: mutex,
}

/* Register Addresses */
pub const LOCHNAGAR_SOFTWARE_RESET: u32 = 0x00;
pub const LOCHNAGAR_FIRMWARE_ID1: u32 = 0x01;
pub const LOCHNAGAR_FIRMWARE_ID2: u32 = 0x02;

/* (0x0000)  Software Reset */
pub const LOCHNAGAR_DEVICE_ID_MASK: u32 = 0xFFFC;
pub const LOCHNAGAR_DEVICE_ID_SHIFT: u32 = 2;
pub const LOCHNAGAR_REV_ID_MASK: u32 = 0x0003;
pub const LOCHNAGAR_REV_ID_SHIFT: u32 = 0;

unsafe extern "C" {
    pub fn lochnagar_update_config(lochnagar: *mut lochnagar) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
