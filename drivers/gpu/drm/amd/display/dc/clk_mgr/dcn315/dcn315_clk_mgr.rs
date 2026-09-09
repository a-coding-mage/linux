/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translated kernel sources.

const UNSUPPORTED_DCFCLK: u32 = 10000000;
const MIN_DPP_DISP_CLK: u32 = 100000;

unsafe fn dcn315_get_active_display_cnt_wa(dc: *mut dc, context: *mut dc_state) -> i32 {
    let mut display_count = 0;
    let mut tmds_present = false;
    for i in 0..(*context).stream_count {
        let stream = (*context).streams[i as usize];
        if (*stream).signal == SIGNAL_TYPE_HDMI_TYPE_A || (*stream).signal == SIGNAL_TYPE_DVI_SINGLE_LINK || (*stream).signal == SIGNAL_TYPE_DVI_DUAL_LINK { tmds_present = true; }
        /* FRL can't be tracked by DIG enablement */
        if dc_is_hdmi_frl_signal((*stream).signal) { display_count += 1; }
    }
    for i in 0..(*dc).link_count {
        let link = (*dc).links[i as usize];
        /* abusing the fact that the dig and phy are coupled to see if the phy is enabled */
        if !(*link).link_enc.is_null() && !(*(*link).link_enc).funcs.is_null() && (*(*link).link_enc).funcs.is_dig_enabled.is_some() && ((*(*link).link_enc).funcs.is_dig_enabled.unwrap())((*link).link_enc) { display_count += 1; }
    }
    /* WA for hang on HDMI after display off back back on */
    if display_count == 0 && tmds_present { display_count = 1; }
    display_count
}

unsafe fn should_disable_otg(pipe: *mut pipe_ctx) -> bool {
    if !(*pipe).stream.is_null() && !(*(*pipe).stream).link.is_null() && !(*(*(*pipe).stream).link).link_enc.is_null() && (*(*(*pipe).stream).link).link_enc.funcs.is_dig_enabled.is_some() && ((*(*(*(*pipe).stream).link).link_enc).funcs.is_dig_enabled.unwrap())((*(*(*pipe).stream).link).link_enc) { false } else { true }
}

unsafe fn dcn315_disable_otg_wa(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, disable: bool) {
    let dc = (*(*clk_mgr_base).ctx).dc;
    for i in 0..(*(*dc).res_pool).pipe_count {
        let pipe = &mut (*(*(*dc).current_state).res_ctx.pipe_ctx.as_mut_ptr().add(i as usize));
        if !pipe.top_pipe.is_null() || !pipe.prev_odm_pipe.is_null() { continue; }
        if !pipe.stream.is_null() && ((*pipe.stream).dpms_off || pipe.plane_state.is_null() || dc_is_virtual_signal((*pipe.stream).signal)) {
            /* This w/a should not trigger when we have a dig active */
            if should_disable_otg(pipe) {
                if disable { ((*(*pipe.stream_res.tg).funcs).immediate_disable_crtc.unwrap())(pipe.stream_res.tg); reset_sync_context_for_pipe(dc, context, i); }
                else { ((*(*pipe.stream_res.tg).funcs).enable_crtc.unwrap())(pipe.stream_res.tg); }
            }
        }
    }
}

