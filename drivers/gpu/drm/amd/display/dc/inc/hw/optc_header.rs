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
 */

/**
 * DOC: overview
 *
 * Output Pipe Timing Combiner (OPTC) includes two major functional blocks:
 * Output Data Mapper (ODM) and Output Timing Generator (OTG).
 *
 * - ODM: It is Output Data Mapping block. It can combine input data from
 *   multiple OPP data pipes into one single data stream or split data from one
 *   OPP data pipe into multiple data streams or just bypass OPP data to DIO.
 * - OTG: It is Output Timing Generator. It generates display timing signals to
 *   drive the display output.
 */

// Dependency declarations are supplied by the translated timing-generator headers.

#[repr(C)]
pub struct optc {
    pub base: timing_generator,
    pub tg_regs: *const dcn_optc_registers,
    pub tg_shift: *const dcn_optc_shift,
    pub tg_mask: *const dcn_optc_mask,
    pub opp_count: core::ffi::c_int,
    pub max_h_total: u32,
    pub max_v_total: u32,
    pub min_h_blank: u32,
    pub min_h_sync_width: u32,
    pub min_v_sync_width: u32,
    pub min_v_blank: u32,
    pub min_v_blank_interlace: u32,
    pub vstartup_start: core::ffi::c_int,
    pub vupdate_offset: core::ffi::c_int,
    pub vupdate_width: core::ffi::c_int,
    pub vready_offset: core::ffi::c_int,
    pub pstate_keepout: core::ffi::c_int,
    pub orginal_patched_timing: dc_crtc_timing,
    pub signal: signal_type,
}

unsafe extern "C" {
    pub fn optc1_read_otg_state(optc: *mut timing_generator, s: *mut dcn_otg_state);
    pub fn optc1_get_hw_timing(tg: *mut timing_generator, hw_crtc_timing: *mut dc_crtc_timing) -> bool;
    pub fn optc1_validate_timing(optc: *mut timing_generator, timing: *const dc_crtc_timing) -> bool;
    pub fn optc1_program_timing(optc: *mut timing_generator, dc_crtc_timing: *const dc_crtc_timing, vready_offset: core::ffi::c_int, vstartup_start: core::ffi::c_int, vupdate_offset: core::ffi::c_int, vupdate_width: core::ffi::c_int, pstate_keepout: core::ffi::c_int, signal: signal_type, use_vbios: bool);
    pub fn optc1_setup_vertical_interrupt0(optc: *mut timing_generator, start_line: u32, end_line: u32);
    pub fn optc1_setup_vertical_interrupt1(optc: *mut timing_generator, start_line: u32);
    pub fn optc1_setup_vertical_interrupt2(optc: *mut timing_generator, start_line: u32);
    pub fn optc1_program_global_sync(optc: *mut timing_generator, vready_offset: core::ffi::c_int, vstartup_start: core::ffi::c_int, vupdate_offset: core::ffi::c_int, vupdate_width: core::ffi::c_int, pstate_keepout: core::ffi::c_int);
    pub fn optc1_disable_crtc(optc: *mut timing_generator) -> bool;
    pub fn optc1_is_counter_moving(optc: *mut timing_generator) -> bool;
    pub fn optc1_get_position(optc: *mut timing_generator, position: *mut crtc_position);
    pub fn optc1_get_vblank_counter(optc: *mut timing_generator) -> u32;
    pub fn optc1_get_crtc_scanoutpos(optc: *mut timing_generator, v_blank_start: *mut u32, v_blank_end: *mut u32, h_position: *mut u32, v_position: *mut u32);
    pub fn optc1_set_early_control(optc: *mut timing_generator, early_cntl: u32);
    pub fn optc1_wait_for_state(optc: *mut timing_generator, state: crtc_state);
    pub fn optc1_set_blank(optc: *mut timing_generator, enable_blanking: bool);
    pub fn optc1_is_blanked(optc: *mut timing_generator) -> bool;
    pub fn optc1_program_blank_color(optc: *mut timing_generator, black_color: *const tg_color);
    pub fn optc1_did_triggered_reset_occur(optc: *mut timing_generator) -> bool;
    pub fn optc1_enable_reset_trigger(optc: *mut timing_generator, source_tg_inst: core::ffi::c_int);
    pub fn optc1_disable_reset_trigger(optc: *mut timing_generator);
    pub fn optc1_lock(optc: *mut timing_generator);
    pub fn optc1_unlock(optc: *mut timing_generator);
    pub fn optc1_enable_optc_clock(optc: *mut timing_generator, enable: bool);
    pub fn optc1_set_drr(optc: *mut timing_generator, params: *const drr_params);
    pub fn optc1_set_vtotal_min_max(optc: *mut timing_generator, vtotal_min: core::ffi::c_int, vtotal_max: core::ffi::c_int);
    pub fn optc1_set_static_screen_control(optc: *mut timing_generator, event_triggers: u32, num_frames: u32);
    pub fn optc1_program_stereo(optc: *mut timing_generator, timing: *const dc_crtc_timing, flags: *mut crtc_stereo_flags);
    pub fn optc1_is_stereo_left_eye(optc: *mut timing_generator) -> bool;
    pub fn optc1_clear_optc_underflow(optc: *mut timing_generator);
    pub fn optc1_tg_init(optc: *mut timing_generator);
    pub fn optc1_is_tg_enabled(optc: *mut timing_generator) -> bool;
    pub fn optc1_is_optc_underflow_occurred(optc: *mut timing_generator) -> bool;
    pub fn optc1_set_blank_data_double_buffer(optc: *mut timing_generator, enable: bool);
    pub fn optc1_set_timing_double_buffer(optc: *mut timing_generator, enable: bool);
    pub fn optc1_get_otg_active_size(optc: *mut timing_generator, otg_active_width: *mut u32, otg_active_height: *mut u32) -> bool;
    pub fn optc1_enable_crtc_reset(optc: *mut timing_generator, source_tg_inst: core::ffi::c_int, crtc_tp: *mut crtc_trigger_info);
    pub fn optc1_configure_crc(optc: *mut timing_generator, params: *const crc_params) -> bool;
    pub fn optc1_get_crc(optc: *mut timing_generator, idx: u8, r_cr: *mut u32, g_y: *mut u32, b_cb: *mut u32) -> bool;
    pub fn optc1_set_vtg_params(optc: *mut timing_generator, dc_crtc_timing: *const dc_crtc_timing, program_fp2: bool);
    pub fn optc1_is_two_pixels_per_container(timing: *const dc_crtc_timing) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
