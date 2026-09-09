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

// Dependencies supplied by the surrounding translation unit:
// mem_input.h, cursor_reg_cache.h, dml_top_dchub_registers.h, dml_top_types.h

pub const OPP_ID_INVALID: u32 = 0xf;
pub const MAX_TTU: u32 = 0xffffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cursor_pitch { CURSOR_PITCH_64_PIXELS = 0, CURSOR_PITCH_128_PIXELS, CURSOR_PITCH_256_PIXELS }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum cursor_lines_per_chunk { CURSOR_LINE_PER_CHUNK_1 = 0, CURSOR_LINE_PER_CHUNK_2 = 1, CURSOR_LINE_PER_CHUNK_4, CURSOR_LINE_PER_CHUNK_8, CURSOR_LINE_PER_CHUNK_16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum hubp_ind_block_size { hubp_ind_block_unconstrained = 0, hubp_ind_block_64b, hubp_ind_block_128b, hubp_ind_block_64b_no_128bcl }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum hubp_3dlut_fl_mode { hubp_3dlut_fl_mode_disable = 0, hubp_3dlut_fl_mode_native_1 = 1, hubp_3dlut_fl_mode_native_2 = 2, hubp_3dlut_fl_mode_transform = 3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum hubp_3dlut_fl_format { hubp_3dlut_fl_format_unorm_12msb_bitslice = 0, hubp_3dlut_fl_format_unorm_12lsb_bitslice = 1, hubp_3dlut_fl_format_float_fp1_5_10 = 2 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum hubp_3dlut_fl_addressing_mode { hubp_3dlut_fl_addressing_mode_sw_linear = 0, hubp_3dlut_fl_addressing_mode_simple_linear = 1 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum hubp_3dlut_fl_width { hubp_3dlut_fl_width_17 = 17, hubp_3dlut_fl_width_33 = 33, hubp_3dlut_fl_width_17_transformed = 4916 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum hubp_3dlut_fl_crossbar_bit_slice { hubp_3dlut_fl_crossbar_bit_slice_0_15 = 0, hubp_3dlut_fl_crossbar_bit_slice_16_31 = 1, hubp_3dlut_fl_crossbar_bit_slice_32_47 = 2, hubp_3dlut_fl_crossbar_bit_slice_48_63 = 3 }

#[repr(C)]
pub struct hubp {
    pub funcs: *const hubp_funcs,
    pub ctx: *mut dc_context,
    pub request_address: dc_plane_address,
    pub inst: ::core::ffi::c_int,
    pub opp_id: ::core::ffi::c_int,
    pub mpcc_id: ::core::ffi::c_int,
    pub curs_attr: dc_cursor_attributes,
    pub curs_pos: dc_cursor_position,
    pub cursor_offload: bool,
    pub power_gated: bool,
    pub pos: cursor_position_cache_hubp,
    pub att: cursor_attribute_cache_hubp,
    pub cur_rect: cursor_rect,
    pub use_mall_for_cursor: bool,
}

#[repr(C)]
pub struct surface_flip_registers {
    pub DCSURF_SURFACE_CONTROL: u32, pub DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH: u32, pub DCSURF_PRIMARY_META_SURFACE_ADDRESS: u32,
    pub DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH: u32, pub DCSURF_PRIMARY_SURFACE_ADDRESS: u32, pub DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C: u32,
    pub DCSURF_PRIMARY_META_SURFACE_ADDRESS_C: u32, pub DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C: u32, pub DCSURF_PRIMARY_SURFACE_ADDRESS_C: u32,
    pub DCSURF_SECONDARY_META_SURFACE_ADDRESS_HIGH: u32, pub DCSURF_SECONDARY_META_SURFACE_ADDRESS: u32,
    pub DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH: u32, pub DCSURF_SECONDARY_SURFACE_ADDRESS: u32,
    pub tmz_surface: u8, pub immediate: bool, pub vmid: u8, pub grph_stereo: bool,
}

// Function-pointer interface preserved from struct hubp_funcs. External types are supplied by dependencies.
#[repr(C)]
pub struct hubp_funcs {
    pub hubp_setup: Option<unsafe extern "C" fn(*mut hubp, *mut _vcs_dpi_display_dlg_regs_st, *mut _vcs_dpi_display_ttu_regs_st, *mut _vcs_dpi_display_rq_regs_st, *mut _vcs_dpi_display_pipe_dest_params_st)>,
    pub hubp_setup2: Option<unsafe extern "C" fn(*mut hubp, *mut dml2_dchub_per_pipe_register_set, *mut dml2_global_sync_programming, *mut dc_crtc_timing)>,
    pub hubp_setup_interdependent: Option<unsafe extern "C" fn(*mut hubp, *mut _vcs_dpi_display_dlg_regs_st, *mut _vcs_dpi_display_ttu_regs_st)>,
    pub hubp_setup_interdependent2: Option<unsafe extern "C" fn(*mut hubp, *mut dml2_dchub_per_pipe_register_set)>,
    pub dcc_control: Option<unsafe extern "C" fn(*mut hubp, bool, hubp_ind_block_size)>,
    pub hubp_reset: Option<unsafe extern "C" fn(*mut hubp)>,
    pub mem_program_viewport: Option<unsafe extern "C" fn(*mut hubp, *const rect, *const rect)>,
    pub hubp_program_surface_flip_and_addr: Option<unsafe extern "C" fn(*mut hubp, *const dc_plane_address, bool) -> bool>,
    pub hubp_program_pte_vm: Option<unsafe extern "C" fn(*mut hubp, surface_pixel_format, *mut dc_tiling_info, dc_rotation_angle)>,
    pub hubp_set_vm_system_aperture_settings: Option<unsafe extern "C" fn(*mut hubp, *mut vm_system_aperture_param)>,
    pub hubp_set_vm_context0_settings: Option<unsafe extern "C" fn(*mut hubp, *const vm_context0_param)>,
    pub hubp_program_surface_config: Option<unsafe extern "C" fn(*mut hubp, surface_pixel_format, *mut dc_tiling_info, *mut plane_size, dc_rotation_angle, *mut dc_plane_dcc_param, bool, ::core::ffi::c_uint)>,
    pub hubp_is_flip_pending: Option<unsafe extern "C" fn(*mut hubp) -> bool>,
    pub set_blank: Option<unsafe extern "C" fn(*mut hubp, bool)>, pub set_blank_regs: Option<unsafe extern "C" fn(*mut hubp, bool)>,
    pub phantom_hubp_post_enable: Option<unsafe extern "C" fn(*mut hubp)>, pub set_hubp_blank_en: Option<unsafe extern "C" fn(*mut hubp, bool)>,
    pub set_cursor_attributes: Option<unsafe extern "C" fn(*mut hubp, *const dc_cursor_attributes)>,
    pub set_cursor_position: Option<unsafe extern "C" fn(*mut hubp, *const dc_cursor_position, *const dc_cursor_mi_param)>,
    pub hubp_disconnect: Option<unsafe extern "C" fn(*mut hubp)>, pub hubp_clk_cntl: Option<unsafe extern "C" fn(*mut hubp, bool)>,
    pub hubp_vtg_sel: Option<unsafe extern "C" fn(*mut hubp, u32)>, pub hubp_read_state: Option<unsafe extern "C" fn(*mut hubp)>,
    pub hubp_read_reg_state: Option<unsafe extern "C" fn(*mut hubp, *mut dcn_hubp_reg_state)>, pub hubp_clear_underflow: Option<unsafe extern "C" fn(*mut hubp)>,
    pub hubp_disable_control: Option<unsafe extern "C" fn(*mut hubp, bool)>, pub hubp_get_underflow_status: Option<unsafe extern "C" fn(*mut hubp) -> ::core::ffi::c_uint>,
    pub hubp_init: Option<unsafe extern "C" fn(*mut hubp)>, pub dmdata_set_attributes: Option<unsafe extern "C" fn(*mut hubp, *const dc_dmdata_attributes)>,
    pub dmdata_load: Option<unsafe extern "C" fn(*mut hubp, u32, *const u32)>, pub dmdata_status_done: Option<unsafe extern "C" fn(*mut hubp) -> bool>,
    pub hubp_enable_tripleBuffer: Option<unsafe extern "C" fn(*mut hubp, bool)>, pub hubp_is_triplebuffer_enabled: Option<unsafe extern "C" fn(*mut hubp) -> bool>,
    pub hubp_set_flip_control_surface_gsl: Option<unsafe extern "C" fn(*mut hubp, bool)>,
    pub validate_dml_output: Option<unsafe extern "C" fn(*mut hubp, *mut dc_context, *mut _vcs_dpi_display_rq_regs_st, *mut _vcs_dpi_display_dlg_regs_st, *mut _vcs_dpi_display_ttu_regs_st)>,
    pub set_unbounded_requesting: Option<unsafe extern "C" fn(*mut hubp, bool)>, pub hubp_in_blank: Option<unsafe extern "C" fn(*mut hubp) -> bool>,
    pub hubp_soft_reset: Option<unsafe extern "C" fn(*mut hubp, bool)>, pub hubp_set_flip_int: Option<unsafe extern "C" fn(*mut hubp)>,
    pub hubp_update_force_pstate_disallow: Option<unsafe extern "C" fn(*mut hubp, bool)>, pub hubp_update_force_cursor_pstate_disallow: Option<unsafe extern "C" fn(*mut hubp, bool)>,
    pub hubp_update_mall_sel: Option<unsafe extern "C" fn(*mut hubp, u32, bool)>, pub hubp_prepare_subvp_buffering: Option<unsafe extern "C" fn(*mut hubp, bool)>,
    pub hubp_surface_update_lock: Option<unsafe extern "C" fn(*mut hubp, bool)>, pub program_extended_blank: Option<unsafe extern "C" fn(*mut hubp, ::core::ffi::c_uint)>,
    pub hubp_wait_pipe_read_start: Option<unsafe extern "C" fn(*mut hubp)>, pub hubp_program_mcache_id_and_split_coordinate: Option<unsafe extern "C" fn(*mut hubp, *mut dml2_hubp_pipe_mcache_regs)>,
    pub hubp_program_3dlut_fl_addr: Option<unsafe extern "C" fn(*mut hubp, *const dc_plane_address)>, pub hubp_program_3dlut_fl_config: Option<unsafe extern "C" fn(*mut hubp, *const dc_3dlut_dma)>,
    pub hubp_program_3dlut_fl_dlg_param: Option<unsafe extern "C" fn(*mut hubp, ::core::ffi::c_int)>, pub hubp_enable_3dlut_fl: Option<unsafe extern "C" fn(*mut hubp, bool)>,
    pub hubp_program_3dlut_fl_crossbar: Option<unsafe extern "C" fn(*mut hubp, dc_cm_lut_pixel_format)>, pub hubp_get_3dlut_fl_done: Option<unsafe extern "C" fn(*mut hubp) -> u32>,
    pub hubp_clear_tiling: Option<unsafe extern "C" fn(*mut hubp)>, pub hubp_get_current_read_line: Option<unsafe extern "C" fn(*mut hubp) -> u32>,
    pub hubp_get_det_config_error: Option<unsafe extern "C" fn(*mut hubp) -> u32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
