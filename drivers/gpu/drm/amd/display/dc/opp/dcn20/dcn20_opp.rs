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

// C includes: core_types.h, dm_services.h, dcn20_opp.h, reg_helper.h

pub unsafe fn opp2_set_disp_pattern_generator(
    opp: *mut output_pixel_processor,
    test_pattern: controller_dp_test_pattern,
    color_space: controller_dp_color_space,
    color_depth: dc_color_depth,
    solid_color: *const tg_color,
    width: i32,
    height: i32,
    offset: i32,
) {
    let oppn20 = TO_DCN20_OPP(opp);
    let bit_depth: test_pattern_color_format;
    let dyn_range: test_pattern_dyn_range;
    let mode: test_pattern_mode;
    let src_bpc: u32 = 16;
    let mut dst_bpc: u32;
    let mut index: u32;
    let src_color: [u16; 6] = [0xffff, 0xffff, 0xffff, 0, 0, 0];
    let mut dst_color: [u16; 6] = [0; 6];
    let mut inc_base: u32;

    bit_depth = match color_depth {
        COLOR_DEPTH_666 => TEST_PATTERN_COLOR_FORMAT_BPC_6,
        COLOR_DEPTH_888 => TEST_PATTERN_COLOR_FORMAT_BPC_8,
        COLOR_DEPTH_101010 => TEST_PATTERN_COLOR_FORMAT_BPC_10,
        COLOR_DEPTH_121212 => TEST_PATTERN_COLOR_FORMAT_BPC_12,
        _ => TEST_PATTERN_COLOR_FORMAT_BPC_8,
    };

    if (*(*opp).ctx).dc.debug.disable_dynamic_expansion_for_test_pattern {
        match test_pattern {
            CONTROLLER_DP_TEST_PATTERN_COLORSQUARES
            | CONTROLLER_DP_TEST_PATTERN_COLORSQUARES_CEA
            | CONTROLLER_DP_TEST_PATTERN_VERTICALBARS
            | CONTROLLER_DP_TEST_PATTERN_HORIZONTALBARS
            | CONTROLLER_DP_TEST_PATTERN_COLORRAMP => {
                if color_depth == COLOR_DEPTH_121212 {
                    REG_UPDATE!(oppn20, FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, 0);
                }
            }
            CONTROLLER_DP_TEST_PATTERN_VIDEOMODE => {
                REG_UPDATE!(oppn20, FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, 1);
            }
            _ => {}
        }
    }

    REG_SET_2!(oppn20, DPG_DIMENSIONS, 0, DPG_ACTIVE_WIDTH, width, DPG_ACTIVE_HEIGHT, height);
    REG_SET_2!(oppn20, DPG_OFFSET_SEGMENT, 0, DPG_X_OFFSET, offset, DPG_SEGMENT_WIDTH, 0);

    match test_pattern {
        CONTROLLER_DP_TEST_PATTERN_COLORSQUARES | CONTROLLER_DP_TEST_PATTERN_COLORSQUARES_CEA => {
            dyn_range = if test_pattern == CONTROLLER_DP_TEST_PATTERN_COLORSQUARES_CEA {
                TEST_PATTERN_DYN_RANGE_CEA
            } else { TEST_PATTERN_DYN_RANGE_VESA };
            mode = match color_space {
                CONTROLLER_DP_COLOR_SPACE_YCBCR601 => TEST_PATTERN_MODE_COLORSQUARES_YCBCR601,
                CONTROLLER_DP_COLOR_SPACE_YCBCR709 => TEST_PATTERN_MODE_COLORSQUARES_YCBCR709,
                _ => TEST_PATTERN_MODE_COLORSQUARES_RGB,
            };
            REG_UPDATE_6!(oppn20, DPG_CONTROL, DPG_EN, 1, DPG_MODE, mode,
                DPG_DYNAMIC_RANGE, dyn_range, DPG_BIT_DEPTH, bit_depth, DPG_VRES, 6, DPG_HRES, 6);
        }
        CONTROLLER_DP_TEST_PATTERN_VERTICALBARS | CONTROLLER_DP_TEST_PATTERN_HORIZONTALBARS => {
            mode = if test_pattern == CONTROLLER_DP_TEST_PATTERN_VERTICALBARS {
                TEST_PATTERN_MODE_VERTICALBARS
            } else { TEST_PATTERN_MODE_HORIZONTALBARS };
            dst_bpc = match bit_depth {
                TEST_PATTERN_COLOR_FORMAT_BPC_6 => 6,
                TEST_PATTERN_COLOR_FORMAT_BPC_8 => 8,
                TEST_PATTERN_COLOR_FORMAT_BPC_10 => 10,
                TEST_PATTERN_COLOR_FORMAT_BPC_12 => 12,
                _ => 8,
            };
            for i in 0..6 { index = i; dst_color[index as usize] = src_color[index as usize] >> (src_bpc - dst_bpc); dst_color[index as usize] <<= 16 - dst_bpc; }
            REG_SET_2!(oppn20, DPG_COLOUR_R_CR, 0, DPG_COLOUR1_R_CR, dst_color[0], DPG_COLOUR0_R_CR, dst_color[3]);
            REG_SET_2!(oppn20, DPG_COLOUR_G_Y, 0, DPG_COLOUR1_G_Y, dst_color[1], DPG_COLOUR0_G_Y, dst_color[4]);
            REG_SET_2!(oppn20, DPG_COLOUR_B_CB, 0, DPG_COLOUR1_B_CB, dst_color[2], DPG_COLOUR0_B_CB, dst_color[5]);
            REG_UPDATE_6!(oppn20, DPG_CONTROL, DPG_EN, 1, DPG_MODE, mode, DPG_DYNAMIC_RANGE, 0, DPG_BIT_DEPTH, bit_depth, DPG_VRES, 0, DPG_HRES, 0);
        }
        CONTROLLER_DP_TEST_PATTERN_COLORRAMP => {
            (mode, dst_bpc) = match bit_depth {
                TEST_PATTERN_COLOR_FORMAT_BPC_6 => (TEST_PATTERN_MODE_SINGLERAMP_RGB, 6),
                TEST_PATTERN_COLOR_FORMAT_BPC_8 => (TEST_PATTERN_MODE_SINGLERAMP_RGB, 8),
                TEST_PATTERN_COLOR_FORMAT_BPC_10 => (TEST_PATTERN_MODE_DUALRAMP_RGB, 10),
                TEST_PATTERN_COLOR_FORMAT_BPC_12 => (TEST_PATTERN_MODE_DUALRAMP_RGB, 12),
                _ => (TEST_PATTERN_MODE_SINGLERAMP_RGB, 8),
            };
            inc_base = src_bpc - dst_bpc;
            match bit_depth {
                TEST_PATTERN_COLOR_FORMAT_BPC_6 => { REG_SET_3!(oppn20, DPG_RAMP_CONTROL, 0, DPG_RAMP0_OFFSET, 0, DPG_INC0, inc_base, DPG_INC1, 0); REG_UPDATE_2!(oppn20, DPG_CONTROL, DPG_VRES, 6, DPG_HRES, 6); }
                TEST_PATTERN_COLOR_FORMAT_BPC_8 => { REG_SET_3!(oppn20, DPG_RAMP_CONTROL, 0, DPG_RAMP0_OFFSET, 0, DPG_INC0, inc_base, DPG_INC1, 0); REG_UPDATE_2!(oppn20, DPG_CONTROL, DPG_VRES, 6, DPG_HRES, 8); }
                TEST_PATTERN_COLOR_FORMAT_BPC_10 => { REG_SET_3!(oppn20, DPG_RAMP_CONTROL, 0, DPG_RAMP0_OFFSET, 384 << inc_base, DPG_INC0, inc_base, DPG_INC1, inc_base + 2); REG_UPDATE_2!(oppn20, DPG_CONTROL, DPG_VRES, 5, DPG_HRES, 8); }
                TEST_PATTERN_COLOR_FORMAT_BPC_12 => { REG_SET_3!(oppn20, DPG_RAMP_CONTROL, 0, DPG_RAMP0_OFFSET, 1920 << inc_base, DPG_INC0, inc_base, DPG_INC1, inc_base + 4); REG_UPDATE_2!(oppn20, DPG_CONTROL, DPG_VRES, 5, DPG_HRES, 8); }
                _ => {}
            }
            REG_UPDATE_4!(oppn20, DPG_CONTROL, DPG_EN, 1, DPG_MODE, mode, DPG_DYNAMIC_RANGE, 0, DPG_BIT_DEPTH, bit_depth);
        }
        CONTROLLER_DP_TEST_PATTERN_VIDEOMODE => { REG_WRITE!(oppn20, DPG_CONTROL, 0); REG_WRITE!(oppn20, DPG_COLOUR_R_CR, 0); REG_WRITE!(oppn20, DPG_COLOUR_G_Y, 0); REG_WRITE!(oppn20, DPG_COLOUR_B_CB, 0); REG_WRITE!(oppn20, DPG_RAMP_CONTROL, 0); }
        CONTROLLER_DP_TEST_PATTERN_SOLID_COLOR => { opp2_dpg_set_blank_color(opp, solid_color); REG_UPDATE_2!(oppn20, DPG_CONTROL, DPG_EN, 1, DPG_MODE, TEST_PATTERN_MODE_HORIZONTALBARS); REG_SET_2!(oppn20, DPG_DIMENSIONS, 0, DPG_ACTIVE_WIDTH, width, DPG_ACTIVE_HEIGHT, height); }
        _ => {}
    }
}

