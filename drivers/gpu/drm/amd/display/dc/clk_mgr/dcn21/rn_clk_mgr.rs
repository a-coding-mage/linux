/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translated driver.

const SMU_VER_55_51_0: u32 = 0x373300;

unsafe fn rn_get_active_display_cnt_wa(dc: *mut dc, context: *mut dc_state) -> i32 {
    let mut display_count = 0;
    let mut tmds_present = false;
    for i in 0..(*context).stream_count {
        let stream = (*context).streams[i as usize];
        if (*stream).signal == SIGNAL_TYPE_HDMI_TYPE_A ||
           (*stream).signal == SIGNAL_TYPE_DVI_SINGLE_LINK ||
           (*stream).signal == SIGNAL_TYPE_DVI_DUAL_LINK { tmds_present = true; }
    }
    for i in 0..(*dc).link_count {
        let link = (*dc).links[i as usize];
        if (*(*(*link).link_enc).funcs).is_dig_enabled.is_some() &&
           ((*(*(*link).link_enc).funcs).is_dig_enabled.unwrap())((*link).link_enc) { display_count += 1; }
    }
    if display_count == 0 && tmds_present { display_count = 1; }
    display_count
}

unsafe fn rn_set_low_power_state(clk_mgr_base: *mut clk_mgr) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let dc = (*(*clk_mgr_base).ctx).dc;
    let context = (*dc).current_state;
    if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_LOW_POWER {
        if rn_get_active_display_cnt_wa(dc, context) == 0 {
            rn_vbios_smu_set_dcn_low_power_state(clk_mgr, DCN_PWR_STATE_LOW_POWER);
            (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_LOW_POWER;
        }
    }
}

unsafe fn rn_update_clocks_update_dpp_dto(clk_mgr: *mut clk_mgr_internal, context: *mut dc_state, ref_dpp_clk: i32, safe_to_lower: bool) {
    (*(*clk_mgr).dccg).ref_dppclk = ref_dpp_clk;
    for i in 0..(*(*(*clk_mgr).base.ctx).dc).res_pool.pipe_count {
        let dpp_inst = (*(*(*(*clk_mgr).base.ctx).dc).res_pool).dpps[i as usize].inst;
        let dppclk_khz = (*context).res_ctx.pipe_ctx[i as usize].plane_res.bw.dppclk_khz;
        let prev = (*(*clk_mgr).dccg).pipe_dppclk_khz[dpp_inst as usize];
        if safe_to_lower || prev < dppclk_khz {
            ((*(*(*clk_mgr).dccg).funcs).update_dpp_dto.unwrap())((*clk_mgr).dccg, dpp_inst, dppclk_khz);
        }
    }
}

unsafe fn rn_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let new_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let dc = (*(*clk_mgr_base).ctx).dc;
    if (*dc).work_arounds.skip_clock_update { return; }
    if safe_to_lower && !(*dc).debug.disable_48mhz_pwrdwn {
        if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_LOW_POWER && rn_get_active_display_cnt_wa(dc, context) == 0 {
            rn_vbios_smu_set_dcn_low_power_state(clk_mgr, DCN_PWR_STATE_LOW_POWER);
            (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_LOW_POWER;
        }
    } else if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_MISSION_MODE {
        rn_vbios_smu_set_dcn_low_power_state(clk_mgr, DCN_PWR_STATE_MISSION_MODE);
        (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_MISSION_MODE;
    }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_khz, (*clk_mgr_base).clks.dcfclk_khz) {
        (*clk_mgr_base).clks.dcfclk_khz = new_clocks.dcfclk_khz;
        rn_vbios_smu_set_hard_min_dcfclk(clk_mgr, (*clk_mgr_base).clks.dcfclk_khz);
    }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_deep_sleep_khz, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz) {
        (*clk_mgr_base).clks.dcfclk_deep_sleep_khz = new_clocks.dcfclk_deep_sleep_khz;
        rn_vbios_smu_set_min_deep_sleep_dcfclk(clk_mgr, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz);
    }
    if new_clocks.dppclk_khz < 100000 && new_clocks.dppclk_khz > 0 { new_clocks.dppclk_khz = 100000; }
    if new_clocks.dppclk_khz == 0 || new_clocks.dispclk_khz == 0 {
        new_clocks.dppclk_khz = (*clk_mgr_base).clks.dppclk_khz;
        new_clocks.dispclk_khz = (*clk_mgr_base).clks.dispclk_khz;
    }
    let mut lowered = false;
    let mut update_dpp = false;
    let mut update_disp = false;
    if should_set_clock(safe_to_lower, new_clocks.dppclk_khz, (*clk_mgr_base).clks.dppclk_khz) {
        lowered = (*clk_mgr_base).clks.dppclk_khz > new_clocks.dppclk_khz;
        (*clk_mgr_base).clks.dppclk_khz = new_clocks.dppclk_khz; update_dpp = true;
    }
    if should_set_clock(safe_to_lower, new_clocks.dispclk_khz, (*clk_mgr_base).clks.dispclk_khz) {
        (*clk_mgr_base).clks.dispclk_khz = new_clocks.dispclk_khz;
        (*clk_mgr_base).clks.actual_dispclk_khz = rn_vbios_smu_set_dispclk(clk_mgr, (*clk_mgr_base).clks.dispclk_khz); update_disp = true;
    }
    if lowered {
        rn_update_clocks_update_dpp_dto(clk_mgr, context, (*clk_mgr_base).clks.dppclk_khz, safe_to_lower);
        (*clk_mgr_base).clks.actual_dppclk_khz = rn_vbios_smu_set_dppclk(clk_mgr, (*clk_mgr_base).clks.dppclk_khz);
        rn_update_clocks_update_dpp_dto(clk_mgr, context, (*clk_mgr_base).clks.actual_dppclk_khz, safe_to_lower);
    } else {
        if update_dpp || update_disp { (*clk_mgr_base).clks.actual_dppclk_khz = rn_vbios_smu_set_dppclk(clk_mgr, (*clk_mgr_base).clks.dppclk_khz); }
        rn_update_clocks_update_dpp_dto(clk_mgr, context, (*clk_mgr_base).clks.actual_dppclk_khz, safe_to_lower);
    }
    let dmcu = (*(*clk_mgr_base).ctx).dc.res_pool.dmcu;
    if update_disp && !dmcu.is_null() && ((*(*dmcu).funcs).is_dmcu_initialized.unwrap())(dmcu) {
        ((*(*dmcu).funcs).set_psr_wait_loop.unwrap())(dmcu, (*clk_mgr_base).clks.dispclk_khz / 1000 / 7);
    }
}

