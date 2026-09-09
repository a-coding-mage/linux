// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Forward declaration of the externally defined hardware translation object.
#[repr(C)]
pub struct hw_translate {
    _private: [u8; 0],
}

/* Initialize Hw translate function pointers */
unsafe extern "C" {
    pub fn dal_hw_translate_dcn42b_init(tr: *mut hw_translate);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
