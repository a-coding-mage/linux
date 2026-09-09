// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// use dml2_external_lib_deps::*;
// use dml2_core_calcs_dsc_shared_types::*;

// dcn5_dsc_compute_delay - DSC delay using the updated formula (DCN5.1 and newer)
extern "C" {
    pub fn dcn5_dsc_compute_delay(
        p: *mut delay_uncertainty_t,
        bpc: ::core::ffi::c_int,
        bpp: f32,
        slice_width: ::core::ffi::c_int,
        num_slices: ::core::ffi::c_int,
        pixel_format: dml2_output_format_class,
        dscclk_dynamic_gating_en: ::core::ffi::c_int,
        dispclk_dynamic_gating_en: ::core::ffi::c_int,
        initial_xmit_delay_offset: ::core::ffi::c_int,
        group_delay_after_initial_xmit_delay_override_en: ::core::ffi::c_int,
        group_delay_after_initial_xmit_delay: ::core::ffi::c_int,
    );

    // dcn5_dsc_compute_delay_legacy - DSC delay using the original DCN5 formula
    pub fn dcn5_dsc_compute_delay_legacy(
        p: *mut delay_uncertainty_t,
        bpc: ::core::ffi::c_int,
        bpp: f32,
        slice_width: ::core::ffi::c_int,
        num_slices: ::core::ffi::c_int,
        pixel_format: dml2_output_format_class,
        dscclk_dynamic_gating_en: ::core::ffi::c_int,
        dispclk_dynamic_gating_en: ::core::ffi::c_int,
        initial_xmit_delay_offset: ::core::ffi::c_int,
        group_delay_after_initial_xmit_delay_override_en: ::core::ffi::c_int,
        group_delay_after_initial_xmit_delay: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
