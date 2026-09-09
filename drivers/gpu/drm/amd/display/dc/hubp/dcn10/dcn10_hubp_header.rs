/* Copyright 2012-15 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C dependency: hubp.h

// C preprocessor register-list and mask/shift-generation macros are retained
// here as declarative macro intent; their register identifiers are supplied by
// the generated register definitions in the surrounding translation unit.
macro_rules! TO_DCN10_HUBP { ($hubp:expr) => { unsafe { &mut *((($hubp as *mut u8).sub(core::mem::offset_of!(dcn10_hubp, base))) as *mut dcn10_hubp) } }; }

#[repr(C)]
pub struct dcn_mi_registers {
    pub fields: [u32; 122],
}

#[repr(C)]
pub struct dcn_mi_shift {
    pub fields: [u8; 190],
}

#[repr(C)]
pub struct dcn_mi_mask {
    pub fields: [u32; 190],
}

#[repr(C)]
pub struct dcn_fl_regs_st {
    pub lut_enable: u32, pub lut_done: u32, pub lut_addr_mode: u32,
    pub lut_width: u32, pub lut_mpc_width: u32, pub lut_tmz: u32,
    pub lut_crossbar_sel_r: u32, pub lut_crossbar_sel_g: u32,
    pub lut_crossbar_sel_b: u32, pub lut_addr_hi: u32, pub lut_addr_lo: u32,
    pub refcyc_3dlut_group: u32, pub lut_fl_bias: u32, pub lut_fl_scale: u32,
    pub lut_fl_mode: u32, pub lut_fl_format: u32,
}

// The following state structures preserve the C field order and integer layout.
#[repr(C)]
pub struct dcn_hubp_reg_state {
    pub values: [u32; 133],
}

#[repr(C)]
pub struct dcn_hubp_state {
    pub dlg_attr: _vcs_dpi_display_dlg_regs_st,
    pub ttu_attr: _vcs_dpi_display_ttu_regs_st,
    pub rq_regs: _vcs_dpi_display_rq_regs_st,
    pub fl_regs: dcn_fl_regs_st,
    pub pixel_format: u32, pub inuse_addr_hi: u32, pub inuse_addr_lo: u32,
    pub viewport_width: u32, pub viewport_height: u32, pub rotation_angle: u32,
    pub h_mirror_en: u32, pub sw_mode: u32, pub dcc_en: u32,
    pub blank_en: u32, pub clock_en: u32, pub underflow_status: u32,
    pub ttu_disable: u32, pub min_ttu_vblank: u32, pub qos_level_low_wm: u32,
    pub qos_level_high_wm: u32, pub primary_surface_addr_lo: u32,
    pub primary_surface_addr_hi: u32, pub primary_meta_addr_lo: u32,
    pub primary_meta_addr_hi: u32, pub uclk_pstate_force: u32,
    pub hubp_cntl: u32, pub flip_control: u32,
}

#[repr(C)]
pub struct dcn10_hubp {
    pub base: hubp,
    pub state: dcn_hubp_state,
    pub hubp_regs: *const dcn_mi_registers,
    pub hubp_shift: *const dcn_mi_shift,
    pub hubp_mask: *const dcn_mi_mask,
}

extern "C" {
    pub fn hubp1_program_surface_config(hubp: *mut hubp, format: surface_pixel_format, tiling_info: *mut dc_tiling_info, plane_size: *mut plane_size, rotation: dc_rotation_angle, dcc: *mut dc_plane_dcc_param, horizontal_mirror: bool, compat_level: ::core::ffi::c_uint);
    pub fn hubp1_program_deadline(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st);
    pub fn hubp1_program_requestor(hubp: *mut hubp, rq_regs: *mut _vcs_dpi_display_rq_regs_st);
    pub fn hubp1_program_pixel_format(hubp: *mut hubp, format: surface_pixel_format);
    pub fn hubp1_program_size(hubp: *mut hubp, format: surface_pixel_format, plane_size: *const plane_size, dcc: *mut dc_plane_dcc_param);
    pub fn hubp1_program_rotation(hubp: *mut hubp, rotation: dc_rotation_angle, horizontal_mirror: bool);
    pub fn hubp1_program_tiling(hubp: *mut hubp, info: *const dc_tiling_info, pixel_format: surface_pixel_format);
    pub fn hubp1_dcc_control(hubp: *mut hubp, enable: bool, independent_64b_blks: hubp_ind_block_size);
    pub fn hubp_reset(hubp: *mut hubp);
    pub fn hubp1_program_surface_flip_and_addr(hubp: *mut hubp, address: *const dc_plane_address, flip_immediate: bool) -> bool;
    pub fn hubp1_is_flip_pending(hubp: *mut hubp) -> bool;
    pub fn hubp1_cursor_set_attributes(hubp: *mut hubp, attr: *const dc_cursor_attributes);
    pub fn hubp1_cursor_set_position(hubp: *mut hubp, pos: *const dc_cursor_position, param: *const dc_cursor_mi_param);
    pub fn hubp1_set_blank(hubp: *mut hubp, blank: bool);
    pub fn min_set_viewport(hubp: *mut hubp, viewport: *const rect, viewport_c: *const rect);
    pub fn hubp1_clk_cntl(hubp: *mut hubp, enable: bool);
    pub fn hubp1_vtg_sel(hubp: *mut hubp, otg_inst: u32);
    pub fn dcn10_hubp_construct(hubp1: *mut dcn10_hubp, ctx: *mut dc_context, inst: u32, hubp_regs: *const dcn_mi_registers, hubp_shift: *const dcn_mi_shift, hubp_mask: *const dcn_mi_mask);
    pub fn hubp1_read_state(hubp: *mut hubp);
    pub fn hubp1_clear_underflow(hubp: *mut hubp);
    pub fn hubp1_get_cursor_pitch(pitch: ::core::ffi::c_uint) -> cursor_pitch;
    pub fn hubp1_vready_workaround(hubp: *mut hubp, pipe_dest: *mut _vcs_dpi_display_pipe_dest_params_st);
    pub fn hubp1_init(hubp: *mut hubp);
    pub fn hubp1_read_state_common(hubp: *mut hubp);
    pub fn hubp1_in_blank(hubp: *mut hubp) -> bool;
    pub fn hubp1_soft_reset(hubp: *mut hubp, reset: bool);
    pub fn hubp1_set_flip_int(hubp: *mut hubp);
    pub fn hubp1_clear_tiling(hubp: *mut hubp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