pub unsafe fn opp2_program_dpg_dimensions(opp: *mut output_pixel_processor, width: u32, height: u32) { let oppn20 = TO_DCN20_OPP(opp); REG_SET_2!(oppn20, DPG_DIMENSIONS, 0, DPG_ACTIVE_WIDTH, width, DPG_ACTIVE_HEIGHT, height); }

pub unsafe fn opp2_dpg_set_blank_color(opp: *mut output_pixel_processor, color: *const tg_color) {
    let oppn20 = TO_DCN20_OPP(opp); ASSERT!(color);
    REG_SET_2!(oppn20, DPG_COLOUR_B_CB, 0, DPG_COLOUR1_B_CB, (*color).color_b_cb << 6, DPG_COLOUR0_B_CB, (*color).color_b_cb << 6);
    REG_SET_2!(oppn20, DPG_COLOUR_G_Y, 0, DPG_COLOUR1_G_Y, (*color).color_g_y << 6, DPG_COLOUR0_G_Y, (*color).color_g_y << 6);
    REG_SET_2!(oppn20, DPG_COLOUR_R_CR, 0, DPG_COLOUR1_R_CR, (*color).color_r_cr << 6, DPG_COLOUR0_R_CR, (*color).color_r_cr << 6);
}

pub unsafe fn opp2_dpg_is_blanked(opp: *mut output_pixel_processor) -> bool { let oppn20 = TO_DCN20_OPP(opp); let mut dpg_en=0; let mut dpg_mode=0; let mut pending=0; REG_GET_2!(oppn20, DPG_CONTROL, DPG_EN, &mut dpg_en, DPG_MODE, &mut dpg_mode); REG_GET!(oppn20, DPG_STATUS, DPG_DOUBLE_BUFFER_PENDING, &mut pending); dpg_en == 1 && pending == 0 }
pub unsafe fn opp2_dpg_is_pending(opp: *mut output_pixel_processor) -> bool { let oppn20 = TO_DCN20_OPP(opp); let mut pending=0; let mut en=0; REG_GET!(oppn20, DPG_CONTROL, DPG_EN, &mut en); REG_GET!(oppn20, DPG_STATUS, DPG_DOUBLE_BUFFER_PENDING, &mut pending); en == 1 && pending == 1 }
pub unsafe fn opp2_program_left_edge_extra_pixel(opp: *mut output_pixel_processor, pixel_encoding: dc_pixel_encoding, is_primary: bool) { let oppn20=TO_DCN20_OPP(opp); let count=opp2_get_left_edge_extra_pixel_count(opp,pixel_encoding,is_primary); REG_UPDATE!(oppn20,FMT_422_CONTROL,FMT_LEFT_EDGE_EXTRA_PIXEL_COUNT,count); }
pub unsafe fn opp2_get_left_edge_extra_pixel_count(opp: *mut output_pixel_processor, pixel_encoding: dc_pixel_encoding, is_primary: bool) -> u32 { if (pixel_encoding == PIXEL_ENCODING_YCBCR422 || pixel_encoding == PIXEL_ENCODING_YCBCR420) && !(*(*opp).ctx).dc.debug.force_chroma_subsampling_1tap && !is_primary { 1 } else { 0 } }
pub unsafe fn opp2_read_reg_state(opp: *mut output_pixel_processor, state: *mut dcn_opp_reg_state) { let oppn20=TO_DCN20_OPP(opp); (*state).dpg_control=REG_READ!(oppn20,DPG_CONTROL); (*state).fmt_control=REG_READ!(oppn20,FMT_CONTROL); (*state).opp_pipe_control=REG_READ!(oppn20,OPP_PIPE_CONTROL); (*state).opp_pipe_crc_control=REG_READ!(oppn20,OPP_PIPE_CRC_CONTROL); (*state).oppbuf_control=REG_READ!(oppn20,OPPBUF_CONTROL); (*state).dscrm_dsc_forward_config=REG_READ!(oppn20,DSCRM_DSC_FORWARD_CONFIG); }

