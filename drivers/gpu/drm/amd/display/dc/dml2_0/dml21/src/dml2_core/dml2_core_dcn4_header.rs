// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// The C header guard is not needed in Rust.

// Opaque declarations for structures supplied by dependent headers.
#[repr(C)]
pub struct dml2_core_initialize_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_core_mode_support_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_core_mode_programming_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_core_populate_informative_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_calculate_mcache_allocation_in_out {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn core_dcn4_initialize(in_out: *mut dml2_core_initialize_in_out) -> bool;
    pub fn core_dcn42_initialize(in_out: *mut dml2_core_initialize_in_out) -> bool;
    pub fn core_dcn4_mode_support(in_out: *mut dml2_core_mode_support_in_out) -> bool;
    pub fn core_dcn4_mode_programming(
        in_out: *mut dml2_core_mode_programming_in_out,
    ) -> bool;
    pub fn core_dcn4_populate_informative(
        in_out: *mut dml2_core_populate_informative_in_out,
    ) -> bool;
    pub fn core_dcn4_calculate_mcache_allocation(
        in_out: *mut dml2_calculate_mcache_allocation_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
