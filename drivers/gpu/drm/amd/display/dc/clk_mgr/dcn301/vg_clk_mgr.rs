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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C headers and register/macro dependencies are supplied by the surrounding driver.

const LPDDR_MEM_RETRAIN_LATENCY: f64 = 4.977;

unsafe fn vg_get_active_display_cnt_wa(dc: *mut dc, context: *mut dc_state) -> c_int {
    let mut display_count = 0;
    let mut tmds_present = false;
    for i in 0..(*context).stream_count {
        let stream = (*context).streams[i as usize];
        if (*stream).signal == SIGNAL_TYPE_HDMI_TYPE_A || (*stream).signal == SIGNAL_TYPE_DVI_SINGLE_LINK || (*stream).signal == SIGNAL_TYPE_DVI_DUAL_LINK { tmds_present = true; }
    }
    for i in 0..(*dc).link_count {
        let link = (*dc).links[i as usize];
        if (*(*link).link_enc).funcs.is_dig_enabled.is_some() && ((*(*link).link_enc).funcs.is_dig_enabled.unwrap())((*link).link_enc) { display_count += 1; }
    }
    if display_count == 0 && tmds_present { display_count = 1; }
    display_count
}

unsafe fn vg_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let new_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let dc = (*clk_mgr_base).ctx.dc;
    if (*dc).work_arounds.skip_clock_update { return; }
    let mut update_dppclk = false;
    let mut update_dispclk = false;
    let mut dpp_clock_lowered = false;
    if safe_to_lower {
        if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_LOW_POWER {
            if vg_get_active_display_cnt_wa(dc, context) == 0 {
                let mut idle_info: display_idle_optimization_u = core::mem::zeroed();
                idle_info.idle_info.df_request_disabled = 1;
                idle_info.idle_info.phy_ref_clk_off = 1;
                dcn301_smu_set_display_idle_optimization(clk_mgr, idle_info.data);
                (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_LOW_POWER;
            }
        }
    } else if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_MISSION_MODE {
        let idle_info: display_idle_optimization_u = core::mem::zeroed();
        dcn301_smu_set_display_idle_optimization(clk_mgr, idle_info.data);
        (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_MISSION_MODE;
    }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_khz, (*clk_mgr_base).clks.dcfclk_khz) && !(*dc).debug.disable_min_fclk { (*clk_mgr_base).clks.dcfclk_khz = new_clocks.dcfclk_khz; dcn301_smu_set_hard_min_dcfclk(clk_mgr, (*clk_mgr_base).clks.dcfclk_khz); }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_deep_sleep_khz, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz) && !(*dc).debug.disable_min_fclk { (*clk_mgr_base).clks.dcfclk_deep_sleep_khz = new_clocks.dcfclk_deep_sleep_khz; dcn301_smu_set_min_deep_sleep_dcfclk(clk_mgr, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz); }
    if new_clocks.dppclk_khz < 100000 { new_clocks.dppclk_khz = 100000; }
    if should_set_clock(safe_to_lower, new_clocks.dppclk_khz, (*clk_mgr).base.clks.dppclk_khz) { if (*clk_mgr).base.clks.dppclk_khz > new_clocks.dppclk_khz { dpp_clock_lowered = true; } (*clk_mgr_base).clks.dppclk_khz = new_clocks.dppclk_khz; update_dppclk = true; }
    if should_set_clock(safe_to_lower, new_clocks.dispclk_khz, (*clk_mgr_base).clks.dispclk_khz) { (*clk_mgr_base).clks.dispclk_khz = new_clocks.dispclk_khz; dcn301_smu_set_dispclk(clk_mgr, (*clk_mgr_base).clks.dispclk_khz); update_dispclk = true; }
    if dpp_clock_lowered { dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); dcn301_smu_set_dppclk(clk_mgr, (*clk_mgr_base).clks.dppclk_khz); } else { if update_dppclk || update_dispclk { dcn301_smu_set_dppclk(clk_mgr, (*clk_mgr_base).clks.dppclk_khz); } dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); }
}

