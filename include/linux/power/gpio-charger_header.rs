/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 2010, Lars-Peter Clausen <lars@metafoo.de>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/power_supply.h, linux/types.h

/**
 * struct gpio_charger_platform_data - platform_data for gpio_charger devices
 * @name:               Name for the chargers power_supply device
 * @type:               Type of the charger
 * @supplied_to:        Array of battery names to which this chargers supplies power
 * @num_supplicants:    Number of entries in the supplied_to array
 */
#[repr(C)]
pub struct gpio_charger_platform_data {
    pub name: *const core::ffi::c_char,
    pub r#type: power_supply_type,
    pub supplied_to: *mut *mut core::ffi::c_char,
    pub num_supplicants: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
