// SPDX-License-Identifier: GPL-2.0-only
//
// lv5207lp.h - Sanyo LV5207LP LEDs Driver

/// Opaque declaration corresponding to `struct device`.
pub struct device;

#[repr(C)]
pub struct lv5207lp_platform_data {
    pub dev: *mut device,
    pub max_value: u32,
    pub def_value: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
