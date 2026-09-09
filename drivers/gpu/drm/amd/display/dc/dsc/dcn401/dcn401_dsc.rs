// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit.

unsafe extern "C" {
    fn dsc401_read_state(dsc: *mut display_stream_compressor, s: *mut dcn_dsc_state);
    fn dsc401_validate_stream(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config) -> bool;
    fn dsc401_set_config(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config, dsc_optc_cfg: *mut dsc_optc_config);
    fn dsc2_get_packed_pps();
    fn dsc401_enable(dsc: *mut display_stream_compressor, opp_pipe: i32);
    fn dsc401_disable(dsc: *mut display_stream_compressor);
    fn dsc401_disconnect(dsc: *mut display_stream_compressor);
    fn dsc401_wait_disconnect_pending_clear(dsc: *mut display_stream_compressor);
    fn dsc2_read_reg_state();
}

// Object I/F functions
// static void dsc401_get_enc_caps(struct dsc_enc_caps *dsc_enc_caps, int pixel_clock_100Hz);
// static bool dsc401_get_packed_pps(struct display_stream_compressor *dsc, const struct dsc_config *dsc_cfg, uint8_t *dsc_packed_pps);
unsafe fn dsc401_get_single_enc_caps(dsc_enc_caps: *mut dsc_enc_caps, max_dscclk_khz: u32) {
    (*dsc_enc_caps).dsc_version = 0x21; // v1.2 - DP spec defined it in reverse order and we kept it

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
    (*dsc_enc_caps).max_slice_width = 5184;
    (*dsc_enc_caps).bpp_increment_div = 16;
}

pub unsafe fn dsc401_construct(dsc: *mut dcn401_dsc, ctx: *mut dc_context, inst: i32,
    dsc_regs: *const dcn401_dsc_registers, dsc_shift: *const dcn401_dsc_shift,
    dsc_mask: *const dcn401_dsc_mask) {
    (*dsc).base.ctx = ctx;
    (*dsc).base.inst = inst;
    (*dsc).base.funcs = &dcn401_dsc_funcs;
    (*dsc).dsc_regs = dsc_regs;
    (*dsc).dsc_shift = dsc_shift;
    (*dsc).dsc_mask = dsc_mask;
    (*dsc).max_image_width = 5184;
}

// The register helper macros retain their C register-field semantics in the Rust binding.
pub unsafe fn dsc401_read_state(dsc: *mut display_stream_compressor, s: *mut dcn_dsc_state) {
    let dsc401 = TO_DCN401_DSC!(dsc);
    REG_GET!(dsc401, DSC_TOP_CONTROL, DSC_CLOCK_EN, &mut (*s).dsc_clock_en);
    REG_GET!(dsc401, DSCC_PPS_CONFIG3, SLICE_WIDTH, &mut (*s).dsc_slice_width);
    REG_GET!(dsc401, DSCC_PPS_CONFIG1, BITS_PER_PIXEL, &mut (*s).dsc_bits_per_pixel);
    REG_GET!(dsc401, DSCC_PPS_CONFIG3, SLICE_HEIGHT, &mut (*s).dsc_slice_height);
    REG_GET!(dsc401, DSCC_PPS_CONFIG1, CHUNK_SIZE, &mut (*s).dsc_chunk_size);
    REG_GET!(dsc401, DSCC_PPS_CONFIG2, PIC_WIDTH, &mut (*s).dsc_pic_width);
    REG_GET!(dsc401, DSCC_PPS_CONFIG2, PIC_HEIGHT, &mut (*s).dsc_pic_height);
    REG_GET!(dsc401, DSCC_PPS_CONFIG7, SLICE_BPG_OFFSET, &mut (*s).dsc_slice_bpg_offset);
    REG_GET_2!(dsc401, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, &mut (*s).dsc_fw_en,
        DSCRM_DSC_OPP_PIPE_SOURCE, &mut (*s).dsc_opp_source);
    REG_GET!(dsc401, DSCC_PPS_CONFIG1, BLOCK_PRED_ENABLE, &mut (*s).dsc_block_pred_enable);
    REG_GET!(dsc401, DSCC_PPS_CONFIG0, LINEBUF_DEPTH, &mut (*s).dsc_line_buf_depth);
    REG_GET!(dsc401, DSCC_PPS_CONFIG0, DSC_VERSION_MINOR, &mut (*s).dsc_version_minor);
    REG_GET!(dsc401, DSCC_CONFIG1, DSCC_RATE_CONTROL_BUFFER_MODEL_SIZE, &mut (*s).dsc_rc_buffer_size);
    REG_GET!(dsc401, DSCC_PPS_CONFIG0, SIMPLE_422, &mut (*s).dsc_simple_422);
}

