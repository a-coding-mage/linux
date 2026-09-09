/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019-2021 Intel Corporation
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum time_travel_mode {
    TT_MODE_OFF,
    TT_MODE_BASIC,
    TT_MODE_INFCPU,
    TT_MODE_EXTERNAL,
}

/* CONFIG_UML_TIME_TRAVEL_SUPPORT is a build-time configuration condition. */
#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
extern "C" {
    pub static mut time_travel_mode: time_travel_mode;
    pub static mut time_travel_should_print_bc_msg: ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
pub const time_travel_mode: time_travel_mode = time_travel_mode::TT_MODE_OFF;

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
pub const time_travel_should_print_bc_msg: ::core::ffi::c_int = 0;

extern "C" {
    pub fn _time_travel_print_bc_msg();
}

#[inline]
pub unsafe fn time_travel_print_bc_msg() {
    if time_travel_should_print_bc_msg != 0 {
        _time_travel_print_bc_msg();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
