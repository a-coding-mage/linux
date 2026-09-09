/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

use core::ffi::c_void;

// Types supplied by the included timing-generator and graphics headers.
#[repr(C)] pub struct timing_generator { _private: [u8; 0] }
#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct dc_crtc_timing { _private: [u8; 0] }
#[repr(C)] pub struct crtc_position { _private: [u8; 0] }
#[repr(C)] pub struct dcp_gsl_params { _private: [u8; 0] }
#[repr(C)] pub struct crtc_trigger_info { _private: [u8; 0] }
#[repr(C)] pub struct tg_color { _private: [u8; 0] }
#[repr(C)] pub struct drr_params { _private: [u8; 0] }
#[repr(C)] pub struct crc_params { _private: [u8; 0] }
pub type controller_id = i32;
pub type signal_type = i32;
pub type controller_dp_test_pattern = i32;
pub type dc_color_depth = i32;
pub type dc_color_space = i32;
pub type crtc_state = i32;

pub const VFLIP_READY_DELAY: u32 = 4;
pub const HFLIP_READY_DELAY: u32 = 2;
pub const HFLIP_CHECK_DELAY: u32 = 6;
pub const FLIP_READY_BACK_LOOKUP: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum trigger_source_select {
    TRIGGER_SOURCE_SELECT_LOGIC_ZERO = 0,
    TRIGGER_SOURCE_SELECT_CRTC_VSYNCA = 1,
    TRIGGER_SOURCE_SELECT_CRTC_HSYNCA = 2,
    TRIGGER_SOURCE_SELECT_CRTC_VSYNCB = 3,
    TRIGGER_SOURCE_SELECT_CRTC_HSYNCB = 4,
    TRIGGER_SOURCE_SELECT_GENERICF = 5,
    TRIGGER_SOURCE_SELECT_GENERICE = 6,
    TRIGGER_SOURCE_SELECT_VSYNCA = 7,
    TRIGGER_SOURCE_SELECT_HSYNCA = 8,
    TRIGGER_SOURCE_SELECT_VSYNCB = 9,
    TRIGGER_SOURCE_SELECT_HSYNCB = 10,
    TRIGGER_SOURCE_SELECT_HPD1 = 11,
    TRIGGER_SOURCE_SELECT_HPD2 = 12,
    TRIGGER_SOURCE_SELECT_GENERICD = 13,
    TRIGGER_SOURCE_SELECT_GENERICC = 14,
    TRIGGER_SOURCE_SELECT_VIDEO_CAPTURE = 15,
    TRIGGER_SOURCE_SELECT_GSL_GROUP0 = 16,
    TRIGGER_SOURCE_SELECT_GSL_GROUP1 = 17,
    TRIGGER_SOURCE_SELECT_GSL_GROUP2 = 18,
    TRIGGER_SOURCE_SELECT_BLONY = 19,
    TRIGGER_SOURCE_SELECT_GENERICA = 20,
    TRIGGER_SOURCE_SELECT_GENERICB = 21,
    TRIGGER_SOURCE_SELECT_GSL_ALLOW_FLIP = 22,
    TRIGGER_SOURCE_SELECT_MANUAL_TRIGGER = 23,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum trigger_polarity_select {
    TRIGGER_POLARITY_SELECT_LOGIC_ZERO = 0,
    TRIGGER_POLARITY_SELECT_CRTC = 1,
    TRIGGER_POLARITY_SELECT_GENERICA = 2,
    TRIGGER_POLARITY_SELECT_GENERICB = 3,
    TRIGGER_POLARITY_SELECT_HSYNCA = 4,
    TRIGGER_POLARITY_SELECT_HSYNCB = 5,
    TRIGGER_POLARITY_SELECT_VIDEO_CAPTURE = 6,
    TRIGGER_POLARITY_SELECT_GENERICC = 7,
}

#[repr(C)]
pub struct dce110_timing_generator_offsets {
    pub crtc: i32,
    pub dcp: i32,
    /* DCE80 use only */
    pub dmif: i32,
}

#[repr(C)]
pub struct dce110_timing_generator {
    pub base: timing_generator,
    pub offsets: dce110_timing_generator_offsets,
    pub derived_offsets: dce110_timing_generator_offsets,
    pub controller_id: controller_id,
    pub max_h_total: u32,
    pub max_v_total: u32,
    pub min_h_blank: u32,
    pub min_h_front_porch: u32,
    pub min_h_back_porch: u32,
    /* DCE 12 */
    pub min_h_sync_width: u32,
    pub min_v_sync_width: u32,
    pub min_v_blank: u32,
}

// DCE110TG_FROM_TG(tg): container_of(tg, struct dce110_timing_generator, base)
pub unsafe fn DCE110TG_FROM_TG(tg: *mut timing_generator) -> *mut dce110_timing_generator {
    (tg as *mut u8).sub(core::mem::offset_of!(dce110_timing_generator, base))
        as *mut dce110_timing_generator
}

extern "C" {
    pub fn dce110_timing_generator_construct(tg: *mut dce110_timing_generator, ctx: *mut dc_context, instance: u32, offsets: *const dce110_timing_generator_offsets);
    pub fn dce110_timing_generator_validate_timing(tg: *mut timing_generator, timing: *const dc_crtc_timing, signal: signal_type) -> bool;
    pub fn dce110_timing_generator_program_timing_generator(tg: *mut timing_generator, dc_crtc_timing: *const dc_crtc_timing) -> bool;
    pub fn dce110_timing_generator_enable_crtc(tg: *mut timing_generator) -> bool;
    pub fn dce110_timing_generator_disable_crtc(tg: *mut timing_generator) -> bool;
    pub fn dce110_timing_generator_set_early_control(tg: *mut timing_generator, early_cntl: u32);
    pub fn dce110_timing_generator_get_vblank_counter(tg: *mut timing_generator) -> u32;
    pub fn dce110_timing_generator_get_position(tg: *mut timing_generator, position: *mut crtc_position);
    pub fn dce110_timing_generator_is_counter_moving(tg: *mut timing_generator) -> bool;
    pub fn dce110_timing_generator_wait_for_vblank(tg: *mut timing_generator);
    pub fn dce110_timing_generator_wait_for_vactive(tg: *mut timing_generator);
    pub fn dce110_timing_generator_setup_global_swap_lock(tg: *mut timing_generator, gsl_params: *const dcp_gsl_params);
    pub fn dce110_timing_generator_tear_down_global_swap_lock(tg: *mut timing_generator);
    pub fn dce110_timing_generator_enable_crtc_reset(tg: *mut timing_generator, source: i32, crtc_tp: *mut crtc_trigger_info);
    pub fn dce110_timing_generator_enable_reset_trigger(tg: *mut timing_generator, source: i32);
    pub fn dce110_timing_generator_disable_reset_trigger(tg: *mut timing_generator);
    pub fn dce110_timing_generator_did_triggered_reset_occur(tg: *mut timing_generator) -> bool;
    pub fn dce110_timing_generator_disable_vga(tg: *mut timing_generator);
    pub fn dce110_timing_generator_program_blanking(tg: *mut timing_generator, timing: *const dc_crtc_timing);
    pub fn dce110_timing_generator_program_blank_color(tg: *mut timing_generator, black_color: *const tg_color);
    pub fn dce110_timing_generator_set_overscan_color_black(tg: *mut timing_generator, color: *const tg_color);
    pub fn dce110_timing_generator_color_space_to_black_color(colorspace: dc_color_space, black_color: *mut tg_color);
    pub fn dce110_timing_generator_set_test_pattern(tg: *mut timing_generator, test_pattern: controller_dp_test_pattern, color_depth: dc_color_depth);
    pub fn dce110_timing_generator_set_drr(tg: *mut timing_generator, params: *const drr_params);
    pub fn dce110_timing_generator_set_static_screen_control(tg: *mut timing_generator, event_triggers: u32, num_frames: u32);
    pub fn dce110_timing_generator_get_crtc_scanoutpos(tg: *mut timing_generator, v_blank_start: *mut u32, v_blank_end: *mut u32, h_position: *mut u32, v_position: *mut u32);
    pub fn dce110_timing_generator_enable_advanced_request(tg: *mut timing_generator, enable: bool, timing: *const dc_crtc_timing);
    pub fn dce110_timing_generator_set_lock_master(tg: *mut timing_generator, lock: bool);
    pub fn dce110_tg_program_blank_color(tg: *mut timing_generator, black_color: *const tg_color);
    pub fn dce110_tg_set_overscan_color(tg: *mut timing_generator, overscan_color: *const tg_color);
    pub fn dce110_tg_program_timing(tg: *mut timing_generator, timing: *const dc_crtc_timing, vready_offset: i32, vstartup_start: i32, vupdate_offset: i32, vupdate_width: i32, pstate_keepout: i32, signal: signal_type, use_vbios: bool);
    pub fn dce110_tg_is_blanked(tg: *mut timing_generator) -> bool;
    pub fn dce110_tg_set_blank(tg: *mut timing_generator, enable_blanking: bool);
    pub fn dce110_tg_validate_timing(tg: *mut timing_generator, timing: *const dc_crtc_timing) -> bool;
    pub fn dce110_tg_wait_for_state(tg: *mut timing_generator, state: crtc_state);
    pub fn dce110_tg_set_colors(tg: *mut timing_generator, blank_color: *const tg_color, overscan_color: *const tg_color);
    pub fn dce110_arm_vert_intr(tg: *mut timing_generator, width: u8) -> bool;
    pub fn dce110_configure_crc(tg: *mut timing_generator, params: *const crc_params) -> bool;
    pub fn dce110_get_crc(tg: *mut timing_generator, idx: u8, r_cr: *mut u32, g_y: *mut u32, b_cb: *mut u32) -> bool;
    pub fn dce110_is_two_pixels_per_container(timing: *const dc_crtc_timing) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