static mut DCN20_OPP_FUNCS: opp_funcs = opp_funcs { opp_set_dyn_expansion: opp1_set_dyn_expansion, opp_program_fmt: opp1_program_fmt, opp_program_bit_depth_reduction: opp1_program_bit_depth_reduction, opp_program_stereo: opp1_program_stereo, opp_pipe_clock_control: opp1_pipe_clock_control, opp_set_disp_pattern_generator: opp2_set_disp_pattern_generator, opp_program_dpg_dimensions: opp2_program_dpg_dimensions, dpg_is_blanked: opp2_dpg_is_blanked, dpg_is_pending: opp2_dpg_is_pending, opp_dpg_set_blank_color: opp2_dpg_set_blank_color, opp_destroy: opp1_destroy, opp_program_left_edge_extra_pixel: opp2_program_left_edge_extra_pixel, opp_get_left_edge_extra_pixel_count: opp2_get_left_edge_extra_pixel_count, opp_read_reg_state: opp2_read_reg_state };

pub unsafe fn dcn20_opp_construct(oppn20: *mut dcn20_opp, ctx: *mut dc_context, inst: u32, regs: *const dcn20_opp_registers, opp_shift: *const dcn20_opp_shift, opp_mask: *const dcn20_opp_mask) { (*oppn20).base.ctx=ctx; (*oppn20).base.inst=inst; (*oppn20).base.funcs=&mut DCN20_OPP_FUNCS; (*oppn20).regs=regs; (*oppn20).opp_shift=opp_shift; (*oppn20).opp_mask=opp_mask; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
