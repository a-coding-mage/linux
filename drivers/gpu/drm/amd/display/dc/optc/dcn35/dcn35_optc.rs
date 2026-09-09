/* SPDX-License-Identifier: MIT */
/* Copyright 2023 Advanced Micro Devices, Inc. */

// C dependencies are supplied by the surrounding translation unit.

macro_rules! REG { ($r:ident) => { optc1.tg_regs.$r }; }
macro_rules! CTX { () => { optc1.base.ctx }; }
macro_rules! FN { ($r:ident, $f:ident) => { (optc1.tg_shift.$f, optc1.tg_mask.$f) }; }

unsafe fn optc35_set_odm_combine(
    optc: *mut timing_generator, opp_id: *mut i32, opp_cnt: i32,
    segment_width: i32, _last_segment_width: i32,
) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let mut memory_mask: u32 = 0;
    let h_active = segment_width * opp_cnt;
    let odm_mem_count = (h_active + 2047) / 2048;

    if opp_cnt == 4 {
        memory_mask = if odm_mem_count <= 2 { 0x3 } else if odm_mem_count <= 4 { 0xf } else { 0x3f };
    } else {
        memory_mask = if odm_mem_count <= 2 {
            (0x1 << (*opp_id.add(0) * 2)) | (0x1 << (*opp_id.add(1) * 2))
        } else if odm_mem_count <= 4 {
            (0x3 << (*opp_id.add(0) * 2)) | (0x3 << (*opp_id.add(1) * 2))
        } else { 0x77 };
    }
    REG_SET!(OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, memory_mask);
    if opp_cnt == 2 {
        REG_SET_3!(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 1,
            OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1));
    } else if opp_cnt == 4 {
        REG_SET_5!(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 3,
            OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1),
            OPTC_SEG2_SRC_SEL, *opp_id.add(2), OPTC_SEG3_SRC_SEL, *opp_id.add(3));
    }
    REG_UPDATE!(OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width);
    REG_UPDATE!(OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, opp_cnt - 1);
    (*optc1).opp_count = opp_cnt;
}

unsafe fn optc35_enable_crtc(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, (*optc).inst);
    REG_UPDATE!(CONTROL, VTG0_ENABLE, 1);
    REG_UPDATE_2!(OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 2, OTG_MASTER_EN, 1);
    true
}

unsafe fn optc35_disable_crtc(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_5!(OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, 0xf, OPTC_SEG1_SRC_SEL, 0xf,
        OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf, OPTC_NUM_OF_INPUT_SEGMENT, 0);
    REG_UPDATE!(OPTC_MEMORY_CONFIG, OPTC_MEM_SEL, 0);
    REG_UPDATE!(OTG_CONTROL, OTG_MASTER_EN, 0);
    REG_UPDATE!(CONTROL, VTG0_ENABLE, 0);
    REG_WAIT!(OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 100000);
    REG_WAIT!(OTG_CONTROL, OTG_CURRENT_MASTER_EN_STATE, 0, 1, 100000);
    optc1_clear_optc_underflow(optc);
    true
}

unsafe fn optc35_phantom_crtc_post_enable(optc: *mut timing_generator) {
    let _optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_2!(OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 0, OTG_MASTER_EN, 0);
    REG_WAIT!(OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 100000);
}