unsafe fn get_vco_frequency_from_reg(clk_mgr: *mut clk_mgr_internal) -> c_int {
    let mut pll_req: fixed31_32;
    let mut fbmult_frac_val = 0u32;
    let mut fbmult_int_val = 0u32;
    REG_GET!(CLK1_0_CLK1_CLK_PLL_REQ, FbMult_frac, &mut fbmult_frac_val);
    REG_GET!(CLK1_0_CLK1_CLK_PLL_REQ, FbMult_int, &mut fbmult_int_val);
    pll_req = dc_fixpt_from_int(fbmult_int_val);
    pll_req.value |= fbmult_frac_val << 16;
    pll_req = dc_fixpt_mul_int(pll_req, (*clk_mgr).dfs_ref_freq_khz);
    dc_fixpt_floor(pll_req)
}

unsafe fn vg_dump_clk_registers_internal(internal: *mut dcn301_clk_internal, clk_mgr_base: *mut clk_mgr) {
    let _clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    (*internal).CLK1_CLK3_CURRENT_CNT = REG_READ!(CLK1_0_CLK1_CLK3_CURRENT_CNT);
    (*internal).CLK1_CLK3_BYPASS_CNTL = REG_READ!(CLK1_0_CLK1_CLK3_BYPASS_CNTL);
    (*internal).CLK1_CLK3_DS_CNTL = REG_READ!(CLK1_0_CLK1_CLK3_DS_CNTL);
    (*internal).CLK1_CLK3_ALLOW_DS = REG_READ!(CLK1_0_CLK1_CLK3_ALLOW_DS);
    (*internal).CLK1_CLK1_CURRENT_CNT = REG_READ!(CLK1_0_CLK1_CLK1_CURRENT_CNT);
    (*internal).CLK1_CLK1_BYPASS_CNTL = REG_READ!(CLK1_0_CLK1_CLK1_BYPASS_CNTL);
    (*internal).CLK1_CLK2_CURRENT_CNT = REG_READ!(CLK1_0_CLK1_CLK2_CURRENT_CNT);
    (*internal).CLK1_CLK2_BYPASS_CNTL = REG_READ!(CLK1_0_CLK1_CLK2_BYPASS_CNTL);
    (*internal).CLK1_CLK0_CURRENT_CNT = REG_READ!(CLK1_0_CLK1_CLK0_CURRENT_CNT);
    (*internal).CLK1_CLK0_BYPASS_CNTL = REG_READ!(CLK1_0_CLK1_CLK0_BYPASS_CNTL);
}

unsafe fn vg_dump_clk_registers(regs: *mut clk_state_registers_and_bypass, clk_mgr: *mut clk_mgr, log_info: *mut clk_log_info) {
    let mut internal: dcn301_clk_internal = core::mem::zeroed();
    vg_dump_clk_registers_internal(&mut internal, clk_mgr);
    (*regs).dcfclk = internal.CLK1_CLK3_CURRENT_CNT / 10; (*regs).dcf_deep_sleep_divider = internal.CLK1_CLK3_DS_CNTL / 10; (*regs).dcf_deep_sleep_allow = internal.CLK1_CLK3_ALLOW_DS;
    (*regs).dprefclk = internal.CLK1_CLK2_CURRENT_CNT / 10; (*regs).dispclk = internal.CLK1_CLK0_CURRENT_CNT / 10; (*regs).dppclk = internal.CLK1_CLK1_CURRENT_CNT / 10;
    (*regs).dppclk_bypass = internal.CLK1_CLK1_BYPASS_CNTL & 7; (*regs).dcfclk_bypass = internal.CLK1_CLK3_BYPASS_CNTL & 7; (*regs).dispclk_bypass = internal.CLK1_CLK0_BYPASS_CNTL & 7; (*regs).dprefclk_bypass = internal.CLK1_CLK2_BYPASS_CNTL & 7;
    if (*regs).dppclk_bypass > 4 { (*regs).dppclk_bypass = 0; } if (*regs).dcfclk_bypass > 4 { (*regs).dcfclk_bypass = 0; } if (*regs).dispclk_bypass > 4 { (*regs).dispclk_bypass = 0; } if (*regs).dprefclk_bypass > 4 { (*regs).dprefclk_bypass = 0; }
    if (*log_info).enabled { let mut p = (*log_info).pBuf; let mut n = (*log_info).bufSize; let names = ["0x0 DFS", "0x1 REFCLK", "0x2 ERROR", "0x3 400 FCH", "0x4 600 FCH"]; macro_rules! out { ($fmt:expr, $($arg:expr),*) => { let k = snprintf_count(p, n, $fmt, $($arg),*); n -= k; *(*log_info).sum_chars_printed += k; p = p.add(k); }; } out!("clk_type,clk_value,deepsleep_cntl,deepsleep_allow,bypass\n"); out!("dcfclk,%d,%d,%d,%s\n", (*regs).dcfclk, (*regs).dcf_deep_sleep_divider, (*regs).dcf_deep_sleep_allow, names[(*regs).dcfclk_bypass as usize]); out!("dprefclk,%d,N/A,N/A,%s\n", (*regs).dprefclk, names[(*regs).dprefclk_bypass as usize]); out!("dispclk,%d,N/A,N/A,%s\n", (*regs).dispclk, names[(*regs).dispclk_bypass as usize]); out!("SPLIT\n"); out!("reg_name,value,clk_type\n"); out!("CLK1_CLK3_CURRENT_CNT,%d,dcfclk\n", internal.CLK1_CLK3_CURRENT_CNT); out!("CLK1_CLK3_DS_CNTL,%d,dcf_deep_sleep_divider\n", internal.CLK1_CLK3_DS_CNTL); out!("CLK1_CLK3_ALLOW_DS,%d,dcf_deep_sleep_allow\n", internal.CLK1_CLK3_ALLOW_DS); out!("CLK1_CLK2_CURRENT_CNT,%d,dprefclk\n", internal.CLK1_CLK2_CURRENT_CNT); out!("CLK1_CLK0_CURRENT_CNT,%d,dispclk\n", internal.CLK1_CLK0_CURRENT_CNT); out!("CLK1_CLK1_CURRENT_CNT,%d,dppclk\n", internal.CLK1_CLK1_CURRENT_CNT); out!("CLK1_CLK3_BYPASS_CNTL,%d,dcfclk_bypass\n", internal.CLK1_CLK3_BYPASS_CNTL); out!("CLK1_CLK2_BYPASS_CNTL,%d,dprefclk_bypass\n", internal.CLK1_CLK2_BYPASS_CNTL); out!("CLK1_CLK0_BYPASS_CNTL,%d,dispclk_bypass\n", internal.CLK1_CLK0_BYPASS_CNTL); out!("CLK1_CLK1_BYPASS_CNTL,%d,dppclk_bypass\n", internal.CLK1_CLK1_BYPASS_CNTL); }
}