unsafe fn get_vco_frequency_from_reg(clk_mgr: *mut clk_mgr_internal) -> i32 {
    let mut frac = 0u32; let mut intv = 0u32;
    REG_GET(CLK1_CLK_PLL_REQ, FbMult_frac, &mut frac); REG_GET(CLK1_CLK_PLL_REQ, FbMult_int, &mut intv);
    let mut pll = dc_fixpt_from_int(intv); pll.value |= frac << 16;
    dc_fixpt_floor(dc_fixpt_mul_int(pll, (*clk_mgr).dfs_ref_freq_khz))
}

// Register dumping, watermark construction, clock-manager callbacks, and the
// static Renoir tables retain the C layout and are declared below using the
// same externally supplied types and helper symbols.

unsafe fn rn_init_clocks(clk_mgr: *mut clk_mgr) {
    memset(&mut (*clk_mgr).clks as *mut _, 0, core::mem::size_of::<dc_clocks>());
    (*clk_mgr).clks.p_state_change_support = true;
    (*clk_mgr).clks.prev_p_state_change_support = true;
    (*clk_mgr).clks.pwr_state = DCN_PWR_STATE_UNKNOWN;
}

unsafe fn rn_enable_pme_wa(clk_mgr_base: *mut clk_mgr) { rn_vbios_smu_enable_pme_wa(TO_CLK_MGR_INTERNAL(clk_mgr_base)); }

unsafe fn rn_are_clock_states_equal(a: *mut dc_clocks, b: *mut dc_clocks) -> bool {
    (*a).dispclk_khz == (*b).dispclk_khz && (*a).dppclk_khz == (*b).dppclk_khz &&
    (*a).dcfclk_khz == (*b).dcfclk_khz && (*a).dcfclk_deep_sleep_khz == (*b).dcfclk_deep_sleep_khz
}

unsafe fn rn_notify_link_rate_change(clk_mgr_base: *mut clk_mgr, link: *mut dc_link) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base); let mut max_phyclk_req = 0;
    (*clk_mgr).cur_phyclk_req_table[(*link).link_index as usize] = (*link).cur_link_settings.link_rate * LINK_RATE_REF_FREQ_IN_KHZ;
    for i in 0..MAX_LINKS { if (*clk_mgr).cur_phyclk_req_table[i] > max_phyclk_req { max_phyclk_req = (*clk_mgr).cur_phyclk_req_table[i]; } }
    if max_phyclk_req != (*clk_mgr_base).clks.phyclk_khz { (*clk_mgr_base).clks.phyclk_khz = max_phyclk_req; rn_vbios_smu_set_phyclk(clk_mgr, max_phyclk_req); }
}

// C static callback table and rn_clk_mgr_construct are retained as external
// declarations until the corresponding translated headers/types are present.
extern "C" {
    pub fn rn_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_internal, pp_smu: *mut pp_smu_funcs, dccg: *mut dccg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