unsafe fn optc35_get_crc(optc: *mut timing_generator, idx: u8, r_cr: *mut u32, g_y: *mut u32, b_cb: *mut u32) -> bool {
    let mut field: u32 = 0;
    let optc1 = DCN10TG_FROM_TG(optc);
    if idx == 1 && (*(*optc1).tg_mask).OTG_CRC1_EN != 0 { REG_GET!(OTG_CRC_CNTL, OTG_CRC1_EN, &mut field); }
    else { REG_GET!(OTG_CRC_CNTL, OTG_CRC_EN, &mut field); }
    if field == 0 { return false; }
    if (*(*optc1).tg_mask).CRC0_R_CR32 != 0 && (*(*optc1).tg_mask).CRC1_R_CR32 != 0 &&
       (*(*optc1).tg_mask).CRC0_G_Y32 != 0 && (*(*optc1).tg_mask).CRC1_G_Y32 != 0 &&
       (*(*optc1).tg_mask).CRC0_B_CB32 != 0 && (*(*optc1).tg_mask).CRC1_B_CB32 != 0 {
        match idx { 0 => { REG_GET!(OTG_CRC0_DATA_R32, CRC0_R_CR32, r_cr); REG_GET!(OTG_CRC0_DATA_G32, CRC0_G_Y32, g_y); REG_GET!(OTG_CRC0_DATA_B32, CRC0_B_CB32, b_cb); },
        1 => { REG_GET!(OTG_CRC1_DATA_R32, CRC1_R_CR32, r_cr); REG_GET!(OTG_CRC1_DATA_G32, CRC1_G_Y32, g_y); REG_GET!(OTG_CRC1_DATA_B32, CRC1_B_CB32, b_cb); }, _ => return false }
    } else { match idx { 0 => { REG_GET_2!(OTG_CRC0_DATA_RG, CRC0_R_CR, r_cr, CRC0_G_Y, g_y); REG_GET!(OTG_CRC0_DATA_B, CRC0_B_CB, b_cb); },
        1 => { REG_GET_2!(OTG_CRC1_DATA_RG, CRC1_R_CR, r_cr, CRC1_G_Y, g_y); REG_GET!(OTG_CRC1_DATA_B, CRC1_B_CB, b_cb); }, _ => return false } }
    true
}

unsafe fn optc35_setup_manual_trigger(optc: *mut timing_generator) {
    if optc.is_null() || (*optc).ctx.is_null() { return; }
    let optc1 = DCN10TG_FROM_TG(optc);
    let dc = (*(*optc).ctx).dc;
    if (*dc).caps.dmub_caps.mclk_sw && !(*dc).debug.disable_fams { dc_dmub_srv_set_drr_manual_trigger_cmd(dc, (*optc).inst); }
    else { REG_UPDATE_4!(OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MIN_SEL, 1, OTG_V_TOTAL_MAX_SEL, 1, OTG_FORCE_LOCK_ON_EVENT, 0, OTG_SET_V_TOTAL_MIN_MASK, 1 << 1); if !(*optc).funcs.is_null() && (*(*optc).funcs).setup_manual_trigger.is_some() { ((*(*optc).funcs).setup_manual_trigger.unwrap())(optc); } }
}

pub unsafe fn optc35_configure_crc(optc: *mut timing_generator, params: *const crc_params) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    if !optc1_is_tg_enabled(optc) { return false; }
    if !(*params).enable || (*params).reset {
        match (*params).crc_eng_inst { 0 => REG_UPDATE!(OTG_CRC_CNTL, OTG_CRC_EN, 0), 1 => { if (*(*optc1).tg_mask).OTG_CRC1_EN != 0 { REG_UPDATE!(OTG_CRC_CNTL, OTG_CRC1_EN, 0); } else { REG_UPDATE!(OTG_CRC_CNTL, OTG_CRC_EN, 0); } }, _ => return false }
    }
    if !(*params).enable { return true; }
    let (x1,x2,y1,y2,wx1,wx2,wy1,wy2,sel,en) = match (*params).crc_eng_inst {
        0 => (OTG_CRC0_WINDOWA_X_START, OTG_CRC0_WINDOWA_X_END, OTG_CRC0_WINDOWA_Y_START, OTG_CRC0_WINDOWA_Y_END, OTG_CRC0_WINDOWB_X_START, OTG_CRC0_WINDOWB_X_END, OTG_CRC0_WINDOWB_Y_START, OTG_CRC0_WINDOWB_Y_END, OTG_CRC0_SELECT, OTG_CRC_EN),
        1 => (OTG_CRC1_WINDOWA_X_START, OTG_CRC1_WINDOWA_X_END, OTG_CRC1_WINDOWA_Y_START, OTG_CRC1_WINDOWA_Y_END, OTG_CRC1_WINDOWB_X_START, OTG_CRC1_WINDOWB_X_END, OTG_CRC1_WINDOWB_Y_START, OTG_CRC1_WINDOWB_Y_END, OTG_CRC1_SELECT, OTG_CRC1_EN),
        _ => return false,
    };
    REG_UPDATE_2!(OTG_CRC_WINDOWA_X_CONTROL, x1, (*params).windowa_x_start, x2, (*params).windowa_x_end);
    REG_UPDATE_2!(OTG_CRC_WINDOWA_Y_CONTROL, y1, (*params).windowa_y_start, y2, (*params).windowa_y_end);
    REG_UPDATE_2!(OTG_CRC_WINDOWB_X_CONTROL, wx1, (*params).windowb_x_start, wx2, (*params).windowb_x_end);
    REG_UPDATE_2!(OTG_CRC_WINDOWB_Y_CONTROL, wy1, (*params).windowb_y_start, wy2, (*params).windowb_y_end);
    REG_UPDATE_3!(OTG_CRC_CNTL, OTG_CRC_CONT_EN, if (*params).continuous_mode {1} else {0}, sel, (*params).selection, en, 1);
    if (*(*optc1).tg_mask).OTG_CRC_POLY_SEL != 0 { REG_UPDATE!(OTG_CRC_CNTL, OTG_CRC_POLY_SEL, (*params).crc_poly_mode); }
    true
}

