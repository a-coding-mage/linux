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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C includes and macro-generated register lists are supplied by other translation units.

static CLK_MGR_REGISTERS: clk_mgr_registers = CLK_REG_LIST_NV10!();
static CLK_MGR_SHIFT: clk_mgr_shift = CLK_MASK_SH_LIST_NV10!(__SHIFT);
static CLK_MGR_MASK: clk_mgr_mask = CLK_MASK_SH_LIST_NV10!(_MASK);

pub unsafe fn dentist_get_did_from_divider(divider: i32) -> u32 {
    let mut divider_id: u32;
    if divider < DENTIST_DIVIDER_RANGE_2_START {
        if divider < DENTIST_DIVIDER_RANGE_1_START {
            divider_id = DENTIST_BASE_DID_1;
        } else {
            divider_id = DENTIST_BASE_DID_1
                + ((divider - DENTIST_DIVIDER_RANGE_1_START) / DENTIST_DIVIDER_RANGE_1_STEP) as u32;
        }
    } else if divider < DENTIST_DIVIDER_RANGE_3_START {
        divider_id = DENTIST_BASE_DID_2
            + ((divider - DENTIST_DIVIDER_RANGE_2_START) / DENTIST_DIVIDER_RANGE_2_STEP) as u32;
    } else if divider < DENTIST_DIVIDER_RANGE_4_START {
        divider_id = DENTIST_BASE_DID_3
            + ((divider - DENTIST_DIVIDER_RANGE_3_START) / DENTIST_DIVIDER_RANGE_3_STEP) as u32;
    } else {
        divider_id = DENTIST_BASE_DID_4
            + ((divider - DENTIST_DIVIDER_RANGE_4_START) / DENTIST_DIVIDER_RANGE_4_STEP) as u32;
        if divider_id > DENTIST_MAX_DID { divider_id = DENTIST_MAX_DID; }
    }
    divider_id
}

pub unsafe fn dcn20_update_clocks_update_dpp_dto(clk_mgr: *mut clk_mgr_internal, context: *mut dc_state, safe_to_lower: bool) {
    (*(*clk_mgr).dccg).ref_dppclk = (*clk_mgr).base.clks.dppclk_khz;
    for i in 0..(*(*(*clk_mgr).base.ctx).dc).res_pool.pipe_count {
        let dppclk_khz = (*context).res_ctx.pipe_ctx[i as usize].plane_res.bw.dppclk_khz;
        let prev = (*(*clk_mgr).dccg).pipe_dppclk_khz[i as usize];
        if safe_to_lower || prev < dppclk_khz {
            ((*(*(*clk_mgr).dccg).funcs).update_dpp_dto)((*clk_mgr).dccg, i as i32, dppclk_khz);
        }
    }
}

