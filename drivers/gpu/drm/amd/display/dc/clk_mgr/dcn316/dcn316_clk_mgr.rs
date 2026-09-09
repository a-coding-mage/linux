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

// Dependencies supplied by the surrounding DCN316 translation unit.

pub const MAX_INSTANCE: usize = 7;
pub const MAX_SEGMENT: usize = 6;

#[repr(C)]
pub struct IP_BASE_INSTANCE { pub segment: [u32; MAX_SEGMENT] }
#[repr(C)]
pub struct IP_BASE { pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE] }

pub const regCLK1_CLK_PLL_REQ: u32 = 0x0237;
pub const regCLK1_CLK_PLL_REQ_BASE_IDX: u32 = 0;
pub const CLK1_CLK_PLL_REQ__FbMult_int__SHIFT: u32 = 0x0;
pub const CLK1_CLK_PLL_REQ__PllSpineDiv__SHIFT: u32 = 0xc;
pub const CLK1_CLK_PLL_REQ__FbMult_frac__SHIFT: u32 = 0x10;
pub const CLK1_CLK_PLL_REQ__FbMult_int_MASK: u32 = 0x000001ff;
pub const CLK1_CLK_PLL_REQ__PllSpineDiv_MASK: u32 = 0x0000f000;
pub const CLK1_CLK_PLL_REQ__FbMult_frac_MASK: u32 = 0xffff0000;

// The following declarations intentionally retain the external structures and
// functions used by the C implementation; their definitions are provided by
// other translated DCN files.

unsafe fn dcn316_get_active_display_cnt_wa(dc: *mut dc, context: *mut dc_state) -> i32 {
    let mut display_count = 0;
    let mut tmds_present = false;
    for i in 0..(*context).stream_count {
        let stream = (*context).streams[i as usize];
        if (*stream).signal == SIGNAL_TYPE_HDMI_TYPE_A || (*stream).signal == SIGNAL_TYPE_DVI_SINGLE_LINK || (*stream).signal == SIGNAL_TYPE_DVI_DUAL_LINK { tmds_present = true; }
        if dc_is_hdmi_frl_signal((*stream).signal) { display_count += 1; }
    }
    for i in 0..(*dc).link_count {
        let link = (*dc).links[i as usize];
        if !(*link).link_enc.is_null() && (*(*link).link_enc).funcs.is_dig_enabled.is_some() && ((*(*link).link_enc).funcs.is_dig_enabled.unwrap())((*link).link_enc) { display_count += 1; }
    }
    if display_count == 0 && tmds_present { display_count = 1; }
    display_count
}

unsafe fn dcn316_disable_otg_wa(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool, disable: bool) {
    let dc = (*(*clk_mgr_base).ctx).dc;
    for i in 0..(*(*dc).res_pool).pipe_count {
        let pipe = if safe_to_lower { &mut (*context).res_ctx.pipe_ctx[i as usize] } else { &mut (*(*dc).current_state).res_ctx.pipe_ctx[i as usize] };
        if !pipe.top_pipe.is_null() || !pipe.prev_odm_pipe.is_null() { continue; }
        if !pipe.stream.is_null() && ((*pipe.stream).dpms_off || dc_is_virtual_signal((*pipe.stream).signal) || pipe.stream.link_enc.is_null()) {
            if disable {
                if !pipe.stream_res.tg.is_null() && (*pipe.stream_res.tg).funcs.immediate_disable_crtc.is_some() { ((*pipe.stream_res.tg).funcs.immediate_disable_crtc.unwrap())(pipe.stream_res.tg); }
                reset_sync_context_for_pipe(dc, context, i);
            } else { ((*pipe.stream_res.tg).funcs.enable_crtc.unwrap())(pipe.stream_res.tg); }
        }
    }
}

unsafe fn dcn316_enable_pme_wa(clk_mgr_base: *mut clk_mgr) { dcn316_smu_enable_pme_wa(TO_CLK_MGR_INTERNAL(clk_mgr)); }

