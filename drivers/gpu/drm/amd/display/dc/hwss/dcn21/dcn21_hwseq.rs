/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// External driver declarations and register helpers are supplied by the surrounding crate.

/* Temporary read settings, future will get values from kmd directly */
unsafe fn mmhub_update_page_table_config(
    config: *mut dcn_hubbub_phys_addr_config,
    hws: *mut dce_hwseq,
) {
    let mut page_table_base_hi: u32 = 0;
    let mut page_table_base_lo: u32 = 0;

    REG_GET!(hws, VM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,
        PAGE_DIRECTORY_ENTRY_HI32, &mut page_table_base_hi);
    REG_GET!(hws, VM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,
        PAGE_DIRECTORY_ENTRY_LO32, &mut page_table_base_lo);

    (*config).gart_config.page_table_base_addr =
        ((page_table_base_hi as u64) << 32) | page_table_base_lo as u64;
}

pub unsafe fn dcn21_init_sys_ctx(
    hws: *mut dce_hwseq,
    dc: *mut dc,
    pa_config: *mut dc_phy_addr_space_config,
) -> i32 {
    let mut config: dcn_hubbub_phys_addr_config = core::mem::zeroed();

    config.system_aperture.fb_top = (*pa_config).system_aperture.fb_top;
    config.system_aperture.fb_offset = (*pa_config).system_aperture.fb_offset;
    config.system_aperture.fb_base = (*pa_config).system_aperture.fb_base;
    config.system_aperture.agp_top = (*pa_config).system_aperture.agp_top;
    config.system_aperture.agp_bot = (*pa_config).system_aperture.agp_bot;
    config.system_aperture.agp_base = (*pa_config).system_aperture.agp_base;
    config.gart_config.page_table_start_addr = (*pa_config).gart_config.page_table_start_addr;
    config.gart_config.page_table_end_addr = (*pa_config).gart_config.page_table_end_addr;
    config.gart_config.page_table_base_addr = (*pa_config).gart_config.page_table_base_addr;

    mmhub_update_page_table_config(&mut config, hws);
    ((*(*dc).res_pool).hubbub).funcs.init_dchub_sys_ctx(
        (*dc).res_pool.hubbub, &mut config)
}

// Work around for Renoir s0i3, if register is programmed, bypass golden init.
pub unsafe fn dcn21_s0i3_golden_init_wa(dc: *mut dc) -> bool {
    if !(*dc).res_pool.dccg.is_null()
        && !(*(*dc).res_pool).dccg.funcs.is_null()
        && (*(*(*dc).res_pool).dccg).funcs.is_s0i3_golden_init_wa_done.is_some()
    {
        return !((*(*(*dc).res_pool).dccg).funcs.is_s0i3_golden_init_wa_done)(
            (*dc).res_pool.dccg);
    }
    false
}

pub unsafe fn dcn21_exit_optimized_pwr_state(
    dc: *const dc,
    context: *mut dc_state,
) {
    ((*(*dc).clk_mgr).funcs.update_clocks)(
        (*dc).clk_mgr, context, false);
}

pub unsafe fn dcn21_optimize_pwr_state(
    dc: *const dc,
    context: *mut dc_state,
) {
    ((*(*dc).clk_mgr).funcs.update_clocks)(
        (*dc).clk_mgr, context, true);
}

/* If user hotplug a HDMI monitor while in monitor off, OS will do a mode set
 * (with output timing) but keep output off. */
pub unsafe fn dcn21_PLAT_58856_wa(
    context: *mut dc_state,
    pipe_ctx: *mut pipe_ctx,
) {
    if !(*(*pipe_ctx).stream).dpms_off { return; }
    (*(*pipe_ctx).stream).dpms_off = false;
    ((*(*(*pipe_ctx).stream).ctx).dc.link_srv).set_dpms_on(context, pipe_ctx);
    ((*(*(*pipe_ctx).stream).ctx).dc.link_srv).set_dpms_off(pipe_ctx);
    (*(*pipe_ctx).stream).dpms_off = true;
}