pub unsafe fn dsc401_validate_stream(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config) -> bool {
    let dsc401 = TO_DCN401_DSC!(dsc);
    if (*dsc_cfg).pic_width > (*dsc401).max_image_width as u32 { return false; }
    let mut dsc_optc_cfg = core::mem::MaybeUninit::<dsc_optc_config>::uninit();
    dsc_prepare_config(dsc_cfg, &mut (*dsc401).reg_vals, dsc_optc_cfg.as_mut_ptr())
}

pub unsafe fn dsc401_set_config(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config,
    dsc_optc_cfg: *mut dsc_optc_config) {
    let dsc401 = TO_DCN401_DSC!(dsc);
    DC_LOG_DSC!(dsc, "Setting DSC Config at DSC inst %d", (*dsc).inst);
    dsc_config_log(dsc, dsc_cfg);
    let is_config_ok = dsc_prepare_config(dsc_cfg, &mut (*dsc401).reg_vals, dsc_optc_cfg);
    ASSERT!(is_config_ok);
    DC_LOG_DSC!(dsc, "programming DSC Picture Parameter Set (PPS):");
    dsc_log_pps(dsc, &(*dsc401).reg_vals.pps);
    dsc_write_to_registers(dsc, &(*dsc401).reg_vals);
}

pub unsafe fn dsc401_enable(dsc: *mut display_stream_compressor, opp_pipe: i32) {
    let dsc401 = TO_DCN401_DSC!(dsc);
    let mut dsc_clock_en = 0u32;
    let mut dsc_fw_config = 0u32;
    let mut enabled_opp_pipe = 0u32;
    DC_LOG_DSC!(dsc, "enable DSC %d at opp pipe %d", (*dsc).inst, opp_pipe);
    REG_GET!(dsc401, DSC_TOP_CONTROL, DSC_CLOCK_EN, &mut dsc_clock_en);
    REG_GET_2!(dsc401, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, &mut dsc_fw_config,
        DSCRM_DSC_OPP_PIPE_SOURCE, &mut enabled_opp_pipe);
    if (dsc_clock_en != 0 || dsc_fw_config != 0) && enabled_opp_pipe != opp_pipe as u32 {
        DC_LOG_DSC!(dsc, "ERROR: DSC %d at opp pipe %u already enabled!", (*dsc).inst, enabled_opp_pipe);
        ASSERT!(false);
    }
    REG_UPDATE!(dsc401, DSC_TOP_CONTROL, DSC_CLOCK_EN, 1);
    REG_UPDATE_2!(dsc401, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, 1,
        DSCRM_DSC_OPP_PIPE_SOURCE, opp_pipe);
}

pub unsafe fn dsc401_disable(dsc: *mut display_stream_compressor) {
    let dsc401 = TO_DCN401_DSC!(dsc);
    let mut dsc_clock_en = 0u32;
    DC_LOG_DSC!(dsc, "disable DSC %d", (*dsc).inst);
    REG_GET!(dsc401, DSC_TOP_CONTROL, DSC_CLOCK_EN, &mut dsc_clock_en);
    if dsc_clock_en == 0 { DC_LOG_DSC!(dsc, "DSC %d already disabled!", (*dsc).inst); }
    REG_UPDATE!(dsc401, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, 0);
    REG_UPDATE!(dsc401, DSC_TOP_CONTROL, DSC_CLOCK_EN, 0);
}

pub unsafe fn dsc401_wait_disconnect_pending_clear(dsc: *mut display_stream_compressor) {
    let dsc401 = TO_DCN401_DSC!(dsc);
    REG_WAIT!(dsc401, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN_STATUS, 0, 2, 50000);
}

pub unsafe fn dsc401_disconnect(dsc: *mut display_stream_compressor) {
    let dsc401 = TO_DCN401_DSC!(dsc);
    DC_LOG_DSC!(dsc, "disconnect DSC %d", (*dsc).inst);
    REG_UPDATE!(dsc401, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, 0);
}

