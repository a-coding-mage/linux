// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// C dependency: `struct hw_factory` is declared by the surrounding codebase.
#[repr(C)]
pub struct hw_factory {
    _private: [u8; 0],
}

/* Initialize HW factory function pointers and pin info */
unsafe extern "C" {
    pub fn dal_hw_factory_dcn60_init(factory: *mut hw_factory);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
