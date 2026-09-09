/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Routines for handling backlight control on PowerBooks
 *
 * For now, implementation resides in
 * arch/powerpc/platforms/powermac/backlight.c
 *
 */

/* The declarations below are available only when building the kernel. */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct backlight_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

extern "C" {
    /* For locking instructions, see the implementation file */
    pub static mut pmac_backlight: *mut backlight_device;
    pub static mut pmac_backlight_mutex: mutex;

    pub fn pmac_has_backlight_type(type_: *const c_char) -> c_int;

    pub fn pmac_backlight_key(direction: c_int);

    pub fn pmac_backlight_set_legacy_brightness_pmu(brightness: c_int);
    pub fn pmac_backlight_set_legacy_brightness(brightness: c_int) -> c_int;
    pub fn pmac_backlight_get_legacy_brightness() -> c_int;

    pub fn pmac_backlight_enable();
    pub fn pmac_backlight_disable();
}

#[inline]
pub unsafe fn pmac_backlight_key_up() {
    pmac_backlight_key(0);
}

#[inline]
pub unsafe fn pmac_backlight_key_down() {
    pmac_backlight_key(1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
