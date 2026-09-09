/* Translated from dcn31_hpo_dp_link_encoder.c. */

const DP_SAT_UPDATE_MAX_RETRY: u32 = 200;

pub unsafe fn dcn31_hpo_dp_link_enc_enable(enc: *mut hpo_dp_link_encoder, num_lanes: dc_lane_count) {
    let enc3 = DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc);
    let mut dp_link_enabled: u32 = 0;
    REG_GET!(enc3, DP_DPHY_SYM32_STATUS, STATUS, &mut dp_link_enabled);
    REG_UPDATE!(enc3, DP_LINK_ENC_CLOCK_CONTROL, DP_LINK_ENC_CLOCK_EN, 1);
    if dp_link_enabled == 0 {
        REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, DPHY_RESET, 1);
        REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, DPHY_RESET, 0);
    }
    REG_UPDATE_3!(enc3, DP_DPHY_SYM32_CONTROL, DPHY_ENABLE, 1, PRECODER_ENABLE, 1,
        NUM_LANES, if num_lanes == LANE_COUNT_ONE { 0 } else if num_lanes == LANE_COUNT_TWO { 1 } else { 3 });
}

pub unsafe fn dcn31_hpo_dp_link_enc_disable(enc: *mut hpo_dp_link_encoder) {
    let enc3 = DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc);
    REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, DPHY_ENABLE, 0);
    REG_UPDATE!(enc3, DP_LINK_ENC_CLOCK_CONTROL, DP_LINK_ENC_CLOCK_EN, 0);
}