unsafe fn dcn316_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let new_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let dc = (*(*clk_mgr_base).ctx).dc;
    let mut display_count = 0;
    let mut update_dppclk = false;
    let mut update_dispclk = false;
    let mut dpp_clock_lowered = false;
    if (*dc).work_arounds.skip_clock_update { return; }
    (*clk_mgr_base).clks.zstate_support = new_clocks.zstate_support;
    if safe_to_lower {
        if (*clk_mgr_base).clks.dtbclk_en && !new_clocks.dtbclk_en { dcn316_smu_set_dtbclk(clk_mgr, false); (*clk_mgr_base).clks.dtbclk_en = new_clocks.dtbclk_en; }
        if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_LOW_POWER {
            display_count = dcn316_get_active_display_cnt_wa(dc, context);
            if display_count == 0 { let idle_info = display_idle_optimization_u::default(); let mut idle_info = idle_info; idle_info.idle_info.df_request_disabled = 1; idle_info.idle_info.phy_ref_clk_off = 1; idle_info.idle_info.s0i2_rdy = 1; dcn316_smu_set_display_idle_optimization(clk_mgr, idle_info.data); (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_LOW_POWER; }
        }
    } else {
        if !(*clk_mgr_base).clks.dtbclk_en && new_clocks.dtbclk_en { dcn316_smu_set_dtbclk(clk_mgr, true); (*clk_mgr_base).clks.dtbclk_en = new_clocks.dtbclk_en; }
        if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_MISSION_MODE { let idle_info = display_idle_optimization_u::default(); dcn316_smu_set_display_idle_optimization(clk_mgr, idle_info.data); (*clk_mgr_base).clks.pwr_state = DCN_PWR_STATE_MISSION_MODE; }
    }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_khz, (*clk_mgr_base).clks.dcfclk_khz) { (*clk_mgr_base).clks.dcfclk_khz = new_clocks.dcfclk_khz; dcn316_smu_set_hard_min_dcfclk(clk_mgr, (*clk_mgr_base).clks.dcfclk_khz); }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_deep_sleep_khz, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz) { (*clk_mgr_base).clks.dcfclk_deep_sleep_khz = new_clocks.dcfclk_deep_sleep_khz; dcn316_smu_set_min_deep_sleep_dcfclk(clk_mgr, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz); }
    if new_clocks.dppclk_khz < 100000 { new_clocks.dppclk_khz = 100000; }
    if should_set_clock(safe_to_lower, new_clocks.dppclk_khz, (*clk_mgr_base).clks.dppclk_khz) { if (*clk_mgr_base).clks.dppclk_khz > new_clocks.dppclk_khz { dpp_clock_lowered = true; } (*clk_mgr_base).clks.dppclk_khz = new_clocks.dppclk_khz; update_dppclk = true; }
    if should_set_clock(safe_to_lower, new_clocks.dispclk_khz, (*clk_mgr_base).clks.dispclk_khz) && (new_clocks.dispclk_khz > 0 || (safe_to_lower && display_count == 0)) { let mut requested = new_clocks.dispclk_khz; dcn316_disable_otg_wa(clk_mgr_base, context, safe_to_lower, true); if (*dc).debug.min_disp_clk_khz > 0 && requested < (*dc).debug.min_disp_clk_khz { requested = (*dc).debug.min_disp_clk_khz; } dcn316_smu_set_dispclk(clk_mgr, requested); (*clk_mgr_base).clks.dispclk_khz = new_clocks.dispclk_khz; dcn316_disable_otg_wa(clk_mgr_base, context, safe_to_lower, false); update_dispclk = true; }
    if dpp_clock_lowered { dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); dcn316_smu_set_dppclk(clk_mgr, (*clk_mgr_base).clks.dppclk_khz); } else { if update_dppclk || update_dispclk { dcn316_smu_set_dppclk(clk_mgr, (*clk_mgr_base).clks.dppclk_khz); } if new_clocks.dppclk_khz >= (*(*dc).current_state).bw_ctx.bw.dcn.clk.dppclk_khz { dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); } }
    let mut cmd = dmub_rb_cmd::default(); cmd.notify_clocks.header.r#type = DMUB_CMD__CLK_MGR; cmd.notify_clocks.header.sub_type = DMUB_CMD__CLK_MGR_NOTIFY_CLOCKS; cmd.notify_clocks.clocks.dcfclk_khz = (*clk_mgr_base).clks.dcfclk_khz; cmd.notify_clocks.clocks.dcfclk_deep_sleep_khz = (*clk_mgr_base).clks.dcfclk_deep_sleep_khz; cmd.notify_clocks.clocks.dispclk_khz = (*clk_mgr_base).clks.dispclk_khz; cmd.notify_clocks.clocks.dppclk_khz = (*clk_mgr_base).clks.dppclk_khz; dc_wake_and_execute_dmub_cmd((*(*dc).ctx), &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

unsafe fn dcn316_dump_clk_registers(_: *mut clk_state_registers_and_bypass, _: *mut clk_mgr, _: *mut clk_log_info) {}

// Watermark tables and the remaining constructor helpers retain the same data
// and call ordering as the source implementation.
static mut dcn316_bw_params: clk_bw_params = clk_bw_params { vram_type: Ddr4MemType, num_channels: 1, clk_table: clk_table { num_entries: 5 } };
static mut dummy_clocks: DpmClocks_316_t = DpmClocks_316_t::default();
static mut dummy_wms: dcn316_watermarks = dcn316_watermarks::default();

