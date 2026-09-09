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

// C dependencies supplied by the surrounding translation unit.

pub unsafe fn hubp35_set_fgcg(hubp: *mut hubp, enable: bool) {
    let hubp2: *mut dcn20_hubp = TO_DCN20_HUBP(hubp);
    REG_UPDATE!(
        (*(*hubp2).hubp_regs).HUBP_CLK_CNTL,
        HUBP_FGCG_REP_DIS,
        !enable
    );
}

pub unsafe fn hubp35_init(hubp: *mut hubp) {
    hubp3_init(hubp);
    hubp35_set_fgcg(
        hubp,
        (*(*(*hubp).ctx).dc).debug.enable_fine_grain_clock_gating.bits.dchub,
    );
    /* do nothing for now for dcn3.5 or later */
}

pub unsafe fn hubp35_program_pixel_format(
    hubp: *mut hubp,
    format: surface_pixel_format,
) {
    let hubp2: *mut dcn20_hubp = TO_DCN20_HUBP(hubp);
    let mut green_bar: u32 = 1;
    let mut red_bar: u32 = 3;
    let mut blue_bar: u32 = 2;

    /* swap for ABGR format */
    if format == SURFACE_PIXEL_FORMAT_GRPH_ABGR8888
        || format == SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010
        || format == SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS
        || format == SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616
        || format == SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F
    {
        red_bar = 2;
        blue_bar = 3;
    }

    REG_UPDATE_3!(
        HUBPRET_CONTROL,
        CROSSBAR_SRC_Y_G, green_bar,
        CROSSBAR_SRC_CB_B, blue_bar,
        CROSSBAR_SRC_CR_R, red_bar
    );

    /* Mapping is same as ipp programming (cnvc) */
    match format {
        SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 1),
        SURFACE_PIXEL_FORMAT_GRPH_RGB565 => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 3),
        SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 8),
        SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010
        | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010
        | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 10),
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616
        | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 26),
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F
        | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 24),
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 65),
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 64),
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 67),
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 66),
        SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888 => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 12),
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 112),
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 113),
        SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010 => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 114),
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 118),
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT => REG_UPDATE!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 119),
        SURFACE_PIXEL_FORMAT_GRPH_RGBE => REG_UPDATE_2!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 116, ALPHA_PLANE_EN, 0),
        SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA => REG_UPDATE_2!(DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, 116, ALPHA_PLANE_EN, 1),
        _ => BREAK_TO_DEBUGGER!(),
    }
    /* don't see the need of program the xbar in DCN 1.0 */
}

pub unsafe fn hubp35_program_surface_config(
    hubp: *mut hubp,
    format: surface_pixel_format,
    tiling_info: *mut dc_tiling_info,
    plane_size: *mut plane_size,
    rotation: dc_rotation_angle,
    dcc: *mut dc_plane_dcc_param,
    horizontal_mirror: bool,
    compat_level: c_uint,
) {
    let _ = compat_level;
    let hubp2: *mut dcn20_hubp = TO_DCN20_HUBP(hubp);
    hubp3_dcc_control_sienna_cichlid(hubp, dcc);
    hubp3_program_tiling(hubp2, tiling_info, format);
    hubp2_program_size(hubp, format, plane_size, dcc);
    hubp2_program_rotation(hubp, rotation, horizontal_mirror);
    hubp35_program_pixel_format(hubp, format);
}

static mut dcn35_hubp_funcs: hubp_funcs = hubp_funcs {
    hubp_enable_tripleBuffer: Some(hubp2_enable_triplebuffer),
    hubp_is_triplebuffer_enabled: Some(hubp2_is_triplebuffer_enabled),
    hubp_program_surface_flip_and_addr: Some(hubp3_program_surface_flip_and_addr),
    hubp_program_surface_config: Some(hubp35_program_surface_config),
    hubp_is_flip_pending: Some(hubp2_is_flip_pending),
    hubp_setup: Some(hubp3_setup),
    hubp_setup_interdependent: Some(hubp2_setup_interdependent),
    hubp_set_vm_system_aperture_settings: Some(hubp3_set_vm_system_aperture_settings),
    set_blank: Some(hubp2_set_blank),
    dcc_control: Some(hubp3_dcc_control),
    hubp_reset: Some(hubp_reset),
    mem_program_viewport: Some(min_set_viewport),
    set_cursor_attributes: Some(hubp2_cursor_set_attributes),
    set_cursor_position: Some(hubp2_cursor_set_position),
    hubp_clk_cntl: Some(hubp2_clk_cntl),
    hubp_vtg_sel: Some(hubp2_vtg_sel),
    dmdata_set_attributes: Some(hubp3_dmdata_set_attributes),
    dmdata_load: Some(hubp2_dmdata_load),
    dmdata_status_done: Some(hubp2_dmdata_status_done),
    hubp_read_state: Some(hubp3_read_state),
    hubp_read_reg_state: Some(hubp3_read_reg_state),
    hubp_clear_underflow: Some(hubp2_clear_underflow),
    hubp_set_flip_control_surface_gsl: Some(hubp2_set_flip_control_surface_gsl),
    hubp_init: Some(hubp35_init),
    set_unbounded_requesting: Some(hubp31_set_unbounded_requesting),
    hubp_soft_reset: Some(hubp31_soft_reset),
    hubp_set_flip_int: Some(hubp1_set_flip_int),
    hubp_in_blank: Some(hubp1_in_blank),
    program_extended_blank: Some(hubp31_program_extended_blank_value),
    hubp_clear_tiling: Some(hubp3_clear_tiling),
};

pub unsafe fn hubp35_construct(
    hubp2: *mut dcn20_hubp,
    ctx: *mut dc_context,
    inst: u32,
    hubp_regs: *const dcn_hubp2_registers,
    hubp_shift: *const dcn35_hubp2_shift,
    hubp_mask: *const dcn35_hubp2_mask,
) -> bool {
    (*hubp2).base.funcs = &raw const dcn35_hubp_funcs;
    (*hubp2).base.ctx = ctx;
    (*hubp2).hubp_regs = hubp_regs;
    (*hubp2).hubp_shift = hubp_shift as *const dcn_hubp2_shift;
    (*hubp2).hubp_mask = hubp_mask as *const dcn_hubp2_mask;
    (*hubp2).base.inst = inst;
    (*hubp2).base.opp_id = OPP_ID_INVALID;
    (*hubp2).base.mpcc_id = 0xf;
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
