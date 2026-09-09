/* SPDX-License-Identifier: MIT */

/* Copyright 2024 Advanced Micro Devices, Inc. */

// Dependency supplied by dc_spl_types.h is intentionally not implemented here.

pub const NUM_PHASES_COEFF: usize = 33;

// C declaration uses SPL_NAMESPACE(...); the namespace-expanding macro is
// supplied by the surrounding build and cannot be represented in Rust.
extern "C" {
    pub fn convert_filter_s1_10_to_s1_12(
        s1_10_filter: *const u16,
        s1_12_filter: *mut u16,
        num_taps: i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
