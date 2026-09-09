/* SPDX-License-Identifier: MIT */
/* Translation of link_dp_training_dpia.c; external kernel/DC symbols are supplied by dependencies. */

pub const DPIA_DEBUG_EXTENDED_AUX_RD_INTERVAL_US: u32 = 60000000;
pub const TRAINING_AUX_RD_INTERVAL: u32 = 100;

#[repr(C)]
pub enum dpia_set_config_type { DPIA_SET_CFG_SET_LINK = 0x01, DPIA_SET_CFG_SET_PHY_TEST_MODE = 0x05, DPIA_SET_CFG_SET_TRAINING = 0x18, DPIA_SET_CFG_SET_VSPE = 0x19 }
#[repr(C)]
pub enum dpia_set_config_ts { DPIA_TS_DPRX_DONE = 0x00, DPIA_TS_TPS1 = 0x01, DPIA_TS_TPS2 = 0x02, DPIA_TS_TPS3 = 0x03, DPIA_TS_TPS4 = 0x07, DPIA_TS_UFP_DONE = 0xff }

#[repr(C)]
pub union dpia_set_config_data {
    pub set_link: dpia_set_link,
    pub set_training: dpia_set_training,
    pub set_vspe: dpia_set_vspe,
    pub raw: u8,
}
#[repr(C)] pub struct dpia_set_link { pub mode: u8, pub reserved: u8 }
#[repr(C)] pub struct dpia_set_training { pub stage: u8 }
#[repr(C)] pub struct dpia_set_vspe { pub swing: u8, pub max_swing_reached: u8, pub pre_emph: u8, pub max_pre_emph_reached: u8, pub reserved: u8 }

unsafe fn dpia_configure_link(link: *mut dc_link, link_res: *const link_resource, link_setting: *const dc_link_settings, lt_settings: *mut link_training_settings) -> link_training_result {
    let mut status: dc_status;
    let fec_enable: bool;
    dp_decide_training_settings(link, link_res, link_setting, lt_settings);
    dp_get_lttpr_mode_override(link, &mut (*lt_settings).lttpr_mode);
    status = dpcd_configure_channel_coding(link, lt_settings);
    if status != DC_OK && (*link).is_hpd_pending { return LINK_TRAINING_ABORT; }
    status = dpcd_configure_lttpr_mode(link, lt_settings);
    if status != DC_OK && (*link).is_hpd_pending { return LINK_TRAINING_ABORT; }
    status = dpcd_set_link_settings(link, lt_settings);
    if status != DC_OK && (*link).is_hpd_pending { return LINK_TRAINING_ABORT; }
    if link_dp_get_encoding_format(link_setting) == DP_8b_10b_ENCODING {
        fec_enable = match (*link).preferred_training_settings.fec_enable { p if !p.is_null() => *p, _ => true };
        status = dp_set_fec_ready(link, link_res, fec_enable);
    }
    if status != DC_OK && (*link).is_hpd_pending { return LINK_TRAINING_ABORT; }
    LINK_TRAINING_SUCCESS
}

unsafe fn core_link_send_set_config(link: *mut dc_link, msg_type: u8, msg_data: u8) -> dc_status {
    let mut payload = set_config_cmd_payload { msg_type, msg_data };
    let mut result = SET_CONFIG_PENDING;
    if (*link).ddc->ddc_pin.is_null() && !(*link).aux_access_disabled && dm_helpers_dmub_set_config_sync((*link).ctx, link, &mut payload, &mut result) == -1 { return DC_ERROR_UNEXPECTED; }
    if result == SET_CONFIG_ACK_RECEIVED { DC_OK } else { DC_ERROR_UNEXPECTED }
}

unsafe fn dpia_build_set_config_data(typ: dpia_set_config_type, _link: *mut dc_link, lt: *mut link_training_settings) -> u8 {
    let mut data = dpia_set_config_data { raw: 0 };
    match typ {
        dpia_set_config_type::DPIA_SET_CFG_SET_LINK => { (*data.set_link.as_mut()).mode = if (*lt).lttpr_mode == LTTPR_MODE_NON_TRANSPARENT { 1 } else { 0 }; }
        dpia_set_config_type::DPIA_SET_CFG_SET_PHY_TEST_MODE => {}
        dpia_set_config_type::DPIA_SET_CFG_SET_VSPE => {
            let h = (*lt).hw_lane_settings[0];
            (*data.set_vspe.as_mut()).swing = h.VOLTAGE_SWING; (*data.set_vspe.as_mut()).pre_emph = h.PRE_EMPHASIS;
            (*data.set_vspe.as_mut()).max_swing_reached = if h.VOLTAGE_SWING == VOLTAGE_SWING_MAX_LEVEL {1} else {0};
            (*data.set_vspe.as_mut()).max_pre_emph_reached = if h.PRE_EMPHASIS == PRE_EMPHASIS_MAX_LEVEL {1} else {0};
        }
    }
    data.raw
}

