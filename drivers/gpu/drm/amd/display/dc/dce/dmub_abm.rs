/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

const ABM_FEATURE_NO_SUPPORT: u32 = 0;
const ABM_LCD_SUPPORT: u32 = 1;
const ABM_CACP_SUPPORT: u32 = 2;

unsafe fn abm_feature_support(abm: *mut abm, panel_inst: u32) -> u32 {
    let dc = (*abm).ctx;
    let mut edp_links: [*mut dc_link; MAX_NUM_EDP] = [core::ptr::null_mut(); MAX_NUM_EDP];
    let mut edp_num: u32 = 0;
    let mut ret = ABM_FEATURE_NO_SUPPORT;

    dc_get_edp_links((*dc).dc, edp_links.as_mut_ptr(), &mut edp_num);

    let mut i = 0;
    while i < edp_num {
        if panel_inst == i { break; }
        i += 1;
    }

    if i < edp_num {
        if (*edp_links[panel_inst as usize]).panel_config.cacp.cacp_supported {
            ret = ABM_CACP_SUPPORT;
        } else if (*edp_links[panel_inst as usize]).panel_type == PANEL_TYPE_LCD
            || (*edp_links[panel_inst as usize]).panel_type == PANEL_TYPE_MINILED {
            ret = ABM_LCD_SUPPORT;
        }
    }
    ret
}

unsafe fn abm_get_paneltype(abm: *mut abm, panel_inst: u32) -> dc_panel_type {
    let dc = (*abm).ctx;
    let mut edp_links: [*mut dc_link; MAX_NUM_EDP] = [core::ptr::null_mut(); MAX_NUM_EDP];
    let mut edp_num: u32 = 0;
    let mut ret = PANEL_TYPE_NONE;

    dc_get_edp_links((*dc).dc, edp_links.as_mut_ptr(), &mut edp_num);

    let mut i = 0;
    while i < edp_num {
        if (*edp_links[i as usize]).link_status.link_active && panel_inst == i { break; }
        i += 1;
    }
    if i < edp_num { ret = (*edp_links[panel_inst as usize]).panel_type; }
    ret
}

unsafe fn dmub_abm_init_ex(abm: *mut abm, backlight: u32, user_level: u32) {
    dmub_abm_init(abm, backlight, user_level);
    let mut panel_mask: u8 = 0;
    for i in 0..MAX_NUM_EDP { panel_mask |= 0x01u8 << i; }
    if panel_mask != 0 { dmub_cacp_enable_fractional_pwm(abm, panel_mask); }
}

unsafe fn dmub_abm_get_current_backlight_ex(abm: *mut abm) -> u32 {
    dc_allow_idle_optimizations((*(*abm).ctx).dc, false);
    dmub_abm_get_current_backlight(abm)
}

unsafe fn dmub_abm_get_target_backlight_ex(abm: *mut abm) -> u32 {
    dc_allow_idle_optimizations((*(*abm).ctx).dc, false);
    dmub_abm_get_target_backlight(abm)
}

unsafe fn dmub_abm_set_level_ex(abm: *mut abm, level: u32) -> bool {
    let mut ret = false;
    let mut panel_mask0 = 0u8;
    let mut panel_mask1 = 0u8;
    for i in 0..MAX_NUM_EDP {
        match abm_feature_support(abm, i as u32) {
            ABM_LCD_SUPPORT => panel_mask0 |= 0x01u8 << i,
            ABM_CACP_SUPPORT => panel_mask1 |= 0x01u8 << i,
            _ => {}
        }
    }
    if panel_mask0 != 0 { ret = dmub_abm_set_level(abm, level, panel_mask0); }
    if panel_mask1 != 0 { ret = dmub_cacp_set_level(abm, level, panel_mask1); }
    ret
}

unsafe fn dmub_abm_init_config_ex(abm: *mut abm, src: *const i8, bytes: u32, inst: u32) -> bool {
    match abm_feature_support(abm, inst) {
        ABM_LCD_SUPPORT => dmub_abm_init_config(abm, src, bytes, inst),
        ABM_CACP_SUPPORT => dmub_cacp_init(abm, src, bytes, inst),
        _ => {}
    }
    true
}

unsafe fn dmub_abm_set_pause_ex(abm: *mut abm, pause: bool, panel_inst: u32, stream_inst: u32) -> bool {
    match abm_feature_support(abm, panel_inst) {
        ABM_LCD_SUPPORT => dmub_abm_set_pause(abm, pause, panel_inst, stream_inst),
        ABM_CACP_SUPPORT => dmub_cacp_set_pause(abm, pause, panel_inst, stream_inst),
        _ => false
    }
}

