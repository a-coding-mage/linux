// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dcn60_dsc.c. External types, functions, and register macros
// are supplied by the surrounding kernel translation.

static dcn60_dsc_funcs: dsc_funcs = dsc_funcs {
    dsc_read_state: dsc401_read_state,
    dsc_validate_stream: dsc401_validate_stream,
    dsc_set_config: dsc60_set_config,
    dsc_get_packed_pps: dsc2_get_packed_pps,
    dsc_enable: dsc401_enable,
    dsc_disable: dsc401_disable,
    dsc_disconnect: dsc401_disconnect,
    dsc_wait_disconnect_pending_clear: dsc401_wait_disconnect_pending_clear,
    dsc_get_single_enc_caps: dsc60_get_single_enc_caps,
    dsc_read_reg_state: dsc2_read_reg_state,
    set_fgcg: dsc60_set_fgcg,
};

pub unsafe fn dsc60_set_fgcg(dsc: *mut display_stream_compressor, enable: bool) {
    let dsc60 = TO_DCN60_DSC(dsc);
    REG_UPDATE!(dsc60, DSC_TOP_CONTROL, DSC_FGCG_REP_DIS, !enable);
}

pub unsafe fn dsc60_construct(
    dsc: *mut dcn60_dsc,
    ctx: *mut dc_context,
    inst: i32,
    dsc_regs: *const dcn401_dsc_registers,
    dsc_shift: *const dcn60_dsc_shift,
    dsc_mask: *const dcn60_dsc_mask,
) {
    (*dsc).base.ctx = ctx;
    (*dsc).base.inst = inst;
    (*dsc).base.funcs = &dcn60_dsc_funcs;
    (*dsc).dsc_regs = dsc_regs;
    (*dsc).dsc_shift = dsc_shift;
    (*dsc).dsc_mask = dsc_mask;
    (*dsc).max_image_width = 5184;
}

unsafe fn dsc60_init_reg_values(reg_vals: *mut dsc60_reg_values) {
    core::ptr::write_bytes(reg_vals, 0, 1);
    (*reg_vals).dsc_clock_enable = 1;
    (*reg_vals).dsc_clock_gating_disable = 0;
    (*reg_vals).underflow_recovery_en = 0;
    (*reg_vals).underflow_occurred_int_en = 0;
    (*reg_vals).underflow_occurred_status = 0;
    (*reg_vals).ich_reset_at_eol = 0;
    (*reg_vals).alternate_ich_encoding_en = 0;
    (*reg_vals).rc_buffer_model_size = 0;
    (*reg_vals).dsc_dbg_en = 0;
    for i in 0..8 { (*reg_vals).rc_buffer_model_overflow_int_en[i] = 0; }

    (*reg_vals).pps.dsc_version_minor = 2;
    (*reg_vals).pps.dsc_version_major = 1;
    (*reg_vals).pps.line_buf_depth = 9;
    (*reg_vals).pps.bits_per_component = 8;
    (*reg_vals).pps.block_pred_enable = 1;
    (*reg_vals).pps.slice_chunk_size = 0;
    (*reg_vals).pps.pic_width = 0;
    (*reg_vals).pps.pic_height = 0;
    (*reg_vals).pps.slice_width = 0;
    (*reg_vals).pps.slice_height = 0;
    (*reg_vals).pps.initial_xmit_delay = 170;
    (*reg_vals).pps.initial_dec_delay = 0;
    (*reg_vals).pps.initial_scale_value = 0;
    (*reg_vals).pps.scale_increment_interval = 0;
    (*reg_vals).pps.scale_decrement_interval = 0;
    (*reg_vals).pps.nfl_bpg_offset = 0;
    (*reg_vals).pps.slice_bpg_offset = 0;
    (*reg_vals).pps.nsl_bpg_offset = 0;
    (*reg_vals).pps.initial_offset = 6144;
    (*reg_vals).pps.final_offset = 0;
    (*reg_vals).pps.flatness_min_qp = 3;
    (*reg_vals).pps.flatness_max_qp = 12;
    (*reg_vals).pps.rc_model_size = 8192;
    (*reg_vals).pps.rc_edge_factor = 6;
    (*reg_vals).pps.rc_quant_incr_limit0 = 11;
    (*reg_vals).pps.rc_quant_incr_limit1 = 11;
    (*reg_vals).pps.rc_tgt_offset_low = 3;
    (*reg_vals).pps.rc_tgt_offset_high = 3;
}