unsafe fn convert_trng_ptn_to_trng_stg(tps: dc_dp_training_pattern, ts: *mut dpia_set_config_ts) -> dc_status {
    *ts = match tps { DP_TRAINING_PATTERN_SEQUENCE_1 => dpia_set_config_ts::DPIA_TS_TPS1, DP_TRAINING_PATTERN_SEQUENCE_2 => dpia_set_config_ts::DPIA_TS_TPS2, DP_TRAINING_PATTERN_SEQUENCE_3 => dpia_set_config_ts::DPIA_TS_TPS3, DP_TRAINING_PATTERN_SEQUENCE_4 => dpia_set_config_ts::DPIA_TS_TPS4, DP_TRAINING_PATTERN_VIDEOIDLE => dpia_set_config_ts::DPIA_TS_DPRX_DONE, _ => { return DC_UNSUPPORTED_VALUE; } }; DC_OK
}

unsafe fn dpcd_set_lt_pattern(link: *mut dc_link, pattern: dc_dp_training_pattern, hop: u32) -> dc_status {
    let mut p = dpcd_training_pattern { raw: 0 };
    let off = if hop != DPRX { DP_TRAINING_PATTERN_SET_PHY_REPEATER1 + DP_REPEATER_CONFIGURATION_AND_STATUS_SIZE * (hop - 1) } else { DP_TRAINING_PATTERN_SET };
    p.v1_4.TRAINING_PATTERN_SET = dp_training_pattern_to_dpcd_training_pattern(link, pattern);
    p.v1_4.SCRAMBLING_DISABLE = dp_initialize_scrambling_data_symbols(link, pattern);
    core_link_write_dpcd(link, off, &p.raw, core::mem::size_of::<u8>())
}

unsafe fn dpia_training_cr_non_transparent(link:*mut dc_link, _res:*const link_resource, lt:*mut link_training_settings, hop:u32)->link_training_result {
    let mut result=LINK_TRAINING_CR_FAIL_LANE0; let mut status=DC_ERROR_UNEXPECTED; let mut retries=0; let mut count=0; let mut wait=TRAINING_AUX_RD_INTERVAL; let lane_count=(*lt).link_settings.lane_count; let mut ls=[lane_status{raw:0};LANE_COUNT_DP_MAX]; let mut upd=lane_align_status_updated{raw:0}; let mut adj=[lane_adjust{raw:0};LANE_COUNT_DP_MAX]; let mut ts=dpia_set_config_ts::DPIA_TS_DPRX_DONE; let reps=dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt);
    while retries < LINK_TRAINING_MAX_RETRY_COUNT && count < LINK_TRAINING_MAX_CR_RETRY { if hop==reps { status=core_link_send_set_config(link, dpia_set_config_type::DPIA_SET_CFG_SET_LINK as u8, dpia_build_set_config_data(dpia_set_config_type::DPIA_SET_CFG_SET_LINK,link,lt)); result=if status==DC_OK{LINK_TRAINING_SUCCESS}else{LINK_TRAINING_ABORT}; break; } if count==0 { status=convert_trng_ptn_to_trng_stg((*lt).pattern_for_cr,&mut ts); if status!=DC_OK{result=LINK_TRAINING_ABORT;break;} status=core_link_send_set_config(link,dpia_set_config_type::DPIA_SET_CFG_SET_TRAINING as u8,ts as u8); if status!=DC_OK{result=LINK_TRAINING_ABORT;break;} status=dpcd_set_lt_pattern(link,(*lt).pattern_for_cr,hop); if status!=DC_OK{result=LINK_TRAINING_ABORT;break;} } if hop==reps-1 { status=core_link_send_set_config(link,dpia_set_config_type::DPIA_SET_CFG_SET_VSPE as u8,dpia_build_set_config_data(dpia_set_config_type::DPIA_SET_CFG_SET_VSPE,link,lt)); if status!=DC_OK{result=LINK_TRAINING_ABORT;break;} } status=dpcd_set_lane_settings(link,lt,hop); if status!=DC_OK{result=LINK_TRAINING_ABORT;break;} dp_wait_for_training_aux_rd_interval(link,wait); status=dp_get_lane_status_and_lane_adjust(link,lt,&mut ls,&mut upd,&mut adj,hop); if status!=DC_OK{result=LINK_TRAINING_ABORT;break;} if dp_is_cr_done(lane_count,&ls){result=LINK_TRAINING_SUCCESS;break;} result=dp_get_cr_failure(lane_count,&ls); if dp_is_max_vs_reached(lt){break;} if (*lt).dpcd_lane_settings[0].bits.VOLTAGE_SWING_SET==adj[0].bits.VOLTAGE_SWING_LANE && (*lt).dpcd_lane_settings[0].bits.PRE_EMPHASIS_SET==adj[0].bits.PRE_EMPHASIS_LANE{retries+=1}else{retries=0} dp_decide_lane_settings(lt,&adj,&mut (*lt).hw_lane_settings,&mut (*lt).dpcd_lane_settings); count+=1; } result
}