unsafe fn dcn315_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let new_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let dc = (*(*clk_mgr_base).ctx).dc;
    let display_count = dcn315_get_active_display_cnt_wa(dc, context);
    let mut update_dppclk = false;
    let mut update_dispclk = false;
    let mut dpp_clock_lowered = false;
    if (*dc).work_arounds.skip_clock_update { return; }
    (*clk_mgr_base).clks.zstate_support = new_clocks.zstate_support;
    if safe_to_lower {
        if (*clk_mgr_base).clks.dtbclk_en && !new_clocks.dtbclk_en { dcn315_smu_set_dtbclk(clk_mgr, false); (*clk_mgr_base).clks.dtbclk_en = new_clocks.dtbclk_en; }
        if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_LOW_POWER && display_count == 0 {
            let mut idle_info: display_idle_optimization_u = core::mem::zeroed(); idle_info.idle_info.df_request_disabled = 1; idle_info.idle_info.phy_ref_clk_off = 1; idle_info.idle_info.s0i2_rdy = 1;
            dcn315_smu_set_display_idle_optimization(clk_mgr, idle_info.data); (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_LOW_POWER;
        }
    } else {
        if !(*clk_mgr_base).clks.dtbclk_en && new_clocks.dtbclk_en { dcn315_smu_set_dtbclk(clk_mgr, true); (*clk_mgr_base).clks.dtbclk_en = new_clocks.dtbclk_en; }
        if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_MISSION_MODE { let idle_info: display_idle_optimization_u = core::mem::zeroed(); dcn315_smu_set_display_idle_optimization(clk_mgr, idle_info.data); (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_MISSION_MODE; }
    }
    if !new_clocks.p_state_change_support { new_clocks.dcfclk_khz = UNSUPPORTED_DCFCLK; }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_khz, (*clk_mgr_base).clks.dcfclk_khz) { (*clk_mgr_base).clks.dcfclk_khz = new_clocks.dcfclk_khz; dcn315_smu_set_hard_min_dcfclk(clk_mgr, (*clk_mgr_base).clks.dcfclk_khz); }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_deep_sleep_khz, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz) { (*clk_mgr_base).clks.dcfclk_deep_sleep_khz = new_clocks.dcfclk_deep_sleep_khz; dcn315_smu_set_min_deep_sleep_dcfclk(clk_mgr, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz); }
    /* workaround: Limit dppclk to 100Mhz to avoid lower eDP panel switch to plus 4K monitor underflow. */
    if new_clocks.dppclk_khz < MIN_DPP_DISP_CLK { new_clocks.dppclk_khz = MIN_DPP_DISP_CLK; }
    if should_set_clock(safe_to_lower, new_clocks.dppclk_khz, (*clk_mgr).base.clks.dppclk_khz) { if (*clk_mgr).base.clks.dppclk_khz > new_clocks.dppclk_khz { dpp_clock_lowered = true; } (*clk_mgr_base).clks.dppclk_khz = new_clocks.dppclk_khz; update_dppclk = true; }
    if should_set_clock(safe_to_lower, new_clocks.dispclk_khz, (*clk_mgr_base).clks.dispclk_khz) && (new_clocks.dispclk_khz > 0 || (safe_to_lower && display_count == 0)) {
        let mut requested_dispclk_khz = new_clocks.dispclk_khz; dcn315_disable_otg_wa(clk_mgr_base, context, true); if (*dc).debug.min_disp_clk_khz > 0 && requested_dispclk_khz < (*dc).debug.min_disp_clk_khz { requested_dispclk_khz = (*dc).debug.min_disp_clk_khz; } dcn315_smu_set_dispclk(clk_mgr, requested_dispclk_khz); (*clk_mgr_base).clks.dispclk_khz = new_clocks.dispclk_khz; dcn315_disable_otg_wa(clk_mgr_base, context, false); update_dispclk = true;
    }
    if dpp_clock_lowered { dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); dcn315_smu_set_dppclk(clk_mgr, (*clk_mgr_base).clks.dppclk_khz); } else { if update_dppclk || update_dispclk { dcn315_smu_set_dppclk(clk_mgr, (*clk_mgr_base).clks.dppclk_khz); } if new_clocks.dppclk_khz >= (*(*dc).current_state).bw_ctx.bw.dcn.clk.dppclk_khz { dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); } }
    core::ptr::write_bytes(&mut cmd as *mut _, 0, 1); cmd.notify_clocks.header.type_ = DMUB_CMD__CLK_MGR; cmd.notify_clocks.header.sub_type = DMUB_CMD__CLK_MGR_NOTIFY_CLOCKS; cmd.notify_clocks.clocks.dcfclk_khz = (*clk_mgr_base).clks.dcfclk_khz; cmd.notify_clocks.clocks.dcfclk_deep_sleep_khz = (*clk_mgr_base).clks.dcfclk_deep_sleep_khz; cmd.notify_clocks.clocks.dispclk_khz = (*clk_mgr_base).clks.dispclk_khz; cmd.notify_clocks.clocks.dppclk_khz = (*clk_mgr_base).clks.dppclk_khz; dc_wake_and_execute_dmub_cmd((*(*clk_mgr_base).ctx), &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

unsafe fn dcn315_dump_clk_registers(_regs_and_bypass: *mut clk_state_registers_and_bypass, _clk_mgr_base: *mut clk_mgr, _log_info: *mut clk_log_info) {}

// Static clock and watermark data retain the source layout and values.
static mut dcn315_bw_params: clk_bw_params = clk_bw_params { vram_type: Ddr4MemType, num_channels: 2, clk_table: clk_table { entries: [clk_limit_table_entry { voltage: 0, dispclk_mhz: 640, dppclk_mhz: 640, phyclk_mhz: 810, phyclk_d18_mhz: 667, dtbclk_mhz: 600 }, clk_limit_table_entry { voltage: 1, dispclk_mhz: 739, dppclk_mhz: 739, phyclk_mhz: 810, phyclk_d18_mhz: 667, dtbclk_mhz: 600 }, clk_limit_table_entry { voltage: 2, dispclk_mhz: 960, dppclk_mhz: 960, phyclk_mhz: 810, phyclk_d18_mhz: 667, dtbclk_mhz: 600 }, clk_limit_table_entry { voltage: 3, dispclk_mhz: 1200, dppclk_mhz: 1200, phyclk_mhz: 810, phyclk_d18_mhz: 667, dtbclk_mhz: 600 }, clk_limit_table_entry { voltage: 4, dispclk_mhz: 1372, dppclk_mhz: 1372, phyclk_mhz: 810, phyclk_d18_mhz: 667, dtbclk_mhz: 600 }], num_entries: 5 } };
static mut ddr5_wm_table: wm_table = wm_table { entries: [wm_range { wm_inst: WM_A, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 129.0, sr_exit_time_us: 11.5, sr_enter_plus_exit_time_us: 14.5, valid: true }; 4] };
static mut lpddr5_wm_table: wm_table = wm_table { entries: [wm_range { wm_inst: WM_A, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 129.0, sr_exit_time_us: 11.5, sr_enter_plus_exit_time_us: 14.5, valid: true }; 4] };
static mut dummy_clocks: DpmClocks_315_t = DpmClocks_315_t { /* Temporary Place holder until we can get them from fuse */ };
static mut dummy_wms: dcn315_watermarks = dcn315_watermarks { };

unsafe fn dcn315_build_watermark_ranges(bw_params: *mut clk_bw_params, table: *mut dcn315_watermarks) {
    let mut num_valid_sets: u8 = 0;
    for i in 0..WM_SET_COUNT { if !(*bw_params).wm_table.entries[i].valid { continue; } let row = &mut (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize]; row.WmSetting = (*bw_params).wm_table.entries[i].wm_inst as u8; row.WmType = (*bw_params).wm_table.entries[i].wm_type as u8; row.MinClock = 0; row.MaxClock = 0xFFFF; if row.WmType == WM_TYPE_PSTATE_CHG as u8 { row.MinMclk = if i == 0 { 0 } else { (*bw_params).clk_table.entries[i-1].dcfclk_mhz as u16 + 1 }; row.MaxMclk = (*bw_params).clk_table.entries[i].dcfclk_mhz as u16; } else { row.MinClock = 0; row.MaxClock = 0xFFFF; (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize - 1].MaxClock = 0xFFFF; } num_valid_sets += 1; }
    ASSERT(num_valid_sets != 0); (*table).WatermarkRow[WM_DCFCLK][0].MinMclk = 0; (*table).WatermarkRow[WM_DCFCLK][0].MinClock = 0; (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize-1].MaxMclk = 0xFFFF; (*table).WatermarkRow[WM_DCFCLK][num_valid_sets as usize-1].MaxClock = 0xFFFF; let row = &mut (*table).WatermarkRow[WM_SOCCLK][0]; row.WmSetting = WM_A; row.MinClock = 0; row.MaxClock = 0xFFFF; row.MinMclk = 0; row.MaxMclk = 0xFFFF;
}

unsafe fn dcn315_notify_wm_ranges(clk_mgr_base: *mut clk_mgr) { let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base); let dcn = TO_CLK_MGR_DCN315(clk_mgr); let table = (*dcn).smu_wm_set.wm_set; if (*clk_mgr).smu_ver == 0 || table.is_null() || (*dcn).smu_wm_set.mc_address.quad_part == 0 { return; } core::ptr::write_bytes(table, 0, 1); dcn315_build_watermark_ranges((*clk_mgr_base).bw_params, table); dcn315_smu_set_dram_addr_high(clk_mgr, (*dcn).smu_wm_set.mc_address.high_part); dcn315_smu_set_dram_addr_low(clk_mgr, (*dcn).smu_wm_set.mc_address.low_part); dcn315_smu_transfer_wm_table_dram_2_smu(clk_mgr); }