pub unsafe fn dcn20_update_clocks_update_dentist(clk_mgr: *mut clk_mgr_internal, context: *mut dc_state) {
    let mut dpp_divider = 0;
    let mut disp_divider = 0;
    if (*clk_mgr).base.clks.dppclk_khz == 0 || (*clk_mgr).base.clks.dispclk_khz == 0 { return; }
    dpp_divider = DENTIST_DIVIDER_RANGE_SCALE_FACTOR * (*clk_mgr).base.dentist_vco_freq_khz / (*clk_mgr).base.clks.dppclk_khz;
    disp_divider = DENTIST_DIVIDER_RANGE_SCALE_FACTOR * (*clk_mgr).base.dentist_vco_freq_khz / (*clk_mgr).base.clks.dispclk_khz;
    let dppclk_wdivider = dentist_get_did_from_divider(dpp_divider);
    let dispclk_wdivider = dentist_get_did_from_divider(disp_divider);
    let mut current_dispclk_wdivider = 0u32;
    REG_GET!(clk_mgr, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_WDIVIDER, &mut current_dispclk_wdivider);
    if current_dispclk_wdivider == 127 && dispclk_wdivider != 127 {
        for i in 0..(*(*(*clk_mgr).base.ctx).dc).res_pool.pipe_count {
            let pipe_ctx = &mut (*context).res_ctx.pipe_ctx[i as usize];
            if !resource_is_pipe_type(pipe_ctx, OTG_MASTER) { continue; }
            let stream_enc = pipe_ctx.stream_res.stream_enc;
            if (*(*stream_enc).funcs).get_fifo_cal_average_level.is_none() { continue; }
            let fifo_level = ((*(*stream_enc).funcs).get_fifo_cal_average_level.unwrap())(stream_enc);
            let n = fifo_level / 4;
            let dccg = (*(*(*clk_mgr).base.ctx).dc).res_pool.dccg;
            ((*(*dccg).funcs).set_fifo_errdet_ovr_en)(dccg, true);
            for _ in 0..(n - 4) { ((*(*dccg).funcs).otg_drop_pixel)(dccg, pipe_ctx.stream_res.tg.inst); }
            ((*(*dccg).funcs).set_fifo_errdet_ovr_en)(dccg, false);
        }
    } else if dispclk_wdivider == 127 && current_dispclk_wdivider != 127 {
        REG_UPDATE!(clk_mgr, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_WDIVIDER, 126);
        REG_WAIT!(clk_mgr, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_CHG_DONE, 1, 50, 2000);
        for i in 0..(*(*(*clk_mgr).base.ctx).dc).res_pool.pipe_count {
            let pipe_ctx = &mut (*context).res_ctx.pipe_ctx[i as usize];
            if !resource_is_pipe_type(pipe_ctx, OTG_MASTER) { continue; }
            let stream_enc = pipe_ctx.stream_res.stream_enc;
            if (*(*stream_enc).funcs).get_fifo_cal_average_level.is_none() { continue; }
            let n = ((*(*stream_enc).funcs).get_fifo_cal_average_level.unwrap())(stream_enc) / 4;
            let dccg = (*(*(*clk_mgr).base.ctx).dc).res_pool.dccg;
            ((*(*dccg).funcs).set_fifo_errdet_ovr_en)(dccg, true);
            for _ in 0..(12 - n) { ((*(*dccg).funcs).otg_add_pixel)(dccg, pipe_ctx.stream_res.tg.inst); }
            ((*(*dccg).funcs).set_fifo_errdet_ovr_en)(dccg, false);
        }
    }
    REG_UPDATE!(clk_mgr, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_WDIVIDER, dispclk_wdivider);
    REG_WAIT!(clk_mgr, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_CHG_DONE, 1, 50, 2000);
    REG_UPDATE!(clk_mgr, DENTIST_DISPCLK_CNTL, DENTIST_DPPCLK_WDIVIDER, dppclk_wdivider);
    REG_WAIT!(clk_mgr, DENTIST_DISPCLK_CNTL, DENTIST_DPPCLK_CHG_DONE, 1, 5, 100);
}

