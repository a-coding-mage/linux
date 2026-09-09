/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

pub const MAX_DRM_LUT_VALUE: u32 = 0xFFFF;
pub const MAX_DRM_LUT32_VALUE: u32 = 0xFFFFFFFF;

// Forward declarations supplied by the surrounding kernel and display code.
#[repr(C)] pub struct drm_color_lut { _private: [u8; 0] }
#[repr(C)] pub struct drm_color_lut32 { _private: [u8; 0] }
#[repr(C)] pub struct drm_color_ctm { _private: [u8; 0] }
#[repr(C)] pub struct drm_color_ctm_3x4 { _private: [u8; 0] }
#[repr(C)] pub struct drm_colorop_state { _private: [u8; 0] }
#[repr(C)] pub struct drm_property_blob { _private: [u8; 0] }
#[repr(C)] pub struct dc_gamma { _private: [u8; 0] }
#[repr(C)] pub struct dc_rgb { _private: [u8; 0] }
#[repr(C)] pub struct dc_plane_state { _private: [u8; 0] }
#[repr(C)] pub struct fixed31_32 { _private: [u8; 0] }
#[repr(C)] pub struct tetrahedral_params { _private: [u8; 0] }
#[repr(C)] pub struct dc_transfer_func { _private: [u8; 0] }
#[repr(C)] pub struct dc_3dlut { _private: [u8; 0] }
#[repr(C)] pub struct dc_color_caps { _private: [u8; 0] }
#[repr(C)] pub struct dc_plane_cm { _private: [u8; 0] }
#[repr(C)] pub struct drm_plane_state { _private: [u8; 0] }
#[repr(C)] pub struct drm_colorop { _private: [u8; 0] }

// The source exposes these enum types from other headers.
pub type dc_transfer_func_predefined = i32;
pub type amdgpu_transfer_function = i32;
pub type drm_colorop_curve_1d_type = i32;

// Prototypes are enabled when CONFIG_DRM_AMD_DC_KUNIT_TEST is enabled.
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
extern "C" {
    pub fn amdgpu_dm_fixpt_from_s3132(x: u64) -> fixed31_32;
    pub fn __is_lut_linear(lut: *const drm_color_lut, size: u32) -> bool;
    pub fn __drm_lut_to_dc_gamma(lut: *const drm_color_lut, gamma: *mut dc_gamma, is_legacy: bool);
    pub fn __drm_lut32_to_dc_gamma(lut: *const drm_color_lut32, gamma: *mut dc_gamma);
    pub fn __drm_ctm_to_dc_matrix(ctm: *const drm_color_ctm, matrix: *mut fixed31_32);
    pub fn __drm_ctm_3x4_to_dc_matrix(ctm: *const drm_color_ctm_3x4, matrix: *mut fixed31_32);
    pub fn __set_legacy_tf(func: *mut dc_transfer_func, lut: *const drm_color_lut, lut_size: u32, has_rom: bool) -> i32;
    pub fn __set_output_tf(func: *mut dc_transfer_func, lut: *const drm_color_lut, lut_size: u32, has_rom: bool) -> i32;
    pub fn __set_output_tf_32(func: *mut dc_transfer_func, lut: *const drm_color_lut32, lut_size: u32, has_rom: bool) -> i32;
    pub fn __set_input_tf(caps: *mut dc_color_caps, func: *mut dc_transfer_func, lut: *const drm_color_lut, lut_size: u32) -> i32;
    pub fn __set_input_tf_32(caps: *mut dc_color_caps, func: *mut dc_transfer_func, lut: *const drm_color_lut32, lut_size: u32) -> i32;
    pub fn amdgpu_tf_to_dc_tf(tf: amdgpu_transfer_function) -> dc_transfer_func_predefined;
    pub fn amdgpu_colorop_tf_to_dc_tf(tf: drm_colorop_curve_1d_type) -> dc_transfer_func_predefined;
    pub fn __extract_blob_lut(blob: *const drm_property_blob, size: *mut u32) -> *const drm_color_lut;
    pub fn __extract_blob_lut32(blob: *const drm_property_blob, size: *mut u32) -> *const drm_color_lut32;
    pub fn __to_dc_lut3d_color(rgb: *mut dc_rgb, lut: drm_color_lut, bit_precision: i32);
    pub fn __drm_3dlut_to_dc_3dlut(lut: *const drm_color_lut, lut3d_size: u32, params: *mut tetrahedral_params, use_tetrahedral_9: bool, bit_depth: i32);
    pub fn __to_dc_lut3d_32_color(rgb: *mut dc_rgb, lut: drm_color_lut32, bit_precision: i32);
    pub fn __drm_3dlut32_to_dc_3dlut(lut: *const drm_color_lut32, lut3d_size: u32, params: *mut tetrahedral_params, use_tetrahedral_9: bool, bit_depth: i32);
    pub fn amdgpu_dm_atomic_lut3d(drm_lut3d: *const drm_color_lut, drm_lut3d_size: u32, cm: *mut dc_plane_cm);
    pub fn __set_colorop_3dlut(drm_lut3d: *const drm_color_lut32, drm_lut3d_size: u32, lut: *mut dc_3dlut) -> i32;
    pub fn __set_tf_bypass(tf: *mut dc_transfer_func);
    pub fn __set_tf_distributed_points(tf: *mut dc_transfer_func, predefined_tf: dc_transfer_func_predefined);
    pub fn amdgpu_dm_set_atomic_regamma(out_tf: *mut dc_transfer_func, regamma_lut: *const drm_color_lut, regamma_size: u32, has_rom: bool, tf: dc_transfer_func_predefined) -> i32;
    pub fn amdgpu_dm_atomic_shaper_lut(shaper_lut: *const drm_color_lut, has_rom: bool, tf: dc_transfer_func_predefined, shaper_size: u32, cm: *mut dc_plane_cm) -> i32;
    pub fn amdgpu_dm_atomic_blend_lut(blend_lut: *const drm_color_lut, has_rom: bool, tf: dc_transfer_func_predefined, blend_size: u32, cm: *mut dc_plane_cm) -> i32;
    pub fn __set_colorop_in_tf_1d_curve(dc_plane_state: *mut dc_plane_state, colorop_state: *mut drm_colorop_state) -> i32;
    pub fn __set_dm_plane_colorop_degamma(plane_state: *mut drm_plane_state, dc_plane_state: *mut dc_plane_state, colorop: *mut drm_colorop) -> i32;
    pub fn __set_dm_plane_colorop_3x4_matrix(plane_state: *mut drm_plane_state, dc_plane_state: *mut dc_plane_state, colorop: *mut drm_colorop) -> i32;
    pub fn __set_dm_plane_colorop_multiplier(plane_state: *mut drm_plane_state, dc_plane_state: *mut dc_plane_state, colorop: *mut drm_colorop) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
