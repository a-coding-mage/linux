/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/input/ad714x.h
 *
 * AD714x is very flexible, it can be used as buttons, scrollwheel,
 * slider, touchpad at the same time. That depends on the boards.
 * The platform_data for the device's "struct device" holds this
 * information.
 *
 * Copyright 2009-2011 Analog Devices Inc.
 */

pub const STAGE_NUM: usize = 12;
pub const STAGE_CFGREG_NUM: usize = 8;
pub const SYS_CFGREG_NUM: usize = 8;

/* Board information which needs to be initialized in arch/mach... */
#[repr(C)]
pub struct ad714x_slider_plat {
    pub start_stage: std::os::raw::c_int,
    pub end_stage: std::os::raw::c_int,
    pub max_coord: std::os::raw::c_int,
}

#[repr(C)]
pub struct ad714x_wheel_plat {
    pub start_stage: std::os::raw::c_int,
    pub end_stage: std::os::raw::c_int,
    pub max_coord: std::os::raw::c_int,
}

#[repr(C)]
pub struct ad714x_touchpad_plat {
    pub x_start_stage: std::os::raw::c_int,
    pub x_end_stage: std::os::raw::c_int,
    pub x_max_coord: std::os::raw::c_int,

    pub y_start_stage: std::os::raw::c_int,
    pub y_end_stage: std::os::raw::c_int,
    pub y_max_coord: std::os::raw::c_int,
}

#[repr(C)]
pub struct ad714x_button_plat {
    pub keycode: std::os::raw::c_int,
    pub l_mask: u16,
    pub h_mask: u16,
}

#[repr(C)]
pub struct ad714x_platform_data {
    pub slider_num: std::os::raw::c_int,
    pub wheel_num: std::os::raw::c_int,
    pub touchpad_num: std::os::raw::c_int,
    pub button_num: std::os::raw::c_int,
    pub slider: *mut ad714x_slider_plat,
    pub wheel: *mut ad714x_wheel_plat,
    pub touchpad: *mut ad714x_touchpad_plat,
    pub button: *mut ad714x_button_plat,
    pub stage_cfg_reg: [[u16; STAGE_CFGREG_NUM]; STAGE_NUM],
    pub sys_cfg_reg: [u16; SYS_CFGREG_NUM],
    pub irqflags: std::os::raw::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
