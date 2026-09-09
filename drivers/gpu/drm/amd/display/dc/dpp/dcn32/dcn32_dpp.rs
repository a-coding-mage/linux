/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translation unit.

unsafe extern "C" {
    fn dpp3_program_gamcor_lut();
    fn dpp30_read_state();
    fn dpp_reset();
    fn dpp1_dscl_set_scaler_manual_scale();
    fn dpp3_get_optimal_number_of_taps();
    fn dpp3_cm_set_gamut_remap();
    fn dpp3_set_pre_degam();
    fn dpp1_full_bypass();
    fn dpp3_cnv_setup();
    fn dpp3_program_cm_dealpha();
    fn dpp3_program_cm_bias();
    fn dpp2_cnv_set_alpha_keyer();
    fn dpp3_set_cursor_attributes();
    fn dpp1_set_cursor_position();
    fn dpp1_cnv_set_optional_cursor_attributes();
    fn dpp1_dppclk_control();
    fn dpp3_set_hdr_multiplier();
    fn dpp3_cm_get_gamut_remap();
    fn dpp30_read_reg_state();
}

unsafe fn dscl32_calc_lb_num_partitions(
    scl_data: *const scaler_data,
    lb_config: lb_memory_config,
    num_part_y: *mut core::ffi::c_int,
    num_part_c: *mut core::ffi::c_int,
) {
    let (mut lb_memory_size, mut lb_memory_size_c, mut lb_memory_size_a): (i32, i32, i32);
    let mut num_partitions_a: i32;
    let line_size = if (*scl_data).viewport.width < (*scl_data).recout.width {
        (*scl_data).viewport.width
    } else { (*scl_data).recout.width };
    let line_size_c = if (*scl_data).viewport_c.width < (*scl_data).recout.width {
        (*scl_data).viewport_c.width
    } else { (*scl_data).recout.width };
    let line_size = if line_size == 0 { 1 } else { line_size };
    let line_size_c = if line_size_c == 0 { 1 } else { line_size_c };
    let memory_line_size_y = (line_size + 5) / 6; // +5 to ceil
    let memory_line_size_c = (line_size_c + 5) / 6; // +5 to ceil
    let memory_line_size_a = (line_size + 5) / 6; // +5 to ceil

    if lb_config == LB_MEMORY_CONFIG_1 {
        lb_memory_size = 970; lb_memory_size_c = 970; lb_memory_size_a = 970;
    } else if lb_config == LB_MEMORY_CONFIG_2 {
        lb_memory_size = 1290; lb_memory_size_c = 1290; lb_memory_size_a = 1290;
    } else if lb_config == LB_MEMORY_CONFIG_3 {
        if (*scl_data).viewport.width == (*scl_data).h_active && (*scl_data).viewport.height == (*scl_data).v_active {
            // 420 mode: luma using all 3 mem from Y, plus 3rd mem from Cr and Cb
            // use increased LB size for calculation only if Scaler not enabled
            lb_memory_size = 970 + 1290 + 1170 + 1170 + 1170;
            lb_memory_size_c = 970 + 1290;
            lb_memory_size_a = 970 + 1290 + 1170;
        } else {
            // 420 mode: luma using all 3 mem from Y, plus 3rd mem from Cr and Cb
            lb_memory_size = 970 + 1290 + 484 + 484 + 484;
            lb_memory_size_c = 970 + 1290;
            lb_memory_size_a = 970 + 1290 + 484;
        }
    } else if (*scl_data).viewport.width == (*scl_data).h_active && (*scl_data).viewport.height == (*scl_data).v_active {
        // use increased LB size for calculation only if Scaler not enabled
        lb_memory_size = 970 + 1290 + 1170;
        lb_memory_size_c = 970 + 1290 + 1170;
        lb_memory_size_a = 970 + 1290 + 1170;
    } else {
        lb_memory_size = 970 + 1290 + 484;
        lb_memory_size_c = 970 + 1290 + 484;
        lb_memory_size_a = 970 + 1290 + 484;
    }
    *num_part_y = lb_memory_size / memory_line_size_y;
    *num_part_c = lb_memory_size_c / memory_line_size_c;
    num_partitions_a = lb_memory_size_a / memory_line_size_a;
    if (*scl_data).lb_params.alpha_en && num_partitions_a < *num_part_y { *num_part_y = num_partitions_a; }
    if *num_part_y > 32 { *num_part_y = 32; }
    if *num_part_c > 32 { *num_part_c = 32; }
}

