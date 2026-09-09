/* Rust translation of link_dp_training_fixed_vs_pe_retimer.c. */

pub unsafe fn dp_fixed_vs_pe_read_lane_adjust(
    link: *mut dc_link,
    dpcd_lane_adjust: *mut dpcd_training_lane,
) {
    let vendor_lttpr_write_data_vs: [u8; 3] = [0x0, 0x53, 0x63];
    let vendor_lttpr_write_data_pe: [u8; 3] = [0x0, 0x54, 0x63];
    let mut dprx_vs: u8 = 0;
    let mut dprx_pe: u8 = 0;

    link_configure_fixed_vs_pe_retimer((*link).ddc, vendor_lttpr_write_data_vs.as_ptr(), vendor_lttpr_write_data_vs.len());
    link_query_fixed_vs_pe_retimer((*link).ddc, &mut dprx_vs, 1);
    link_configure_fixed_vs_pe_retimer((*link).ddc, vendor_lttpr_write_data_pe.as_ptr(), vendor_lttpr_write_data_pe.len());
    link_query_fixed_vs_pe_retimer((*link).ddc, &mut dprx_pe, 1);

    for lane in 0..LANE_COUNT_DP_MAX {
        (*dpcd_lane_adjust.add(lane)).bits.VOLTAGE_SWING_SET = (dprx_vs >> (2 * lane)) & 0x3;
        (*dpcd_lane_adjust.add(lane)).bits.PRE_EMPHASIS_SET = (dprx_pe >> (2 * lane)) & 0x3;
    }
}

pub unsafe fn dp_fixed_vs_pe_set_retimer_lane_settings(
    link: *mut dc_link,
    dpcd_lane_adjust: *const dpcd_training_lane,
    lane_count: u8,
) {
    let vendor_lttpr_write_data_reset = [0x1u8, 0x50, 0x63, 0xFF];
    let mut vendor_lttpr_write_data_vs = [0x1u8, 0x51, 0x63, 0x0];
    let mut vendor_lttpr_write_data_pe = [0x1u8, 0x52, 0x63, 0x0];
    for lane in 0..lane_count as usize {
        vendor_lttpr_write_data_vs[3] |= (*dpcd_lane_adjust.add(lane)).bits.VOLTAGE_SWING_SET << (2 * lane);
        vendor_lttpr_write_data_pe[3] |= (*dpcd_lane_adjust.add(lane)).bits.PRE_EMPHASIS_SET << (2 * lane);
    }
    link_configure_fixed_vs_pe_retimer((*link).ddc, vendor_lttpr_write_data_reset.as_ptr(), vendor_lttpr_write_data_reset.len());
    link_configure_fixed_vs_pe_retimer((*link).ddc, vendor_lttpr_write_data_vs.as_ptr(), vendor_lttpr_write_data_vs.len());
    link_configure_fixed_vs_pe_retimer((*link).ddc, vendor_lttpr_write_data_pe.as_ptr(), vendor_lttpr_write_data_pe.len());
}

unsafe fn perform_fixed_vs_pe_nontransparent_training_sequence(
    link: *mut dc_link, link_res: *const link_resource, lt_settings: *mut link_training_settings,
) -> link_training_result {
    let mut status = LINK_TRAINING_SUCCESS;
    let mut toggle_rate: u8 = 0x6;
    let mut target_rate: u8 = 0x6;
    let apply_toggle_rate_wa = (*link).vendor_specific_lttpr_link_rate_wa == target_rate || (*link).vendor_specific_lttpr_link_rate_wa == 0;
    if (*lt_settings).cr_pattern_time < 16000 { (*lt_settings).cr_pattern_time = 16000; }
    target_rate = get_dpcd_link_rate(&(*lt_settings).link_settings);
    toggle_rate = if target_rate == 0x6 { 0xA } else { 0x6 };
    if apply_toggle_rate_wa { (*lt_settings).link_settings.link_rate = toggle_rate; }
    if (*link).ctx.dc.work_arounds.lt_early_cr_pattern { start_clock_recovery_pattern_early(link, link_res, lt_settings, DPRX); }
    dpcd_set_link_settings(link, lt_settings);
    if apply_toggle_rate_wa { core_link_write_dpcd(link, DP_LINK_BW_SET, &target_rate, 1); }
    (*link).vendor_specific_lttpr_link_rate_wa = target_rate;
    if (*lt_settings).lttpr_mode == LTTPR_MODE_NON_TRANSPARENT {
        let repeater_cnt = dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt);
        let mut repeater_id = repeater_cnt;
        while repeater_id > 0 && status == LINK_TRAINING_SUCCESS {
            status = perform_8b_10b_clock_recovery_sequence(link, link_res, lt_settings, repeater_id);
            if status != LINK_TRAINING_SUCCESS { repeater_training_done(link, repeater_id); break; }
            status = perform_8b_10b_channel_equalization_sequence(link, link_res, lt_settings, repeater_id);
            repeater_training_done(link, repeater_id);
            if status != LINK_TRAINING_SUCCESS { break; }
            for lane in 0..LANE_COUNT_DP_MAX { (*lt_settings).dpcd_lane_settings[lane].raw = 0; (*lt_settings).hw_lane_settings[lane].VOLTAGE_SWING = 0; (*lt_settings).hw_lane_settings[lane].PRE_EMPHASIS = 0; }
            repeater_id -= 1;
        }
    }
    if status == LINK_TRAINING_SUCCESS {
        status = perform_8b_10b_clock_recovery_sequence(link, link_res, lt_settings, DPRX);
        if status == LINK_TRAINING_SUCCESS { status = perform_8b_10b_channel_equalization_sequence(link, link_res, lt_settings, DPRX); }
    }
    status
}

