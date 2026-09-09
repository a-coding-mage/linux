// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency supplied by dc_spl_types.h.

pub const BLACK_OFFSET_RGB_Y: u32 = 0x0;
pub const BLACK_OFFSET_CBCR: u32 = 0x8000;

// SPL_NAMESPACE is a build-time naming macro in the C header. The declarations
// below retain the underlying interface names; callers may apply the required
// namespace through their linkage configuration.
unsafe extern "C" {
    pub fn spl_calculate_scaler_params(
        spl_in: *mut spl_in,
        spl_out: *mut spl_out,
    ) -> bool;

    pub fn spl_get_number_of_taps(
        spl_in: *mut spl_in,
        spl_out: *mut spl_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
