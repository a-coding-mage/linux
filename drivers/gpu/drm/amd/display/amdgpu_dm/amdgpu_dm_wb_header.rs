/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// C dependency: <drm/drm_writeback.h>

#[repr(C)]
pub struct amdgpu_display_manager {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_dm_wb_connector {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    pub fn amdgpu_dm_wb_connector_init(
        dm: *mut amdgpu_display_manager,
        dm_wbcon: *mut amdgpu_dm_wb_connector,
        link_index: u32,
    ) -> ::core::ffi::c_int;
}

// Equivalent to: #if IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST)
// C dependencies: <drm/drm_connector.h>, <drm/drm_crtc.h>
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
mod config_drm_amd_dc_kunit_test {
    #[repr(C)]
    pub struct drm_encoder {
        _opaque: [u8; 0],
    }

    #[repr(C)]
    pub struct drm_crtc_state {
        _opaque: [u8; 0],
    }

    #[repr(C)]
    pub struct drm_connector_state {
        _opaque: [u8; 0],
    }

    #[repr(C)]
    pub struct drm_connector {
        _opaque: [u8; 0],
    }

    #[repr(C)]
    pub struct drm_writeback_connector {
        _opaque: [u8; 0],
    }

    #[repr(C)]
    pub struct drm_writeback_job {
        _opaque: [u8; 0],
    }

    unsafe extern "C" {
        pub fn amdgpu_dm_wb_encoder_atomic_check(
            encoder: *mut drm_encoder,
            crtc_state: *mut drm_crtc_state,
            conn_state: *mut drm_connector_state,
        ) -> ::core::ffi::c_int;

        pub fn amdgpu_dm_wb_connector_get_modes(
            connector: *mut drm_connector,
        ) -> ::core::ffi::c_int;

        pub fn amdgpu_dm_wb_prepare_job(
            wb_connector: *mut drm_writeback_connector,
            job: *mut drm_writeback_job,
        ) -> ::core::ffi::c_int;

        pub fn amdgpu_dm_wb_cleanup_job(
            connector: *mut drm_writeback_connector,
            job: *mut drm_writeback_job,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