// The remaining inherited timing-generator operations are supplied by the DCN1/DCN2 implementations.
static dcn35_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: optc1_validate_timing, program_timing: optc1_program_timing,
    enable_crtc: optc35_enable_crtc, disable_crtc: optc35_disable_crtc,
    immediate_disable_crtc: optc31_immediate_disable_crtc,
    phantom_crtc_post_enable: optc35_phantom_crtc_post_enable,
    get_crc: optc35_get_crc, configure_crc: optc35_configure_crc,
    set_drr: optc35_set_drr, set_long_vtotal: optc35_set_long_vtotal,
    wait_otg_disable: optc35_wait_otg_disable,
    set_odm_combine: optc35_set_odm_combine,
    ..unsafe { core::mem::zeroed() }
};

pub unsafe fn optc35_set_drr(optc: *mut timing_generator, params: *const drr_params) {
    if optc.is_null() || params.is_null() { return; }
    let optc1 = DCN10TG_FROM_TG(optc); let max_otg_v_total = (*optc1).max_v_total - 1;
    if (*params).vertical_total_max > 0 && (*params).vertical_total_min > 0 {
        if (*params).vertical_total_mid != 0 { REG_SET!(OTG_V_TOTAL_MID, 0, OTG_V_TOTAL_MID, (*params).vertical_total_mid - 1); REG_UPDATE_2!(OTG_V_TOTAL_CONTROL, OTG_VTOTAL_MID_REPLACING_MAX_EN, 1, OTG_VTOTAL_MID_FRAME_NUM, (*params).vertical_total_mid_frame_num as u8); }
        if !(*optc).funcs.is_null() && (*(*optc).funcs).set_vtotal_min_max.is_some() { ((*(*optc).funcs).set_vtotal_min_max.unwrap())(optc, (*params).vertical_total_min - 1, (*params).vertical_total_max - 1); } optc35_setup_manual_trigger(optc);
    } else { REG_UPDATE_4!(OTG_V_TOTAL_CONTROL, OTG_SET_V_TOTAL_MIN_MASK, 0, OTG_V_TOTAL_MIN_SEL, 0, OTG_V_TOTAL_MAX_SEL, 0, OTG_FORCE_LOCK_ON_EVENT, 0); if !(*optc).funcs.is_null() && (*(*optc).funcs).set_vtotal_min_max.is_some() { ((*(*optc).funcs).set_vtotal_min_max.unwrap())(optc, 0, 0); } }
    REG_WRITE!(OTG_V_COUNT_STOP_CONTROL, max_otg_v_total); REG_WRITE!(OTG_V_COUNT_STOP_CONTROL2, 0);
}