pub unsafe fn dcn2_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let clk_mgr = TO_CLK_MGR_INTERNAL!(clk_mgr_base);
    let new_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let dc = (*(*clk_mgr_base).ctx).dc;
    if (*dc).work_arounds.skip_clock_update { return; }
    let mut force_reset = false;
    if (*clk_mgr_base).clks.dispclk_khz == 0 || ((*dc).debug.force_clock_mode & 0x1) != 0 {
        force_reset = true;
        dcn2_read_clocks_from_hw_dentist(clk_mgr_base);
    }
    let display_count = clk_mgr_helper_get_active_display_cnt(dc, context);
    let mut pp_smu = core::ptr::null_mut();
    if !(*dc).res_pool.pp_smu.is_null() { pp_smu = &mut (*(*dc).res_pool.pp_smu).nv_funcs; }
    let enter_display_off = display_count == 0;
    if enter_display_off == safe_to_lower && !pp_smu.is_null() && (*pp_smu).set_display_count.is_some() { ((*pp_smu).set_display_count.unwrap())(&mut (*pp_smu).pp_smu, display_count); }
    if (*dc).debug.force_min_dcfclk_mhz > 0 { let f = (*dc).debug.force_min_dcfclk_mhz * 1000; if new_clocks.dcfclk_khz < f { new_clocks.dcfclk_khz = f; } }
    macro_rules! SET { ($field:ident, $smu:ident) => { if should_set_clock(safe_to_lower, new_clocks.$field, (*clk_mgr_base).clks.$field) { (*clk_mgr_base).clks.$field = new_clocks.$field; if !pp_smu.is_null() && (*pp_smu).$smu.is_some() { ((*pp_smu).$smu.unwrap())(&mut (*pp_smu).pp_smu, khz_to_mhz_ceil((*clk_mgr_base).clks.$field)); } } }; }
    SET!(dcfclk_khz, set_hard_min_dcfclk_by_freq); SET!(dcfclk_deep_sleep_khz, set_min_deep_sleep_dcfclk); SET!(socclk_khz, set_hard_min_socclk_by_freq); SET!(dramclk_khz, set_hard_min_uclk_by_freq);
    let total_plane_count = clk_mgr_helper_get_active_plane_cnt(dc, context);
    let pstate = new_clocks.p_state_change_support || total_plane_count == 0;
    if should_update_pstate_support(safe_to_lower, pstate, (*clk_mgr_base).clks.p_state_change_support) { (*clk_mgr_base).clks.prev_p_state_change_support = (*clk_mgr_base).clks.p_state_change_support; (*clk_mgr_base).clks.p_state_change_support = pstate; if !pp_smu.is_null() && (*pp_smu).set_pstate_handshake_support.is_some() { ((*pp_smu).set_pstate_handshake_support.unwrap())(&mut (*pp_smu).pp_smu, pstate); } }
    let mut dpp_lowered = false; let mut upd_dpp = false; let mut upd_disp = false;
    if should_set_clock(safe_to_lower, new_clocks.dppclk_khz, (*clk_mgr).base.clks.dppclk_khz) { dpp_lowered = (*clk_mgr).base.clks.dppclk_khz > new_clocks.dppclk_khz; (*clk_mgr).base.clks.dppclk_khz = new_clocks.dppclk_khz; upd_dpp = true; }
    if should_set_clock(safe_to_lower, new_clocks.dispclk_khz, (*clk_mgr_base).clks.dispclk_khz) { (*clk_mgr_base).clks.dispclk_khz = new_clocks.dispclk_khz; upd_disp = true; }
    if upd_dpp || upd_disp { new_clocks.disp_dpp_voltage_level_khz = if upd_disp && new_clocks.dispclk_khz > new_clocks.dppclk_khz { new_clocks.dispclk_khz } else { new_clocks.dppclk_khz }; (*clk_mgr_base).clks.disp_dpp_voltage_level_khz = new_clocks.disp_dpp_voltage_level_khz; if !pp_smu.is_null() && (*pp_smu).set_voltage_by_freq.is_some() { ((*pp_smu).set_voltage_by_freq.unwrap())(&mut (*pp_smu).pp_smu, PP_SMU_NV_DISPCLK, khz_to_mhz_ceil((*clk_mgr_base).clks.disp_dpp_voltage_level_khz)); } }
    if !(*dc).config.forced_clocks || (force_reset && safe_to_lower) { if dpp_lowered { dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); dcn20_update_clocks_update_dentist(clk_mgr, context); } else { if upd_dpp || upd_disp { dcn20_update_clocks_update_dentist(clk_mgr, context); } dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower); } }
    let dmcu = (*dc).res_pool.dmcu; if upd_disp && !dmcu.is_null() && ((*(*dmcu).funcs).is_dmcu_initialized)(dmcu) { ((*(*dmcu).funcs).set_psr_wait_loop)(dmcu, (*clk_mgr_base).clks.dispclk_khz / 1000 / 7); }
}

