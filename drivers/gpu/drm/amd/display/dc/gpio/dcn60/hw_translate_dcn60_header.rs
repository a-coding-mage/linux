// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

#[repr(C)]
pub struct hw_translate {
    _private: [u8; 0],
}

/* Initialize Hw translate function pointers */
extern "C" {
    pub fn dal_hw_translate_dcn60_init(tr: *mut hw_translate);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