pub unsafe fn dp_perform_fixed_vs_pe_training_sequence(link: *mut dc_link, link_res: *const link_resource, lt_settings: *mut link_training_settings) -> link_training_result {
    let reset = [0x1u8, 0x50, 0x63, 0xFF];
    let offset = dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt);
    let intercept_en = [0x1u8, 0x55, 0x63, 0x0]; let intercept_dis = [0x1u8, 0x55, 0x63, 0x6E];
    let adicora_eq1 = [0x1u8, 0x55, 0x63, 0x2E]; let adicora_eq2 = [0x1u8, 0x55, 0x63, 0x01]; let adicora_eq3 = [0x1u8, 0x55, 0x63, 0x68];
    let mut pre_disable_intercept_delay_ms = 0u32;
    let mut vendor_vs = [0x1u8, 0x51, 0x63, 0x0]; let mut vendor_pe = [0x1u8, 0x52, 0x63, 0x0];
    let lane4_1 = [1,0x6E,0xF2,0x19]; let lane4_2 = [1,0x6B,0xF2,1]; let lane4_3 = [1,0x6D,0xF2,0x18]; let lane4_4 = [1,0x6C,0xF2,3]; let lane4_5 = [1,3,0xF3,6]; let dpmf = [1,6,0x70,0x87];
    let mut status = LINK_TRAINING_SUCCESS;
    ASSERT(link_dp_get_encoding_format(&(*lt_settings).link_settings) == DP_8b_10b_ENCODING);
    if (*lt_settings).lttpr_mode == LTTPR_MODE_NON_TRANSPARENT { return perform_fixed_vs_pe_nontransparent_training_sequence(link, link_res, lt_settings); }
    if offset != 0xFF { if offset == 2 { pre_disable_intercept_delay_ms = (*link).dc.debug.fixed_vs_aux_delay_config_wa; } else if offset > 2 { pre_disable_intercept_delay_ms = (*link).dc.debug.fixed_vs_aux_delay_config_wa * 2; } }
    link_configure_fixed_vs_pe_retimer((*link).ddc, reset.as_ptr(), 4); link_configure_fixed_vs_pe_retimer((*link).ddc, vendor_vs.as_ptr(), 4); link_configure_fixed_vs_pe_retimer((*link).ddc, vendor_pe.as_ptr(), 4); link_configure_fixed_vs_pe_retimer((*link).ddc, intercept_en.as_ptr(), 4);
    let mut downspread = down_spread_ctrl::default(); downspread.raw = (*lt_settings).link_settings.link_spread as u8;
    let mut lane_count_set = lane_count_set::default(); lane_count_set.bits.LANE_COUNT_SET = (*lt_settings).link_settings.lane_count; lane_count_set.bits.ENHANCED_FRAMING = (*lt_settings).enhanced_framing; lane_count_set.bits.POST_LT_ADJ_REQ_GRANTED = if (*lt_settings).pattern_for_eq < DP_TRAINING_PATTERN_SEQUENCE_4 { (*link).dpcd_caps.max_ln_count.bits.POST_LT_ADJ_REQ_SUPPORTED } else { 0 };
    core_link_write_dpcd(link, DP_DOWNSPREAD_CTRL, &downspread.raw, 1); core_link_write_dpcd(link, DP_LANE_COUNT_SET, &lane_count_set.raw, 1);
    let rate = get_dpcd_link_rate(&(*lt_settings).link_settings); let toggle_rate = if rate == 0x6 { 0xA } else { 0x6 };
    if (*link).dpcd_caps.lttpr_caps.lttpr_ieee_oui == [0,0,0] { if (*link).vendor_specific_lttpr_link_rate_wa == rate || (*link).vendor_specific_lttpr_link_rate_wa == 0 { core_link_write_dpcd(link, DP_LINK_BW_SET, &toggle_rate, 1); } (*link).vendor_specific_lttpr_link_rate_wa = rate; }
    core_link_write_dpcd(link, DP_LINK_BW_SET, &rate, 1); link_configure_fixed_vs_pe_retimer((*link).ddc, dpmf.as_ptr(), 4);
    if (*lt_settings).link_settings.lane_count == LANE_COUNT_FOUR { for data in [&lane4_1, &lane4_2, &lane4_3, &lane4_4, &lane4_5] { link_configure_fixed_vs_pe_retimer((*link).ddc, data.as_ptr(), 4); } }
    if status == LINK_TRAINING_SUCCESS { status = fixed_vs_pe_clock_recovery(link, link_res, lt_settings, &mut vendor_vs, &mut vendor_pe, &intercept_dis, &intercept_en, pre_disable_intercept_delay_ms); }
    if status == LINK_TRAINING_SUCCESS { status = fixed_vs_pe_channel_equalization(link, link_res, lt_settings, &mut vendor_vs, &mut vendor_pe, &adicora_eq1, &adicora_eq2, &adicora_eq3); }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