pub unsafe fn optc35_set_long_vtotal(optc: *mut timing_generator, params: *const long_vtotal_params) {
    if optc.is_null() || params.is_null() { return; }
    let optc1 = DCN10TG_FROM_TG(optc); let max = (*optc1).max_v_total - 1;
    if (*params).vertical_total_min <= max && (*params).vertical_total_max <= max { return; }
    if (*params).vertical_total_max == 0 || (*params).vertical_total_min == 0 { REG_UPDATE_4!(OTG_V_TOTAL_CONTROL, OTG_SET_V_TOTAL_MIN_MASK, 0, OTG_V_TOTAL_MIN_SEL, 0, OTG_V_TOTAL_MAX_SEL, 0, OTG_FORCE_LOCK_ON_EVENT, 0); if !(*optc).funcs.is_null() && (*(*optc).funcs).set_vtotal_min_max.is_some() { ((*(*optc).funcs).set_vtotal_min_max.unwrap())(optc,0,0); } }
    else if (*params).vertical_total_max == (*params).vertical_total_min { REG_UPDATE_4!(OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MIN_SEL,1, OTG_V_TOTAL_MAX_SEL,1, OTG_FORCE_LOCK_ON_EVENT,0, OTG_SET_V_TOTAL_MIN_MASK,0); if !(*optc).funcs.is_null() && (*(*optc).funcs).set_vtotal_min_max.is_some() { ((*(*optc).funcs).set_vtotal_min_max.unwrap())(optc,max,max); } REG_WRITE!(OTG_V_COUNT_STOP_CONTROL, (*params).vertical_blank_start); REG_WRITE!(OTG_V_COUNT_STOP_CONTROL2, (*params).vertical_total_max-max); }
    else if (*params).vertical_total_min > max { ASSERT!(0); REG_UPDATE_4!(OTG_V_TOTAL_CONTROL, OTG_SET_V_TOTAL_MIN_MASK,0, OTG_V_TOTAL_MIN_SEL,0, OTG_V_TOTAL_MAX_SEL,0, OTG_FORCE_LOCK_ON_EVENT,0); if !(*optc).funcs.is_null() && (*(*optc).funcs).set_vtotal_min_max.is_some() { ((*(*optc).funcs).set_vtotal_min_max.unwrap())(optc,0,0); } REG_WRITE!(OTG_V_COUNT_STOP_CONTROL,max); REG_WRITE!(OTG_V_COUNT_STOP_CONTROL2,0); }
    else { if !(*optc).funcs.is_null() && (*(*optc).funcs).set_vtotal_min_max.is_some() { ((*(*optc).funcs).set_vtotal_min_max.unwrap())(optc,(*params).vertical_total_min-1,max); } optc35_setup_manual_trigger(optc); REG_WRITE!(OTG_V_COUNT_STOP_CONTROL,(*params).vertical_total_min); REG_WRITE!(OTG_V_COUNT_STOP_CONTROL2,(*params).vertical_total_max-max); }
}

pub unsafe fn optc35_wait_otg_disable(optc: *mut timing_generator) { if optc.is_null() || (*optc).ctx.is_null() { return; } let mut en=0; let _optc1=DCN10TG_FROM_TG(optc); REG_GET!(OTG_CONTROL,OTG_MASTER_EN,&mut en); if en==0 { REG_WAIT!(OTG_CLOCK_CONTROL,OTG_CURRENT_MASTER_EN_STATE,0,1,100000); } }

pub unsafe fn dcn35_timing_generator_init(optc1: *mut optc) { (*optc1).base.funcs=&dcn35_tg_funcs; (*optc1).max_h_total=(*optc1).tg_mask.OTG_H_TOTAL+1; (*optc1).max_v_total=(*optc1).tg_mask.OTG_V_TOTAL+1; (*optc1).min_h_blank=32; (*optc1).min_v_blank=3; (*optc1).min_v_blank_interlace=5; (*optc1).min_h_sync_width=4; (*optc1).min_v_sync_width=1; dcn35_timing_generator_set_fgcg(optc1, (*optc1).base.ctx.dc.debug.enable_fine_grain_clock_gating.bits.optc); }
pub unsafe fn dcn35_timing_generator_set_fgcg(optc1: *mut optc, enable: bool) { REG_UPDATE!(OPTC_CLOCK_CONTROL, OPTC_FGCG_REP_DIS, !enable); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
