/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// The C header depends on declarations supplied by dc.h and mod_power.h.

pub struct amdgpu_display_manager;
pub struct amdgpu_dm_connector;
pub struct dc_link;
pub struct dc_stream_state;
pub struct psr_caps;
pub enum psr_event {}

/// The number of pageflips before enabling PSR.
pub const AMDGPU_DM_PSR_ENTRY_DELAY: u32 = 5;

unsafe extern "C" {
    pub fn amdgpu_dm_set_psr_caps(
        link: *mut dc_link,
        aconnector: *mut amdgpu_dm_connector,
    ) -> bool;
    pub fn amdgpu_dm_psr_is_active_allowed(dm: *mut amdgpu_display_manager) -> bool;
    pub fn amdgpu_dm_psr_set_event(
        dm: *mut amdgpu_display_manager,
        stream: *mut dc_stream_state,
        set_event: bool,
        event: psr_event,
        wait_for_disable: bool,
    ) -> bool;

    // Preserved from IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST): these
    // declarations are available only when the KUnit test configuration is enabled.
    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn link_supports_psrsu(link: *mut dc_link) -> bool;
    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_psr_fill_caps(link: *mut dc_link, caps: *mut psr_caps);
    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_psr_get_dc_feature_mask() -> u32;
    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_psr_set_dc_feature_mask(feature_mask: u32);
    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_psr_get_dc_debug_mask() -> u32;
    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_psr_set_dc_debug_mask(debug_mask: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