unsafe fn dsc_write_to_registers(dsc: *mut display_stream_compressor, reg_vals: *const dsc_reg_values) {
    let dsc401 = TO_DCN401_DSC!(dsc);
    let temp_int: u32;
    REG_SET!(dsc401, DSC_DEBUG_CONTROL, 0, DSC_DBG_EN, (*reg_vals).dsc_dbg_en);
    REG_SET_2!(dsc401, DSCCIF_CONFIG0, 0, INPUT_PIXEL_FORMAT, (*reg_vals).pixel_format,
        DSCCIF_CONFIG0__BITS_PER_COMPONENT, (*reg_vals).pps.bits_per_component);
    if (*dsc401).dsc_mask.as_ref().unwrap().ICH_RESET_AT_END_OF_LINE == 0 {
        REG_SET_3!(dsc401, DSCC_CONFIG0, 0, NUMBER_OF_SLICES_PER_LINE, (*reg_vals).num_slices_h - 1,
            ALTERNATE_ICH_ENCODING_EN, (*reg_vals).alternate_ich_encoding_en,
            NUMBER_OF_SLICES_IN_VERTICAL_DIRECTION, (*reg_vals).num_slices_v - 1);
    } else {
        REG_SET_4!(dsc401, DSCC_CONFIG0, 0, ICH_RESET_AT_END_OF_LINE, (*reg_vals).ich_reset_at_eol,
            NUMBER_OF_SLICES_PER_LINE, (*reg_vals).num_slices_h - 1,
            ALTERNATE_ICH_ENCODING_EN, (*reg_vals).alternate_ich_encoding_en,
            NUMBER_OF_SLICES_IN_VERTICAL_DIRECTION, (*reg_vals).num_slices_v - 1);
    }
    REG_SET!(dsc401, DSCC_CONFIG1, 0, DSCC_RATE_CONTROL_BUFFER_MODEL_SIZE, (*reg_vals).rc_buffer_model_size);
    REG_SET_4!(dsc401, DSCC_INTERRUPT_CONTROL0, 0,
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN0, (*reg_vals).rc_buffer_model_overflow_int_en[0],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN1, (*reg_vals).rc_buffer_model_overflow_int_en[1],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN2, (*reg_vals).rc_buffer_model_overflow_int_en[2],
        DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN3, (*reg_vals).rc_buffer_model_overflow_int_en[3]);
    REG_SET_3!(dsc401, DSCC_PPS_CONFIG0, 0, DSC_VERSION_MINOR, (*reg_vals).pps.dsc_version_minor,
        LINEBUF_DEPTH, (*reg_vals).pps.line_buf_depth, DSCC_PPS_CONFIG0__BITS_PER_COMPONENT, (*reg_vals).pps.bits_per_component);
    temp_int = if (*reg_vals).pixel_format == DSC_PIXFMT_NATIVE_YCBCR420 || (*reg_vals).pixel_format == DSC_PIXFMT_NATIVE_YCBCR422 { (*reg_vals).bpp_x32 } else { (*reg_vals).bpp_x32 >> 1 };
    REG_SET_7!(dsc401, DSCC_PPS_CONFIG1, 0, BITS_PER_PIXEL, temp_int, SIMPLE_422, (*reg_vals).pixel_format == DSC_PIXFMT_SIMPLE_YCBCR422,
        CONVERT_RGB, (*reg_vals).pixel_format == DSC_PIXFMT_RGB, BLOCK_PRED_ENABLE, (*reg_vals).pps.block_pred_enable,
        NATIVE_422, (*reg_vals).pixel_format == DSC_PIXFMT_NATIVE_YCBCR422, NATIVE_420, (*reg_vals).pixel_format == DSC_PIXFMT_NATIVE_YCBCR420,
        CHUNK_SIZE, (*reg_vals).pps.slice_chunk_size);
    REG_SET_2!(dsc401, DSCC_PPS_CONFIG2, 0, PIC_WIDTH, (*reg_vals).pps.pic_width, PIC_HEIGHT, (*reg_vals).pps.pic_height);
    REG_SET_2!(dsc401, DSCC_PPS_CONFIG3, 0, SLICE_WIDTH, (*reg_vals).pps.slice_width, SLICE_HEIGHT, (*reg_vals).pps.slice_height);
    REG_SET!(dsc401, DSCC_PPS_CONFIG4, 0, INITIAL_XMIT_DELAY, (*reg_vals).pps.initial_xmit_delay);
    REG_SET_2!(dsc401, DSCC_PPS_CONFIG5, 0, INITIAL_SCALE_VALUE, (*reg_vals).pps.initial_scale_value,
        SCALE_INCREMENT_INTERVAL, (*reg_vals).pps.scale_increment_interval);
    REG_SET_3!(dsc401, DSCC_PPS_CONFIG6, 0, SCALE_DECREMENT_INTERVAL, (*reg_vals).pps.scale_decrement_interval,
        FIRST_LINE_BPG_OFFSET, (*reg_vals).pps.first_line_bpg_offset, SECOND_LINE_BPG_OFFSET, (*reg_vals).pps.second_line_bpg_offset);
    REG_SET_2!(dsc401, DSCC_PPS_CONFIG7, 0, NFL_BPG_OFFSET, (*reg_vals).pps.nfl_bpg_offset, SLICE_BPG_OFFSET, (*reg_vals).pps.slice_bpg_offset);
    REG_SET_2!(dsc401, DSCC_PPS_CONFIG8, 0, NSL_BPG_OFFSET, (*reg_vals).pps.nsl_bpg_offset,
        SECOND_LINE_OFFSET_ADJ, (*reg_vals).pps.second_line_offset_adj);
    REG_SET_2!(dsc401, DSCC_PPS_CONFIG9, 0, INITIAL_OFFSET, (*reg_vals).pps.initial_offset, FINAL_OFFSET, (*reg_vals).pps.final_offset);
    REG_SET_3!(dsc401, DSCC_PPS_CONFIG10, 0, FLATNESS_MIN_QP, (*reg_vals).pps.flatness_min_qp,
        FLATNESS_MAX_QP, (*reg_vals).pps.flatness_max_qp, RC_MODEL_SIZE, (*reg_vals).pps.rc_model_size);
    REG_SET_5!(dsc401, DSCC_PPS_CONFIG11, 0, RC_EDGE_FACTOR, (*reg_vals).pps.rc_edge_factor,
        RC_QUANT_INCR_LIMIT0, (*reg_vals).pps.rc_quant_incr_limit0, RC_QUANT_INCR_LIMIT1, (*reg_vals).pps.rc_quant_incr_limit1,
        RC_TGT_OFFSET_LO, (*reg_vals).pps.rc_tgt_offset_low, RC_TGT_OFFSET_HI, (*reg_vals).pps.rc_tgt_offset_high);
    REG_SET_4!(dsc401, DSCC_PPS_CONFIG12, 0, RC_BUF_THRESH0, (*reg_vals).pps.rc_buf_thresh[0], RC_BUF_THRESH1, (*reg_vals).pps.rc_buf_thresh[1],
        RC_BUF_THRESH2, (*reg_vals).pps.rc_buf_thresh[2], RC_BUF_THRESH3, (*reg_vals).pps.rc_buf_thresh[3]);
    REG_SET_4!(dsc401, DSCC_PPS_CONFIG13, 0, RC_BUF_THRESH4, (*reg_vals).pps.rc_buf_thresh[4], RC_BUF_THRESH5, (*reg_vals).pps.rc_buf_thresh[5],
        RC_BUF_THRESH6, (*reg_vals).pps.rc_buf_thresh[6], RC_BUF_THRESH7, (*reg_vals).pps.rc_buf_thresh[7]);
    REG_SET_4!(dsc401, DSCC_PPS_CONFIG14, 0, RC_BUF_THRESH8, (*reg_vals).pps.rc_buf_thresh[8], RC_BUF_THRESH9, (*reg_vals).pps.rc_buf_thresh[9],
        RC_BUF_THRESH10, (*reg_vals).pps.rc_buf_thresh[10], RC_BUF_THRESH11, (*reg_vals).pps.rc_buf_thresh[11]);
    REG_SET_5!(dsc401, DSCC_PPS_CONFIG15, 0, RC_BUF_THRESH12, (*reg_vals).pps.rc_buf_thresh[12], RC_BUF_THRESH13, (*reg_vals).pps.rc_buf_thresh[13],
        RANGE_MIN_QP0, (*reg_vals).pps.rc_range_params[0].range_min_qp, RANGE_MAX_QP0, (*reg_vals).pps.rc_range_params[0].range_max_qp,
        RANGE_BPG_OFFSET0, (*reg_vals).pps.rc_range_params[0].range_bpg_offset);
    // The remaining RC range fields are laid out consecutively in CONFIG16..CONFIG22.
    REG_SET_RC_RANGES!(dsc401, (*reg_vals).pps.rc_range_params);
}

pub unsafe fn dsc401_set_fgcg(dsc401: *mut dcn401_dsc, enable: bool) {
    REG_UPDATE!(dsc401, DSC_TOP_CONTROL, DSC_FGCG_REP_DIS, !enable);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
