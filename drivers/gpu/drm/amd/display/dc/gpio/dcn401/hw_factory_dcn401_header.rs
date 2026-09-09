// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// C header guard: __DAL_HW_FACTORY_DCN401_H__

/* Initialize HW factory function pointers and pin info */
#[allow(non_camel_case_types)]
pub enum hw_factory {}

extern "C" {
    pub fn dal_hw_factory_dcn401_init(factory: *mut hw_factory);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
