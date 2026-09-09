/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// External register definitions, helpers, and structure declarations are
// supplied by the surrounding driver.

unsafe fn mmhubbub3_warmup_mcif(
    mcif_wb: *mut mcif_wb,
    params: *mut mcif_warmup_params,
) {
    let mcif_wb30: *mut dcn30_mmhubbub = TO_DCN30_MMHUBBUB(mcif_wb);
    let start_address_shift = large_integer {
        quad_part: (*params).start_address.quad_part >> 5,
    };

    REG_SET!(MMHUBBUB_WARMUP_BASE_ADDR_HIGH, 0, MMHUBBUB_WARMUP_BASE_ADDR_HIGH, start_address_shift.high_part);
    REG_SET!(MMHUBBUB_WARMUP_BASE_ADDR_LOW, 0, MMHUBBUB_WARMUP_BASE_ADDR_LOW, start_address_shift.low_part);
    REG_SET!(MMHUBBUB_WARMUP_ADDR_REGION, 0, MMHUBBUB_WARMUP_ADDR_REGION, (*params).region_size >> 5);

    REG_SET_3!(MMHUBBUB_WARMUP_CONTROL_STATUS, 0,
        MMHUBBUB_WARMUP_EN, true,
        MMHUBBUB_WARMUP_SW_INT_EN, true,
        MMHUBBUB_WARMUP_INC_ADDR, (*params).address_increment >> 5);

    REG_WAIT!(MMHUBBUB_WARMUP_CONTROL_STATUS, MMHUBBUB_WARMUP_SW_INT_STATUS, 1, 20, 100);
    REG_UPDATE!(MMHUBBUB_WARMUP_CONTROL_STATUS, MMHUBBUB_WARMUP_SW_INT_ACK, 1);
    REG_UPDATE!(MMHUBBUB_WARMUP_CONTROL_STATUS, MMHUBBUB_WARMUP_EN, false);
}

unsafe fn mmhubbub3_config_mcif_buf(
    mcif_wb: *mut mcif_wb,
    params: *mut mcif_buf_params,
    dest_height: u32,
) {
    let _mcif_wb30: *mut dcn30_mmhubbub = TO_DCN30_MMHUBBUB(mcif_wb);

    REG_UPDATE!(MCIF_WB_BUF_1_ADDR_Y, MCIF_WB_BUF_1_ADDR_Y, MCIF_ADDR!((*params).luma_address[0]));
    REG_UPDATE!(MCIF_WB_BUF_1_ADDR_Y_HIGH, MCIF_WB_BUF_1_ADDR_Y_HIGH, MCIF_ADDR_HIGH!((*params).luma_address[0]));
    REG_UPDATE!(MCIF_WB_BUF_1_ADDR_C, MCIF_WB_BUF_1_ADDR_C, MCIF_ADDR!((*params).chroma_address[0]));
    REG_UPDATE!(MCIF_WB_BUF_1_ADDR_C_HIGH, MCIF_WB_BUF_1_ADDR_C_HIGH, MCIF_ADDR_HIGH!((*params).chroma_address[0]));
    REG_UPDATE!(MCIF_WB_BUF_2_ADDR_Y, MCIF_WB_BUF_2_ADDR_Y, MCIF_ADDR!((*params).luma_address[1]));
    REG_UPDATE!(MCIF_WB_BUF_2_ADDR_Y_HIGH, MCIF_WB_BUF_2_ADDR_Y_HIGH, MCIF_ADDR_HIGH!((*params).luma_address[1]));
    REG_UPDATE!(MCIF_WB_BUF_2_ADDR_C, MCIF_WB_BUF_2_ADDR_C, MCIF_ADDR!((*params).chroma_address[1]));
    REG_UPDATE!(MCIF_WB_BUF_2_ADDR_C_HIGH, MCIF_WB_BUF_2_ADDR_C_HIGH, MCIF_ADDR_HIGH!((*params).chroma_address[1]));
    REG_UPDATE!(MCIF_WB_BUF_3_ADDR_Y, MCIF_WB_BUF_3_ADDR_Y, MCIF_ADDR!((*params).luma_address[2]));
    REG_UPDATE!(MCIF_WB_BUF_3_ADDR_Y_HIGH, MCIF_WB_BUF_3_ADDR_Y_HIGH, MCIF_ADDR_HIGH!((*params).luma_address[2]));
    REG_UPDATE!(MCIF_WB_BUF_3_ADDR_C, MCIF_WB_BUF_3_ADDR_C, MCIF_ADDR!((*params).chroma_address[2]));
    REG_UPDATE!(MCIF_WB_BUF_3_ADDR_C_HIGH, MCIF_WB_BUF_3_ADDR_C_HIGH, MCIF_ADDR_HIGH!((*params).chroma_address[2]));
    REG_UPDATE!(MCIF_WB_BUF_4_ADDR_Y, MCIF_WB_BUF_4_ADDR_Y, MCIF_ADDR!((*params).luma_address[3]));
    REG_UPDATE!(MCIF_WB_BUF_4_ADDR_Y_HIGH, MCIF_WB_BUF_4_ADDR_Y_HIGH, MCIF_ADDR_HIGH!((*params).luma_address[3]));
    REG_UPDATE!(MCIF_WB_BUF_4_ADDR_C, MCIF_WB_BUF_4_ADDR_C, MCIF_ADDR!((*params).chroma_address[3]));
    REG_UPDATE!(MCIF_WB_BUF_4_ADDR_C_HIGH, MCIF_WB_BUF_4_ADDR_C_HIGH, MCIF_ADDR_HIGH!((*params).chroma_address[3]));
    REG_UPDATE!(MCIF_WB_BUF_LUMA_SIZE, MCIF_WB_BUF_LUMA_SIZE, ((*params).luma_pitch >> 8) * dest_height);
    REG_UPDATE!(MCIF_WB_BUF_CHROMA_SIZE, MCIF_WB_BUF_CHROMA_SIZE, ((*params).chroma_pitch >> 8) * dest_height);
    REG_UPDATE!(MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB_BUF_ADDR_FENCE_EN, 1);
    REG_UPDATE_2!(MCIF_WB_BUF_PITCH, MCIF_WB_BUF_LUMA_PITCH, (*params).luma_pitch >> 8,
        MCIF_WB_BUF_CHROMA_PITCH, (*params).chroma_pitch >> 8);
}