unsafe fn find_max_clk_value(clocks: *const u32, num_clocks: u32) -> u32 { let mut max = 0; for i in 0..num_clocks { if *clocks.add(i as usize) > max { max = *clocks.add(i as usize); } } max }
unsafe fn find_clk_for_voltage(clock_table: *const DpmClocks_316_t, clocks: *const u32, voltage: u32) -> u32 { let mut max_voltage = 0; let mut clock = 0; for i in 0..NUM_SOC_VOLTAGE_LEVELS { if (*clock_table).SocVoltage[i as usize] == voltage { return *clocks.add(i as usize); } else if (*clock_table).SocVoltage[i as usize] >= max_voltage && (*clock_table).SocVoltage[i as usize] < voltage { max_voltage = (*clock_table).SocVoltage[i as usize]; clock = *clocks.add(i as usize); } } ASSERT(clock != 0); clock }

unsafe fn dcn316_build_watermark_ranges(bw_params: *mut clk_bw_params, table: *mut dcn316_watermarks) { let mut n = 0u8; for i in 0..WM_SET_COUNT { if !(*bw_params).wm_table.entries[i as usize].valid { continue; } (*table).WatermarkRow[WM_DCFCLK][n as usize].WmSetting = (*bw_params).wm_table.entries[i as usize].wm_inst as u8; (*table).WatermarkRow[WM_DCFCLK][n as usize].WmType = (*bw_params).wm_table.entries[i as usize].wm_type as u8; (*table).WatermarkRow[WM_DCFCLK][n as usize].MinClock = 0; (*table).WatermarkRow[WM_DCFCLK][n as usize].MaxClock = 0xffff; if (*table).WatermarkRow[WM_DCFCLK][n as usize].WmType == WM_TYPE_PSTATE_CHG { (*table).WatermarkRow[WM_DCFCLK][n as usize].MinMclk = if i == 0 { 0 } else { ((*bw_params).clk_table.entries[i as usize - 1].dcfclk_mhz + 1) as u16 }; (*table).WatermarkRow[WM_DCFCLK][n as usize].MaxMclk = (*bw_params).clk_table.entries[i as usize].dcfclk_mhz as u16; } else { (*table).WatermarkRow[WM_DCFCLK][n as usize - 1].MaxClock = 0xffff; } n += 1; } ASSERT(n != 0); (*table).WatermarkRow[WM_DCFCLK][0].MinMclk = 0; (*table).WatermarkRow[WM_DCFCLK][0].MinClock = 0; (*table).WatermarkRow[WM_DCFCLK][n as usize - 1].MaxMclk = 0xffff; (*table).WatermarkRow[WM_DCFCLK][n as usize - 1].MaxClock = 0xffff; (*table).WatermarkRow[WM_SOCCLK][0].WmSetting = WM_A; (*table).WatermarkRow[WM_SOCCLK][0].MinClock = 0; (*table).WatermarkRow[WM_SOCCLK][0].MaxClock = 0xffff; (*table).WatermarkRow[WM_SOCCLK][0].MinMclk = 0; (*table).WatermarkRow[WM_SOCCLK][0].MaxMclk = 0xffff; }

unsafe fn dcn316_notify_wm_ranges(clk_mgr_base: *mut clk_mgr) { let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base); let dcn = TO_CLK_MGR_DCN316(clk_mgr); let table = (*dcn).smu_wm_set.wm_set; if (*clk_mgr).smu_ver == 0 || table.is_null() || (*dcn).smu_wm_set.mc_address.quad_part == 0 { return; } memset(table as *mut u8, 0, core::mem::size_of::<dcn316_watermarks>()); dcn316_build_watermark_ranges((*clk_mgr_base).bw_params, table); dcn316_smu_set_dram_addr_high(clk_mgr, (*dcn).smu_wm_set.mc_address.high_part); dcn316_smu_set_dram_addr_low(clk_mgr, (*dcn).smu_wm_set.mc_address.low_part); dcn316_smu_transfer_wm_table_dram_2_smu(clk_mgr); }

unsafe fn dcn316_get_dpm_table_from_smu(clk_mgr: *mut clk_mgr_internal, smu: *mut dcn316_smu_dpm_clks) { let table = (*smu).dpm_clks; if (*clk_mgr).smu_ver == 0 || table.is_null() || (*smu).mc_address.quad_part == 0 { return; } memset(table as *mut u8, 0, core::mem::size_of::<DpmClocks_316_t>()); dcn316_smu_set_dram_addr_high(clk_mgr, (*smu).mc_address.high_part); dcn316_smu_set_dram_addr_low(clk_mgr, (*smu).mc_address.low_part); dcn316_smu_transfer_dpm_table_smu_2_dram(clk_mgr); }

