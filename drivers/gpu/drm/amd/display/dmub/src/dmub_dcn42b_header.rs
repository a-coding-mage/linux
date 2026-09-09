/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// C dependency: #include "dmub_dcn42.h"

#[repr(C)]
pub struct dmub_srv {
    _private: [u8; 0],
}

// Supplied by the dmub_dcn42 dependency.
#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dmub_srv_dcn42b_regs_init(
        dmub: *mut dmub_srv,
        ctx: *mut dc_context,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
