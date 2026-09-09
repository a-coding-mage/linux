/*
 * public include for LM8333 keypad driver - same license as driver
 * Copyright (C) 2012 Wolfram Sang, Pengutronix <kernel@pengutronix.de>
 */

#[repr(C)]
pub struct lm8333 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct matrix_keymap_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lm8333_platform_data {
    /* Keymap data */
    pub matrix_data: *const matrix_keymap_data,
    /* Active timeout before enter HALT mode in microseconds */
    pub active_time: core::ffi::c_uint,
    /* Debounce interval in microseconds */
    pub debounce_time: core::ffi::c_uint,
}

extern "C" {
    pub fn lm8333_read8(lm8333: *mut lm8333, cmd: u8) -> core::ffi::c_int;
    pub fn lm8333_write8(lm8333: *mut lm8333, cmd: u8, val: u8) -> core::ffi::c_int;
    pub fn lm8333_read_block(
        lm8333: *mut lm8333,
        cmd: u8,
        len: u8,
        buf: *mut u8,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