unsafe fn dpia_training_cr_transparent(link:*mut dc_link,_res:*const link_resource,lt:*mut link_training_settings)->link_training_result { let mut result=LINK_TRAINING_CR_FAIL_LANE0; let mut retries=0; let mut count=0; let wait=(*lt).cr_pattern_time; let lc=(*lt).link_settings.lane_count; let mut ls=[lane_status{raw:0};LANE_COUNT_DP_MAX]; let mut up=lane_align_status_updated{raw:0}; let mut ad=[lane_adjust{raw:0};LANE_COUNT_DP_MAX]; while retries<LINK_TRAINING_MAX_RETRY_COUNT&&count<LINK_TRAINING_MAX_CR_RETRY { if count==0&&dpcd_set_lt_pattern(link,(*lt).pattern_for_cr,DPRX)!=DC_OK{break} dp_wait_for_training_aux_rd_interval(link,wait); if dp_get_lane_status_and_lane_adjust(link,lt,&mut ls,&mut up,&mut ad,DPRX)!=DC_OK{result=LINK_TRAINING_ABORT;break} if dp_is_cr_done(lc,&ls){result=LINK_TRAINING_SUCCESS;break} result=dp_get_cr_failure(lc,&ls); if dp_is_max_vs_reached(lt){break} if (*lt).dpcd_lane_settings[0].bits.VOLTAGE_SWING_SET==ad[0].bits.VOLTAGE_SWING_LANE&&(*lt).dpcd_lane_settings[0].bits.PRE_EMPHASIS_SET==ad[0].bits.PRE_EMPHASIS_LANE{retries+=1}else{retries=0} dp_decide_lane_settings(lt,&ad,&mut (*lt).hw_lane_settings,&mut (*lt).dpcd_lane_settings);count+=1;} result }

unsafe fn dpia_training_cr_phase(l:*mut dc_link,r:*const link_resource,s:*mut link_training_settings,h:u32)->link_training_result { if (*s).lttpr_mode==LTTPR_MODE_NON_TRANSPARENT{dpia_training_cr_non_transparent(l,r,s,h)}else{dpia_training_cr_transparent(l,r,s)} }

