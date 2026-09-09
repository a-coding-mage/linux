/* Translated from dcn30_clk_mgr.c. C headers and externally supplied symbols
 * remain dependencies of the surrounding translation unit. */

// C preprocessor register helpers are represented by the corresponding
// externally supplied Rust macros/items.

static mut CLK_MGR_REGS: clk_mgr_registers = clk_mgr_registers { };
static mut CLK_MGR_SHIFT: clk_mgr_shift = clk_mgr_shift { };
static mut CLK_MGR_MASK: clk_mgr_mask = clk_mgr_mask { };

unsafe fn dcn3_init_single_clock(clk_mgr: *mut clk_mgr_internal, clk: u32,
    entry_0: *mut u32, num_levels: *mut u8) {
    let mut i: u8;
    let mut entry_i = entry_0 as *mut u8;
    let ret = dcn30_smu_get_dpm_freq_by_index(clk_mgr, clk, 0xff);
    if (ret & (1 << 31)) != 0 { *num_levels = 2; }
    else { *num_levels = (ret & 0xff) as u8; }
    i = 0;
    while i < *num_levels {
        *(entry_i as *mut u32) = dcn30_smu_get_dpm_freq_by_index(clk_mgr, clk, i as u32) & 0xffff;
        entry_i = entry_i.add(core::mem::size_of::<clk_entry>());
        i += 1;
    }
}

unsafe fn dcn3_build_wm_range_table(clk_mgr: *mut clk_mgr_internal) {
    DC_FP_START!(); dcn3_fpu_build_wm_range_table(&mut (*clk_mgr).base); DC_FP_END!();
}

pub unsafe fn dcn3_init_clocks(clk_mgr_base: *mut clk_mgr) {
    let clk_mgr = TO_CLK_MGR_INTERNAL!(clk_mgr_base);
    let mut num_levels: u8 = 0;
    core::ptr::write_bytes(&mut (*clk_mgr_base).clks as *mut dc_clocks as *mut u8, 0,
        core::mem::size_of::<dc_clocks>());
    (*clk_mgr_base).clks.p_state_change_support = true;
    (*clk_mgr_base).clks.prev_p_state_change_support = true;
    (*clk_mgr).smu_present = false;
    if (*clk_mgr_base).bw_params.is_null() { return; }
    if !(*clk_mgr_base).force_smu_not_present && dcn30_smu_get_smu_version(clk_mgr, &mut (*clk_mgr).smu_ver) { (*clk_mgr).smu_present = true; }
    if !(*clk_mgr).smu_present { return; }
    dcn30_smu_check_driver_if_version(clk_mgr); dcn30_smu_check_msg_header_version(clk_mgr);
    dcn3_init_single_clock(clk_mgr, PPCLK_DCEFCLK, &mut (*(*clk_mgr_base).bw_params).clk_table.entries[0].dcfclk_mhz, &mut num_levels);
    dcn30_smu_set_min_deep_sleep_dcef_clk(clk_mgr, 0);
    dcn3_init_single_clock(clk_mgr, PPCLK_DTBCLK, &mut (*(*clk_mgr_base).bw_params).clk_table.entries[0].dtbclk_mhz, &mut num_levels);
    dcn3_init_single_clock(clk_mgr, PPCLK_SOCCLK, &mut (*(*clk_mgr_base).bw_params).clk_table.entries[0].socclk_mhz, &mut num_levels);
    dcn3_init_single_clock(clk_mgr, PPCLK_DISPCLK, &mut (*(*clk_mgr_base).bw_params).clk_table.entries[0].dispclk_mhz, &mut num_levels);
    dcn3_init_single_clock(clk_mgr, PPCLK_PIXCLK, &mut (*(*clk_mgr_base).bw_params).clk_table.entries[0].dppclk_mhz, &mut num_levels);
    dcn3_init_single_clock(clk_mgr, PPCLK_PHYCLK, &mut (*(*clk_mgr_base).bw_params).clk_table.entries[0].phyclk_mhz, &mut num_levels);
    (*clk_mgr_base).funcs.as_ref().unwrap().get_memclk_states_from_smu(clk_mgr_base);
    DC_FP_START!(); dcn3_build_wm_range_table(clk_mgr); DC_FP_END!();
}