pub unsafe fn dcn31_hpo_dp_link_enc_set_link_test_pattern(enc: *mut hpo_dp_link_encoder, tp_params: *mut encoder_set_dp_phy_pattern_param) {
    let enc3 = DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc);
    let mut tp_custom: u32;
    match (*tp_params).dp_phy_pattern {
        DP_TEST_PATTERN_VIDEO_MODE => REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, MODE, DP2_LINK_ACTIVE),
        DP_TEST_PATTERN_128b_132b_TPS1_TRAINING_MODE => REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, MODE, DP2_LINK_TRAINING_TPS1),
        DP_TEST_PATTERN_128b_132b_TPS2_TRAINING_MODE => REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, MODE, DP2_LINK_TRAINING_TPS2),
        DP_TEST_PATTERN_128b_132b_TPS1 | DP_TEST_PATTERN_128b_132b_TPS2 |
        DP_TEST_PATTERN_PRBS7 | DP_TEST_PATTERN_PRBS9 | DP_TEST_PATTERN_PRBS11 |
        DP_TEST_PATTERN_PRBS15 | DP_TEST_PATTERN_PRBS23 | DP_TEST_PATTERN_PRBS31 => {
            let sel = match (*tp_params).dp_phy_pattern {
                DP_TEST_PATTERN_128b_132b_TPS1 => DP_DPHY_TP_SELECT_TPS1,
                DP_TEST_PATTERN_128b_132b_TPS2 => DP_DPHY_TP_SELECT_TPS2,
                _ => DP_DPHY_TP_SELECT_PRBS,
            };
            if (*tp_params).dp_phy_pattern != DP_TEST_PATTERN_128b_132b_TPS1 && (*tp_params).dp_phy_pattern != DP_TEST_PATTERN_128b_132b_TPS2 {
                let prbs = match (*tp_params).dp_phy_pattern { DP_TEST_PATTERN_PRBS7=>DP_DPHY_TP_PRBS7, DP_TEST_PATTERN_PRBS9=>DP_DPHY_TP_PRBS9, DP_TEST_PATTERN_PRBS11=>DP_DPHY_TP_PRBS11, DP_TEST_PATTERN_PRBS15=>DP_DPHY_TP_PRBS15, DP_TEST_PATTERN_PRBS23=>DP_DPHY_TP_PRBS23, _=>DP_DPHY_TP_PRBS31 };
                REG_UPDATE_4!(enc3, DP_DPHY_SYM32_TP_CONFIG, TP_PRBS_SEL0, prbs, TP_PRBS_SEL1, prbs, TP_PRBS_SEL2, prbs, TP_PRBS_SEL3, prbs);
            }
            REG_UPDATE_4!(enc3, DP_DPHY_SYM32_TP_CONFIG, TP_SELECT0, sel, TP_SELECT1, sel, TP_SELECT2, sel, TP_SELECT3, sel);
            REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, MODE, DP2_TEST_PATTERN);
        }
        DP_TEST_PATTERN_264BIT_CUSTOM => {
            let p = &(*tp_params).custom_pattern;
            for i in 0..11 { tp_custom = ((p[i*3+2] as u32)<<16) | ((p[i*3+1] as u32)<<8) | p[i*3] as u32; REG_SET!(enc3, DP_DPHY_SYM32_TP_CUSTOM0 + i, 0, TP_CUSTOM, tp_custom); }
            REG_UPDATE_4!(enc3, DP_DPHY_SYM32_TP_CONFIG, TP_SELECT0, DP_DPHY_TP_SELECT_CUSTOM, TP_SELECT1, DP_DPHY_TP_SELECT_CUSTOM, TP_SELECT2, DP_DPHY_TP_SELECT_CUSTOM, TP_SELECT3, DP_DPHY_TP_SELECT_CUSTOM);
            REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, MODE, DP2_TEST_PATTERN);
        }
        DP_TEST_PATTERN_SQUARE | DP_TEST_PATTERN_SQUARE_PRESHOOT_DISABLED | DP_TEST_PATTERN_SQUARE_DEEMPHASIS_DISABLED | DP_TEST_PATTERN_SQUARE_PRESHOOT_DEEMPHASIS_DISABLED => {
            REG_SET!(enc3, DP_DPHY_SYM32_TP_SQ_PULSE, 0, TP_SQ_PULSE_WIDTH, (*tp_params).custom_pattern[0]);
            REG_UPDATE_4!(enc3, DP_DPHY_SYM32_TP_CONFIG, TP_SELECT0, DP_DPHY_TP_SELECT_SQUARE, TP_SELECT1, DP_DPHY_TP_SELECT_SQUARE, TP_SELECT2, DP_DPHY_TP_SELECT_SQUARE, TP_SELECT3, DP_DPHY_TP_SELECT_SQUARE);
            REG_UPDATE!(enc3, DP_DPHY_SYM32_CONTROL, MODE, DP2_TEST_PATTERN);
        }
        _ => {}
    }
}

pub unsafe fn dcn31_fill_stream_allocation_row_info(a: *const link_mst_stream_allocation, src: *mut u32, slots: *mut u32) {
    let stream_enc = (*a).hpo_dp_stream_enc;
    if !stream_enc.is_null() && (*stream_enc).id >= ENGINE_ID_HPO_DP_0 { *src = (*stream_enc).id - ENGINE_ID_HPO_DP_0; *slots = (*a).slot_count; } else { *src=0; *slots=0; }
}

pub unsafe fn dcn31_hpo_dp_link_enc_update_stream_allocation_table(enc: *mut hpo_dp_link_encoder, table: *const link_mst_stream_allocation_table) {
    let enc3 = DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc); let mut slots=0; let mut src=0;
    for i in 0..4 { if (*table).stream_count >= i+1 { dcn31_fill_stream_allocation_row_info(&(*table).stream_allocations[i], &mut src, &mut slots); } else {src=0; slots=0;} REG_UPDATE_2!(enc3, DP_DPHY_SYM32_SAT_VC0+i, SAT_STREAM_SOURCE, src, SAT_SLOT_COUNT, slots); }
    REG_UPDATE!(enc3, DP_DPHY_SYM32_SAT_UPDATE, SAT_UPDATE, 1);
    REG_WAIT!(enc3, DP_DPHY_SYM32_STATUS, SAT_UPDATE_PENDING, 0, 100, DP_SAT_UPDATE_MAX_RETRY);
}