unsafe fn dsc60_update_from_dsc_parameters(reg_vals: *mut dsc60_reg_values, dsc_params: *const dsc_parameters) {
    (*reg_vals).pps = (*dsc_params).pps;
    for i in 0..(NUM_BUF_RANGES - 1) { (*reg_vals).pps.rc_buf_thresh[i] >>= 6; }
    (*reg_vals).rc_buffer_model_size = (*dsc_params).rc_buffer_model_size;
}

unsafe fn dsc60_prepare_config(cfg: *const dsc_config, vals: *mut dsc60_reg_values, optc: *mut dsc_optc_config) -> bool {
    ASSERT!((*cfg).dc_dsc_cfg.num_slices_h);
    ASSERT!((*cfg).dc_dsc_cfg.num_slices_v);
    ASSERT!((*cfg).dc_dsc_cfg.version_minor == 1 || (*cfg).dc_dsc_cfg.version_minor == 2);
    ASSERT!((*cfg).pic_width);
    ASSERT!((*cfg).pic_height);
    ASSERT!(((*cfg).dc_dsc_cfg.version_minor == 1 && (*cfg).dc_dsc_cfg.linebuf_depth >= 8 && (*cfg).dc_dsc_cfg.linebuf_depth <= 13) || ((*cfg).dc_dsc_cfg.version_minor == 2 && (((*cfg).dc_dsc_cfg.linebuf_depth >= 8 && (*cfg).dc_dsc_cfg.linebuf_depth <= 15) || (*cfg).dc_dsc_cfg.linebuf_depth == 0)));
    ASSERT!((*cfg).dc_dsc_cfg.bits_per_pixel >= 96 && (*cfg).dc_dsc_cfg.bits_per_pixel <= 0x3ff);
    if (*cfg).dc_dsc_cfg.num_slices_v == 0 || (*cfg).dc_dsc_cfg.num_slices_h == 0 ||
       !((*cfg).dc_dsc_cfg.version_minor == 1 || (*cfg).dc_dsc_cfg.version_minor == 2) ||
       (*cfg).pic_width == 0 || (*cfg).pic_height == 0 ||
       !(((*cfg).dc_dsc_cfg.version_minor == 1 && (*cfg).dc_dsc_cfg.linebuf_depth >= 8 && (*cfg).dc_dsc_cfg.linebuf_depth <= 13) || ((*cfg).dc_dsc_cfg.version_minor == 2 && (((*cfg).dc_dsc_cfg.linebuf_depth >= 8 && (*cfg).dc_dsc_cfg.linebuf_depth <= 15) || (*cfg).dc_dsc_cfg.linebuf_depth == 0))) ||
       (*cfg).dc_dsc_cfg.bits_per_pixel < 96 || (*cfg).dc_dsc_cfg.bits_per_pixel > 0x3ff { return false; }
    dsc60_init_reg_values(vals);
    (*vals).pixel_format = dsc_dc_pixel_encoding_to_dsc_pixel_format((*cfg).pixel_encoding, (*cfg).dc_dsc_cfg.ycbcr422_simple);
    (*vals).num_slices_h = (*cfg).dc_dsc_cfg.num_slices_h;
    (*vals).num_slices_v = (*cfg).dc_dsc_cfg.num_slices_v;
    (*vals).pps.dsc_version_minor = (*cfg).dc_dsc_cfg.version_minor as u8;
    (*vals).pps.pic_width = (*cfg).pic_width as u16;
    (*vals).pps.pic_height = (*cfg).pic_height as u16;
    (*vals).pps.bits_per_component = dsc_dc_color_depth_to_dsc_bits_per_comp((*cfg).color_depth);
    (*vals).pps.block_pred_enable = (*cfg).dc_dsc_cfg.block_pred_enable;
    (*vals).pps.line_buf_depth = (*cfg).dc_dsc_cfg.linebuf_depth as u8;
    (*vals).alternate_ich_encoding_en = if (*vals).pps.dsc_version_minor == 1 { 0 } else { 1 };
    (*vals).ich_reset_at_eol = if (*cfg).is_odm || (*vals).num_slices_h > 1 { 0xF } else { 0 };
    (*vals).pps.slice_width = ((*cfg).pic_width / (*cfg).dc_dsc_cfg.num_slices_h) as u16;
    (*vals).pps.slice_height = ((*cfg).pic_height / (*cfg).dc_dsc_cfg.num_slices_v) as u16;
    if (*vals).pps.slice_height as u32 * (*cfg).dc_dsc_cfg.num_slices_v != (*cfg).pic_height { return false; }
    (*vals).bpp_x32 = (*cfg).dc_dsc_cfg.bits_per_pixel << 1;
    if (*vals).pixel_format == DSC_PIXFMT_NATIVE_YCBCR420 || (*vals).pixel_format == DSC_PIXFMT_NATIVE_YCBCR422 { (*vals).pps.bits_per_pixel = (*vals).bpp_x32 as u16; } else { (*vals).pps.bits_per_pixel = ((*vals).bpp_x32 >> 1) as u16; }
    (*vals).pps.convert_rgb = ((*vals).pixel_format == DSC_PIXFMT_RGB) as _;
    (*vals).pps.native_422 = (*vals).pixel_format == DSC_PIXFMT_NATIVE_YCBCR422;
    (*vals).pps.native_420 = (*vals).pixel_format == DSC_PIXFMT_NATIVE_YCBCR420;
    (*vals).pps.simple_422 = (*vals).pixel_format == DSC_PIXFMT_SIMPLE_YCBCR422;
    let mut rc: rc_params = core::mem::zeroed(); let mut params: dsc_parameters = core::mem::zeroed();
    calc_rc_params(&mut rc, &(*vals).pps);
    if (*cfg).dc_dsc_cfg.rc_params_ovrd != core::ptr::null() { dsc_override_rc_params(&mut rc, (*cfg).dc_dsc_cfg.rc_params_ovrd); }
    if dscc_compute_dsc_parameters(&mut (*vals).pps, &mut rc, &mut params) { return false; }
    dsc60_update_from_dsc_parameters(vals, &params);
    (*optc).bytes_per_pixel = params.bytes_per_pixel;
    (*optc).slice_width = (*vals).pps.slice_width;
    (*optc).is_pixel_format_444 = (*vals).pixel_format == DSC_PIXFMT_RGB || (*vals).pixel_format == DSC_PIXFMT_YCBCR444 || (*vals).pixel_format == DSC_PIXFMT_SIMPLE_YCBCR422;
    true
}

