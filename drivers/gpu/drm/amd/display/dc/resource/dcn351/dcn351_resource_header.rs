/* SPDX-License-Identifier: MIT */
/* Copyright 2024 Advanced Micro Devices, Inc. */

// Translated from dcn351_resource.h.
// Dependency declarations from core_types.h are supplied externally.

extern "C" {
    pub static mut dcn3_51_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn3_51_soc: _vcs_dpi_soc_bounding_box_st;
}

// C container_of macro; the referenced types and implementation are supplied
// by the surrounding translation unit.
#[macro_export]
macro_rules! TO_DCN351_RES_POOL {
    ($pool:expr) => {
        container_of!($pool, dcn351_resource_pool, base)
    };
}

#[repr(C)]
pub struct dcn351_resource_pool {
    pub base: resource_pool,
}

extern "C" {
    pub fn dcn351_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
