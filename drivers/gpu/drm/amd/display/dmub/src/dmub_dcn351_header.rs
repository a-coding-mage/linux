/* SPDX-License-Identifier: MIT */
/* Copyright 2024 Advanced Micro Devices, Inc. */

// Translated from dmub_dcn351.h.
// Dependency: dmub_dcn35.h

/// Opaque forward declaration of `struct dmub_srv`.
#[repr(C)]
pub struct dmub_srv {
    _private: [u8; 0],
}

/// Opaque type supplied by the DC context dependency.
#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

extern "C" {
    pub fn dmub_srv_dcn351_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
