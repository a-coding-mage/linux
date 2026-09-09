/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2012 ARM Limited
 */

// C header guard: _LINUX_VEXPRESS_H

// Dependencies supplied by other translated files:
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

/* Config regmap API */

unsafe extern "C" {
    pub fn devm_regmap_init_vexpress_config(dev: *mut device) -> *mut regmap;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
