/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding translation unit: dc_types.h.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_dm_pipe_crc_source {
    AMDGPU_DM_PIPE_CRC_SOURCE_NONE = 0,
    AMDGPU_DM_PIPE_CRC_SOURCE_CRTC,
    AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER,
    AMDGPU_DM_PIPE_CRC_SOURCE_DPRX,
    AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER,
    AMDGPU_DM_PIPE_CRC_SOURCE_MAX,
    AMDGPU_DM_PIPE_CRC_SOURCE_INVALID = -1,
}

// CONFIG_DRM_AMD_SECURE_DISPLAY conditional declarations.
pub const MAX_CRTC: usize = 6;

#[repr(C)]
pub enum secure_display_mode {
    LEGACY_MODE = 0,
    DISPLAY_CRC_MODE,
    SECURE_DISPLAY_MODE_MAX,
}

#[repr(C)]
pub struct phy_id_mapping {
    pub assigned: bool,
    pub is_mst: bool,
    pub enc_hw_inst: u8,
    pub lct: u8,
    pub port_num: u8,
    pub rad: [u8; 8],
}

#[repr(C)]
pub struct crc_data {
    pub crc_R: u32,
    pub crc_G: u32,
    pub crc_B: u32,
    pub frame_count: u32,
    pub crc_ready: bool,
}

#[repr(C)]
pub struct crc_info {
    pub crc: [crc_data; MAX_CRC_WINDOW_NUM],
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct crc_window_param {
    pub x_start: u16,
    pub y_start: u16,
    pub x_end: u16,
    pub y_end: u16,
    /* CRC window is activated or not */
    pub enable: bool,
    /* Update crc window during vertical blank or not */
    pub update_win: bool,
    /* skip reading/writing for few frames */
    pub skip_frame_cnt: i32,
}

#[repr(C)]
pub struct secure_display_crtc_context {
    /* work to notify PSP TA */
    pub notify_ta_work: work_struct,
    /* work to forward ROI to dmcu/dmub */
    pub forward_roi_work: work_struct,
    pub crtc: *mut drm_crtc,
    /* Region of Interest (ROI) */
    pub roi: [crc_window; MAX_CRC_WINDOW_NUM],
    pub crc_info: crc_info,
}

#[repr(C)]
pub struct secure_display_context {
    pub crtc_ctx: *mut secure_display_crtc_context,
    /* Whether dmub support multiple ROI setting */
    pub support_mul_roi: bool,
    pub op_mode: secure_display_mode,
    pub phy_mapping_updated: bool,
    pub phy_id_mapping_cnt: i32,
    pub phy_id_mapping: [phy_id_mapping; MAX_CRTC],
}

#[inline]
pub fn amdgpu_dm_is_valid_crc_source(source: amdgpu_dm_pipe_crc_source) -> bool {
    (source as i32 > AMDGPU_DM_PIPE_CRC_SOURCE_NONE as i32)
        && (source as i32 < AMDGPU_DM_PIPE_CRC_SOURCE_MAX as i32)
}

// amdgpu_dm_crc.c; CONFIG_DEBUG_FS controls these declarations.
extern "C" {
    pub fn amdgpu_dm_crtc_configure_crc_source(
        crtc: *mut drm_crtc,
        dm_crtc_state: *mut dm_crtc_state,
        source: amdgpu_dm_pipe_crc_source,
    ) -> i32;
    pub fn amdgpu_dm_crtc_set_crc_source(crtc: *mut drm_crtc, src_name: *const i8) -> i32;
    pub fn amdgpu_dm_crtc_verify_crc_source(
        crtc: *mut drm_crtc,
        src_name: *const i8,
        values_cnt: *mut usize,
    ) -> i32;
    pub fn amdgpu_dm_crtc_get_crc_sources(
        crtc: *mut drm_crtc,
        count: *mut usize,
    ) -> *const *const i8;
    pub fn amdgpu_dm_crtc_handle_crc_irq(crtc: *mut drm_crtc);

    pub fn amdgpu_dm_crc_window_is_activated(crtc: *mut drm_crtc) -> bool;
    pub fn amdgpu_dm_crtc_handle_crc_window_irq(crtc: *mut drm_crtc);
    pub fn amdgpu_dm_crtc_secure_display_create_contexts(adev: *mut amdgpu_device);

    pub fn dm_parse_crc_source(source: *const i8) -> amdgpu_dm_pipe_crc_source;
    pub fn dm_is_crc_source_crtc(src: amdgpu_dm_pipe_crc_source) -> bool;
    pub fn dm_is_crc_source_dprx(src: amdgpu_dm_pipe_crc_source) -> bool;
    pub fn dm_need_crc_dither(src: amdgpu_dm_pipe_crc_source) -> bool;
    pub fn dm_need_dp_aux(
        source: amdgpu_dm_pipe_crc_source,
        cur_crc_src: amdgpu_dm_pipe_crc_source,
    ) -> bool;
    pub fn dm_crc_source_should_start_dprx(
        source: amdgpu_dm_pipe_crc_source,
        cur_crc_src: amdgpu_dm_pipe_crc_source,
    ) -> bool;
    pub fn dm_crc_source_should_stop_dprx(
        source: amdgpu_dm_pipe_crc_source,
        cur_crc_src: amdgpu_dm_pipe_crc_source,
    ) -> bool;
}

// Forward declarations supplied by included dependencies.
#[allow(non_camel_case_types)] pub enum drm_crtc {}
#[allow(non_camel_case_types)] pub enum dm_crtc_state {}
#[allow(non_camel_case_types)] pub enum amdgpu_device {}
#[allow(non_camel_case_types)] pub enum spinlock_t {}
#[allow(non_camel_case_types)] pub enum work_struct {}
#[allow(non_camel_case_types)] pub enum crc_window {}
extern "C" { static MAX_CRC_WINDOW_NUM: usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