pub unsafe fn dcn21_dmub_cacp_set_pipe(
    abm: *mut abm, otg_inst: u32, option: u32, panel_inst: u32, pwrseq_inst: u32,
) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*abm).ctx;
    cmd.cacp_set_pipe.header.r#type = DMUB_CMD__CACP;
    cmd.cacp_set_pipe.header.sub_type = DMUB_CMD__CACP_SET_PIPE;
    cmd.cacp_set_pipe.cacp_set_pipe_data.otg_inst = otg_inst as u8;
    cmd.cacp_set_pipe.cacp_set_pipe_data.pwrseq_inst = pwrseq_inst as u8;
    cmd.cacp_set_pipe.cacp_set_pipe_data.set_pipe_option = option as u8;
    cmd.cacp_set_pipe.cacp_set_pipe_data.panel_inst = panel_inst as u8;
    cmd.cacp_set_pipe.header.payload_bytes = core::mem::size_of::<dmub_cmd_cacp_set_pipe_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dcn21_dmub_abm_set_pipe(
    abm: *mut abm, otg_inst: u32, option: u32, panel_inst: u32, pwrseq_inst: u32,
) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*abm).ctx;
    cmd.abm_set_pipe.header.r#type = DMUB_CMD__ABM;
    cmd.abm_set_pipe.header.sub_type = DMUB_CMD__ABM_SET_PIPE;
    cmd.abm_set_pipe.abm_set_pipe_data.otg_inst = otg_inst as u8;
    cmd.abm_set_pipe.abm_set_pipe_data.pwrseq_inst = pwrseq_inst as u8;
    cmd.abm_set_pipe.abm_set_pipe_data.set_pipe_option = option as u8;
    cmd.abm_set_pipe.abm_set_pipe_data.panel_inst = panel_inst as u8;
    cmd.abm_set_pipe.abm_set_pipe_data.ramping_boundary = 0xff;
    cmd.abm_set_pipe.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_set_pipe_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

unsafe fn dmub_abm_set_backlight(
    dc: *mut dc_context, backlight_pwm_u16_16: u32, frame_ramp: u32, panel_inst: u32,
) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    cmd.abm_set_backlight.header.r#type = DMUB_CMD__ABM;
    cmd.abm_set_backlight.header.sub_type = DMUB_CMD__ABM_SET_BACKLIGHT;
    cmd.abm_set_backlight.abm_set_backlight_data.frame_ramp = frame_ramp;
    cmd.abm_set_backlight.abm_set_backlight_data.backlight_user_level = backlight_pwm_u16_16;
    cmd.abm_set_backlight.abm_set_backlight_data.version = DMUB_CMD_ABM_CONTROL_VERSION_1;
    cmd.abm_set_backlight.abm_set_backlight_data.panel_mask = 0x01u32 << panel_inst;
    cmd.abm_set_backlight.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_set_backlight_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

