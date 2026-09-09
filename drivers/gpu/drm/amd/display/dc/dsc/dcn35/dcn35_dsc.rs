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

// Dependencies are supplied by the surrounding driver.

extern "C" {
    fn dsc2_read_state();
    fn dsc2_read_reg_state();
    fn dsc2_validate_stream();
    fn dsc2_set_config();
    fn dsc2_get_packed_pps();
    fn dsc2_disable();
    fn dsc2_disconnect();
    fn dsc2_wait_disconnect_pending_clear();
}

unsafe fn dsc35_enable(dsc: *mut display_stream_compressor, opp_pipe: i32);
unsafe fn dsc35_get_single_enc_caps(dsc_enc_caps: *mut dsc_enc_caps, max_dscclk_khz: u32);

static dcn35_dsc_funcs: dsc_funcs = dsc_funcs {
    dsc_read_state: dsc2_read_state,
    dsc_read_reg_state: dsc2_read_reg_state,
    dsc_validate_stream: dsc2_validate_stream,
    dsc_set_config: dsc2_set_config,
    dsc_get_packed_pps: dsc2_get_packed_pps,
    dsc_enable: dsc35_enable,
    dsc_disable: dsc2_disable,
    dsc_disconnect: dsc2_disconnect,
    dsc_wait_disconnect_pending_clear: dsc2_wait_disconnect_pending_clear,
    dsc_get_single_enc_caps: dsc35_get_single_enc_caps,
};

// Macro definitions for REG_SET macros.  The register and logging macros are
// provided by the surrounding driver and retain their original intent here.

pub unsafe fn dsc35_construct(
    dsc: *mut dcn20_dsc,
    ctx: *mut dc_context,
    inst: i32,
    dsc_regs: *const dcn20_dsc_registers,
    dsc_shift: *const dcn35_dsc_shift,
    dsc_mask: *const dcn35_dsc_mask,
) {
    (*dsc).base.ctx = ctx;
    (*dsc).base.inst = inst;
    (*dsc).base.funcs = &dcn35_dsc_funcs;

    (*dsc).dsc_regs = dsc_regs;
    (*dsc).dsc_shift = dsc_shift as *const dcn20_dsc_shift;
    (*dsc).dsc_mask = dsc_mask as *const dcn20_dsc_mask;

    (*dsc).max_image_width = 5184;
}

unsafe fn dsc35_enable(dsc: *mut display_stream_compressor, opp_pipe: i32) {
    let dsc20: *mut dcn20_dsc = TO_DCN20_DSC!(dsc);
    let mut dsc_clock_en: u32 = 0;
    let mut dsc_fw_config: u32 = 0;
    let mut enabled_opp_pipe: u32 = 0;

    DC_LOG_DSC!("enable DSC %d at opp pipe %d", (*dsc).inst, opp_pipe);

    // TODO: After an idle exit, the HW default values for power control
    // are changed intermittently due to unknown reasons. There are cases
    // when dscc memory are still in shutdown state during enablement.
    // Reset power control to hw default values.
    REG_UPDATE_2!(dsc20, DSCC_MEM_POWER_CONTROL,
        DSCC_MEM_PWR_FORCE, 0,
        DSCC_MEM_PWR_DIS, 0);

    REG_GET!(dsc20, DSC_TOP_CONTROL, DSC_CLOCK_EN, &mut dsc_clock_en);
    REG_GET_2!(dsc20, DSCRM_DSC_FORWARD_CONFIG,
        DSCRM_DSC_FORWARD_EN, &mut dsc_fw_config,
        DSCRM_DSC_OPP_PIPE_SOURCE, &mut enabled_opp_pipe);
    if (dsc_clock_en != 0 || dsc_fw_config != 0) && enabled_opp_pipe != opp_pipe as u32 {
        DC_LOG_DSC!("ERROR: DSC %d at opp pipe %u already enabled!", (*dsc).inst, enabled_opp_pipe);
        ASSERT!(false);
    }

    REG_UPDATE!(dsc20, DSC_TOP_CONTROL, DSC_CLOCK_EN, 1);
    REG_UPDATE_2!(dsc20, DSCRM_DSC_FORWARD_CONFIG,
        DSCRM_DSC_FORWARD_EN, 1,
        DSCRM_DSC_OPP_PIPE_SOURCE, opp_pipe);
}

pub unsafe fn dsc35_set_fgcg(dsc20: *mut dcn20_dsc, enable: bool) {
    REG_UPDATE!(dsc20, DSC_TOP_CONTROL, DSC_FGCG_REP_DIS, !enable);
}

pub unsafe fn dsc35_get_single_enc_caps(
    dsc_enc_caps: *mut dsc_enc_caps,
    max_dscclk_khz: u32,
) {
    (*dsc_enc_caps).dsc_version = 0x21; /* v1.2 - DP spec defined it in reverse order and we kept it */

    (*dsc_enc_caps).slice_caps.bits.NUM_SLICES_1 = 1;
    (*dsc_enc_caps).slice_caps.bits.NUM_SLICES_2 = 1;
    (*dsc_enc_caps).slice_caps.bits.NUM_SLICES_3 = 1;
    (*dsc_enc_caps).slice_caps.bits.NUM_SLICES_4 = 1;

    (*dsc_enc_caps).lb_bit_depth = 13;
    (*dsc_enc_caps).is_block_pred_supported = true;

    (*dsc_enc_caps).color_formats.bits.RGB = 1;
    (*dsc_enc_caps).color_formats.bits.YCBCR_444 = 1;
    (*dsc_enc_caps).color_formats.bits.YCBCR_SIMPLE_422 = 1;
    (*dsc_enc_caps).color_formats.bits.YCBCR_NATIVE_422 = 1;
    (*dsc_enc_caps).color_formats.bits.YCBCR_NATIVE_420 = 1;

    (*dsc_enc_caps).color_depth.bits.COLOR_DEPTH_8_BPC = 1;
    (*dsc_enc_caps).color_depth.bits.COLOR_DEPTH_10_BPC = 1;
    (*dsc_enc_caps).color_depth.bits.COLOR_DEPTH_12_BPC = 1;

    (*dsc_enc_caps).max_total_throughput_mps = max_dscclk_khz * 3 / 1000;
    (*dsc_enc_caps).max_slice_width = 5184; /* (including 64 overlap pixels for eDP MSO mode) */
    (*dsc_enc_caps).bpp_increment_div = 16; /* 1/16th of a bit */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
