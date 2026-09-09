// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// C headers and build-time register/helper macros are supplied by dependencies.

unsafe fn update_dsc_on_stream(pipe_ctx: *mut pipe_ctx, enable: bool) {
    let dsc = (*pipe_ctx).stream_res.dsc;
    let stream = (*pipe_ctx).stream;
    let mut odm_pipe = (*pipe_ctx).next_odm_pipe;
    let mut opp_cnt = 1;
    assert!(!dsc.is_null());
    while !odm_pipe.is_null() { opp_cnt += 1; odm_pipe = (*odm_pipe).next_odm_pipe; }
    if enable {
        let mut dsc_cfg: dsc_config = core::mem::zeroed();
        let mut dsc_optc_cfg: dsc_optc_config = core::mem::zeroed();
        let mut dsc_state: dcn_dsc_state = core::mem::zeroed();
        if dsc.is_null() { return; }
        if (*dsc).funcs.dsc_read_state.is_some() {
            ((*dsc).funcs.dsc_read_state.unwrap())(dsc, &mut dsc_state);
            if !dsc_state.dsc_fw_en { return; }
        }
        dsc_cfg.pic_width = ((*stream).timing.h_addressable + (*stream).timing.h_border_left + (*stream).timing.h_border_right) / opp_cnt;
        dsc_cfg.pic_height = (*stream).timing.v_addressable + (*stream).timing.v_border_top + (*stream).timing.v_border_bottom;
        dsc_cfg.pixel_encoding = (*stream).timing.pixel_encoding;
        dsc_cfg.color_depth = (*stream).timing.display_color_depth;
        dsc_cfg.is_odm = !(*pipe_ctx).next_odm_pipe.is_null();
        dsc_cfg.dc_dsc_cfg = (*stream).timing.dsc_cfg;
        assert!(dsc_cfg.dc_dsc_cfg.num_slices_h % opp_cnt == 0);
        dsc_cfg.dc_dsc_cfg.num_slices_h /= opp_cnt;
        dsc_cfg.dsc_padding = 0;
        ((*dsc).funcs.dsc_set_config)(dsc, &dsc_cfg, &mut dsc_optc_cfg);
        ((*dsc).funcs.dsc_enable)(dsc, (*pipe_ctx).stream_res.opp.inst);
        odm_pipe = (*pipe_ctx).next_odm_pipe;
        while !odm_pipe.is_null() {
            let odm_dsc = (*odm_pipe).stream_res.dsc;
            assert!(!odm_dsc.is_null());
            ((*odm_dsc).funcs.dsc_set_config)(odm_dsc, &dsc_cfg, &mut dsc_optc_cfg);
            ((*odm_dsc).funcs.dsc_enable)(odm_dsc, (*odm_pipe).stream_res.opp.inst);
            odm_pipe = (*odm_pipe).next_odm_pipe;
        }
        dsc_cfg.dc_dsc_cfg.num_slices_h *= opp_cnt;
        dsc_cfg.pic_width *= opp_cnt;
        let mode = if dsc_optc_cfg.is_pixel_format_444 { OPTC_DSC_ENABLED_444 } else { OPTC_DSC_ENABLED_NATIVE_SUBSAMPLED };
        ((*pipe_ctx).stream_res.tg.funcs.set_dsc_config)((*pipe_ctx).stream_res.tg, mode, dsc_optc_cfg.bytes_per_pixel, dsc_optc_cfg.slice_width);
    } else {
        ((*pipe_ctx).stream_res.tg.funcs.set_dsc_config)((*pipe_ctx).stream_res.tg, OPTC_DSC_DISABLED, 0, 0);
        ((*dsc).funcs.dsc_disable)(dsc);
        odm_pipe = (*pipe_ctx).next_odm_pipe;
        while !odm_pipe.is_null() { let odm_dsc = (*odm_pipe).stream_res.dsc; assert!(!odm_dsc.is_null()); ((*odm_dsc).funcs.dsc_disable)(odm_dsc); odm_pipe = (*odm_pipe).next_odm_pipe; }
    }
}

unsafe fn get_odm_config(pipe_ctx: *mut pipe_ctx, opp_instances: *mut i32) -> u32 {
    let mut count = 1; let mut p = pipe_ctx;
    while !(*p).prev_odm_pipe.is_null() { p = (*p).prev_odm_pipe; }
    if !opp_instances.is_null() { *opp_instances = (*p).stream_res.opp.inst; }
    p = (*p).next_odm_pipe;
    while !p.is_null() { if !opp_instances.is_null() { *opp_instances.add(count as usize) = (*p).stream_res.opp.inst; } count += 1; p = (*p).next_odm_pipe; }
    count
}