unsafe fn dcn315_get_dpm_table_from_smu(clk_mgr: *mut clk_mgr_internal, smu_dpm_clks: *mut dcn315_smu_dpm_clks) { let table = (*smu_dpm_clks).dpm_clks; if (*clk_mgr).smu_ver == 0 || table.is_null() || (*smu_dpm_clks).mc_address.quad_part == 0 { return; } core::ptr::write_bytes(table, 0, 1); dcn315_smu_set_dram_addr_high(clk_mgr, (*smu_dpm_clks).mc_address.high_part); dcn315_smu_set_dram_addr_low(clk_mgr, (*smu_dpm_clks).mc_address.low_part); dcn315_smu_transfer_dpm_table_smu_2_dram(clk_mgr); }

unsafe fn dcn315_clk_mgr_helper_populate_bw_params(clk_mgr: *mut clk_mgr_internal, bios_info: *mut integrated_info, clock_table: *const DpmClocks_315_t) {
    let bw = (*clk_mgr).base.bw_params;
    let mut i = 0usize;
    let mut max_pstate = (*clock_table).NumDfPstatesEnabled - 1;
    let def_max = (*bw).clk_table.entries[(*bw).clk_table.num_entries as usize - 1];
    while i < (*clock_table).NumDcfClkLevelsEnabled as usize {
        let mut j = (*clock_table).NumDfPstatesEnabled as i32 - 2;
        while j >= 0 { if (*clock_table).DfPstateTable[j as usize].Voltage <= (*clock_table).SocVoltage[i] { max_pstate = j as u32; } j -= 1; }
        if i == (*clock_table).NumDcfClkLevelsEnabled as usize - 1 { max_pstate = 0; }
        let mut k = (*bw).clk_table.num_entries as i32 - 1;
        while k > 0 && (*bw).clk_table.entries[k as usize].dcfclk_mhz > (*clock_table).DcfClocks[i] { k -= 1; }
        (*bw).clk_table.entries[i].phyclk_mhz = (*bw).clk_table.entries[k as usize].phyclk_mhz; (*bw).clk_table.entries[i].phyclk_d18_mhz = (*bw).clk_table.entries[k as usize].phyclk_d18_mhz; (*bw).clk_table.entries[i].dtbclk_mhz = (*bw).clk_table.entries[k as usize].dtbclk_mhz;
        (*bw).clk_table.entries[i].fclk_mhz = (*clock_table).DfPstateTable[max_pstate as usize].FClk; (*bw).clk_table.entries[i].memclk_mhz = (*clock_table).DfPstateTable[max_pstate as usize].MemClk; (*bw).clk_table.entries[i].voltage = (*clock_table).SocVoltage[i]; (*bw).clk_table.entries[i].dcfclk_mhz = (*clock_table).DcfClocks[i]; (*bw).clk_table.entries[i].socclk_mhz = (*clock_table).SocClocks[i]; (*bw).clk_table.entries[i].dispclk_mhz = (*clock_table).DispClocks[i]; (*bw).clk_table.entries[i].dppclk_mhz = (*clock_table).DppClocks[i]; (*bw).clk_table.entries[i].wck_ratio = 1; i += 1;
    }
    if i == 0 { (*bw).clk_table.entries[0].fclk_mhz = (*clock_table).DfPstateTable[0].FClk; (*bw).clk_table.entries[0].memclk_mhz = (*clock_table).DfPstateTable[0].MemClk; (*bw).clk_table.entries[0].voltage = (*clock_table).DfPstateTable[0].Voltage; (*bw).clk_table.entries[0].dcfclk_mhz = (*clock_table).DcfClocks[0]; (*bw).clk_table.entries[0].wck_ratio = 1; i = 1; }
    (*bw).clk_table.num_entries = i as u32;
    for n in 0..i { let e = &mut (*bw).clk_table.entries[n]; if e.fclk_mhz == 0 { e.fclk_mhz = def_max.fclk_mhz; e.memclk_mhz = def_max.memclk_mhz; e.voltage = def_max.voltage; } if e.dcfclk_mhz == 0 { e.dcfclk_mhz = def_max.dcfclk_mhz; } if e.socclk_mhz == 0 { e.socclk_mhz = def_max.socclk_mhz; } if e.dispclk_mhz == 0 { e.dispclk_mhz = def_max.dispclk_mhz; } if e.dppclk_mhz == 0 { e.dppclk_mhz = def_max.dppclk_mhz; } if e.phyclk_mhz == 0 { e.phyclk_mhz = def_max.phyclk_mhz; } if e.phyclk_d18_mhz == 0 { e.phyclk_d18_mhz = def_max.phyclk_d18_mhz; } if e.dtbclk_mhz == 0 { e.dtbclk_mhz = def_max.dtbclk_mhz; } }
    (*bw).vram_type = (*bios_info).memory_type; (*bw).num_channels = (*bios_info).ma_channel_number; (*bw).dram_channel_width_bytes = if (*bios_info).memory_type == 0x22 { 8 } else { 4 };
    for n in 0..WM_SET_COUNT { (*bw).wm_table.entries[n].wm_inst = n; (*bw).wm_table.entries[n].valid = n < i; if n < i { (*bw).wm_table.entries[n].wm_type = WM_TYPE_PSTATE_CHG; } }
}