unsafe fn mmhubbub3_config_mcif_arb(mcif_wb: *mut mcif_wb, params: *mut mcif_arb_params) {
    let _mcif_wb30: *mut dcn30_mmhubbub = TO_DCN30_MMHUBBUB(mcif_wb);
    REG_UPDATE!(MCIF_WB_ARBITRATION_CONTROL, MCIF_WB_TIME_PER_PIXEL, (*params).time_per_pixel);
    REG_UPDATE!(MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK_MASK, 0x0);
    REG_UPDATE!(MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK, (*params).cli_watermark[0]);
    REG_UPDATE!(MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK_MASK, 0x1);
    REG_UPDATE!(MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK, (*params).cli_watermark[1]);
    REG_UPDATE!(MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK_MASK, 0x2);
    REG_UPDATE!(MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK, (*params).cli_watermark[2]);
    REG_UPDATE!(MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK_MASK, 0x3);
    REG_UPDATE!(MCIF_WB_WATERMARK, MCIF_WB_CLI_WATERMARK, (*params).cli_watermark[3]);
    REG_UPDATE!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_WATERMARK_MASK, 0x0);
    REG_UPDATE!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_REFRESH_WATERMARK, (*params).pstate_watermark[0]);
    REG_UPDATE!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_WATERMARK_MASK, 0x1);
    REG_UPDATE!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_REFRESH_WATERMARK, (*params).pstate_watermark[1]);
    REG_UPDATE!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_WATERMARK_MASK, 0x2);
    REG_UPDATE!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_REFRESH_WATERMARK, (*params).pstate_watermark[2]);
    REG_UPDATE!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_WATERMARK_MASK, 0x3);
    REG_UPDATE!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, NB_PSTATE_CHANGE_REFRESH_WATERMARK, (*params).pstate_watermark[3]);
    REG_UPDATE!(MCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI, MCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI, (*params).dram_speed_change_duration);
    REG_UPDATE!(MULTI_LEVEL_QOS_CTRL, MAX_SCALED_TIME_TO_URGENT, (*params).max_scaled_time);
    REG_UPDATE!(MCIF_WB_BUFMGR_VCE_CONTROL, MCIF_WB_BUFMGR_SLICE_SIZE, (*params).slice_lines - 1);
    REG_UPDATE!(MCIF_WB_ARBITRATION_CONTROL, MCIF_WB_CLIENT_ARBITRATION_SLICE, (*params).arbitration_slice);
}

static dcn30_mmhubbub_funcs: mcif_wb_funcs = mcif_wb_funcs {
    warmup_mcif: Some(mmhubbub3_warmup_mcif), enable_mcif: Some(mmhubbub2_enable_mcif),
    disable_mcif: Some(mmhubbub2_disable_mcif), config_mcif_buf: Some(mmhubbub3_config_mcif_buf),
    config_mcif_arb: Some(mmhubbub3_config_mcif_arb), config_mcif_irq: Some(mmhubbub2_config_mcif_irq),
    dump_frame: Some(mcifwb2_dump_frame),
};

unsafe fn dcn30_mmhubbub_construct(
    mcif_wb30: *mut dcn30_mmhubbub, ctx: *mut dc_context,
    mcif_wb_regs: *const dcn30_mmhubbub_registers,
    mcif_wb_shift: *const dcn30_mmhubbub_shift,
    mcif_wb_mask: *const dcn30_mmhubbub_mask, inst: i32,
) {
    (*mcif_wb30).base.ctx = ctx;
    (*mcif_wb30).base.inst = inst;
    (*mcif_wb30).base.funcs = &dcn30_mmhubbub_funcs;
    (*mcif_wb30).mcif_wb_regs = mcif_wb_regs;
    (*mcif_wb30).mcif_wb_shift = mcif_wb_shift;
    (*mcif_wb30).mcif_wb_mask = mcif_wb_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