pub unsafe fn dcn314_update_odm(dc: *mut dc, context: *mut dc_state, pipe_ctx: *mut pipe_ctx) {
    let _ = context; let mut opp_inst = [0i32; MAX_PIPES as usize];
    let count = get_odm_config(pipe_ctx, opp_inst.as_mut_ptr());
    let sw = resource_get_odm_slice_dst_width(pipe_ctx, false); let last = resource_get_odm_slice_dst_width(pipe_ctx, true);
    if count > 1 { ((*pipe_ctx).stream_res.tg.funcs.set_odm_combine)((*pipe_ctx).stream_res.tg, opp_inst.as_mut_ptr(), count as i32, sw, last); }
    else { ((*pipe_ctx).stream_res.tg.funcs.set_odm_bypass)((*pipe_ctx).stream_res.tg, &(*pipe_ctx).stream.timing); }
    let mpc = (*dc).res_pool.mpc;
    if (*mpc).funcs.set_out_rate_control.is_some() { for i in 0..count as usize { ((*mpc).funcs.set_out_rate_control.unwrap())(mpc, opp_inst[i], false, 0, core::ptr::null_mut()); } }
    let mut p = (*pipe_ctx).next_odm_pipe; while !p.is_null() { ((*p).stream_res.opp.funcs.opp_pipe_clock_control)((*p).stream_res.opp, true); p = (*p).next_odm_pipe; }
    if !(*pipe_ctx).stream_res.dsc.is_null() { update_dsc_on_stream(pipe_ctx, (*pipe_ctx).stream.timing.flags.DSC != 0); }
}

