/* SPDX-License-Identifier: GPL-2.0 */

// Translated from corgi_lcd.h.

pub const CORGI_LCD_MODE_QVGA: ::core::ffi::c_int = 1;
pub const CORGI_LCD_MODE_VGA: ::core::ffi::c_int = 2;

#[repr(C)]
pub struct corgi_lcd_platform_data {
    pub init_mode: ::core::ffi::c_int,
    pub max_intensity: ::core::ffi::c_int,
    pub default_intensity: ::core::ffi::c_int,
    pub limit_mask: ::core::ffi::c_int,

    pub notify: ::core::option::Option<unsafe extern "C" fn(intensity: ::core::ffi::c_int)>,
    pub kick_battery: ::core::option::Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    pub fn corgi_lcd_limit_intensity(limit: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
