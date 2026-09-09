// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

//! Top Level Interface for DML2.
//!
//! The declarations below correspond to the types supplied by `dml_top_types.h`.

use core::ffi::c_uint;

/// Opaque type supplied by `dml_top_types.h`.
#[repr(C)]
pub struct dml2_initialize_instance_in_out {
    _private: [u8; 0],
}

/// Opaque type supplied by `dml_top_types.h`.
#[repr(C)]
pub struct dml2_check_mode_supported_in_out {
    _private: [u8; 0],
}

/// Opaque type supplied by `dml_top_types.h`.
#[repr(C)]
pub struct dml2_build_mode_programming_in_out {
    _private: [u8; 0],
}

/// Opaque type supplied by `dml_top_types.h`.
#[repr(C)]
pub struct dml2_build_mcache_programming_in_out {
    _private: [u8; 0],
}

extern "C" {
    /// Returns the size of the DML instance for the caller to allocate.
    pub fn dml2_get_instance_size_bytes() -> c_uint;

    /// Initializes the DML instance (i.e. with configuration, soc BB, IP params, etc...).
    pub fn dml2_initialize_instance(
        in_out: *mut dml2_initialize_instance_in_out,
    ) -> bool;

    ///
    /// Determines if the input mode is supported (boolean) on the SoC at all.  Does not return
    /// information on how mode should be programmed.
    pub fn dml2_check_mode_supported(
        in_out: *mut dml2_check_mode_supported_in_out,
    ) -> bool;

    ///
    /// Determines the full (optimized) programming for the input mode.  Returns minimum
    /// clocks as well as dchub register programming values for all pipes, additional meta
    /// such as ODM or MPCC combine factors.
    pub fn dml2_build_mode_programming(
        in_out: *mut dml2_build_mode_programming_in_out,
    ) -> bool;

    ///
    /// Determines the correct per pipe mcache register programming for a valid mode.
    /// The mcache allocation must have been calculated (successfully) in a previous
    /// call to dml2_build_mode_programming.
    /// The actual hubp viewport dimensions be what the actual registers will be
    /// programmed to (i.e. based on scaler setup).
    pub fn dml2_build_mcache_programming(
        in_out: *mut dml2_build_mcache_programming_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
