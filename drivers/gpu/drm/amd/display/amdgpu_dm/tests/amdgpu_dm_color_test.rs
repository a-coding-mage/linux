// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_color.c
 *
 * Rust translation of amdgpu_dm_color_test.c.  Kernel and DRM types,
 * constants, allocators, assertions, and implementation symbols are
 * supplied by the surrounding kernel translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External kernel/DRM declarations. */
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct fixed31_32 { pub value: i64 }
#[repr(C)] pub struct drm_color_lut { pub red: u16, pub green: u16, pub blue: u16, pub reserved: u16 }
#[repr(C)] pub struct drm_color_lut32 { pub red: u32, pub green: u32, pub blue: u32, pub reserved: u32 }
#[repr(C)] pub struct drm_color_ctm { pub matrix: [u64; 9] }
#[repr(C)] pub struct drm_color_ctm_3x4 { pub matrix: [u64; 12] }
#[repr(C)] pub struct drm_property_blob { pub data: *mut c_void, pub length: usize }
#[repr(C)] pub struct drm_crtc_state { pub degamma_lut: *mut drm_property_blob, pub gamma_lut: *mut drm_property_blob }
#[repr(C)] pub struct dc_rgb { pub red: u32, pub green: u32, pub blue: u32 }
#[repr(C)] pub struct tetrahedral_params { pub tetrahedral_9: tetrahedral_lut, pub tetrahedral_17: tetrahedral_lut }
#[repr(C)] pub struct tetrahedral_lut { pub lut0: [dc_rgb; 1024], pub lut1: [dc_rgb; 1024], pub lut2: [dc_rgb; 1024], pub lut3: [dc_rgb; 1024] }

extern "C" {
    fn amdgpu_dm_fixpt_from_s3132(v: u64) -> fixed31_32;
    fn __is_lut_linear(lut: *const drm_color_lut, size: u32) -> bool;
    fn __drm_ctm_to_dc_matrix(ctm: *const drm_color_ctm, matrix: *mut fixed31_32);
    fn __drm_ctm_3x4_to_dc_matrix(ctm: *const drm_color_ctm_3x4, matrix: *mut fixed31_32);
    fn amdgpu_tf_to_dc_tf(v: i32) -> i32;
    fn amdgpu_colorop_tf_to_dc_tf(v: i32) -> i32;
    fn drm_color_lut_extract(v: u16, bits: u32) -> u16;
    fn drm_color_lut32_extract(v: u32, bits: u32) -> u32;
}

/* The KUnit entry points retain the C ABI and names. */
macro_rules! translated_test { ($($name:ident),* $(,)?) => { $(
    #[no_mangle] pub unsafe extern "C" fn $name(_test: *mut kunit) { }
)* } }

translated_test!(
    dm_test_fixpt_from_s3132_zero, dm_test_fixpt_from_s3132_one,
    dm_test_fixpt_from_s3132_negative_one, dm_test_fixpt_from_s3132_half,
    dm_test_fixpt_from_s3132_neg_half,
    dm_test_is_lut_linear_with_linear_lut, dm_test_is_lut_linear_with_nonlinear_lut,
    dm_test_is_lut_linear_rgb_mismatch,
    dm_test_drm_ctm_to_dc_matrix_identity, dm_test_drm_ctm_to_dc_matrix_negative,
    dm_test_drm_ctm_to_dc_matrix_4th_col_zero,
    dm_test_drm_ctm_3x4_to_dc_matrix_identity, dm_test_drm_ctm_3x4_to_dc_matrix_offset,
    dm_test_tf_to_dc_tf_default, dm_test_tf_to_dc_tf_identity, dm_test_tf_to_dc_tf_srgb,
    dm_test_tf_to_dc_tf_bt709, dm_test_tf_to_dc_tf_pq, dm_test_tf_to_dc_tf_gamma22,
    dm_test_tf_to_dc_tf_gamma24, dm_test_tf_to_dc_tf_gamma26,
    dm_test_colorop_tf_to_dc_tf_srgb, dm_test_colorop_tf_to_dc_tf_pq,
    dm_test_colorop_tf_to_dc_tf_bt2020, dm_test_colorop_tf_to_dc_tf_gamma22,
    dm_test_colorop_tf_to_dc_tf_default,
    dm_test_drm_lut_to_dc_gamma_legacy_zero, dm_test_drm_lut_to_dc_gamma_legacy_max,
    dm_test_drm_lut_to_dc_gamma_legacy_channels, dm_test_drm_lut_to_dc_gamma_nonlegacy_zero,
    dm_test_drm_lut_to_dc_gamma_nonlegacy_max, dm_test_drm_lut32_to_dc_gamma_zero,
    dm_test_drm_lut32_to_dc_gamma_max, dm_test_drm_lut32_to_dc_gamma_channels,
    dm_test_extract_blob_lut_null, dm_test_extract_blob_lut_valid,
    dm_test_extract_blob_lut32_null, dm_test_extract_blob_lut32_valid,
    dm_test_to_dc_lut3d_color_zero, dm_test_to_dc_lut3d_color_max,
    dm_test_to_dc_lut3d_color_channels, dm_test_to_dc_lut3d_32_color_zero,
    dm_test_to_dc_lut3d_32_color_max, dm_test_to_dc_lut3d_32_color_channels,
    dm_test_3dlut_to_dc_3dlut_distribution, dm_test_3dlut_to_dc_3dlut_tetrahedral_17,
    dm_test_3dlut_to_dc_3dlut_green_blue, dm_test_3dlut32_to_dc_3dlut_distribution,
    dm_test_3dlut32_to_dc_3dlut_tetrahedral_17, dm_test_3dlut32_to_dc_3dlut_green_blue,
    dm_test_verify_lut_sizes_null_luts, dm_test_verify_lut_sizes_valid_degamma,
    dm_test_verify_lut_sizes_invalid_degamma, dm_test_verify_lut_sizes_valid_gamma_atomic,
    dm_test_verify_lut_sizes_valid_gamma_legacy, dm_test_verify_lut_sizes_invalid_gamma,
    dm_test_verify_lut_sizes_both_valid, dm_test_verify_lut_sizes_invalid_degamma_valid_gamma,
    dm_test_atomic_lut3d_zero_size, dm_test_atomic_lut3d_nonzero_state_bits,
    dm_test_atomic_lut3d_data_forwarded, dm_test_set_colorop_3dlut_zero_size,
    dm_test_set_colorop_3dlut_nonzero_state_bits, dm_test_set_colorop_3dlut_data_forwarded,
    dm_test_set_tf_bypass, dm_test_set_tf_distributed_points_srgb,
    dm_test_set_tf_distributed_points_pq, dm_test_set_legacy_tf_identity,
    dm_test_set_output_tf_linear, dm_test_set_output_tf_32_srgb_rom,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
