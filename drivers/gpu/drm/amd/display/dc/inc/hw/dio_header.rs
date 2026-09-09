// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// #include "dc_types.h"

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dio_funcs {
    pub mem_pwr_ctrl: Option<unsafe extern "C" fn(dio: *mut dio, enable_i2c_light_sleep: bool)>,
}

#[repr(C)]
pub struct dio {
    pub funcs: *const dio_funcs,
    pub ctx: *mut dc_context,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
