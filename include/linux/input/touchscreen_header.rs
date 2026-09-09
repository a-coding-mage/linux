/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 Sebastian Reichel <sre@kernel.org>
 */

// Forward declarations from the corresponding input subsystem headers.
#[repr(C)]
pub struct input_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct input_mt_pos {
    _private: [u8; 0],
}

#[repr(C)]
pub struct touchscreen_properties {
    pub max_x: ::core::ffi::c_uint,
    pub max_y: ::core::ffi::c_uint,
    pub invert_x: bool,
    pub invert_y: bool,
    pub swap_x_y: bool,
}

extern "C" {
    pub fn touchscreen_parse_properties(
        input: *mut input_dev,
        multitouch: bool,
        prop: *mut touchscreen_properties,
    );

    pub fn touchscreen_set_mt_pos(
        pos: *mut input_mt_pos,
        prop: *const touchscreen_properties,
        x: ::core::ffi::c_uint,
        y: ::core::ffi::c_uint,
    );

    pub fn touchscreen_report_pos(
        input: *mut input_dev,
        prop: *const touchscreen_properties,
        x: ::core::ffi::c_uint,
        y: ::core::ffi::c_uint,
        multitouch: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
