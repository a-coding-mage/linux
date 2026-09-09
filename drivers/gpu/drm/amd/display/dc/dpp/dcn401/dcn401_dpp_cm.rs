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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies are supplied by the surrounding translation unit.

pub const NUM_PHASES: u32 = 64;
pub const HORZ_MAX_TAPS: u32 = 8;
pub const VERT_MAX_TAPS: u32 = 8;
pub const BLACK_OFFSET_RGB_Y: u32 = 0x0;
pub const BLACK_OFFSET_CBCR: u32 = 0x8000;

#[repr(C)]
pub enum dcn401_coef_filter_type_sel {
    SCL_COEF_LUMA_VERT_FILTER = 0,
    SCL_COEF_LUMA_HORZ_FILTER = 1,
    SCL_COEF_CHROMA_VERT_FILTER = 2,
    SCL_COEF_CHROMA_HORZ_FILTER = 3,
    SCL_COEF_SC_VERT_FILTER = 4,
    SCL_COEF_SC_HORZ_FILTER = 5,
}

#[repr(C)]
pub enum dscl_autocal_mode {
    AUTOCAL_MODE_OFF = 0,
    AUTOCAL_MODE_AUTOSCALE = 1,
    AUTOCAL_MODE_AUTOCENTER = 2,
    AUTOCAL_MODE_AUTOREPLICATE = 3,
}

#[repr(C)]
pub enum dscl_mode_sel {
    DSCL_MODE_SCALING_444_BYPASS = 0,
    DSCL_MODE_SCALING_444_RGB_ENABLE = 1,
    DSCL_MODE_SCALING_444_YCBCR_ENABLE = 2,
    DSCL_MODE_SCALING_YCBCR_ENABLE = 3,
    DSCL_MODE_LUMA_SCALING_BYPASS = 4,
    DSCL_MODE_CHROMA_SCALING_BYPASS = 5,
    DSCL_MODE_DSCL_BYPASS = 6,
}

pub unsafe fn dpp401_set_cursor_attributes(
    dpp_base: *mut dpp,
    cursor_attributes: *mut dc_cursor_attributes,
) {
    let dpp = TO_DCN401_DPP(dpp_base);
    let color_format = (*cursor_attributes).color_format;
    let mut cur_rom_en = 0;

    if color_format == CURSOR_MODE_COLOR_PRE_MULTIPLIED_ALPHA
        || color_format == CURSOR_MODE_COLOR_UN_PRE_MULTIPLIED_ALPHA
    {
        if (*cursor_attributes).attribute_flags.bits.ENABLE_CURSOR_DEGAMMA {
            cur_rom_en = 1;
        }
    }

    if !(*dpp_base).cursor_offload {
        REG_UPDATE_3!(dpp, CURSOR0_CONTROL, CUR0_MODE, color_format,
            CUR0_EXPANSION_MODE, 0, CUR0_ROM_EN, cur_rom_en);
    }

    if color_format == CURSOR_MODE_MONO {
        if !(*dpp_base).cursor_offload {
            REG_UPDATE!(dpp, CURSOR0_COLOR0, CUR0_COLOR0, 0x00000000);
            REG_UPDATE!(dpp, CURSOR0_COLOR1, CUR0_COLOR1, 0xFFFFFFFF);
        }
    }

    (*dpp_base).att.cur0_ctl.bits.expansion_mode = 0;
    (*dpp_base).att.cur0_ctl.bits.cur0_rom_en = cur_rom_en;
    (*dpp_base).att.cur0_ctl.bits.mode = color_format;
}

pub unsafe fn dpp401_set_cursor_position(
    dpp_base: *mut dpp,
    pos: *const dc_cursor_position,
    _param: *const dc_cursor_mi_param,
    _width: u32,
    _height: u32,
) {
    let dpp = TO_DCN401_DPP(dpp_base);
    let cur_en = if (*pos).enable { 1 } else { 0 };

    if (*dpp_base).pos.cur0_ctl.bits.cur0_enable != cur_en {
        if !(*dpp_base).cursor_offload {
            REG_UPDATE!(dpp, CURSOR0_CONTROL, CUR0_ENABLE, cur_en);
        }
    }
    (*dpp_base).pos.cur0_ctl.bits.cur0_enable = cur_en;
    (*dpp_base).att.cur0_ctl.bits.cur0_enable = cur_en;
}

