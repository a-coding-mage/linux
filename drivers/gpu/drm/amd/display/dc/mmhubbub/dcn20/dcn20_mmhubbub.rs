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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C headers and register helper macros are supplied by the surrounding crate.

macro_rules! MCIF_ADDR { ($addr:expr) => { (((($addr as u64) & 0xffffffffffu64).wrapping_add(0xfeu64)) >> 8) as u32 }; }
macro_rules! MCIF_ADDR_HIGH { ($addr:expr) => { (($addr as u64) >> 40) as u32 }; }

unsafe fn mmhubbub2_config_mcif_buf(mcif_wb: *mut mcif_wb, params: *const mcif_buf_params, dest_height: u32) {
    let mcif_wb20 = TO_DCN20_MMHUBBUB(mcif_wb);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUFMGR_SW_LOCK, (*params).swlock);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_1_ADDR_Y, MCIF_WB_BUF_1_ADDR_Y, MCIF_ADDR!((*params).luma_address[0]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_1_ADDR_Y_HIGH, MCIF_WB_BUF_1_ADDR_Y_HIGH, MCIF_ADDR_HIGH!((*params).luma_address[0]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_1_ADDR_Y_OFFSET, MCIF_WB_BUF_1_ADDR_Y_OFFSET, 0);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_1_ADDR_C, MCIF_WB_BUF_1_ADDR_C, MCIF_ADDR!((*params).chroma_address[0]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_1_ADDR_C_HIGH, MCIF_WB_BUF_1_ADDR_C_HIGH, MCIF_ADDR_HIGH!((*params).chroma_address[0]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_1_ADDR_C_OFFSET, MCIF_WB_BUF_1_ADDR_C_OFFSET, 0);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_2_ADDR_Y, MCIF_WB_BUF_2_ADDR_Y, MCIF_ADDR!((*params).luma_address[1]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_2_ADDR_Y_HIGH, MCIF_WB_BUF_2_ADDR_Y_HIGH, MCIF_ADDR_HIGH!((*params).luma_address[1]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_2_ADDR_Y_OFFSET, MCIF_WB_BUF_2_ADDR_Y_OFFSET, 0);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_2_ADDR_C, MCIF_WB_BUF_2_ADDR_C, MCIF_ADDR!((*params).chroma_address[1]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_2_ADDR_C_HIGH, MCIF_WB_BUF_2_ADDR_C_HIGH, MCIF_ADDR_HIGH!((*params).chroma_address[1]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_2_ADDR_C_OFFSET, MCIF_WB_BUF_2_ADDR_C_OFFSET, 0);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_3_ADDR_Y, MCIF_WB_BUF_3_ADDR_Y, MCIF_ADDR!((*params).luma_address[2]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_3_ADDR_Y_HIGH, MCIF_WB_BUF_3_ADDR_Y_HIGH, MCIF_ADDR_HIGH!((*params).luma_address[2]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_3_ADDR_Y_OFFSET, MCIF_WB_BUF_3_ADDR_Y_OFFSET, 0);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_3_ADDR_C, MCIF_WB_BUF_3_ADDR_C, MCIF_ADDR!((*params).chroma_address[2]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_3_ADDR_C_HIGH, MCIF_WB_BUF_3_ADDR_C_HIGH, MCIF_ADDR_HIGH!((*params).chroma_address[2]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_3_ADDR_C_OFFSET, MCIF_WB_BUF_3_ADDR_C_OFFSET, 0);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_4_ADDR_Y, MCIF_WB_BUF_4_ADDR_Y, MCIF_ADDR!((*params).luma_address[3]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_4_ADDR_Y_HIGH, MCIF_WB_BUF_4_ADDR_Y_HIGH, MCIF_ADDR_HIGH!((*params).luma_address[3]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_4_ADDR_Y_OFFSET, MCIF_WB_BUF_4_ADDR_Y_OFFSET, 0);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_4_ADDR_C, MCIF_WB_BUF_4_ADDR_C, MCIF_ADDR!((*params).chroma_address[3]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_4_ADDR_C_HIGH, MCIF_WB_BUF_4_ADDR_C_HIGH, MCIF_ADDR_HIGH!((*params).chroma_address[3]));
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_4_ADDR_C_OFFSET, MCIF_WB_BUF_4_ADDR_C_OFFSET, 0);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_LUMA_SIZE, MCIF_WB_BUF_LUMA_SIZE, ((*params).luma_pitch >> 8) * dest_height);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUF_CHROMA_SIZE, MCIF_WB_BUF_CHROMA_SIZE, ((*params).chroma_pitch >> 8) * dest_height);
    REG_UPDATE!(mcif_wb20, MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUF_ADDR_FENCE_EN, 1);
    REG_UPDATE_2!(mcif_wb20, MCIF_WB_BUF_PITCH, MCIF_WB_BUF_LUMA_PITCH, (*params).luma_pitch >> 8, MCIF_WB_BUF_CHROMA_PITCH, (*params).chroma_pitch >> 8);
    REG_UPDATE!(mcif_wb20, MCIF_WB_WARM_UP_CNTL, MCIF_WB_PITCH_SIZE_WARMUP, (*params).warmup_pitch);
}

unsafe fn mmhubbub2_config_mcif_arb(mcif_wb: *mut mcif_wb, params: *const mcif_arb_params) {
    let w = TO_DCN20_MMHUBBUB(mcif_wb);
    REG_UPDATE!(w, MCIF_WB_ARBITRATION_CONTROL, MCIF_WB_TIME_PER_PIXEL, (*params).time_per_pixel);
    REG_UPDATE!(w, MCIF_WB_SCLK_CHANGE, MCIF_WB_CLI_WATERMARK_MASK, 0);
    REG_UPDATE!(w, MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK, (*params).cli_watermark[0]);
    REG_UPDATE!(w, MCIF_WB_SCLK_CHANGE, MCIF_WB_CLI_WATERMARK_MASK, 1);
    REG_UPDATE!(w, MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK, (*params).cli_watermark[1]);
    REG_UPDATE!(w, MCIF_WB_SCLK_CHANGE, MCIF_WB_CLI_WATERMARK_MASK, 2);
    REG_UPDATE!(w, MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK, (*params).cli_watermark[2]);
    REG_UPDATE!(w, MCIF_WB_SCLK_CHANGE, MCIF_WB_CLI_WATERMARK_MASK, 3);
    REG_UPDATE!(w, MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK, (*params).cli_watermark[3]);
    REG_UPDATE!(w, MCIF_WB_NB_PSTATE_CONTROL, NB_PSTATE_CHANGE_WATERMARK_MASK, 0);
    REG_UPDATE!(w, MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_REFRESH_WATERMARK, (*params).pstate_watermark[0]);
    REG_UPDATE!(w, MCIF_WB_NB_PSTATE_CONTROL, NB_PSTATE_CHANGE_WATERMARK_MASK, 1);
    REG_UPDATE!(w, MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_REFRESH_WATERMARK, (*params).pstate_watermark[1]);
    REG_UPDATE!(w, MCIF_WB_NB_PSTATE_CONTROL, NB_PSTATE_CHANGE_WATERMARK_MASK, 2);
    REG_UPDATE!(w, MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_REFRESH_WATERMARK, (*params).pstate_watermark[2]);
    REG_UPDATE!(w, MCIF_WB_NB_PSTATE_CONTROL, NB_PSTATE_CHANGE_WATERMARK_MASK, 3);
    REG_UPDATE!(w, MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_REFRESH_WATERMARK, (*params).pstate_watermark[3]);
    REG_UPDATE!(w, MULTI_LEVEL_QOS_CTRL, MAX_SCALED_TIME_TO_URGENT, (*params).max_scaled_time);
    REG_UPDATE!(w, MCIF_WB_BUFMGR_VCE_CONTROL, MCIF_WB_BUFMGR_SLICE_SIZE, (*params).slice_lines - 1);
    REG_UPDATE!(w, MCIF_WB_ARBITRATION_CONTROL, MCIF_WB_CLIENT_ARBITRATION_SLICE, (*params).arbitration_slice);
}

pub unsafe fn mmhubbub2_config_mcif_irq(mcif_wb: *mut mcif_wb, params: *const mcif_irq_params) {
    let w = TO_DCN20_MMHUBBUB(mcif_wb);
    REG_UPDATE!(w, MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUFMGR_SW_INT_EN, (*params).sw_int_en);
    REG_UPDATE!(w, MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUFMGR_SW_SLICE_INT_EN, (*params).sw_slice_int_en);
    REG_UPDATE!(w, MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUFMGR_SW_OVERRUN_INT_EN, (*params).sw_overrun_int_en);
    REG_UPDATE!(w, MCIF_WB_BUFMGR_VCE_CONTROL, MCIF_WB_BUFMGR_VCE_INT_EN, (*params).vce_int_en);
    if (*w).mcif_wb_mask.as_ref().unwrap().MCIF_WB_BUFMGR_VCE_SLICE_INT_EN != 0 {
        REG_UPDATE!(w, MCIF_WB_BUFMGR_VCE_CONTROL, MCIF_WB_BUFMGR_VCE_SLICE_INT_EN, (*params).vce_slice_int_en);
    }
}

pub unsafe fn mmhubbub2_enable_mcif(mcif_wb: *mut mcif_wb) { REG_UPDATE!(TO_DCN20_MMHUBBUB(mcif_wb), MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUFMGR_ENABLE, 1); }
pub unsafe fn mmhubbub2_disable_mcif(mcif_wb: *mut mcif_wb) { REG_UPDATE!(TO_DCN20_MMHUBBUB(mcif_wb), MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUFMGR_ENABLE, 0); }

pub unsafe fn mcifwb2_dump_frame(mcif_wb: *mut mcif_wb, mcif_params: *const mcif_buf_params, out_format: dwb_scaler_mode, dest_width: u32, dest_height: u32, dump_info: *mut mcif_wb_frame_dump_info, luma_buffer: *const u8, chroma_buffer: *const u8, dest_luma_buffer: *mut u8, dest_chroma_buffer: *mut u8) {
    let w = TO_DCN20_MMHUBBUB(mcif_wb);
    REG_UPDATE!(w, MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUFMGR_SW_LOCK, 0xf);
    core::ptr::copy_nonoverlapping(luma_buffer, dest_luma_buffer, ((*mcif_params).luma_pitch * dest_height) as usize);
    core::ptr::copy_nonoverlapping(chroma_buffer, dest_chroma_buffer, ((*mcif_params).chroma_pitch * dest_height / 2) as usize);
    REG_UPDATE!(w, MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUFMGR_SW_LOCK, 0);
    (*dump_info).format = out_format; (*dump_info).width = dest_width; (*dump_info).height = dest_height;
    (*dump_info).luma_pitch = (*mcif_params).luma_pitch; (*dump_info).chroma_pitch = (*mcif_params).chroma_pitch;
    (*dump_info).size = dest_height * ((*mcif_params).luma_pitch + (*mcif_params).chroma_pitch);
}

static dcn20_mmhubbub_funcs: mcif_wb_funcs = mcif_wb_funcs {
    enable_mcif: Some(mmhubbub2_enable_mcif),
    disable_mcif: Some(mmhubbub2_disable_mcif),
    config_mcif_buf: Some(mmhubbub2_config_mcif_buf),
    config_mcif_arb: Some(mmhubbub2_config_mcif_arb),
    config_mcif_irq: Some(mmhubbub2_config_mcif_irq),
    dump_frame: Some(mcifwb2_dump_frame),
};

pub unsafe fn dcn20_mmhubbub_construct(mcif_wb20: *mut dcn20_mmhubbub, ctx: *mut dc_context, mcif_wb_regs: *const dcn20_mmhubbub_registers, mcif_wb_shift: *const dcn20_mmhubbub_shift, mcif_wb_mask: *const dcn20_mmhubbub_mask, inst: i32) {
    (*mcif_wb20).base.ctx = ctx;
    (*mcif_wb20).base.inst = inst;
    (*mcif_wb20).base.funcs = &dcn20_mmhubbub_funcs;
    (*mcif_wb20).mcif_wb_regs = mcif_wb_regs;
    (*mcif_wb20).mcif_wb_shift = mcif_wb_shift;
    (*mcif_wb20).mcif_wb_mask = mcif_wb_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