unsafe fn dpia_training_eq_non_transparent(link:*mut dc_link,_r:*const link_resource,lt:*mut link_training_settings,hop:u32)->link_training_result { let mut result=LINK_TRAINING_EQ_FAIL_EQ; let reps=dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt); let pat=if hop==DPRX{(*lt).pattern_for_eq}else{DP_TRAINING_PATTERN_SEQUENCE_4}; let lc=(*lt).link_settings.lane_count; let mut up=lane_align_status_updated{raw:0}; let mut ls=[lane_status{raw:0};LANE_COUNT_DP_MAX]; let mut ad=[lane_adjust{raw:0};LANE_COUNT_DP_MAX]; let mut ts=dpia_set_config_ts::DPIA_TS_DPRX_DONE; for n in 0..LINK_TRAINING_MAX_RETRY_COUNT { if hop==reps{return LINK_TRAINING_SUCCESS} if n==0 {if convert_trng_ptn_to_trng_stg(pat,&mut ts)!=DC_OK{return LINK_TRAINING_ABORT} if core_link_send_set_config(link,dpia_set_config_type::DPIA_SET_CFG_SET_TRAINING as u8,ts as u8)!=DC_OK{return LINK_TRAINING_ABORT} if dpcd_set_lt_pattern(link,pat,hop)!=DC_OK{return LINK_TRAINING_ABORT}} if hop==reps-1&&core_link_send_set_config(link,dpia_set_config_type::DPIA_SET_CFG_SET_VSPE as u8,dpia_build_set_config_data(dpia_set_config_type::DPIA_SET_CFG_SET_VSPE,link,lt))!=DC_OK{return LINK_TRAINING_ABORT} if dpcd_set_lane_settings(link,lt,hop)!=DC_OK{return LINK_TRAINING_ABORT} let wait=if hop==DPRX&&n==1{DPIA_CLK_SYNC_DELAY.max(dpia_get_eq_aux_rd_interval(link,lt,hop))}else{dpia_get_eq_aux_rd_interval(link,lt,hop)};dp_wait_for_training_aux_rd_interval(link,wait);if dp_get_lane_status_and_lane_adjust(link,lt,&mut ls,&mut up,&mut ad,hop)!=DC_OK{return LINK_TRAINING_ABORT}if !dp_is_cr_done(lc,&ls){return LINK_TRAINING_EQ_FAIL_CR}if dp_is_ch_eq_done(lc,&ls)&&dp_is_symbol_locked((*link).cur_link_settings.lane_count,&ls)&&dp_is_interlane_aligned(up){return LINK_TRAINING_SUCCESS}dp_decide_lane_settings(lt,&ad,&mut (*lt).hw_lane_settings,&mut (*lt).dpcd_lane_settings); result=LINK_TRAINING_EQ_FAIL_EQ} result }

unsafe fn dpia_training_eq_transparent(link:*mut dc_link,_r:*const link_resource,lt:*mut link_training_settings)->link_training_result { let mut result=LINK_TRAINING_EQ_FAIL_EQ; let pat=(*lt).pattern_for_eq;let lc=(*lt).link_settings.lane_count;let wait=dpia_get_eq_aux_rd_interval(link,lt,DPRX);let mut up=lane_align_status_updated{raw:0};let mut ls=[lane_status{raw:0};LANE_COUNT_DP_MAX];let mut ad=[lane_adjust{raw:0};LANE_COUNT_DP_MAX];for n in 0..LINK_TRAINING_MAX_RETRY_COUNT{if n==0&&dpcd_set_lt_pattern(link,pat,DPRX)!=DC_OK{return LINK_TRAINING_ABORT}dp_wait_for_training_aux_rd_interval(link,wait);if dp_get_lane_status_and_lane_adjust(link,lt,&mut ls,&mut up,&mut ad,DPRX)!=DC_OK{return LINK_TRAINING_ABORT}if !dp_is_cr_done(lc,&ls){return LINK_TRAINING_EQ_FAIL_CR}if dp_is_ch_eq_done(lc,&ls)&&dp_is_symbol_locked((*link).cur_link_settings.lane_count,&ls)&&(dp_is_interlane_aligned(up)||((*link).skip_fallback_on_link_loss&&n!=0)){return LINK_TRAINING_SUCCESS}dp_decide_lane_settings(lt,&ad,&mut (*lt).hw_lane_settings,&mut (*lt).dpcd_lane_settings);result=LINK_TRAINING_EQ_FAIL_EQ}result}
unsafe fn dpia_training_eq_phase(l:*mut dc_link,r:*const link_resource,s:*mut link_training_settings,h:u32)->link_training_result{if (*s).lttpr_mode==LTTPR_MODE_NON_TRANSPARENT{dpia_training_eq_non_transparent(l,r,s,h)}else{dpia_training_eq_transparent(l,r,s)}}

