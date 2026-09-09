/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// C forward declaration: struct dc;
#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

extern "C" {
    pub fn dcn42_hw_sequencer_init_functions(dc: *mut dc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