unsafe fn dsc60_write_to_registers(dsc: *mut display_stream_compressor, vals: *const dsc60_reg_values) {
    let dsc60 = TO_DCN60_DSC(dsc);
    REG_SET!(dsc60, DSC_DEBUG_CONTROL, 0, DSC_DBG_EN, (*vals).dsc_dbg_en);
    // Register programming below intentionally retains the source register macros.
    REG_SET_3!(dsc60, DSCC_CONFIG0, 0, NUMBER_OF_SLICES_PER_LINE, (*vals).num_slices_h - 1, ALTERNATE_ICH_ENCODING_EN, (*vals).alternate_ich_encoding_en, NUMBER_OF_SLICES_IN_VERTICAL_DIRECTION, (*vals).num_slices_v - 1);
    REG_SET!(dsc60, DSCC_CONFIG1, 0, DSCC_RATE_CONTROL_BUFFER_MODEL_SIZE, (*vals).rc_buffer_model_size);
    REG_SET_8!(dsc60, DSCC_INTERRUPT_CONTROL0, 0,
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN0, (*vals).rc_buffer_model_overflow_int_en[0],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN1, (*vals).rc_buffer_model_overflow_int_en[1],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN2, (*vals).rc_buffer_model_overflow_int_en[2],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN3, (*vals).rc_buffer_model_overflow_int_en[3],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN4, (*vals).rc_buffer_model_overflow_int_en[4],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN5, (*vals).rc_buffer_model_overflow_int_en[5],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN6, (*vals).rc_buffer_model_overflow_int_en[6],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN7, (*vals).rc_buffer_model_overflow_int_en[7]);
    REG_SET_3!(dsc60, DSCC_PPS_CONFIG0, 0, DSC_VERSION_MINOR, (*vals).pps.dsc_version_minor, LINEBUF_DEPTH, (*vals).pps.line_buf_depth, DSCC_PPS_CONFIG0__BITS_PER_COMPONENT, (*vals).pps.bits_per_component);
    REG_SET_7!(dsc60, DSCC_PPS_CONFIG1, 0, BITS_PER_PIXEL, (*vals).pps.bits_per_pixel, SIMPLE_422, (*vals).pps.simple_422, CONVERT_RGB, (*vals).pps.convert_rgb, BLOCK_PRED_ENABLE, (*vals).pps.block_pred_enable, NATIVE_422, (*vals).pps.native_422, NATIVE_420, (*vals).pps.native_420, CHUNK_SIZE, (*vals).pps.slice_chunk_size);
    REG_SET_2!(dsc60, DSCC_PPS_CONFIG2, 0, PIC_WIDTH, (*vals).pps.pic_width, PIC_HEIGHT, (*vals).pps.pic_height);
    REG_SET_2!(dsc60, DSCC_PPS_CONFIG3, 0, SLICE_WIDTH, (*vals).pps.slice_width, SLICE_HEIGHT, (*vals).pps.slice_height);
    REG_SET!(dsc60, DSCC_PPS_CONFIG4, 0, INITIAL_XMIT_DELAY, (*vals).pps.initial_xmit_delay);
    REG_SET_2!(dsc60, DSCC_PPS_CONFIG5, 0, INITIAL_SCALE_VALUE, (*vals).pps.initial_scale_value, SCALE_INCREMENT_INTERVAL, (*vals).pps.scale_increment_interval);
    REG_SET_3!(dsc60, DSCC_PPS_CONFIG6, 0, SCALE_DECREMENT_INTERVAL, (*vals).pps.scale_decrement_interval, FIRST_LINE_BPG_OFFSET, (*vals).pps.first_line_bpg_offset, SECOND_LINE_BPG_OFFSET, (*vals).pps.second_line_bpg_offset);
    REG_SET_2!(dsc60, DSCC_PPS_CONFIG7, 0, NFL_BPG_OFFSET, (*vals).pps.nfl_bpg_offset, SLICE_BPG_OFFSET, (*vals).pps.slice_bpg_offset);
    REG_SET_2!(dsc60, DSCC_PPS_CONFIG8, 0, NSL_BPG_OFFSET, (*vals).pps.nsl_bpg_offset, SECOND_LINE_OFFSET_ADJ, (*vals).pps.second_line_offset_adj);
    REG_SET_2!(dsc60, DSCC_PPS_CONFIG9, 0, INITIAL_OFFSET, (*vals).pps.initial_offset, FINAL_OFFSET, (*vals).pps.final_offset);
    REG_SET_3!(dsc60, DSCC_PPS_CONFIG10, 0, FLATNESS_MIN_QP, (*vals).pps.flatness_min_qp, FLATNESS_MAX_QP, (*vals).pps.flatness_max_qp, RC_MODEL_SIZE, (*vals).pps.rc_model_size);
    REG_SET_5!(dsc60, DSCC_PPS_CONFIG11, 0, RC_EDGE_FACTOR, (*vals).pps.rc_edge_factor, RC_QUANT_INCR_LIMIT0, (*vals).pps.rc_quant_incr_limit0, RC_QUANT_INCR_LIMIT1, (*vals).pps.rc_quant_incr_limit1, RC_TGT_OFFSET_LO, (*vals).pps.rc_tgt_offset_low, RC_TGT_OFFSET_HI, (*vals).pps.rc_tgt_offset_high);
    REG_SET_4!(dsc60, DSCC_PPS_CONFIG12, 0, RC_BUF_THRESH0, (*vals).pps.rc_buf_thresh[0], RC_BUF_THRESH1, (*vals).pps.rc_buf_thresh[1], RC_BUF_THRESH2, (*vals).pps.rc_buf_thresh[2], RC_BUF_THRESH3, (*vals).pps.rc_buf_thresh[3]);
    REG_SET_4!(dsc60, DSCC_PPS_CONFIG13, 0, RC_BUF_THRESH4, (*vals).pps.rc_buf_thresh[4], RC_BUF_THRESH5, (*vals).pps.rc_buf_thresh[5], RC_BUF_THRESH6, (*vals).pps.rc_buf_thresh[6], RC_BUF_THRESH7, (*vals).pps.rc_buf_thresh[7]);
    REG_SET_4!(dsc60, DSCC_PPS_CONFIG14, 0, RC_BUF_THRESH8, (*vals).pps.rc_buf_thresh[8], RC_BUF_THRESH9, (*vals).pps.rc_buf_thresh[9], RC_BUF_THRESH10, (*vals).pps.rc_buf_thresh[10], RC_BUF_THRESH11, (*vals).pps.rc_buf_thresh[11]);
    REG_SET_5!(dsc60, DSCC_PPS_CONFIG15, 0, RC_BUF_THRESH12, (*vals).pps.rc_buf_thresh[12], RC_BUF_THRESH13, (*vals).pps.rc_buf_thresh[13], RANGE_MIN_QP0, (*vals).pps.rc_range_params[0].range_min_qp, RANGE_MAX_QP0, (*vals).pps.rc_range_params[0].range_max_qp, RANGE_BPG_OFFSET0, (*vals).pps.rc_range_params[0].range_bpg_offset);
    REG_SET_6!(dsc60, DSCC_PPS_CONFIG16, 0, RANGE_MIN_QP1, (*vals).pps.rc_range_params[1].range_min_qp, RANGE_MAX_QP1, (*vals).pps.rc_range_params[1].range_max_qp, RANGE_BPG_OFFSET1, (*vals).pps.rc_range_params[1].range_bpg_offset, RANGE_MIN_QP2, (*vals).pps.rc_range_params[2].range_min_qp, RANGE_MAX_QP2, (*vals).pps.rc_range_params[2].range_max_qp, RANGE_BPG_OFFSET2, (*vals).pps.rc_range_params[2].range_bpg_offset);
    REG_SET_6!(dsc60, DSCC_PPS_CONFIG17, 0, RANGE_MIN_QP3, (*vals).pps.rc_range_params[3].range_min_qp, RANGE_MAX_QP3, (*vals).pps.rc_range_params[3].range_max_qp, RANGE_BPG_OFFSET3, (*vals).pps.rc_range_params[3].range_bpg_offset, RANGE_MIN_QP4, (*vals).pps.rc_range_params[4].range_min_qp, RANGE_MAX_QP4, (*vals).pps.rc_range_params[4].range_max_qp, RANGE_BPG_OFFSET4, (*vals).pps.rc_range_params[4].range_bpg_offset);
    REG_SET_6!(dsc60, DSCC_PPS_CONFIG18, 0, RANGE_MIN_QP5, (*vals).pps.rc_range_params[5].range_min_qp, RANGE_MAX_QP5, (*vals).pps.rc_range_params[5].range_max_qp, RANGE_BPG_OFFSET5, (*vals).pps.rc_range_params[5].range_bpg_offset, RANGE_MIN_QP6, (*vals).pps.rc_range_params[6].range_min_qp, RANGE_MAX_QP6, (*vals).pps.rc_range_params[6].range_max_qp, RANGE_BPG_OFFSET6, (*vals).pps.rc_range_params[6].range_bpg_offset);
    REG_SET_6!(dsc60, DSCC_PPS_CONFIG19, 0, RANGE_MIN_QP7, (*vals).pps.rc_range_params[7].range_min_qp, RANGE_MAX_QP7, (*vals).pps.rc_range_params[7].range_max_qp, RANGE_BPG_OFFSET7, (*vals).pps.rc_range_params[7].range_bpg_offset, RANGE_MIN_QP8, (*vals).pps.rc_range_params[8].range_min_qp, RANGE_MAX_QP8, (*vals).pps.rc_range_params[8].range_max_qp, RANGE_BPG_OFFSET8, (*vals).pps.rc_range_params[8].range_bpg_offset);
    REG_SET_6!(dsc60, DSCC_PPS_CONFIG20, 0, RANGE_MIN_QP9, (*vals).pps.rc_range_params[9].range_min_qp, RANGE_MAX_QP9, (*vals).pps.rc_range_params[9].range_max_qp, RANGE_BPG_OFFSET9, (*vals).pps.rc_range_params[9].range_bpg_offset, RANGE_MIN_QP10, (*vals).pps.rc_range_params[10].range_min_qp, RANGE_MAX_QP10, (*vals).pps.rc_range_params[10].range_max_qp, RANGE_BPG_OFFSET10, (*vals).pps.rc_range_params[10].range_bpg_offset);
    REG_SET_6!(dsc60, DSCC_PPS_CONFIG21, 0, RANGE_MIN_QP11, (*vals).pps.rc_range_params[11].range_min_qp, RANGE_MAX_QP11, (*vals).pps.rc_range_params[11].range_max_qp, RANGE_BPG_OFFSET11, (*vals).pps.rc_range_params[11].range_bpg_offset, RANGE_MIN_QP12, (*vals).pps.rc_range_params[12].range_min_qp, RANGE_MAX_QP12, (*vals).pps.rc_range_params[12].range_max_qp, RANGE_BPG_OFFSET12, (*vals).pps.rc_range_params[12].range_bpg_offset);
    REG_SET_6!(dsc60, DSCC_PPS_CONFIG22, 0, RANGE_MIN_QP13, (*vals).pps.rc_range_params[13].range_min_qp, RANGE_MAX_QP13, (*vals).pps.rc_range_params[13].range_max_qp, RANGE_BPG_OFFSET13, (*vals).pps.rc_range_params[13].range_bpg_offset, RANGE_MIN_QP14, (*vals).pps.rc_range_params[14].range_min_qp, RANGE_MAX_QP14, (*vals).pps.rc_range_params[14].range_max_qp, RANGE_BPG_OFFSET14, (*vals).pps.rc_range_params[14].range_bpg_offset);
}