unsafe fn dcn30_get_vco_frequency_from_reg(clk_mgr: *mut clk_mgr_internal) -> i32 {
    let pll_req_reg = REG_READ!(clk_mgr, CLK0_CLK_PLL_REQ);
    let mut pll_req = dc_fixpt_from_int(pll_req_reg & (*clk_mgr).clk_mgr_mask.FbMult_int);
    pll_req.value |= pll_req_reg & (*clk_mgr).clk_mgr_mask.FbMult_frac;
    pll_req = dc_fixpt_mul_int(pll_req, (*clk_mgr).dfs_ref_freq_khz);
    dc_fixpt_floor(pll_req)
}

unsafe fn dcn3_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let clk_mgr = TO_CLK_MGR_INTERNAL!(clk_mgr_base);
    let new_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let dc = (*(*clk_mgr_base).ctx).dc;
    let mut update_dppclk = false; let mut update_dispclk = false; let mut enter_display_off = false;
    let mut dpp_clock_lowered = false; let mut update_pstate_unsupported_clk = false;
    let dmcu = (*(*(*clk_mgr_base).ctx).dc).res_pool.dmcu; let mut force_reset = false; let mut update_uclk = false;
    if (*dc).work_arounds.skip_clock_update || !(*clk_mgr).smu_present { return; }
    if (*clk_mgr_base).clks.dispclk_khz == 0 || ((*dc).debug.force_clock_mode & 1) != 0 {
        force_reset = true; dcn2_read_clocks_from_hw_dentist(clk_mgr_base);
    }
    let display_count = clk_mgr_helper_get_active_display_cnt(dc, context);
    if display_count == 0 { enter_display_off = true; }
    if enter_display_off == safe_to_lower { dcn30_smu_set_num_of_displays(clk_mgr, display_count); }
    if (*dc).debug.force_min_dcfclk_mhz > 0 { let v = (*dc).debug.force_min_dcfclk_mhz * 1000; if new_clocks.dcfclk_khz < v { new_clocks.dcfclk_khz = v; } }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_khz, (*clk_mgr_base).clks.dcfclk_khz) { (*clk_mgr_base).clks.dcfclk_khz = new_clocks.dcfclk_khz; dcn30_smu_set_hard_min_by_freq(clk_mgr, PPCLK_DCEFCLK, khz_to_mhz_ceil((*clk_mgr_base).clks.dcfclk_khz) as u16); }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_deep_sleep_khz, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz) { (*clk_mgr_base).clks.dcfclk_deep_sleep_khz = new_clocks.dcfclk_deep_sleep_khz; dcn30_smu_set_min_deep_sleep_dcef_clk(clk_mgr, khz_to_mhz_ceil((*clk_mgr_base).clks.dcfclk_deep_sleep_khz)); }
    if should_set_clock(safe_to_lower, new_clocks.socclk_khz, (*clk_mgr_base).clks.socclk_khz) { (*clk_mgr_base).clks.socclk_khz = new_clocks.socclk_khz; }
    (*clk_mgr_base).clks.prev_p_state_change_support = (*clk_mgr_base).clks.p_state_change_support;
    let p_state_change_support = new_clocks.p_state_change_support;
    if (*dc).clk_mgr.dc_mode_softmax_enabled && safe_to_lower && !p_state_change_support { let soft = (*dc).clk_mgr.bw_params.dc_mode_softmax_memclk * 1000; if (new_clocks.dramclk_khz <= soft) != ((*clk_mgr_base).clks.dramclk_khz <= soft) { update_pstate_unsupported_clk = true; } }
    if should_update_pstate_support(safe_to_lower, p_state_change_support, (*clk_mgr_base).clks.p_state_change_support) || update_pstate_unsupported_clk { (*clk_mgr_base).clks.p_state_change_support = p_state_change_support; if !(*clk_mgr_base).clks.p_state_change_support { let f = if (*dc).clk_mgr.dc_mode_softmax_enabled && new_clocks.dramclk_khz <= ((*dc).clk_mgr.bw_params.dc_mode_softmax_memclk * 1000) as i32 { (*dc).clk_mgr.bw_params.dc_mode_softmax_memclk } else { (*clk_mgr_base).bw_params.clk_table.entries[(*clk_mgr_base).bw_params.clk_table.num_entries - 1].memclk_mhz }; dcn30_smu_set_hard_min_by_freq(clk_mgr, PPCLK_UCLK, f as u16); } }
    if should_set_clock(safe_to_lower, new_clocks.dramclk_khz, (*clk_mgr_base).clks.dramclk_khz) { (*clk_mgr_base).clks.dramclk_khz = new_clocks.dramclk_khz; update_uclk = true; }
    if (*clk_mgr_base).clks.p_state_change_support && (update_uclk || !(*clk_mgr_base).clks.prev_p_state_change_support) { dcn30_smu_set_hard_min_by_freq(clk_mgr, PPCLK_UCLK, khz_to_mhz_ceil((*clk_mgr_base).clks.dramclk_khz) as u16); }
    if should_set_clock(safe_to_lower, new_clocks.dppclk_khz, (*clk_mgr_base).clks.dppclk_khz) { if (*clk_mgr_base).clks.dppclk_khz > new_clocks.dppclk_khz { dpp_clock_lowered = true; } (*clk_mgr_base).clks.dppclk_khz = new_clocks.dppclk_khz; dcn30_smu_set_hard_min_by_freq(clk_mgr, PPCLK_PIXCLK, khz_to_mhz_ceil((*clk_mgr_base).clks.dppclk_khz) as u16); update_dppclk = true; }
    if should_set_clock(safe_to_lower, new_clocks.dispclk_khz, (*clk_mgr_base).clks.dispclk_khz) { (*clk_mgr_base).clks.dispclk_khz = new_clocks.dispclk_khz; dcn30_smu_set_hard_min_by_freq(clk_mgr, PPCLK_DISPCLK, khz_to_mhz_ceil((*clk_mgr_base).clks.dispclk_khz) as u16); update_dispclk = true; }
    if !(*dc).config.forced_clocks || (force_reset && safe_to_lower) { if dpp_clock_lowered { dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); dcn20_update_clocks_update_dentist(clk_mgr, context); } else { if update_dppclk || update_dispclk { dcn20_update_clocks_update_dentist(clk_mgr, context); } dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); } }
    if update_dispclk && !dmcu.is_null() && (*dmcu).funcs.is_dmcu_initialized(dmcu) { (*dmcu).funcs.set_psr_wait_loop(dmcu, (*clk_mgr_base).clks.dispclk_khz / 1000 / 7); }
}

