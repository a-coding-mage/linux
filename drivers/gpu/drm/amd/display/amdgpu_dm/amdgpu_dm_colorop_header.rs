/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
 *
 */

// C header guard: __AMDGPU_DM_COLOROP_H__.

extern "C" {
    pub static amdgpu_dm_supported_degam_tfs: u64;
    pub static amdgpu_dm_supported_shaper_tfs: u64;
    pub static amdgpu_dm_supported_blnd_tfs: u64;

    pub fn amdgpu_dm_initialize_default_pipeline(
        plane: *mut drm_plane,
        list: *mut drm_prop_enum_list,
    ) -> i32;

    // Preserved from: #if IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST)
    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_build_default_pipeline(
        dev: *mut drm_device,
        plane: *mut drm_plane,
        hw_3d_lut: bool,
        list: *mut drm_prop_enum_list,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