pub unsafe fn dcn21_set_abm_immediate_disable(pipe_ctx: *mut pipe_ctx) {
    let abm = (*pipe_ctx).stream_res.abm;
    let otg_inst = (*(*pipe_ctx).stream_res.tg).inst;
    let panel_cntl = (*(*pipe_ctx).stream).link.panel_cntl;
    let dmcu = (*(*(*pipe_ctx).stream).ctx).dc.res_pool.dmcu;
    let link = (*(*pipe_ctx).stream).link;
    if ((*pipe_ctx).stream).abm_level == 0 || ((*pipe_ctx).stream).abm_level == ABM_LEVEL_IMMEDIATE_DISABLE {
        if !link.is_null() && !(*link).panel_config.cacp.cacp_supported { return; }
    }
    if !dmcu.is_null() { dce110_set_abm_immediate_disable(pipe_ctx); return; }
    if !abm.is_null() && !panel_cntl.is_null() {
        if (*abm).funcs.set_pipe_ex.is_some() {
            ((*abm).funcs.set_pipe_ex)(abm, otg_inst, SET_ABM_PIPE_IMMEDIATELY_DISABLE,
                (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
        } else if !link.is_null() && (*link).panel_config.cacp.cacp_supported {
            dcn21_dmub_cacp_set_pipe(abm, otg_inst, SET_CACP_PIPE_IMMEDIATELY_DISABLE,
                (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
        } else {
            dcn21_dmub_abm_set_pipe(abm, otg_inst, SET_ABM_PIPE_IMMEDIATELY_DISABLE,
                (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
        }
        ((*panel_cntl).funcs.store_backlight_level)(panel_cntl);
    }
}

pub unsafe fn dcn21_set_pipe(pipe_ctx: *mut pipe_ctx) {
    let abm = (*pipe_ctx).stream_res.abm;
    let tg = (*pipe_ctx).stream_res.tg;
    let panel_cntl = (*(*pipe_ctx).stream).link.panel_cntl;
    let dmcu = (*(*(*pipe_ctx).stream).ctx).dc.res_pool.dmcu;
    let link = (*(*pipe_ctx).stream).link;
    if abm.is_null() || tg.is_null() || panel_cntl.is_null() { return; }
    let otg_inst = (*tg).inst;
    if !dmcu.is_null() { dce110_set_pipe(pipe_ctx); return; }
    if (*abm).funcs.set_pipe_ex.is_some() {
        ((*abm).funcs.set_pipe_ex)(abm, otg_inst, SET_ABM_PIPE_NORMAL,
            (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
    } else if !link.is_null() && (*link).panel_config.cacp.cacp_supported {
        dcn21_dmub_cacp_set_pipe(abm, otg_inst, SET_CACP_PIPE_NORMAL,
            (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
    } else {
        dcn21_dmub_abm_set_pipe(abm, otg_inst, SET_ABM_PIPE_NORMAL,
            (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
    }
}

pub unsafe fn dcn21_set_backlight_level(
    pipe_ctx: *mut pipe_ctx, backlight_level_params: *mut set_backlight_level_params,
) -> bool {
    let dc = (*(*pipe_ctx).stream).ctx;
    let abm = (*pipe_ctx).stream_res.abm;
    let tg = (*pipe_ctx).stream_res.tg;
    let panel_cntl = (*(*pipe_ctx).stream).link.panel_cntl;
    let link = (*(*pipe_ctx).stream).link;
    if abm.is_null() || tg.is_null() || panel_cntl.is_null() { return false; }
    let otg_inst = (*tg).inst;
    let backlight_pwm_u16_16 = (*backlight_level_params).backlight_pwm_u16_16;
    let frame_ramp = (*backlight_level_params).frame_ramp;
    if !(*(*dc).dc).res_pool.dmcu.is_null() {
        dce110_set_backlight_level(pipe_ctx, backlight_level_params); return true;
    }
    if (*abm).funcs.set_pipe_ex.is_some() {
        ((*abm).funcs.set_pipe_ex)(abm, otg_inst, SET_ABM_PIPE_NORMAL,
            (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
    } else if !link.is_null() && (*link).panel_config.cacp.cacp_supported {
        dcn21_dmub_cacp_set_pipe(abm, otg_inst, SET_CACP_PIPE_NORMAL,
            (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
    } else {
        dcn21_dmub_abm_set_pipe(abm, otg_inst, SET_ABM_PIPE_NORMAL,
            (*panel_cntl).inst, (*panel_cntl).pwrseq_inst);
    }
    if (*abm).funcs.set_backlight_level_pwm.is_some() {
        ((*abm).funcs.set_backlight_level_pwm)(abm, backlight_pwm_u16_16,
            frame_ramp, 0, (*panel_cntl).inst);
    } else {
        dmub_abm_set_backlight(dc, backlight_pwm_u16_16, frame_ramp, (*panel_cntl).inst);
    }
    true
}

pub unsafe fn dcn21_is_abm_supported(
    dc: *mut dc, context: *mut dc_state, stream: *mut dc_stream_state,
) -> bool {
    let mut i = 0;
    while i < (*dc).res_pool.pipe_count {
        let pipe_ctx = &mut (*context).res_ctx.pipe_ctx[i as usize];
        if pipe_ctx.stream == stream && pipe_ctx.prev_odm_pipe.is_null()
            && pipe_ctx.next_odm_pipe.is_null() { return true; }
        i += 1;
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