unsafe fn dcn3_notify_wm_ranges(clk_mgr_base: *mut clk_mgr) { let clk_mgr = TO_CLK_MGR_INTERNAL!(clk_mgr_base); if !(*clk_mgr).smu_present { return; } let table = (*clk_mgr).wm_range_table as *mut WatermarksExternal_t; if table.is_null() { return; } core::ptr::write_bytes(table as *mut u8, 0, core::mem::size_of::<WatermarksExternal_t>()); for i in 0..WM_SET_COUNT { let e = &(*clk_mgr).base.bw_params.wm_table.nv_entries[i]; if e.valid { (*table).Watermarks.WatermarkRow[WM_DCEFCLK][i].MinClock=e.pmfw_breakdown.min_dcfclk; (*table).Watermarks.WatermarkRow[WM_DCEFCLK][i].MaxClock=e.pmfw_breakdown.max_dcfclk; (*table).Watermarks.WatermarkRow[WM_DCEFCLK][i].MinUclk=e.pmfw_breakdown.min_uclk; (*table).Watermarks.WatermarkRow[WM_DCEFCLK][i].MaxUclk=e.pmfw_breakdown.max_uclk; (*table).Watermarks.WatermarkRow[WM_DCEFCLK][i].WmSetting=i; (*table).Watermarks.WatermarkRow[WM_DCEFCLK][i].Flags=e.pmfw_breakdown.wm_type; } } dcn30_smu_set_dram_addr_high(clk_mgr, (*clk_mgr).wm_range_table_addr >> 32); dcn30_smu_set_dram_addr_low(clk_mgr, (*clk_mgr).wm_range_table_addr & 0xffff_ffff); dcn30_smu_transfer_wm_table_dram_2_smu(clk_mgr); }

