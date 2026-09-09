// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dml2_top_soc15.h.
//
// The declarations below are supplied by dml2_internal_shared_types.h in the
// original source and are kept opaque here.
#[repr(C)]
pub struct dml2_initialize_instance_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct top_mcache_calc_mcache_count_and_offsets_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct top_mcache_assign_global_mcache_ids_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct top_mcache_validate_admissability_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_build_mcache_programming_in_out {
    _private: [u8; 0],
}

extern "C" {
    pub fn dml2_top_soc15_initialize_instance(
        in_out: *mut dml2_initialize_instance_in_out,
    ) -> bool;

    pub fn dml2_top_mcache_calc_mcache_count_and_offsets(
        params: *mut top_mcache_calc_mcache_count_and_offsets_in_out,
    ) -> bool;

    pub fn dml2_top_mcache_assign_global_mcache_ids(
        params: *mut top_mcache_assign_global_mcache_ids_in_out,
    );

    pub fn dml2_top_mcache_validate_admissability(
        params: *mut top_mcache_validate_admissability_in_out,
    ) -> bool;

    pub fn dml2_top_soc15_build_mcache_programming(
        params: *mut dml2_build_mcache_programming_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
