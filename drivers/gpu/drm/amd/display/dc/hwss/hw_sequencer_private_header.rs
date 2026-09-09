/*
 * Copyright 2015-2026 Advanced Micro Devices, Inc.
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

#[repr(C)]
pub enum pipe_gating_control {
    PIPE_GATING_CONTROL_DISABLE = 0,
    PIPE_GATING_CONTROL_ENABLE,
    PIPE_GATING_CONTROL_INIT,
}

#[repr(C)]
pub struct dce_hwseq_wa {
    pub blnd_crtc_trigger: bool,
    pub DEGVIDCN10_253: bool,
    pub false_optc_underflow: bool,
    pub DEGVIDCN10_254: bool,
    pub DEGVIDCN21: bool,
    pub disallow_self_refresh_during_multi_plane_transition: bool,
    pub dp_hpo_and_otg_sequence: bool,
    pub wait_hubpret_read_start_during_mpo_transition: bool,
}

#[repr(C)]
pub struct hwseq_wa_state {
    pub DEGVIDCN10_253_applied: bool,
    pub disallow_self_refresh_during_multi_plane_transition_applied: bool,
    pub disallow_self_refresh_during_multi_plane_transition_applied_on_frame: ::core::ffi::c_uint,
    pub skip_blank_stream: bool,
}

pub enum pipe_ctx {}
pub enum dc_state {}
pub enum dc_stream_status {}
pub enum dc_writeback_info {}
pub enum dchub_init_data {}
pub enum dc_static_screen_params {}
pub enum resource_pool {}
pub enum resource_context {}
pub enum stream_resource {}
pub enum dc_phy_addr_space_config {}
pub enum dc_virtual_addr_space_config {}
pub enum hubp {}
pub enum dpp {}
pub enum transform {}
pub enum mpc {}
pub enum timing_generator {}
pub enum tg_color {}
pub enum output_pixel_processor {}
pub enum mpcc_blnd_cfg {}

#[repr(C)]
pub struct hwseq_private_funcs {
    pub disable_stream_gating: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx)>,
    pub enable_stream_gating: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx)>,
    pub init_pipes: Option<unsafe extern "C" fn(*mut dc, *mut dc_state)>,
    pub reset_hw_ctx_wrap: Option<unsafe extern "C" fn(*mut dc, *mut dc_state)>,
    pub plane_atomic_disconnect: Option<unsafe extern "C" fn(*mut dc, *mut dc_state, *mut pipe_ctx)>,
    pub plane_atomic_disconnect_sequence: Option<unsafe extern "C" fn(*mut dc, *mut dc_state, *mut pipe_ctx, *mut block_sequence_state)>,
    pub update_mpcc: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx)>,
    pub update_mpcc_sequence: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, *mut block_sequence_state)>,
    pub set_input_transfer_func: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, *const dc_plane_state) -> bool>,
    pub set_output_transfer_func: Option<unsafe extern "C" fn(*mut set_output_transfer_func_params) -> bool>,
    pub power_down: Option<unsafe extern "C" fn(*mut dc)>,
    pub enable_display_pipe_clock_gating: Option<unsafe extern "C" fn(*mut dc_context, bool)>,
    pub enable_display_power_gating: Option<unsafe extern "C" fn(*mut dc, u8, *mut dc_bios, pipe_gating_control) -> bool>,
    pub blank_pixel_data: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, bool)>,
    pub blank_pixel_data_sequence: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, bool, *mut block_sequence_state)>,
    pub enable_stream_timing: Option<unsafe extern "C" fn(*mut pipe_ctx, *mut dc_state, *mut dc) -> dc_status>,
    pub edp_backlight_control: Option<unsafe extern "C" fn(*mut dc_link, bool)>,
    pub setup_vupdate_interrupt: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx)>,
    pub setup_vupdate_interrupt_sequence: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, *mut block_sequence_state)>,
    pub did_underflow_occur: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx) -> bool>,
    pub init_blank: Option<unsafe extern "C" fn(*mut dc, *mut timing_generator)>,
    pub disable_vga: Option<unsafe extern "C" fn(*mut dce_hwseq)>,
    pub bios_golden_init: Option<unsafe extern "C" fn(*mut dc)>,
    pub plane_atomic_power_down: Option<unsafe extern "C" fn(*mut dc, *mut dpp, *mut hubp)>,
    pub plane_atomic_power_down_sequence: Option<unsafe extern "C" fn(*mut dc, *mut dpp, *mut hubp, *mut block_sequence_state)>,
    pub plane_atomic_disable: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx)>,
    pub enable_power_gating_plane: Option<unsafe extern "C" fn(*mut dce_hwseq, bool)>,
    pub dpp_root_clock_control: Option<unsafe extern "C" fn(*mut dce_hwseq, ::core::ffi::c_uint, bool)>,
    pub dpstream_root_clock_control: Option<unsafe extern "C" fn(*mut dce_hwseq, ::core::ffi::c_uint, bool)>,
    pub hdmistream_root_clock_control: Option<unsafe extern "C" fn(*mut dce_hwseq, bool)>,
    pub physymclk_root_clock_control: Option<unsafe extern "C" fn(*mut dce_hwseq, ::core::ffi::c_uint, bool)>,
    pub dpp_pg_control: Option<unsafe extern "C" fn(*mut dce_hwseq, ::core::ffi::c_uint, bool)>,
    pub hubp_pg_control: Option<unsafe extern "C" fn(*mut dce_hwseq, ::core::ffi::c_uint, bool)>,
    pub dsc_pg_control: Option<unsafe extern "C" fn(*mut dce_hwseq, ::core::ffi::c_uint, bool)>,
    pub dsc_pg_status: Option<unsafe extern "C" fn(*mut dce_hwseq, ::core::ffi::c_uint) -> bool>,
    pub update_odm: Option<unsafe extern "C" fn(*mut dc, *mut dc_state, *mut pipe_ctx)>,
    pub update_odm_sequence: Option<unsafe extern "C" fn(*mut dc, *mut dc_state, *mut pipe_ctx, *mut block_sequence_state)>,
    pub program_all_writeback_pipes_in_tree: Option<unsafe extern "C" fn(*mut dc, *const dc_stream_state, *mut dc_state)>,
    pub program_all_writeback_pipes_in_tree_sequence: Option<unsafe extern "C" fn(*mut dc, *const dc_stream_state, *mut dc_state, *mut block_sequence_state)>,
    pub s0i3_golden_init_wa: Option<unsafe extern "C" fn(*mut dc) -> bool>,
    pub set_hdr_multiplier: Option<unsafe extern "C" fn(*mut pipe_ctx)>,
    pub set_hdr_multiplier_sequence: Option<unsafe extern "C" fn(*mut pipe_ctx, *mut block_sequence_state)>,
    pub verify_allow_pstate_change_high: Option<unsafe extern "C" fn(*mut dc)>,
    pub verify_allow_pstate_change_high_sequence: Option<unsafe extern "C" fn(*mut dc, *mut block_sequence_state)>,
    pub program_pipe: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, *mut dc_state)>,
    pub program_pipe_sequence: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, *mut dc_state, *mut block_sequence_state)>,
    pub wait_for_blank_complete: Option<unsafe extern "C" fn(*mut output_pixel_processor) -> bool>,
    pub dccg_init: Option<unsafe extern "C" fn(*mut dce_hwseq)>,
    pub set_blend_lut: Option<unsafe extern "C" fn(*mut pipe_ctx, *const dc_plane_state) -> bool>,
    pub set_shaper_3dlut: Option<unsafe extern "C" fn(*mut pipe_ctx, *const dc_plane_state) -> bool>,
    pub set_mcm_luts: Option<unsafe extern "C" fn(*mut pipe_ctx, *const dc_plane_state) -> bool>,
    pub PLAT_58856_wa: Option<unsafe extern "C" fn(*mut dc_state, *mut pipe_ctx)>,
    pub setup_hpo_hw_control: Option<unsafe extern "C" fn(*const dce_hwseq, bool)>,
    pub enable_plane: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, *mut dc_state)>,
    pub program_mall_pipe_config: Option<unsafe extern "C" fn(*mut dc, *mut dc_state)>,
    pub program_mall_pipe_config_sequence: Option<unsafe extern "C" fn(*mut dc, *mut dc_state, *mut block_sequence_state)>,
    pub update_force_pstate: Option<unsafe extern "C" fn(*mut dc, *mut dc_state)>,
    pub update_mall_sel: Option<unsafe extern "C" fn(*mut dc, *mut dc_state)>,
    pub calculate_dccg_k1_k2_values: Option<unsafe extern "C" fn(*mut pipe_ctx, *mut ::core::ffi::c_uint, *mut ::core::ffi::c_uint) -> ::core::ffi::c_uint>,
    pub resync_fifo_dccg_dio: Option<unsafe extern "C" fn(*mut dce_hwseq, *mut dc, *mut dc_state, ::core::ffi::c_uint)>,
    pub apply_single_controller_ctx_to_hw: Option<unsafe extern "C" fn(*mut pipe_ctx, *mut dc_state, *mut dc) -> dc_status>,
    pub is_dp_dig_pixel_rate_div_policy: Option<unsafe extern "C" fn(*mut pipe_ctx) -> bool>,
    pub reset_back_end_for_pipe: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, *mut dc_state)>,
    pub perform_3dlut_wa_unlock: Option<unsafe extern "C" fn(*mut pipe_ctx)>,
    pub wait_for_pipe_update_if_needed: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, bool)>,
    pub set_wait_for_update_needed_for_pipe: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx)>,
    pub dc_ip_request_cntl: Option<unsafe extern "C" fn(*mut dc, bool)>,
    pub program_cm_hist: Option<unsafe extern "C" fn(*mut dc, *mut pipe_ctx, *const dc_plane_state)>,
}

#[repr(C)]
pub struct dce_hwseq {
    pub ctx: *mut dc_context,
    pub regs: *const dce_hwseq_registers,
    pub shifts: *const dce_hwseq_shift,
    pub masks: *const dce_hwseq_mask,
    pub wa: dce_hwseq_wa,
    pub wa_state: hwseq_wa_state,
    pub funcs: hwseq_private_funcs,
    pub fb_base: PHYSICAL_ADDRESS_LOC,
    pub fb_top: PHYSICAL_ADDRESS_LOC,
    pub fb_offset: PHYSICAL_ADDRESS_LOC,
    pub uma_top: PHYSICAL_ADDRESS_LOC,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
