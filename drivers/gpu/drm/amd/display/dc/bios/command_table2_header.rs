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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C header guard: __DAL_COMMAND_TABLE2_H__

pub enum bios_parser {}
pub enum bp_encoder_control {}
pub enum bp_transmitter_control {}
pub enum bp_pixel_clock_parameters {}
pub enum bp_spread_spectrum_parameters {}
pub enum bp_adjust_pixel_clock_parameters {}
pub enum bp_hw_crtc_timing_parameters {}
pub enum bp_external_encoder_control {}
pub enum bp_set_dce_clock_parameters {}
pub enum bp_result {}
pub enum controller_id {}
pub enum bp_pipe_control_action {}

#[repr(C)]
pub struct cmd_tbl {
    pub dig_encoder_control: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_encoder_control) -> bp_result>,
    pub encoder_control_dig1: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_encoder_control) -> bp_result>,
    pub encoder_control_dig2: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_encoder_control) -> bp_result>,
    pub transmitter_control: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_transmitter_control) -> bp_result>,
    pub set_pixel_clock: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_pixel_clock_parameters) -> bp_result>,
    pub enable_spread_spectrum_on_ppll: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_spread_spectrum_parameters, bool) -> bp_result>,
    pub adjust_display_pll: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_adjust_pixel_clock_parameters) -> bp_result>,
    pub dac1_encoder_control: Option<unsafe extern "C" fn(*mut bios_parser, bool, u32, u8) -> bp_result>,
    pub dac2_encoder_control: Option<unsafe extern "C" fn(*mut bios_parser, bool, u32, u8) -> bp_result>,
    pub dac1_output_control: Option<unsafe extern "C" fn(*mut bios_parser, bool) -> bp_result>,
    pub dac2_output_control: Option<unsafe extern "C" fn(*mut bios_parser, bool) -> bp_result>,
    pub set_crtc_timing: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_hw_crtc_timing_parameters) -> bp_result>,
    pub enable_crtc: Option<unsafe extern "C" fn(*mut bios_parser, controller_id, bool) -> bp_result>,
    pub enable_crtc_mem_req: Option<unsafe extern "C" fn(*mut bios_parser, controller_id, bool) -> bp_result>,
    pub program_clock: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_pixel_clock_parameters) -> bp_result>,
    pub external_encoder_control: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_external_encoder_control) -> bp_result>,
    pub enable_disp_power_gating: Option<unsafe extern "C" fn(*mut bios_parser, controller_id, bp_pipe_control_action) -> bp_result>,
    pub set_dce_clock: Option<unsafe extern "C" fn(*mut bios_parser, *mut bp_set_dce_clock_parameters) -> bp_result>,
    pub get_smu_clock_info: Option<unsafe extern "C" fn(*mut bios_parser, u8) -> u32>,
    pub enable_lvtma_control: Option<unsafe extern "C" fn(*mut bios_parser, u8, u8, u8) -> bp_result>,
}

extern "C" {
    pub fn dal_firmware_parser_init_cmd_tbl(bp: *mut bios_parser);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
