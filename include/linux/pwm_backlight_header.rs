/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generic PWM backlight driver data - see drivers/video/backlight/pwm_bl.c
 */

// Dependency supplied by <linux/backlight.h> and related kernel headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_pwm_backlight_data {
    pub max_brightness: u32,
    pub dft_brightness: u32,
    pub lth_brightness: u32,
    pub pwm_period_ns: u32,
    pub levels: *mut u32,
    pub post_pwm_on_delay: u32,
    pub pwm_off_delay: u32,
    pub init: Option<unsafe extern "C" fn(dev: *mut device) -> i32>,
    pub notify: Option<unsafe extern "C" fn(dev: *mut device, brightness: i32) -> i32>,
    pub notify_after: Option<unsafe extern "C" fn(dev: *mut device, brightness: i32)>,
    pub exit: Option<unsafe extern "C" fn(dev: *mut device)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