unsafe fn dpcd_clear_lt_pattern(link:*mut dc_link,hop:u32)->dc_status{let p=dpcd_training_pattern{raw:0};let off=if hop!=DPRX{DP_TRAINING_PATTERN_SET_PHY_REPEATER1+DP_REPEATER_CONFIGURATION_AND_STATUS_SIZE*(hop-1)}else{DP_TRAINING_PATTERN_SET};core_link_write_dpcd(link,off,&p.raw,core::mem::size_of::<u8>())}
unsafe fn dpia_training_end(link:*mut dc_link,lt:*mut link_training_settings,hop:u32)->link_training_result{let mut result=LINK_TRAINING_SUCCESS;if (*lt).lttpr_mode==LTTPR_MODE_NON_TRANSPARENT{let reps=dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt);if hop==reps{if core_link_send_set_config(link,dpia_set_config_type::DPIA_SET_CFG_SET_TRAINING as u8,dpia_set_config_ts::DPIA_TS_UFP_DONE as u8)!=DC_OK{result=LINK_TRAINING_ABORT}}else if dpcd_clear_lt_pattern(link,hop)!=DC_OK{result=LINK_TRAINING_ABORT}if hop==DPRX&&result!=LINK_TRAINING_ABORT&&core_link_send_set_config(link,dpia_set_config_type::DPIA_SET_CFG_SET_TRAINING as u8,dpia_set_config_ts::DPIA_TS_DPRX_DONE as u8)!=DC_OK{result=LINK_TRAINING_ABORT}}else if dpcd_clear_lt_pattern(link,hop)!=DC_OK{result=LINK_TRAINING_ABORT}result}
pub unsafe fn dpia_get_eq_aux_rd_interval(link:*const dc_link,lt:*const link_training_settings,hop:u32)->u32{if (*(*link).dc).debug.dpia_debug.bits.extend_aux_rd_interval{DPIA_DEBUG_EXTENDED_AUX_RD_INTERVAL_US}else if hop==DPRX{(*lt).eq_pattern_time}else{dp_translate_training_aux_read_interval((*link).dpcd_caps.lttpr_caps.aux_rd_interval[(hop-1) as usize])}}
pub unsafe fn dpia_training_abort(link:*mut dc_link,lt:*mut link_training_settings,hop:u32){if (*link).is_hpd_pending{return}let mut d=0u8;let off=if hop!=DPRX{DP_TRAINING_PATTERN_SET_PHY_REPEATER1+DP_REPEATER_CONFIGURATION_AND_STATUS_SIZE*(hop-1)}else{DP_TRAINING_PATTERN_SET};core_link_write_dpcd(link,off,&d,1);core_link_write_dpcd(link,DP_LINK_BW_SET,&d,1);core_link_write_dpcd(link,DP_LANE_COUNT_SET,&d,1);if !(*(*link).dc).config.consolidated_dpia_dp_lt{core_link_send_set_config(link,dpia_set_config_type::DPIA_SET_CFG_SET_LINK as u8,d);}}
pub unsafe fn dpia_set_tps_notification(link:*mut dc_link,lt:*const link_training_settings,pattern:u8,hop:u32){if (*lt).lttpr_mode!=LTTPR_MODE_NON_TRANSPARENT||pattern==DPCD_TRAINING_PATTERN_VIDEOIDLE{return}let reps=dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt);if hop!=reps{dc_process_dmub_dpia_set_tps_notification((*link).ctx->dc,(*link).link_index,pattern)}}
pub unsafe fn dpia_perform_link_training(link:*mut dc_link,res:*const link_resource,setting:*const dc_link_settings,_skip:bool)->link_training_result{let mut lt=core::mem::zeroed::<link_training_settings>();let mut ls=*setting;lt.lttpr_mode=dp_decide_lttpr_mode(link,&mut ls);let mut result=dpia_configure_link(link,res,setting,&mut lt);if result!=LINK_TRAINING_SUCCESS{return result}let reps=if lt.lttpr_mode==LTTPR_MODE_NON_TRANSPARENT{dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt)}else{0};let mut hop=reps as i32;while hop>=0{result=dpia_training_cr_phase(link,res,&mut lt,hop as u32);if result!=LINK_TRAINING_SUCCESS{break}result=dpia_training_eq_phase(link,res,&mut lt,hop as u32);if result!=LINK_TRAINING_SUCCESS{break}result=dpia_training_end(link,&mut lt,hop as u32);if result!=LINK_TRAINING_SUCCESS{break}hop-=1}if result==LINK_TRAINING_SUCCESS{fsleep(5000);if !(*link).skip_fallback_on_link_loss{result=dp_check_link_loss_status(link,&mut lt)}}else if result==LINK_TRAINING_ABORT{dpia_training_abort(link,&mut lt,hop as u32)}else{dpia_training_end(link,&mut lt,hop as u32)}result}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
