// SPDX-License-Identifier: GPL-2.0-only
/*
 * Shared helpers to register GPIO-connected buttons and LEDs
 * on AMD Geode boards.
 */

// Dependency intent: corresponds to the Linux property API include.

#[repr(C)]
pub struct geode_led {
    pub pin: u32,
    pub default_on: bool,
}

unsafe extern "C" {
    pub fn geode_create_restart_key(pin: u32) -> i32;
    pub fn geode_create_leds(
        label: *const core::ffi::c_char,
        leds: *const geode_led,
        n_leds: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
