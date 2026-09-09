/* SPDX-License-Identifier: MIT */
/* Copyright 2025 Advanced Micro Devices, Inc. */

// Dependency equivalent of: #include "dmub_dcn35.h"

#[repr(C)]
pub struct dmub_srv {
    _private: [u8; 0],
}

// Declared by the included dependency.
#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dmub_srv_dcn36_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
