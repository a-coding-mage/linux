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

// C dependencies: dc.h, include/grph_object_id.h,
// dml/display_mode_structs.h, and dml2_0/dml21/inc/dml_top_dchub_registers.h.

#[repr(C)]
pub struct cstate_pstate_watermarks_st {
    pub cstate_exit_ns: u32,
    pub cstate_exit_z8_ns: u32,
    pub cstate_enter_plus_exit_z8_ns: u32,
    pub cstate_enter_plus_exit_ns: u32,
    pub pstate_change_ns: u32,
    pub fclk_pstate_change_ns: u32,
}

#[repr(C)]
pub struct dcn_watermarks {
    pub pte_meta_urgent_ns: u32,
    pub urgent_ns: u32,
    pub frac_urg_bw_nom: u32,
    pub frac_urg_bw_flip: u32,
    pub urgent_latency_ns: u32,
    pub cstate_pstate: cstate_pstate_watermarks_st,
    pub usr_retraining_ns: u32,
}

#[repr(C)]
pub struct dcn_watermark_set_legacy {
    pub a: dcn_watermarks,
    pub b: dcn_watermarks,
    pub c: dcn_watermarks,
    pub d: dcn_watermarks,
}

#[repr(C)]
pub struct dcn_watermark_set_dcn4x {
    pub a: dml2_dchub_watermark_regs,
    pub b: dml2_dchub_watermark_regs,
    pub c: dml2_dchub_watermark_regs,
    pub d: dml2_dchub_watermark_regs,
}

#[repr(C)]
pub union dcn_watermark_set {
    // legacy
    pub legacy: dcn_watermark_set_legacy,
    // dcn4+
    pub dcn4x: dcn_watermark_set_dcn4x,
}

#[repr(C)]
pub struct dce_watermarks {
    pub a_mark: i32,
    pub b_mark: i32,
    pub c_mark: i32,
    pub d_mark: i32,
}

#[repr(C)]
pub struct stutter_modes {
    pub enhanced: bool,
    pub quad_dmif_buffer: bool,
    pub watermark_nb_pstate: bool,
}

#[repr(C)]
pub struct mem_input {
    pub funcs: *const mem_input_funcs,
    pub ctx: *mut dc_context,
    pub request_address: dc_plane_address,
    pub current_address: dc_plane_address,
    pub inst: i32,
    pub stutter_mode: stutter_modes,
}

#[repr(C)]
pub struct vm_system_aperture_param {
    pub sys_default: PHYSICAL_ADDRESS_LOC,
    pub sys_low: PHYSICAL_ADDRESS_LOC,
    pub sys_high: PHYSICAL_ADDRESS_LOC,
}

#[repr(C)]
pub struct vm_context0_param {
    pub pte_base: PHYSICAL_ADDRESS_LOC,
    pub pte_start: PHYSICAL_ADDRESS_LOC,
    pub pte_end: PHYSICAL_ADDRESS_LOC,
    pub fault_default: PHYSICAL_ADDRESS_LOC,
}

#[repr(C)]
pub struct mem_input_funcs {
    pub mem_input_setup: Option<unsafe extern "C" fn(*mut mem_input, *mut _vcs_dpi_display_dlg_regs_st, *mut _vcs_dpi_display_ttu_regs_st, *mut _vcs_dpi_display_rq_regs_st, *mut _vcs_dpi_display_pipe_dest_params_st)>,
    pub dcc_control: Option<unsafe extern "C" fn(*mut mem_input, bool, bool)>,
    pub mem_program_viewport: Option<unsafe extern "C" fn(*mut mem_input, *const rect, *const rect)>,
    pub mem_input_program_display_marks: Option<unsafe extern "C" fn(*mut mem_input, dce_watermarks, dce_watermarks, dce_watermarks, dce_watermarks, u32)>,
    pub mem_input_program_chroma_display_marks: Option<unsafe extern "C" fn(*mut mem_input, dce_watermarks, dce_watermarks, dce_watermarks, u32)>,
    pub allocate_mem_input: Option<unsafe extern "C" fn(*mut mem_input, u32, u32, u32, u32)>,
    pub free_mem_input: Option<unsafe extern "C" fn(*mut mem_input, u32)>,
    pub mem_input_program_surface_flip_and_addr: Option<unsafe extern "C" fn(*mut mem_input, *const dc_plane_address, bool) -> bool>,
    pub mem_input_program_pte_vm: Option<unsafe extern "C" fn(*mut mem_input, surface_pixel_format, *mut dc_tiling_info, dc_rotation_angle)>,
    pub mem_input_set_vm_system_aperture_settings: Option<unsafe extern "C" fn(*mut mem_input, *mut vm_system_aperture_param)>,
    pub mem_input_set_vm_context0_settings: Option<unsafe extern "C" fn(*mut mem_input, *const vm_context0_param)>,
    pub mem_input_program_surface_config: Option<unsafe extern "C" fn(*mut mem_input, surface_pixel_format, *mut dc_tiling_info, *mut plane_size, dc_rotation_angle, *mut dc_plane_dcc_param, bool)>,
    pub mem_input_is_flip_pending: Option<unsafe extern "C" fn(*mut mem_input) -> bool>,
    pub mem_input_update_dchub: Option<unsafe extern "C" fn(*mut mem_input, *mut dchub_init_data)>,
    pub set_blank: Option<unsafe extern "C" fn(*mut mem_input, bool)>,
    pub set_hubp_blank_en: Option<unsafe extern "C" fn(*mut mem_input, bool)>,
    pub set_cursor_attributes: Option<unsafe extern "C" fn(*mut mem_input, *const dc_cursor_attributes)>,
    pub set_cursor_position: Option<unsafe extern "C" fn(*mut mem_input, *const dc_cursor_position, *const dc_cursor_mi_param)>,
    pub mem_input_clear_tiling: Option<unsafe extern "C" fn(*mut mem_input)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