pub unsafe fn dcn2_update_clocks_fpga(clk_mgr: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let ci = TO_CLK_MGR_INTERNAL!(clk_mgr); let n = &(*context).bw_ctx.bw.dcn.clk; let f = if n.fclk_khz > 1200000 { n.fclk_khz } else { 1200000 };
    macro_rules! C { ($x:ident, $v:expr) => { if should_set_clock(safe_to_lower, $v, (*clk_mgr).clks.$x) { (*clk_mgr).clks.$x = $v; } }; }
    C!(phyclk_khz,n.phyclk_khz); C!(dcfclk_khz,n.dcfclk_khz); C!(dcfclk_deep_sleep_khz,n.dcfclk_deep_sleep_khz); C!(socclk_khz,n.socclk_khz); C!(dramclk_khz,n.dramclk_khz); C!(dppclk_khz,n.dppclk_khz); C!(fclk_khz,f); C!(dispclk_khz,n.dispclk_khz);
    if (*clk_mgr).clks.fclk_khz > (*clk_mgr).clks.dppclk_khz { (*clk_mgr).clks.dppclk_khz = (*clk_mgr).clks.fclk_khz; } if (*clk_mgr).clks.dppclk_khz > (*clk_mgr).clks.fclk_khz { (*clk_mgr).clks.fclk_khz = (*clk_mgr).clks.dppclk_khz; }
    (*ci).dccg.ref_dppclk = (*clk_mgr).clks.fclk_khz; (*clk_mgr).clks.dtbclk_en = false; dm_set_dcn_clocks((*clk_mgr).ctx, &mut (*clk_mgr).clks);
}

pub unsafe fn dcn2_init_clocks(clk_mgr: *mut clk_mgr) { core::ptr::write_bytes(&mut (*clk_mgr).clks, 0, 1); (*clk_mgr).clks.p_state_change_support = true; (*clk_mgr).clks.prev_p_state_change_support = true; }

unsafe fn dcn2_enable_pme_wa(clk_mgr_base: *mut clk_mgr) { let c = TO_CLK_MGR_INTERNAL!(clk_mgr_base); if !(*c).pp_smu.is_null() { let p = &mut (*(*c).pp_smu).nv_funcs; if p.set_pme_wa_enable.is_some() { (p.set_pme_wa_enable.unwrap())(&mut p.pp_smu); } } }

pub unsafe fn dcn2_read_clocks_from_hw_dentist(clk_mgr_base: *mut clk_mgr) { let c = TO_CLK_MGR_INTERNAL!(clk_mgr_base); let mut a=0; let mut b=0; REG_GET!(c,DENTIST_DISPCLK_CNTL,DENTIST_DISPCLK_WDIVIDER,&mut a); REG_GET!(c,DENTIST_DISPCLK_CNTL,DENTIST_DPPCLK_WDIVIDER,&mut b); let da=dentist_get_divider_from_did(a); let db=dentist_get_divider_from_did(b); if da != 0 && db != 0 { (*clk_mgr_base).clks.dispclk_khz = DENTIST_DIVIDER_RANGE_SCALE_FACTOR*(*c).base.dentist_vco_freq_khz/da; (*clk_mgr_base).clks.dppclk_khz = DENTIST_DIVIDER_RANGE_SCALE_FACTOR*(*c).base.dentist_vco_freq_khz/db; } }

pub unsafe fn dcn2_get_clock(clk_mgr: *mut clk_mgr, context: *mut dc_state, clock_type: dc_clock_type, clock_cfg: *mut dc_clock_config) { if clock_type == DC_CLOCK_TYPE_DISPCLK { (*clock_cfg).max_clock_khz=(*context).bw_ctx.bw.dcn.clk.max_supported_dispclk_khz; (*clock_cfg).min_clock_khz=DCN_MINIMUM_DISPCLK_Khz; (*clock_cfg).current_clock_khz=(*clk_mgr).clks.dispclk_khz; (*clock_cfg).bw_requirequired_clock_khz=(*context).bw_ctx.bw.dcn.clk.bw_dispclk_khz; } if clock_type == DC_CLOCK_TYPE_DPPCLK { (*clock_cfg).max_clock_khz=(*context).bw_ctx.bw.dcn.clk.max_supported_dppclk_khz; (*clock_cfg).min_clock_khz=DCN_MINIMUM_DPPCLK_Khz; (*clock_cfg).current_clock_khz=(*clk_mgr).clks.dppclk_khz; (*clock_cfg).bw_requirequired_clock_khz=(*context).bw_ctx.bw.dcn.clk.bw_dppclk_khz; } }