unsafe fn dcn3_set_hard_min_memclk(b: *mut clk_mgr, current_mode: bool) { let c=TO_CLK_MGR_INTERNAL!(b); if !(*c).smu_present{return;} let f=if current_mode && (*b).clks.p_state_change_support {khz_to_mhz_ceil((*b).clks.dramclk_khz) as u16} else if current_mode {(*b).bw_params.clk_table.entries[(*b).bw_params.clk_table.num_entries-1].memclk_mhz} else {(*b).bw_params.clk_table.entries[0].memclk_mhz}; dcn30_smu_set_hard_min_by_freq(c,PPCLK_UCLK,f); }
unsafe fn dcn3_set_hard_max_memclk(b:*mut clk_mgr){let c=TO_CLK_MGR_INTERNAL!(b);if (*c).smu_present{dcn30_smu_set_hard_max_by_freq(c,PPCLK_UCLK,(*b).bw_params.clk_table.entries[(*b).bw_params.clk_table.num_entries-1].memclk_mhz as u16);}}
unsafe fn dcn3_set_max_memclk(b:*mut clk_mgr,m:u32){let c=TO_CLK_MGR_INTERNAL!(b);if (*c).smu_present{dcn30_smu_set_hard_max_by_freq(c,PPCLK_UCLK,m as u16);}}
unsafe fn dcn3_set_min_memclk(b:*mut clk_mgr,m:u32){let c=TO_CLK_MGR_INTERNAL!(b);if (*c).smu_present{dcn30_smu_set_hard_min_by_freq(c,PPCLK_UCLK,m as u16);}}
unsafe fn dcn3_get_memclk_states_from_smu(b:*mut clk_mgr){let c=TO_CLK_MGR_INTERNAL!(b);if !(*c).smu_present{return;}let mut n=0;dcn3_init_single_clock(c,PPCLK_UCLK,&mut (*b).bw_params.clk_table.entries[0].memclk_mhz,&mut n);(*b).bw_params.clk_table.num_entries=if n!=0{n as usize}else{1};(*b).bw_params.dc_mode_softmax_memclk=dcn30_smu_get_dc_mode_max_dpm_freq(c,PPCLK_UCLK);(*(*b).ctx).dc.res_pool.funcs.update_bw_bounding_box((*c).base.ctx.dc,(*b).bw_params);}
unsafe fn dcn3_is_smu_present(b:*mut clk_mgr)->bool{(*TO_CLK_MGR_INTERNAL!(b)).smu_present}
unsafe fn dcn3_are_clock_states_equal(a:*mut dc_clocks,b:*mut dc_clocks)->bool{(*a).dispclk_khz==(*b).dispclk_khz&&(*a).dppclk_khz==(*b).dppclk_khz&&(*a).dcfclk_khz==(*b).dcfclk_khz&&(*a).dcfclk_deep_sleep_khz==(*b).dcfclk_deep_sleep_khz&&(*a).dramclk_khz==(*b).dramclk_khz&&(*a).p_state_change_support==(*b).p_state_change_support}
unsafe fn dcn3_enable_pme_wa(b:*mut clk_mgr){let c=TO_CLK_MGR_INTERNAL!(b);if (*c).smu_present{dcn30_smu_set_pme_workaround(c);}}