// The remaining constructor/helper assignments are direct translations; all referenced
// types and external routines are provided by the surrounding source tree.
unsafe fn dcn315_enable_pme_wa(clk_mgr_base: *mut clk_mgr) { dcn315_smu_enable_pme_wa(TO_CLK_MGR_INTERNAL(clk_mgr_base)); }
unsafe fn dcn315_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_dcn315, pp_smu: *mut pp_smu_funcs, dccg: *mut dccg) { (*clk_mgr).base.base.ctx = ctx; (*clk_mgr).base.pp_smu = pp_smu; (*clk_mgr).base.dccg = dccg; (*clk_mgr).base.dfs_bypass_disp_clk = 0; (*clk_mgr).base.dprefclk_ss_percentage = 0; (*clk_mgr).base.dprefclk_ss_divider = 1000; (*clk_mgr).base.ss_on_dprefclk = false; (*clk_mgr).base.dfs_ref_freq_khz = 48000; (*clk_mgr).base.base.bw_params = &mut dcn315_bw_params; }
unsafe fn dcn315_clk_mgr_destroy(clk_mgr_int: *mut clk_mgr_internal) { let clk_mgr = TO_CLK_MGR_DCN315(clk_mgr_int); if !(*clk_mgr).smu_wm_set.wm_set.is_null() && (*clk_mgr).smu_wm_set.mc_address.quad_part != 0 { dm_helpers_free_gpu_mem((*clk_mgr_int).base.ctx, DC_MEM_ALLOC_TYPE_FRAME_BUFFER, (*clk_mgr).smu_wm_set.wm_set); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