unsafe fn vg_enable_pme_wa(clk_mgr_base: *mut clk_mgr) { dcn301_smu_enable_pme_wa(TO_CLK_MGR_INTERNAL(clk_mgr)); }
unsafe fn vg_init_clocks(clk_mgr: *mut clk_mgr) { core::ptr::write_bytes(&mut (*clk_mgr).clks, 0, 1); (*clk_mgr).clks.p_state_change_support = true; (*clk_mgr).clks.prev_p_state_change_support = true; (*clk_mgr).clks.pwr_state = DCN_PWR_STATE_UNKNOWN; }

unsafe fn vg_are_clock_states_equal(a: *mut dc_clocks, b: *mut dc_clocks) -> bool { (*a).dispclk_khz == (*b).dispclk_khz && (*a).dppclk_khz == (*b).dppclk_khz && (*a).dcfclk_khz == (*b).dcfclk_khz && (*a).dcfclk_deep_sleep_khz == (*b).dcfclk_deep_sleep_khz }

unsafe fn find_max_clk_value(clocks: *const u32, num_clocks: u32) -> u32 { let mut max = 0; for i in 0..num_clocks { if *clocks.add(i as usize) > max { max = *clocks.add(i as usize); } } max }
unsafe fn find_dcfclk_for_voltage(clock_table: *const vg_dpm_clocks, voltage: u32) -> u32 { for i in 0..VG_NUM_SOC_VOLTAGE_LEVELS { if i >= VG_NUM_DCFCLK_DPM_LEVELS { break; } if (*clock_table).SocVoltage[i as usize] == voltage { return (*clock_table).DcfClocks[i as usize]; } } ASSERT!(false); 0 }

