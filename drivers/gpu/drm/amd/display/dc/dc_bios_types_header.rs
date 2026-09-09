/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// Interface file for VBIOS implementations.
// The default implementation is inside DC. Display Manager may supply an
// external implementation of VBIOS, which is called by DC through this
// interface.

#[repr(C)]
pub struct dc_vbios_funcs {
    pub get_connectors_number: Option<unsafe extern "C" fn(*mut dc_bios) -> u8>,
    pub get_connector_id: Option<unsafe extern "C" fn(*mut dc_bios, u8) -> graphics_object_id>,
    pub get_src_obj: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, u32, *mut graphics_object_id) -> bp_result>,
    pub get_i2c_info: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, *mut graphics_object_i2c_info) -> bp_result>,
    pub get_hpd_info: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, *mut graphics_object_hpd_info) -> bp_result>,
    pub get_device_tag: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, u32, *mut connector_device_tag_info) -> bp_result>,
    pub get_spread_spectrum_info: Option<unsafe extern "C" fn(*mut dc_bios, as_signal_type, u32, *mut spread_spectrum_info) -> bp_result>,
    pub get_ss_entry_number: Option<unsafe extern "C" fn(*mut dc_bios, as_signal_type) -> u32>,
    pub get_embedded_panel_info: Option<unsafe extern "C" fn(*mut dc_bios, *mut embedded_panel_info) -> bp_result>,
    pub get_gpio_pin_info: Option<unsafe extern "C" fn(*mut dc_bios, u32, *mut gpio_pin_info) -> bp_result>,
    pub get_encoder_cap_info: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, *mut bp_encoder_cap_info) -> bp_result>,
    pub is_accelerated_mode: Option<unsafe extern "C" fn(*mut dc_bios) -> bool>,
    pub set_scratch_critical_state: Option<unsafe extern "C" fn(*mut dc_bios, bool)>,
    pub is_device_id_supported: Option<unsafe extern "C" fn(*mut dc_bios, device_id) -> bool>,
    // COMMANDS
    pub select_crtc_source: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_crtc_source_select) -> bp_result>,
    pub encoder_control: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_encoder_control) -> bp_result>,
    pub external_encoder_control: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_external_encoder_control) -> bp_result>,
    pub dac_load_detection: Option<unsafe extern "C" fn(*mut dc_bios, engine_id, graphics_object_id) -> bp_result>,
    pub transmitter_control: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_transmitter_control) -> bp_result>,
    pub enable_crtc: Option<unsafe extern "C" fn(*mut dc_bios, controller_id, bool) -> bp_result>,
    pub adjust_pixel_clock: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_adjust_pixel_clock_parameters) -> bp_result>,
    pub set_pixel_clock: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_pixel_clock_parameters) -> bp_result>,
    pub set_dce_clock: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_set_dce_clock_parameters) -> bp_result>,
    pub enable_spread_spectrum_on_ppll: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_spread_spectrum_parameters, bool) -> bp_result>,
    pub program_crtc_timing: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_hw_crtc_timing_parameters) -> bp_result>,
    pub program_display_engine_pll: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_pixel_clock_parameters) -> bp_result>,
    pub enable_disp_power_gating: Option<unsafe extern "C" fn(*mut dc_bios, controller_id, bp_pipe_control_action) -> bp_result>,
    pub bios_parser_destroy: Option<unsafe extern "C" fn(*mut *mut dc_bios)>,
    pub get_board_layout_info: Option<unsafe extern "C" fn(*mut dc_bios, *mut board_layout_info) -> bp_result>,
    pub pack_data_tables: Option<unsafe extern "C" fn(*mut dc_bios, *mut core::ffi::c_void) -> u16>,
    pub get_atom_dc_golden_table: Option<unsafe extern "C" fn(*mut dc_bios) -> bp_result>,
    pub enable_lvtma_control: Option<unsafe extern "C" fn(*mut dc_bios, u8, u8, u8) -> bp_result>,
    pub get_soc_bb_info: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_soc_bb_info) -> bp_result>,
    pub get_disp_connector_caps_info: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, *mut bp_disp_connector_caps_info) -> bp_result>,
    pub get_lttpr_caps: Option<unsafe extern "C" fn(*mut dc_bios, *mut u8) -> bp_result>,
    pub get_lttpr_interop: Option<unsafe extern "C" fn(*mut dc_bios, *mut u8) -> bp_result>,
    pub get_connector_speed_cap_info: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, *mut bp_connector_speed_cap_info) -> bp_result>,
    pub get_connector_aux_info: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, *mut graphics_object_i2c_info) -> bp_result>,
}

#[repr(C)]
pub struct bios_registers {
    pub BIOS_SCRATCH_0: u32,
    pub BIOS_SCRATCH_3: u32,
    pub BIOS_SCRATCH_6: u32,
}

#[repr(C)]
pub struct dc_bios {
    pub funcs: *const dc_vbios_funcs,
    pub bios: *mut u8,
    pub bios_size: u32,
    pub bios_local_image: *mut u8,
    pub ctx: *mut dc_context,
    pub regs: *const bios_registers,
    pub integrated_info: *mut integrated_info,
    pub fw_info: dc_firmware_info,
    pub fw_info_valid: bool,
    pub vram_info: dc_vram_info,
    pub bb_info: bp_soc_bb_info,
    pub golden_table: dc_golden_table,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