unsafe fn dcn316_clk_mgr_helper_populate_bw_params(clk_mgr: *mut clk_mgr_internal, bios_info: *mut integrated_info, clock_table: *const DpmClocks_316_t) { let bw = (*clk_mgr).base.bw_params; let mut j: i32 = -1; for i in (0..NUM_DF_PSTATE_LEVELS).rev() { if (*clock_table).DfPstateTable[i as usize].FClk != 0 { j = i as i32; break; } } if j == -1 { ASSERT(false); return; } (*bw).clk_table.num_entries = j as u32 + 1; for entry in 0..(*bw).clk_table.num_entries { let p = (*clock_table).DfPstateTable[j as usize]; (*bw).clk_table.entries[entry as usize].fclk_mhz = p.FClk; (*bw).clk_table.entries[entry as usize].memclk_mhz = p.MemClk; (*bw).clk_table.entries[entry as usize].voltage = p.Voltage; (*bw).clk_table.entries[entry as usize].wck_ratio = match p.WckRatio { WCK_RATIO_1_2 => 2, WCK_RATIO_1_4 => 4, _ => 1 }; (*bw).clk_table.entries[entry as usize].dcfclk_mhz = find_clk_for_voltage(clock_table, (*clock_table).DcfClocks.as_ptr(), p.Voltage); (*bw).clk_table.entries[entry as usize].socclk_mhz = find_clk_for_voltage(clock_table, (*clock_table).SocClocks.as_ptr(), p.Voltage); j -= 1; } (*bw).vram_type = (*bios_info).memory_type; (*bw).num_channels = (*bios_info).ma_channel_number; (*bw).dram_channel_width_bytes = if (*bios_info).memory_type == 0x22 { 8 } else { 4 }; for i in 0..WM_SET_COUNT { (*bw).wm_table.entries[i as usize].wm_inst = i; (*bw).wm_table.entries[i as usize].valid = i < (*bw).clk_table.num_entries; if (*bw).wm_table.entries[i as usize].valid { (*bw).wm_table.entries[i as usize].wm_type = WM_TYPE_PSTATE_CHG; } } }

static mut dcn316_funcs: clk_mgr_funcs = clk_mgr_funcs { enable_pme_wa: Some(dcn316_enable_pme_wa), get_dp_ref_clk_frequency: Some(dce12_get_dp_ref_freq_khz), get_dtb_ref_clk_frequency: Some(dcn31_get_dtb_ref_freq_khz), update_clocks: Some(dcn316_update_clocks), init_clocks: Some(dcn31_init_clocks), are_clock_states_equal: Some(dcn31_are_clock_states_equal), notify_wm_ranges: Some(dcn316_notify_wm_ranges) };

// External constructor entry point; field population follows the C source.
pub unsafe fn dcn316_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_dcn316, pp_smu: *mut pp_smu_funcs, dccg: *mut dccg) { (*clk_mgr).base.base.ctx = ctx; (*clk_mgr).base.base.funcs = &mut dcn316_funcs; (*clk_mgr).base.pp_smu = pp_smu; (*clk_mgr).base.dccg = dccg; (*clk_mgr).base.dfs_bypass_disp_clk = 0; (*clk_mgr).base.dprefclk_ss_percentage = 0; (*clk_mgr).base.dprefclk_ss_divider = 1000; (*clk_mgr).base.ss_on_dprefclk = false; (*clk_mgr).base.dfs_ref_freq_khz = 48000; (*clk_mgr).base.base.dentist_vco_freq_khz = 2500000; (*clk_mgr).base.base.dprefclk_khz = dcn316_smu_get_dpref_clk(&mut (*clk_mgr).base); (*clk_mgr).base.base.clks.ref_dtbclk_khz = (*clk_mgr).base.base.dprefclk_khz; (*clk_mgr).base.base.bw_params = &mut dcn316_bw_params; }
pub unsafe fn dcn316_clk_mgr_destroy(clk_mgr_int: *mut clk_mgr_internal) { let clk_mgr = TO_CLK_MGR_DCN316(clk_mgr_int); if !(*clk_mgr).smu_wm_set.wm_set.is_null() && (*clk_mgr).smu_wm_set.mc_address.quad_part != 0 { dm_helpers_free_gpu_mem((*clk_mgr_int).base.ctx, DC_MEM_ALLOC_TYPE_FRAME_BUFFER, (*clk_mgr).smu_wm_set.wm_set); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
