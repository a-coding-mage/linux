// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

#[repr(C)]
pub struct hw_translate {
    _private: [u8; 0],
}

/* Initialize Hw translate function pointers */
unsafe extern "C" {
    pub fn dal_hw_translate_dcn401_init(tr: *mut hw_translate);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
