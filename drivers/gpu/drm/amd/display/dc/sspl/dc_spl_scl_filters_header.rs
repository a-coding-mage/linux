// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency equivalent of: #include "dc_spl_types.h"

/* public API */
// The C declaration applies SPL_NAMESPACE to this symbol. The namespace
// expansion is supplied by the build configuration; the unexpanded function
// name is retained here as the local Rust declaration.
unsafe extern "C" {
    pub fn spl_dscl_get_filter_coeffs_64p(
        taps: core::ffi::c_int,
        ratio: spl_fixed31_32,
    ) -> *const u16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