unsafe fn dcn30_notify_link_rate_change(b:*mut clk_mgr,link:*mut dc_link){let c=TO_CLK_MGR_INTERNAL!(b);let mut max=(*b).bw_params.clk_table.entries[0].phyclk_mhz*1000;if !(*c).smu_present{return;}(*c).cur_phyclk_req_table[(*link).link_index]=(*link).cur_link_settings.link_rate*LINK_RATE_REF_FREQ_IN_KHZ;for i in 0..MAX_LINKS{if (*c).cur_phyclk_req_table[i]>max{max=(*c).cur_phyclk_req_table[i];}}if max!=(*b).clks.phyclk_khz{(*b).clks.phyclk_khz=max;dcn30_smu_set_hard_min_by_freq(c,PPCLK_PHYCLK,khz_to_mhz_ceil(max) as u16);}}

// Function tables preserve the C interface; field types and callbacks are supplied by dependencies.
static mut DCN3_FUNCS: clk_mgr_funcs = clk_mgr_funcs { get_dp_ref_clk_frequency:dce12_get_dp_ref_freq_khz, update_clocks:dcn3_update_clocks, init_clocks:dcn3_init_clocks, notify_wm_ranges:dcn3_notify_wm_ranges, set_hard_min_memclk:dcn3_set_hard_min_memclk, set_hard_max_memclk:dcn3_set_hard_max_memclk, set_max_memclk:dcn3_set_max_memclk, set_min_memclk:dcn3_set_min_memclk, get_memclk_states_from_smu:dcn3_get_memclk_states_from_smu, are_clock_states_equal:dcn3_are_clock_states_equal, enable_pme_wa:dcn3_enable_pme_wa, notify_link_rate_change:dcn30_notify_link_rate_change, is_smu_present:dcn3_is_smu_present, set_smartmux_switch:dcn30m_set_smartmux_switch };
unsafe fn dcn3_init_clocks_fpga(c:*mut clk_mgr){dcn2_init_clocks(c);}
static mut DCN3_FPGA_FUNCS: clk_mgr_funcs = clk_mgr_funcs {get_dp_ref_clk_frequency:dce12_get_dp_ref_freq_khz,update_clocks:dcn2_update_clocks_fpga,init_clocks:dcn3_init_clocks_fpga};

pub unsafe fn dcn3_clk_mgr_construct(ctx:*mut dc_context, c:*mut clk_mgr_internal, pp_smu:*mut pp_smu_funcs, dccg:*mut dccg){let _=pp_smu;(*c).base.ctx=ctx;(*c).base.funcs=&mut DCN3_FUNCS;(*c).regs=&CLK_MGR_REGS;(*c).clk_mgr_shift=&CLK_MGR_SHIFT;(*c).clk_mgr_mask=&CLK_MGR_MASK;(*c).dccg=dccg;(*c).dfs_bypass_disp_clk=0;(*c).dprefclk_ss_percentage=0;(*c).dprefclk_ss_divider=1000;(*c).ss_on_dprefclk=false;(*c).dfs_ref_freq_khz=100000;(*c).base.dprefclk_khz=730000;(*c).base.dentist_vco_freq_khz=dcn30_get_vco_frequency_from_reg(c);if (*c).base.dentist_vco_freq_khz==0{(*c).base.dentist_vco_freq_khz=3650000;}(*c).dfs_bypass_enabled=false;(*c).smu_present=false;dce_clock_read_ss_info(c);(*c).base.bw_params=kzalloc_obj!((*c).base.bw_params);if (*c).base.bw_params.is_null(){BREAK_TO_DEBUGGER!();return;}(*c).wm_range_table=dm_helpers_allocate_gpu_mem((*c).base.ctx,DC_MEM_ALLOC_TYPE_GART,core::mem::size_of::<WatermarksExternal_t>(),&mut (*c).wm_range_table_addr);if (*c).wm_range_table.is_null(){BREAK_TO_DEBUGGER!();}}
pub unsafe fn dcn3_clk_mgr_destroy(c:*mut clk_mgr_internal){kfree((*c).base.bw_params);if !(*c).wm_range_table.is_null(){dm_helpers_free_gpu_mem((*c).base.ctx,DC_MEM_ALLOC_TYPE_GART,(*c).wm_range_table);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