pub unsafe fn dcn31_hpo_dp_link_enc_set_throttled_vcp_size(enc: *mut hpo_dp_link_encoder, stream_encoder_inst: u32, avg_time_slots_per_mtp: fixed31_32) {
    let enc3=DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc); let mut x=dc_fixpt_floor(avg_time_slots_per_mtp); let mut y=dc_fixpt_ceil(dc_fixpt_shl(dc_fixpt_sub_int(avg_time_slots_per_mtp,x),25));
    if y >> 25 != 0 { x+=1; y=0; }
    match stream_encoder_inst { 0=>REG_SET_2!(enc3,DP_DPHY_SYM32_VC_RATE_CNTL0,0,STREAM_VC_RATE_X,x,STREAM_VC_RATE_Y,y), 1=>REG_SET_2!(enc3,DP_DPHY_SYM32_VC_RATE_CNTL1,0,STREAM_VC_RATE_X,x,STREAM_VC_RATE_Y,y), 2=>REG_SET_2!(enc3,DP_DPHY_SYM32_VC_RATE_CNTL2,0,STREAM_VC_RATE_X,x,STREAM_VC_RATE_Y,y), 3=>REG_SET_2!(enc3,DP_DPHY_SYM32_VC_RATE_CNTL3,0,STREAM_VC_RATE_X,x,STREAM_VC_RATE_Y,y), _=>ASSERT!(0) }
    REG_WAIT!(enc3,DP_DPHY_SYM32_STATUS,RATE_UPDATE_PENDING,0,1,10);
}

static unsafe fn dcn31_hpo_dp_link_enc_is_in_alt_mode(enc: *mut hpo_dp_link_encoder) -> bool { let enc3=DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc); let mut v=0; ASSERT!((*enc).transmitter>=TRANSMITTER_UNIPHY_A && (*enc).transmitter<=TRANSMITTER_UNIPHY_E); REG_GET!(enc3,RDPCSTX_PHY_CNTL6[(*enc).transmitter],RDPCS_PHY_DPALT_DISABLE,&mut v); v==0 }

pub unsafe fn dcn31_hpo_dp_link_enc_read_state(enc: *mut hpo_dp_link_encoder, state: *mut hpo_dp_link_enc_state) { let enc3=DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc); ASSERT!(!state.is_null()); REG_GET!(enc3,DP_DPHY_SYM32_STATUS,STATUS,&mut (*state).link_enc_enabled); REG_GET!(enc3,DP_DPHY_SYM32_CONTROL,NUM_LANES,&mut (*state).lane_count); REG_GET!(enc3,DP_DPHY_SYM32_CONTROL,MODE,&mut (*state).link_mode as *mut _ as *mut u32); for i in 0..4 { REG_GET_2!(enc3,DP_DPHY_SYM32_SAT_VC0+i,SAT_STREAM_SOURCE,&mut (*state).stream_src[i],SAT_SLOT_COUNT,&mut (*state).slot_count[i]); REG_GET_2!(enc3,DP_DPHY_SYM32_VC_RATE_CNTL0+i,STREAM_VC_RATE_X,&mut (*state).vc_rate_x[i],STREAM_VC_RATE_Y,&mut (*state).vc_rate_y[i]); } }

static unsafe fn link_transmitter_control(enc3: *mut dcn31_hpo_dp_link_encoder, cntl: *mut bp_transmitter_control) -> bp_result { (*(*(*enc3).base.ctx).dc_bios).funcs.transmitter_control((*(*enc3).base.ctx).dc_bios, cntl) }