pub unsafe fn dpp401_set_optional_cursor_attributes(
    dpp_base: *mut dpp,
    attr: *mut dpp_cursor_attributes,
) {
    let dpp = TO_DCN401_DPP(dpp_base);
    if !attr.is_null() {
        if !(*dpp_base).cursor_offload {
            REG_UPDATE!(dpp, CURSOR0_FP_SCALE_BIAS_G_Y, CUR0_FP_BIAS_G_Y, (*attr).bias);
            REG_UPDATE!(dpp, CURSOR0_FP_SCALE_BIAS_G_Y, CUR0_FP_SCALE_G_Y, (*attr).scale);
            REG_UPDATE!(dpp, CURSOR0_FP_SCALE_BIAS_RB_CRCB, CUR0_FP_BIAS_RB_CRCB, (*attr).bias);
            REG_UPDATE!(dpp, CURSOR0_FP_SCALE_BIAS_RB_CRCB, CUR0_FP_SCALE_RB_CRCB, (*attr).scale);
        }
        (*dpp_base).att.fp_scale_bias_g_y.bits.fp_bias_g_y = (*attr).bias;
        (*dpp_base).att.fp_scale_bias_g_y.bits.fp_scale_g_y = (*attr).scale;
        (*dpp_base).att.fp_scale_bias_rb_crcb.bits.fp_bias_rb_crcb = (*attr).bias;
        (*dpp_base).att.fp_scale_bias_rb_crcb.bits.fp_scale_rb_crcb = (*attr).scale;
    }
}

unsafe fn dpp401_program_cursor_csc(
    dpp_base: *mut dpp,
    color_space: dc_color_space,
    tbl_entry: *const dpp_input_csc_matrix,
) {
    let dpp = TO_DCN401_DPP(dpp_base);
    let mut mode_select = 0;
    let mut cur_matrix_regs: color_matrices_reg = core::mem::zeroed();
    let mut regval: *const u16 = core::ptr::null();
    let arr_size = core::mem::size_of::<dpp_input_csc_matrix>()
        / core::mem::size_of::<dpp_input_csc_matrix>();

    if color_space < COLOR_SPACE_YCBCR601 {
        REG_SET!(dpp, CUR0_MATRIX_MODE, 0, CUR0_MATRIX_MODE, CUR_MATRIX_BYPASS);
        return;
    }
    if tbl_entry.is_null() {
        for i in 0..arr_size {
            if dpp_input_csc_matrix[i].color_space == color_space {
                regval = dpp_input_csc_matrix[i].regval;
                break;
            }
        }
        if regval.is_null() {
            BREAK_TO_DEBUGGER!();
            REG_SET!(dpp, CUR0_MATRIX_MODE, 0, CUR0_MATRIX_MODE, CUR_MATRIX_BYPASS);
            return;
        }
    } else {
        regval = (*tbl_entry).regval;
    }
    REG_GET!(dpp, CUR0_MATRIX_MODE, CUR0_MATRIX_MODE_CURRENT, &mut mode_select);
    if mode_select != CUR_MATRIX_SET_A { mode_select = CUR_MATRIX_SET_A; } else { mode_select = CUR_MATRIX_SET_B; }
    cur_matrix_regs.shifts.csc_c11 = (*dpp).tf_shift.CUR0_MATRIX_C11_A;
    cur_matrix_regs.masks.csc_c11 = (*dpp).tf_mask.CUR0_MATRIX_C11_A;
    cur_matrix_regs.shifts.csc_c12 = (*dpp).tf_shift.CUR0_MATRIX_C12_A;
    cur_matrix_regs.masks.csc_c12 = (*dpp).tf_mask.CUR0_MATRIX_C12_A;
    if mode_select == CUR_MATRIX_SET_A {
        cur_matrix_regs.csc_c11_c12 = REG!(dpp, CUR0_MATRIX_C11_C12_A);
        cur_matrix_regs.csc_c33_c34 = REG!(dpp, CUR0_MATRIX_C33_C34_A);
    } else {
        cur_matrix_regs.csc_c11_c12 = REG!(dpp, CUR0_MATRIX_C11_C12_B);
        cur_matrix_regs.csc_c33_c34 = REG!(dpp, CUR0_MATRIX_C33_C34_B);
    }
    cm_helper_program_color_matrices((*dpp).base.ctx, regval, &mut cur_matrix_regs);
    REG_SET!(dpp, CUR0_MATRIX_MODE, 0, CUR0_MATRIX_MODE, mode_select);
}

pub unsafe fn dpp401_set_cursor_matrix(
    dpp_base: *mut dpp,
    _color_space: dc_color_space,
    _cursor_csc_color_matrix: dc_csc_transform,
) {
    dpp401_program_cursor_csc(dpp_base, COLOR_SPACE_UNKNOWN, core::ptr::null());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