unsafe fn vg_build_watermark_ranges(bw_params: *mut clk_bw_params, table: *mut watermarks) {
    let mut num_valid_sets: u8 = 0;
    for i in 0..WM_SET_COUNT { if !(*bw_params).wm_table.entries[i as usize].valid { continue; }
        (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize].WmSetting = (*bw_params).wm_table.entries[i as usize].wm_inst as u8;
        (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize].WmType = (*bw_params).wm_table.entries[i as usize].wm_type as u8;
        (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize].MinClock = 0; (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize].MaxClock = 0xffff;
        if (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize].WmType == WM_TYPE_PSTATE_CHG { (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize].MinMclk = if i == 0 { 0 } else { ((*bw_params).clk_table.entries[i as usize - 1].dcfclk_mhz + 1) as u16 }; (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize].MaxMclk = (*bw_params).clk_table.entries[i as usize].dcfclk_mhz as u16; } else { (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize - 1].MaxClock = 0xffff; }
        num_valid_sets += 1;
    }
    ASSERT!(num_valid_sets != 0); (*table).WatermarkRow[WM_DCFCLK][0].MinMclk = 0; (*table).WatermarkRow[WM_DCFCLK][0].MinClock = 0; (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize - 1].MaxMclk = 0xffff; (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize - 1].MaxClock = 0xffff;
    (*table).WatermarkRow[WM_SOCCLK][0].WmSetting = WM_A; (*table).WatermarkRow[WM_SOCCLK][0].MinClock = 0; (*table).WatermarkRow[WM_SOCCLK][0].MaxClock = 0xffff; (*table).WatermarkRow[WM_SOCCLK][0].MinMclk = 0; (*table).WatermarkRow[WM_SOCCLK][0].MaxMclk = 0xffff;
}

unsafe fn vg_notify_wm_ranges(clk_mgr_base: *mut clk_mgr) { let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base); let vgh = TO_CLK_MGR_VGH(clk_mgr); if (*clk_mgr).smu_ver == 0 || (*vgh).smu_wm_set.wm_set.is_null() || (*vgh).smu_wm_set.mc_address.quad_part == 0 { return; } core::ptr::write_bytes((*vgh).smu_wm_set.wm_set, 0, 1); vg_build_watermark_ranges((*clk_mgr_base).bw_params, (*vgh).smu_wm_set.wm_set); dcn301_smu_set_dram_addr_high(clk_mgr, (*vgh).smu_wm_set.mc_address.high_part); dcn301_smu_set_dram_addr_low(clk_mgr, (*vgh).smu_wm_set.mc_address.low_part); dcn301_smu_transfer_wm_table_dram_2_smu(clk_mgr); }

unsafe fn vg_get_dpm_table_from_smu(clk_mgr: *mut clk_mgr_internal, smu: *mut smu_dpm_clks) { if (*clk_mgr).smu_ver == 0 || (*smu).dpm_clks.is_null() || (*smu).mc_address.quad_part == 0 { return; } core::ptr::write_bytes((*smu).dpm_clks, 0, 1); dcn301_smu_set_dram_addr_high(clk_mgr, (*smu).mc_address.high_part); dcn301_smu_set_dram_addr_low(clk_mgr, (*smu).mc_address.low_part); dcn301_smu_transfer_dpm_table_smu_2_dram(clk_mgr); }

unsafe fn vg_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_vgh, pp_smu: *mut pp_smu_funcs, dccg: *mut dccg) {
    (*clk_mgr).base.base.ctx = ctx; (*clk_mgr).base.base.funcs = &vg_funcs; (*clk_mgr).base.pp_smu = pp_smu; (*clk_mgr).base.dccg = dccg; (*clk_mgr).base.dfs_bypass_disp_clk = 0; (*clk_mgr).base.dprefclk_ss_percentage = 0; (*clk_mgr).base.dprefclk_ss_divider = 1000; (*clk_mgr).base.ss_on_dprefclk = false; (*clk_mgr).base.dfs_ref_freq_khz = 48000; (*clk_mgr).base.base.dentist_vco_freq_khz = get_vco_frequency_from_reg(&mut (*clk_mgr).base); if (*clk_mgr).base.base.dentist_vco_freq_khz == 0 { (*clk_mgr).base.base.dentist_vco_freq_khz = 3600000; } (*clk_mgr).base.base.dprefclk_khz = 600000;
}

unsafe fn vg_clk_mgr_destroy(clk_mgr_int: *mut clk_mgr_internal) { let clk_mgr = TO_CLK_MGR_VGH(clk_mgr_int); if !(*clk_mgr).smu_wm_set.wm_set.is_null() && (*clk_mgr).smu_wm_set.mc_address.quad_part != 0 { dm_helpers_free_gpu_mem((*clk_mgr_int).base.ctx, DC_MEM_ALLOC_TYPE_FRAME_BUFFER, (*clk_mgr).smu_wm_set.wm_set); } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