pub unsafe fn dcn31_hpo_dp_link_enc_enable_dp_output(enc:*mut hpo_dp_link_encoder, ls:*const dc_link_settings, transmitter: transmitter, hpd_source: hpd_source_id) { let enc3=DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc); let mut c=core::mem::zeroed::<bp_transmitter_control>(); (*enc3).base.transmitter=transmitter; (*enc3).base.hpd_source=hpd_source; c.action=TRANSMITTER_CONTROL_ENABLE; c.engine_id=ENGINE_ID_UNKNOWN; c.transmitter=transmitter; c.signal=SIGNAL_TYPE_DISPLAY_PORT_MST; c.lanes_number=(*ls).lane_count; c.hpd_sel=hpd_source; c.pixel_clock=(*ls).link_rate*1000; c.color_depth=COLOR_DEPTH_UNDEFINED; c.hpo_engine_id=(*enc).inst+ENGINE_ID_HPO_DP_0; if link_transmitter_control(enc3,&mut c)!=BP_RESULT_OK { DC_LOG_ERROR!("%s: Failed to execute VBIOS command table!\n", __func__); BREAK_TO_DEBUGGER!(); } }

pub unsafe fn dcn31_hpo_dp_link_enc_disable_output(enc:*mut hpo_dp_link_encoder, signal: signal_type) { let enc3=DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc); let mut c=core::mem::zeroed::<bp_transmitter_control>(); c.action=TRANSMITTER_CONTROL_DISABLE; c.transmitter=(*enc3).base.transmitter; c.hpd_sel=(*enc3).base.hpd_source; c.signal=signal; if link_transmitter_control(enc3,&mut c)!=BP_RESULT_OK { DC_LOG_ERROR!("%s: Failed to execute VBIOS command table!\n",__func__); BREAK_TO_DEBUGGER!(); return; } dcn31_hpo_dp_link_enc_disable(enc); }

pub unsafe fn dcn31_hpo_dp_link_enc_set_ffe(enc:*mut hpo_dp_link_encoder, ls:*const dc_link_settings, ffe_preset:u8) { let enc3=DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc); let mut c=core::mem::zeroed::<bp_transmitter_control>(); c.transmitter=(*enc3).base.transmitter; c.action=TRANSMITTER_CONTROL_SET_VOLTAGE_AND_PREEMPASIS; c.signal=SIGNAL_TYPE_DISPLAY_PORT_MST; c.lanes_number=(*ls).lane_count; c.pixel_clock=(*ls).link_rate*1000; c.lane_settings=ffe_preset; if link_transmitter_control(enc3,&mut c)!=BP_RESULT_OK { DC_LOG_ERROR!("%s: Failed to execute VBIOS command table!\n",__func__); BREAK_TO_DEBUGGER!(); } }

static mut dcn31_hpo_dp_link_encoder_funcs: hpo_dp_link_encoder_funcs = hpo_dp_link_encoder_funcs { enable_link_phy:dcn31_hpo_dp_link_enc_enable_dp_output, disable_link_phy:dcn31_hpo_dp_link_enc_disable_output, link_enable:dcn31_hpo_dp_link_enc_enable, link_disable:dcn31_hpo_dp_link_enc_disable, set_link_test_pattern:dcn31_hpo_dp_link_enc_set_link_test_pattern, update_stream_allocation_table:dcn31_hpo_dp_link_enc_update_stream_allocation_table, set_throttled_vcp_size:dcn31_hpo_dp_link_enc_set_throttled_vcp_size, is_in_alt_mode:dcn31_hpo_dp_link_enc_is_in_alt_mode, read_state:dcn31_hpo_dp_link_enc_read_state, set_ffe:dcn31_hpo_dp_link_enc_set_ffe };

pub unsafe fn hpo_dp_link_encoder31_construct(enc31:*mut dcn31_hpo_dp_link_encoder, ctx:*mut dc_context, inst:u32, regs:*const dcn31_hpo_dp_link_encoder_registers, shift:*const dcn31_hpo_dp_link_encoder_shift, mask:*const dcn31_hpo_dp_link_encoder_mask) { (*enc31).base.ctx=ctx; (*enc31).base.inst=inst; (*enc31).base.funcs=&raw const dcn31_hpo_dp_link_encoder_funcs; (*enc31).base.hpd_source=HPD_SOURCEID_UNKNOWN; (*enc31).base.transmitter=TRANSMITTER_UNKNOWN; (*enc31).regs=regs; (*enc31).hpo_le_shift=shift; (*enc31).hpo_le_mask=mask; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
