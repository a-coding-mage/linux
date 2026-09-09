// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C header guard: __DAL_HW_FACTORY_DCN42B_H__

/* Initialize HW factory function pointers and pin info */
extern "C" {
    pub fn dal_hw_factory_dcn42b_init(factory: *mut hw_factory);
}

// Opaque type supplied by the corresponding dependency.
#[repr(C)]
pub struct hw_factory {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