unsafe fn dmub_abm_save_restore_ex(abm: *mut abm, panel_inst: u32, p_data: *mut abm_save_restore) -> bool {
    if abm_feature_support(abm, panel_inst) == ABM_LCD_SUPPORT {
        dmub_abm_save_restore((*(*abm).ctx), panel_inst, p_data)
    } else { false }
}

unsafe fn dmub_abm_set_pipe_ex(abm: *mut abm, otg_inst: u32, option: u32, panel_inst: u32, pwrseq_inst: u32) -> bool {
    match abm_feature_support(abm, panel_inst) {
        ABM_LCD_SUPPORT => dmub_abm_set_pipe(abm, otg_inst, option, panel_inst, pwrseq_inst),
        ABM_CACP_SUPPORT => dmub_cacp_set_pipe(abm, otg_inst, option, panel_inst, pwrseq_inst),
        _ => false
    }
}

unsafe fn dmub_abm_set_event_ex(abm: *mut abm, full_screen: u32, trans_info: u32, hdr_mode: u32, scaling_enable: u32, scaling_strength_map: u32, panel_inst: u32) -> bool {
    match abm_feature_support(abm, panel_inst) {
        ABM_LCD_SUPPORT => dmub_abm_set_event(abm, scaling_enable, scaling_strength_map, panel_inst),
        ABM_CACP_SUPPORT => dmub_cacp_set_event(abm, full_screen, trans_info, hdr_mode, scaling_enable, panel_inst),
        _ => false
    }
}

unsafe fn dmub_abm_set_backlight_level_pwm_ex(abm: *mut abm, backlight_pwm_u16_16: u32, frame_ramp: u32, _controller_id: u32, panel_inst: u32) -> bool {
    if abm_feature_support(abm, panel_inst) == ABM_LCD_SUPPORT {
        dmub_abm_set_backlight_level(abm, backlight_pwm_u16_16, frame_ramp, panel_inst)
    } else if abm_feature_support(abm, panel_inst) == ABM_CACP_SUPPORT && abm_get_paneltype(abm, panel_inst) == PANEL_TYPE_MINILED {
        dmub_cacp_set_backlight_level(abm, backlight_pwm_u16_16, frame_ramp, panel_inst)
    } else { false }
}

static ABM_FUNCS: abm_funcs = abm_funcs {
    abm_init: Some(dmub_abm_init_ex), set_abm_level: Some(dmub_abm_set_level_ex),
    get_current_backlight: Some(dmub_abm_get_current_backlight_ex), get_target_backlight: Some(dmub_abm_get_target_backlight_ex),
    init_abm_config: Some(dmub_abm_init_config_ex), set_abm_pause: Some(dmub_abm_set_pause_ex),
    save_restore: Some(dmub_abm_save_restore_ex), set_pipe_ex: Some(dmub_abm_set_pipe_ex),
    set_abm_event: Some(dmub_abm_set_event_ex), set_backlight_level_pwm: Some(dmub_abm_set_backlight_level_pwm_ex),
};

unsafe fn dmub_abm_construct(abm_dce: *mut dce_abm, ctx: *mut dc_context, regs: *const dce_abm_registers, abm_shift: *const dce_abm_shift, abm_mask: *const dce_abm_mask) {
    let base = &mut (*abm_dce).base;
    base.ctx = ctx; base.funcs = &ABM_FUNCS; base.dmcu_is_running = false;
    (*abm_dce).regs = regs; (*abm_dce).abm_shift = abm_shift; (*abm_dce).abm_mask = abm_mask;
}

unsafe fn dmub_abm_create(ctx: *mut dc_context, regs: *const dce_abm_registers, abm_shift: *const dce_abm_shift, abm_mask: *const dce_abm_mask) -> *mut abm {
    if (*(*ctx).dc).caps.dmcub_support {
        let abm_dce = kzalloc_obj::<dce_abm>();
        if abm_dce.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
        dmub_abm_construct(abm_dce, ctx, regs, abm_shift, abm_mask);
        &mut (*abm_dce).base
    } else { core::ptr::null_mut() }
}

unsafe fn dmub_abm_destroy(abm: *mut *mut abm) {
    let abm_dce = *abm as *mut dce_abm;
    kfree(abm_dce);
    *abm = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
