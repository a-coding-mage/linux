/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * gpio_backlight.h - Simple GPIO-controlled backlight
 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_backlight_platform_data {
    pub dev: *mut device,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