static mut dcn32_dpp_funcs: dpp_funcs = dpp_funcs {
    dpp_program_gamcor_lut: Some(dpp3_program_gamcor_lut), dpp_read_state: Some(dpp30_read_state),
    dpp_reset: Some(dpp_reset), dpp_set_scaler: Some(dpp1_dscl_set_scaler_manual_scale),
    dpp_get_optimal_number_of_taps: Some(dpp3_get_optimal_number_of_taps), dpp_set_gamut_remap: Some(dpp3_cm_set_gamut_remap),
    dpp_set_csc_adjustment: None, dpp_set_csc_default: None, dpp_program_regamma_pwl: None,
    dpp_set_pre_degam: Some(dpp3_set_pre_degam), dpp_program_input_lut: None, dpp_full_bypass: Some(dpp1_full_bypass),
    dpp_setup: Some(dpp3_cnv_setup), dpp_program_degamma_pwl: None, dpp_program_cm_dealpha: Some(dpp3_program_cm_dealpha),
    dpp_program_cm_bias: Some(dpp3_program_cm_bias), dpp_program_blnd_lut: None, dpp_program_shaper_lut: None,
    dpp_program_3dlut: None, dpp_program_bias_and_scale: None, dpp_cnv_set_alpha_keyer: Some(dpp2_cnv_set_alpha_keyer),
    set_cursor_attributes: Some(dpp3_set_cursor_attributes), set_cursor_position: Some(dpp1_set_cursor_position),
    set_optional_cursor_attributes: Some(dpp1_cnv_set_optional_cursor_attributes), dpp_dppclk_control: Some(dpp1_dppclk_control),
    dpp_set_hdr_multiplier: Some(dpp3_set_hdr_multiplier), dpp_get_gamut_remap: Some(dpp3_cm_get_gamut_remap),
    dpp_read_reg_state: Some(dpp30_read_reg_state),
};

static mut dcn32_dpp_cap: dpp_caps = dpp_caps {
    dscl_data_proc_format: DSCL_DATA_PRCESSING_FLOAT_FORMAT,
    max_lb_partitions: 31,
    dscl_calc_lb_num_partitions: Some(dscl32_calc_lb_num_partitions),
};

pub unsafe fn dpp32_construct(
    dpp: *mut dcn3_dpp, ctx: *mut dc_context, inst: u32,
    tf_regs: *const dcn3_dpp_registers, tf_shift: *const dcn3_dpp_shift,
    tf_mask: *const dcn3_dpp_mask,
) -> bool {
    (*dpp).base.ctx = ctx;
    (*dpp).base.inst = inst;
    (*dpp).base.funcs = &raw mut dcn32_dpp_funcs;
    (*dpp).base.caps = &raw mut dcn32_dpp_cap;
    (*dpp).tf_regs = tf_regs;
    (*dpp).tf_shift = tf_shift;
    (*dpp).tf_mask = tf_mask;
    true
}

pub unsafe fn dscl32_spl_calc_lb_num_partitions(
    alpha_en: bool, scl_data: *const spl_scaler_data, lb_config: lb_memory_config,
    num_part_y: *mut core::ffi::c_int, num_part_c: *mut core::ffi::c_int,
) {
    let (mut lb_memory_size, mut lb_memory_size_c, mut lb_memory_size_a): (i32, i32, i32);
    let mut num_partitions_a: i32;
    let line_size = if (*scl_data).viewport.width < (*scl_data).recout.width { (*scl_data).viewport.width } else { (*scl_data).recout.width };
    let line_size_c = if (*scl_data).viewport_c.width < (*scl_data).recout.width { (*scl_data).viewport_c.width } else { (*scl_data).recout.width };
    let line_size = if line_size == 0 { 1 } else { line_size };
    let line_size_c = if line_size_c == 0 { 1 } else { line_size_c };
    let memory_line_size_y = (line_size + 5) / 6; let memory_line_size_c = (line_size_c + 5) / 6; let memory_line_size_a = (line_size + 5) / 6;
    if lb_config == LB_MEMORY_CONFIG_1 { lb_memory_size=970; lb_memory_size_c=970; lb_memory_size_a=970; }
    else if lb_config == LB_MEMORY_CONFIG_2 { lb_memory_size=1290; lb_memory_size_c=1290; lb_memory_size_a=1290; }
    else if lb_config == LB_MEMORY_CONFIG_3 {
        if (*scl_data).viewport.width == (*scl_data).h_active && (*scl_data).viewport.height == (*scl_data).v_active { lb_memory_size=970+1290+1170+1170+1170; lb_memory_size_c=970+1290; lb_memory_size_a=970+1290+1170; }
        else { lb_memory_size=970+1290+484+484+484; lb_memory_size_c=970+1290; lb_memory_size_a=970+1290+484; }
    } else if (*scl_data).viewport.width == (*scl_data).h_active && (*scl_data).viewport.height == (*scl_data).v_active { lb_memory_size=970+1290+1170; lb_memory_size_c=970+1290+1170; lb_memory_size_a=970+1290+1170; }
    else { lb_memory_size=970+1290+484; lb_memory_size_c=970+1290+484; lb_memory_size_a=970+1290+484; }
    *num_part_y=lb_memory_size/memory_line_size_y; *num_part_c=lb_memory_size_c/memory_line_size_c; num_partitions_a=lb_memory_size_a/memory_line_size_a;
    if alpha_en && num_partitions_a < *num_part_y { *num_part_y=num_partitions_a; }
    if *num_part_y > 32 { *num_part_y=32; } if *num_part_c > 32 { *num_part_c=32; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