unsafe fn dcn2_are_clock_states_equal(a:*mut dc_clocks,b:*mut dc_clocks)->bool { (*a).dispclk_khz==(*b).dispclk_khz && (*a).dppclk_khz==(*b).dppclk_khz && (*a).disp_dpp_voltage_level_khz==(*b).disp_dpp_voltage_level_khz && (*a).dcfclk_khz==(*b).dcfclk_khz && (*a).socclk_khz==(*b).socclk_khz && (*a).dcfclk_deep_sleep_khz==(*b).dcfclk_deep_sleep_khz && (*a).dramclk_khz==(*b).dramclk_khz && (*a).p_state_change_support==(*b).p_state_change_support }

unsafe fn dcn2_notify_link_rate_change(c:*mut clk_mgr,l:*mut dc_link) { let i=TO_CLK_MGR_INTERNAL!(c); if (*i).pp_smu.is_null() || (*(*i).pp_smu).nv_funcs.set_voltage_by_freq.is_none() { return; } let p=&mut (*(*i).pp_smu).nv_funcs; (*i).cur_phyclk_req_table[(*l).link_index]=(*l).cur_link_settings.link_rate*LINK_RATE_REF_FREQ_IN_KHZ; let mut m=0; for x in 0..MAX_LINKS { if (*i).cur_phyclk_req_table[x]>m {m=(*i).cur_phyclk_req_table[x];} } if m!=(*c).clks.phyclk_khz {(*c).clks.phyclk_khz=m;(p.set_voltage_by_freq.unwrap())(&mut p.pp_smu,PP_SMU_NV_PHYCLK,khz_to_mhz_ceil(m));} }

static mut DCN2_FUNCS: clk_mgr_funcs = clk_mgr_funcs { get_dp_ref_clk_frequency:dce12_get_dp_ref_freq_khz, update_clocks:dcn2_update_clocks, init_clocks:dcn2_init_clocks, enable_pme_wa:dcn2_enable_pme_wa, get_clock:dcn2_get_clock, are_clock_states_equal:dcn2_are_clock_states_equal, notify_link_rate_change:dcn2_notify_link_rate_change };

pub unsafe fn dcn20_clk_mgr_construct(ctx:*mut dc_context, c:*mut clk_mgr_internal, p:*mut pp_smu_funcs, d:*mut dccg) { (*c).base.ctx=ctx; (*c).pp_smu=p; (*c).base.funcs=&mut DCN2_FUNCS; (*c).regs=&CLK_MGR_REGS; (*c).clk_mgr_shift=&CLK_MGR_SHIFT; (*c).clk_mgr_mask=&CLK_MGR_MASK; (*c).dccg=d; (*c).dfs_bypass_disp_clk=0; (*c).dprefclk_ss_percentage=0; (*c).dprefclk_ss_divider=1000; (*c).ss_on_dprefclk=false; (*c).base.dprefclk_khz=700000; let did=REG_READ!(c,CLK3_CLK2_DFS_CNTL); let div=dentist_get_divider_from_did(did); let req=REG_READ!(c,CLK3_CLK_PLL_REQ); let mut pll=dc_fixpt_from_int(req & (*c).clk_mgr_mask.FbMult_int); pll.value|=req & (*c).clk_mgr_mask.FbMult_frac; pll=dc_fixpt_mul_int(pll,100000); (*c).base.dentist_vco_freq_khz=dc_fixpt_floor(pll); if (*c).base.dentist_vco_freq_khz==0 {(*c).base.dentist_vco_freq_khz=3850000;} (*c).base.dprefclk_khz=DENTIST_DIVIDER_RANGE_SCALE_FACTOR*(*c).base.dentist_vco_freq_khz/div; (*c).dfs_bypass_enabled=false; dce_clock_read_ss_info(c); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