pub unsafe fn dsc60_set_config(dsc: *mut display_stream_compressor, cfg: *const dsc_config, optc: *mut dsc_optc_config) {
    let dsc60 = TO_DCN60_DSC(dsc); dsc_config_log(dsc, cfg);
    let ok = dsc60_prepare_config(cfg, &mut (*dsc60).reg_vals, optc); ASSERT!(ok);
    dsc_log_pps(dsc, &(*dsc60).reg_vals.pps); dsc60_write_to_registers(dsc, &(*dsc60).reg_vals);
}

pub unsafe fn dsc60_get_single_enc_caps(caps: *mut dsc_enc_caps, max_dscclk_khz: u32) {
    (*caps).dsc_version = 0x21;
    (*caps).slice_caps.bits.NUM_SLICES_1 = 1; (*caps).slice_caps.bits.NUM_SLICES_2 = 1;
    (*caps).slice_caps.bits.NUM_SLICES_3 = 1; (*caps).slice_caps.bits.NUM_SLICES_4 = 1; (*caps).slice_caps.bits.NUM_SLICES_8 = 1;
    (*caps).lb_bit_depth = 13; (*caps).is_block_pred_supported = true;
    (*caps).color_formats.bits.RGB = 1; (*caps).color_formats.bits.YCBCR_444 = 1; (*caps).color_formats.bits.YCBCR_SIMPLE_422 = 1; (*caps).color_formats.bits.YCBCR_NATIVE_422 = 1; (*caps).color_formats.bits.YCBCR_NATIVE_420 = 1;
    (*caps).color_depth.bits.COLOR_DEPTH_8_BPC = 1; (*caps).color_depth.bits.COLOR_DEPTH_10_BPC = 1; (*caps).color_depth.bits.COLOR_DEPTH_12_BPC = 1;
    (*caps).max_total_throughput_mps = max_dscclk_khz * 3 / 1000; (*caps).max_slice_width = 5760; (*caps).bpp_increment_div = 16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