pub unsafe fn dcn314_dsc_pg_control(hws: *mut dce_hwseq, dsc_inst: u32, power_on: bool) {
    let power_gate = if power_on { 0 } else { 1 }; let pwr_status = if power_on { 0 } else { 2 };
    if !(*hws).ctx.dc.debug.root_clock_optimization.bits.dsc && false { return; }
    if !(*hws).ctx.dc.debug.disable_dsc_power_gate { match dsc_inst { 0 => { REG_UPDATE!(hws, DOMAIN16_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN16_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }, 1 => { REG_UPDATE!(hws, DOMAIN17_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN17_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }, 2 => { REG_UPDATE!(hws, DOMAIN18_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN18_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }, 3 => { REG_UPDATE!(hws, DOMAIN19_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN19_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }, _ => BREAK_TO_DEBUGGER!(), } }
}

pub unsafe fn dcn314_enable_power_gating_plane(hws: *mut dce_hwseq, enable: bool) {
    let force_on = !(enable && !(*hws).ctx.dc.debug.disable_hubp_power_gate);
    REG_UPDATE!(hws, DOMAIN0_PG_CONFIG, DOMAIN_POWER_FORCEON, force_on); REG_UPDATE!(hws, DOMAIN2_PG_CONFIG, DOMAIN_POWER_FORCEON, force_on); REG_UPDATE!(hws, DOMAIN1_PG_CONFIG, DOMAIN_POWER_FORCEON, force_on); REG_UPDATE!(hws, DOMAIN3_PG_CONFIG, DOMAIN_POWER_FORCEON, force_on);
    let force_on = !(enable && !(*hws).ctx.dc.debug.disable_dsc_power_gate);
    REG_UPDATE!(hws, DOMAIN16_PG_CONFIG, DOMAIN_POWER_FORCEON, force_on); REG_UPDATE!(hws, DOMAIN17_PG_CONFIG, DOMAIN_POWER_FORCEON, force_on); REG_UPDATE!(hws, DOMAIN18_PG_CONFIG, DOMAIN_POWER_FORCEON, force_on); REG_UPDATE!(hws, DOMAIN19_PG_CONFIG, DOMAIN_POWER_FORCEON, force_on);
}

pub unsafe fn dcn314_calculate_dccg_k1_k2_values(pipe_ctx: *mut pipe_ctx, k1_div: *mut u32, k2_div: *mut u32) -> u32 {
    let stream = (*pipe_ctx).stream; let odm = get_odm_config(pipe_ctx, core::ptr::null_mut()); let two = ((*pipe_ctx).stream_res.tg.funcs.is_two_pixels_per_container)(&(*stream).timing);
    if dc_is_hdmi_frl_signal((*stream).signal) || (*stream).ctx.dc.link_srv.dp_is_128b_132b_signal(pipe_ctx) { *k1_div=PIXEL_RATE_DIV_BY_1; *k2_div=PIXEL_RATE_DIV_BY_1; }
    else if dc_is_hdmi_tmds_signal((*stream).signal) || dc_is_dvi_signal((*stream).signal) { *k1_div=PIXEL_RATE_DIV_BY_1; *k2_div=if (*stream).timing.pixel_encoding==PIXEL_ENCODING_YCBCR420 {PIXEL_RATE_DIV_BY_2} else {PIXEL_RATE_DIV_BY_4}; }
    else if dc_is_dp_signal((*stream).signal) || dc_is_virtual_signal((*stream).signal) { *k1_div=PIXEL_RATE_DIV_BY_1; *k2_div=if two {PIXEL_RATE_DIV_BY_2} else if odm==2 {PIXEL_RATE_DIV_BY_2} else {PIXEL_RATE_DIV_BY_4}; }
    odm
}

pub unsafe fn dcn314_calculate_pix_rate_divider(dc: *mut dc, context: *mut dc_state, stream: *const dc_stream_state) { let p=resource_get_otg_master_for_stream(&mut (*context).res_ctx, stream); if !p.is_null() { let mut a=PIXEL_RATE_DIV_NA; let mut b=PIXEL_RATE_DIV_NA; if (*(*dc).hwseq).funcs.calculate_dccg_k1_k2_values.is_some() { ((*(*dc).hwseq).funcs.calculate_dccg_k1_k2_values.unwrap())(p,&mut a,&mut b); } (*p).pixel_rate_divider.div_factor1=a; (*p).pixel_rate_divider.div_factor2=b; } }

unsafe fn dcn314_is_pipe_dig_fifo_on(pipe: *mut pipe_ctx) -> bool { !pipe.is_null() && !(*pipe).stream.is_null() && !(*pipe).stream_res.stream_enc.is_null() && ((*pipe).stream_res.stream_enc.funcs.dig_source_otg)((*pipe).stream_res.stream_enc)==(*pipe).stream_res.tg.inst && !(*pipe).stream.link.is_null() && !(*pipe).stream.link.link_enc.is_null() && ((*pipe).stream.link.link_enc.funcs.is_dig_enabled)((*pipe).stream.link.link_enc.link_enc) && ((*pipe).stream_res.stream_enc.funcs.is_fifo_enabled)((*pipe).stream_res.stream_enc) }

// Remaining hardware sequencing is expressed directly through the supplied C-compatible
// structures and register helper macros.
pub unsafe fn dcn314_dpp_root_clock_control(hws: *mut dce_hwseq, dpp_inst: u32, clock_on: bool) { if (*hws).ctx.dc.debug.root_clock_optimization.bits.dpp { if (*hws).ctx.dc.res_pool.dccg.funcs.dpp_root_clock_control.is_some() { ((*hws).ctx.dc.res_pool.dccg.funcs.dpp_root_clock_control.unwrap())((*hws).ctx.dc.res_pool.dccg,dpp_inst,clock_on); } } }

pub unsafe fn dcn314_resync_fifo_dccg_dio(hws: *mut dce_hwseq, dc: *mut dc, context: *mut dc_state, current_pipe_idx: u32) {
    let mut disabled = [false; MAX_PIPES as usize];
    for i in 0..(*dc).res_pool.pipe_count as usize { let p=if i as u32<=current_pipe_idx { &mut (*context).res_ctx.pipe_ctx[i] } else { &mut (*(*dc).current_state).res_ctx.pipe_ctx[i] }; if !p.top_pipe.is_null() || !p.prev_odm_pipe.is_null() { continue; } if !p.stream.is_null() && ((*p.stream).dpms_off || dc_is_virtual_signal((*p.stream).signal)) && !(*p.stream).apply_seamless_boot_optimization && !(*p.stream).apply_edp_fast_boot_optimization { if dcn314_is_pipe_dig_fifo_on(p) { continue; } ((*p.stream_res.tg.funcs.disable_crtc))(p.stream_res.tg); reset_sync_context_for_pipe(dc,context,i as u8); disabled[i]=true; } }
    ((*(*hws).ctx.dc.res_pool.dccg).funcs.trigger_dio_fifo_resync)((*hws).ctx.dc.res_pool.dccg);
    for i in 0..(*dc).res_pool.pipe_count as usize { let p=if i as u32<=current_pipe_idx { &mut (*context).res_ctx.pipe_ctx[i] } else { &mut (*(*dc).current_state).res_ctx.pipe_ctx[i] }; if disabled[i] { ((*p.stream_res.tg.funcs.enable_crtc))(p.stream_res.tg); } }
}

unsafe fn apply_symclk_on_tx_off_wa(link: *mut dc_link) {
    if (*link).phy_state.symclk_ref_cnts.otg > 0 { for i in 0..MAX_PIPES as usize { let p=&mut (*(*link).ctx.dc.current_state).res_ctx.pipe_ctx[i]; if !p.stream.is_null() && (*p.stream).link==link && p.top_pipe.is_null() { ((*p.clock_source.funcs.program_pix_clk))(p.clock_source,&p.stream_res.pix_clk_params,(*link).ctx.dc.link_srv.dp_get_encoding_format(&p.link_config.dp_link_settings),&p.pll_settings); (*link).phy_state.symclk_state=SYMCLK_ON_TX_OFF; break; } } }
}

pub unsafe fn dcn314_disable_link_output(link: *mut dc_link, link_res: *const link_resource, signal: signal_type) {
    let dc=(*link).ctx.dc; let hwss=get_link_hwss(link,link_res); let dmcu=(*dc).res_pool.dmcu;
    if signal==SIGNAL_TYPE_EDP && (*link).dc.hwss.edp_backlight_control.is_some() && !(*link).skip_implict_edp_power_control { ((*link).dc.hwss.edp_backlight_control.unwrap())(link,false); } else if !dmcu.is_null() && (*dmcu).funcs.lock_phy.is_some() { ((*dmcu).funcs.lock_phy.unwrap())(dmcu); }
    ((*hwss).disable_link_output)(link,link_res,signal); (*link).phy_state.symclk_state=SYMCLK_OFF_TX_OFF;
    if !dmcu.is_null() && (*dmcu).funcs.unlock_phy.is_some() { ((*dmcu).funcs.unlock_phy.unwrap())(dmcu); }
    ((*dc).link_srv.dp_trace_source_sequence)(link,DPCD_SOURCE_SEQ_AFTER_DISABLE_LINK_PHY); apply_symclk_on_tx_off_wa(link);
}

pub unsafe fn dcn314_dpp_pg_control(hws: *mut dce_hwseq, dpp_inst: u32, power_on: bool) {
    let power_gate=if power_on {0} else {1}; let status=if power_on {0} else {2};
    if (*hws).ctx.dc.debug.disable_dpp_power_gate { if !power_on { let d=(*hws).ctx.dc.res_pool.dpps[dpp_inst as usize]; if !d.is_null() && (*d).funcs.dpp_force_disable_cursor.is_some() { ((*d).funcs.dpp_force_disable_cursor.unwrap())(d); } } return; }
    if REG!(hws,DOMAIN1_PG_CONFIG)==0 { return; }
    match dpp_inst { 0=>{REG_UPDATE!(hws,DOMAIN1_PG_CONFIG,DOMAIN1_POWER_GATE,power_gate);REG_WAIT!(hws,DOMAIN1_PG_STATUS,DOMAIN1_PGFSM_PWR_STATUS,status,1,1000);},1=>{REG_UPDATE!(hws,DOMAIN3_PG_CONFIG,DOMAIN3_POWER_GATE,power_gate);REG_WAIT!(hws,DOMAIN3_PG_STATUS,DOMAIN3_PGFSM_PWR_STATUS,status,1,1000);},2=>{REG_UPDATE!(hws,DOMAIN5_PG_CONFIG,DOMAIN5_POWER_GATE,power_gate);REG_WAIT!(hws,DOMAIN5_PG_STATUS,DOMAIN5_PGFSM_PWR_STATUS,status,1,1000);},3=>{REG_UPDATE!(hws,DOMAIN7_PG_CONFIG,DOMAIN7_POWER_GATE,power_gate);REG_WAIT!(hws,DOMAIN7_PG_STATUS,DOMAIN7_PGFSM_PWR_STATUS,status,1,1000);},_=>BREAK_TO_DEBUGGER!() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
