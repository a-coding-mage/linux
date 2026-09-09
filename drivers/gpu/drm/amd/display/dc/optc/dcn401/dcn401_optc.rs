// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// C dependencies are supplied by the surrounding translation unit.

unsafe fn decide_odm_mem_bit_map(opp_id: *const i32, opp_cnt: i32, h_active: i32) -> u32 {
    let mut first_preferred_memory_for_opp = [false; MAX_PIPES as usize];
    let mut second_preferred_memory_for_opp = [false; MAX_PIPES as usize];
    let mut memory_bit_map: u32 = 0;
    let total_required = ((h_active + 4095) / 4096) * 2;
    let mut total_allocated = 0;
    let mut i;

    i = 0;
    while i < opp_cnt {
        first_preferred_memory_for_opp[*opp_id.add(i as usize) as usize] = true;
        total_allocated += 1;
        if total_required == total_allocated { break; }
        i += 1;
    }

    if total_required > total_allocated {
        i = 0;
        while i < opp_cnt {
            second_preferred_memory_for_opp[*opp_id.add(i as usize) as usize] = true;
            total_allocated += 1;
            if total_required == total_allocated { break; }
            i += 1;
        }
    }

    if total_required > total_allocated {
        i = 0;
        while i < MAX_PIPES {
            if !second_preferred_memory_for_opp[i as usize] {
                second_preferred_memory_for_opp[i as usize] = true;
                total_allocated += 1;
                if total_required == total_allocated { break; }
            }
            i += 1;
        }
    }
    ASSERT(total_required == total_allocated);

    i = 0;
    while i < MAX_PIPES {
        if first_preferred_memory_for_opp[i as usize] { memory_bit_map |= 0x1u32 << (i * 2); }
        if second_preferred_memory_for_opp[i as usize] { memory_bit_map |= 0x2u32 << (i * 2); }
        i += 1;
    }
    memory_bit_map
}

pub unsafe fn optc401_set_odm_combine(optc: *mut timing_generator, opp_id: *mut i32, opp_cnt: i32, segment_width: i32, last_segment_width: i32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let h_active = segment_width * (opp_cnt - 1) + last_segment_width;
    let odm_mem_bit_map = decide_odm_mem_bit_map(opp_id, opp_cnt, h_active);
    REG_SET(OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, odm_mem_bit_map);
    match opp_cnt {
        2 => {
            REG_SET_3(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 1, OPTC_SEG0_SRC_SEL, *opp_id, OPTC_SEG1_SRC_SEL, *opp_id.add(1));
            REG_UPDATE(OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width);
            REG_UPDATE(OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, H_TIMING_DIV_BY2);
        },
        3 => {
            REG_SET_4(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 2, OPTC_SEG0_SRC_SEL, *opp_id, OPTC_SEG1_SRC_SEL, *opp_id.add(1), OPTC_SEG2_SRC_SEL, *opp_id.add(2));
            REG_UPDATE(OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width);
            REG_UPDATE(OPTC_WIDTH_CONTROL2, OPTC_SEGMENT_WIDTH_LAST, last_segment_width);
            // ODM combine 3:1 packs 4 pixels per transfer, so divide by 4.
            REG_UPDATE(OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, H_TIMING_DIV_BY4);
        },
        4 => {
            REG_SET_5(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 3, OPTC_SEG0_SRC_SEL, *opp_id, OPTC_SEG1_SRC_SEL, *opp_id.add(1), OPTC_SEG2_SRC_SEL, *opp_id.add(2), OPTC_SEG3_SRC_SEL, *opp_id.add(3));
            REG_UPDATE(OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width);
            REG_UPDATE(OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, H_TIMING_DIV_BY4);
        },
        _ => ASSERT(false),
    }
    (*optc1).opp_count = opp_cnt;
}

pub unsafe fn optc401_set_h_timing_div_manual_mode(optc: *mut timing_generator, manual_mode: bool) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE(OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE_MANUAL, if manual_mode { 1 } else { 0 });
}

pub unsafe fn optc401_enable_crtc(optc: *mut timing_generator) -> bool {
    let _optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE(OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, (*optc).inst);
    REG_UPDATE(CONTROL, VTG0_ENABLE, 1);
    REG_UPDATE_2(OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 2, OTG_MASTER_EN, 1);
    true
}

pub unsafe fn optc401_disable_crtc(optc: *mut timing_generator) -> bool {
    let _optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_5(OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, 0xf, OPTC_SEG1_SRC_SEL, 0xf, OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf, OPTC_NUM_OF_INPUT_SEGMENT, 0);
    REG_UPDATE(OPTC_MEMORY_CONFIG, OPTC_MEM_SEL, 0);
    REG_UPDATE(OTG_CONTROL, OTG_MASTER_EN, 0);
    REG_UPDATE(CONTROL, VTG0_ENABLE, 0);
    REG_WAIT(OTG_CONTROL, OTG_CURRENT_MASTER_EN_STATE, 0, 10, 15000);
    REG_WAIT(OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 150000);
    true
}

pub unsafe fn optc401_phantom_crtc_post_enable(optc: *mut timing_generator) {
    let _optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_2(OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 0, OTG_MASTER_EN, 0);
    REG_WAIT(OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 100000);
}

pub unsafe fn optc401_disable_phantom_otg(optc: *mut timing_generator) {
    let _optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_5(OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, 0xf, OPTC_SEG1_SRC_SEL, 0xf, OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf, OPTC_NUM_OF_INPUT_SEGMENT, 0);
    REG_UPDATE(OTG_CONTROL, OTG_MASTER_EN, 0);
}

pub unsafe fn optc401_set_odm_bypass(optc: *mut timing_generator, dc_crtc_timing: *const dc_crtc_timing) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let h_div = (*optc).funcs.is_two_pixels_per_container(dc_crtc_timing);
    REG_SET_5(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 0, OPTC_SEG0_SRC_SEL, (*optc).inst, OPTC_SEG1_SRC_SEL, 0xf, OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf);
    REG_UPDATE(OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, h_div);
    REG_SET(OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, 0);
    (*optc1).opp_count = 1;
}

pub unsafe fn optc401_setup_manual_trigger(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let dc = (*optc1).ctx.dc;
    if (*dc).caps.dmub_caps.fams_ver == 1 && !(*dc).debug.disable_fams {
        dc_dmub_srv_set_drr_manual_trigger_cmd(dc, (*optc).inst);
    } else {
        REG_UPDATE_4(OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MIN_SEL, 1, OTG_V_TOTAL_MAX_SEL, 1, OTG_FORCE_LOCK_ON_EVENT, 0, OTG_SET_V_TOTAL_MIN_MASK, 1 << 1);
    }
}

pub unsafe fn optc401_set_drr(optc: *mut timing_generator, params: *const drr_params) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let dc = (*optc1).ctx.dc;
    let mut amended_params = drr_params { vertical_total_max: 0, vertical_total_min: 0, vertical_total_mid: 0, vertical_total_mid_frame_num: 0 };
    let mut program_manual_trigger = false;
    if (*dc).caps.dmub_caps.fams_ver == (*dc).debug.fams_version.ver && (*dc).debug.fams2_config.bits.enable {
        if !params.is_null() && (*params).vertical_total_max > 0 && (*params).vertical_total_min > 0 {
            amended_params.vertical_total_max = (*params).vertical_total_max - 1;
            amended_params.vertical_total_min = (*params).vertical_total_min - 1;
            if (*params).vertical_total_mid != 0 { amended_params.vertical_total_mid = (*params).vertical_total_mid - 1; amended_params.vertical_total_mid_frame_num = (*params).vertical_total_mid_frame_num; }
            program_manual_trigger = true;
        }
        dc_dmub_srv_fams2_drr_update(dc, (*optc).inst, amended_params.vertical_total_min, amended_params.vertical_total_max, amended_params.vertical_total_mid, amended_params.vertical_total_mid_frame_num, program_manual_trigger);
    } else if !params.is_null() && (*params).vertical_total_max > 0 && (*params).vertical_total_min > 0 {
        if (*params).vertical_total_mid != 0 { REG_SET(OTG_V_TOTAL_MID, 0, OTG_V_TOTAL_MID, (*params).vertical_total_mid - 1); REG_UPDATE_2(OTG_V_TOTAL_CONTROL, OTG_VTOTAL_MID_REPLACING_MAX_EN, 1, OTG_VTOTAL_MID_FRAME_NUM, (*params).vertical_total_mid_frame_num as u8); }
        (*optc).funcs.set_vtotal_min_max(optc, (*params).vertical_total_min - 1, (*params).vertical_total_max - 1);
        optc401_setup_manual_trigger(optc);
    } else {
        REG_UPDATE_4(OTG_V_TOTAL_CONTROL, OTG_SET_V_TOTAL_MIN_MASK, 0, OTG_V_TOTAL_MIN_SEL, 0, OTG_V_TOTAL_MAX_SEL, 0, OTG_FORCE_LOCK_ON_EVENT, 0);
        (*optc).funcs.set_vtotal_min_max(optc, 0, 0);
    }
}

pub unsafe fn optc401_set_out_mux(optc: *mut timing_generator, mut dest: otg_out_mux_dest) {
    let _optc1 = DCN10TG_FROM_TG(optc);
    if dest == OUT_MUX_HPO_FRL { dest = OUT_MUX_HPO_DP; }
    REG_UPDATE(OTG_CONTROL, OTG_OUT_MUX, dest);
}

pub unsafe fn optc401_set_vtotal_min_max(optc: *mut timing_generator, vtotal_min: i32, vtotal_max: i32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let dc = (*optc1).ctx.dc;
    if (*dc).caps.dmub_caps.fams_ver == (*dc).debug.fams_version.ver && (*dc).debug.fams2_config.bits.enable { dc_dmub_srv_fams2_drr_update(dc, (*optc).inst, vtotal_min, vtotal_max, 0, 0, false); }
    else if (*dc).caps.dmub_caps.fams_ver == 1 && !(*dc).debug.disable_fams { dc_dmub_srv_drr_update_cmd(dc, (*optc).inst, vtotal_min, vtotal_max); }
    else { optc1_set_vtotal_min_max(optc, vtotal_min, vtotal_max); }
}

pub unsafe fn optc401_program_global_sync(optc: *mut timing_generator, vready_offset: i32, vstartup_start: i32, vupdate_offset: i32, vupdate_width: i32, pstate_keepout: i32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    (*optc1).vready_offset = vready_offset; (*optc1).vstartup_start = vstartup_start; (*optc1).vupdate_offset = vupdate_offset; (*optc1).vupdate_width = vupdate_width; (*optc1).pstate_keepout = pstate_keepout;
    if (*optc1).vstartup_start == 0 { BREAK_TO_DEBUGGER(); return; }
    REG_SET(OTG_VSTARTUP_PARAM, 0, VSTARTUP_START, (*optc1).vstartup_start);
    REG_SET_2(OTG_VUPDATE_PARAM, 0, VUPDATE_OFFSET, (*optc1).vupdate_offset, VUPDATE_WIDTH, (*optc1).vupdate_width);
    REG_SET(OTG_VREADY_PARAM, 0, VREADY_OFFSET, (*optc1).vready_offset);
    REG_UPDATE(OTG_PSTATE_REGISTER, OTG_PSTATE_KEEPOUT_START, (*optc1).pstate_keepout);
}

pub unsafe fn optc401_set_vupdate_keepout(tg: *mut timing_generator, enable: bool) {
    let optc1 = DCN10TG_FROM_TG(tg);
    REG_SET_3(OTG_VUPDATE_KEEPOUT, 0, MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_START_OFFSET, 0, MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_END_OFFSET, (*optc1).vready_offset + 10, OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN, enable);
}

pub unsafe fn optc401_wait_update_lock_status(tg: *mut timing_generator, locked: bool) -> bool {
    let _optc1 = DCN10TG_FROM_TG(tg);
    let mut lock_status: u32 = 0;
    REG_WAIT(OTG_MASTER_UPDATE_LOCK, UPDATE_LOCK_STATUS, locked, 1, 150000);
    REG_GET(OTG_MASTER_UPDATE_LOCK, UPDATE_LOCK_STATUS, &mut lock_status);
    lock_status == locked as u32
}

#[allow(non_upper_case_globals)]
static dcn401_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: optc1_validate_timing,
    program_timing: optc1_program_timing,
    setup_vertical_interrupt0: optc1_setup_vertical_interrupt0,
    setup_vertical_interrupt1: optc1_setup_vertical_interrupt1,
    setup_vertical_interrupt2: optc1_setup_vertical_interrupt2,
    program_global_sync: optc401_program_global_sync,
    enable_crtc: optc401_enable_crtc,
    disable_crtc: optc401_disable_crtc,
    phantom_crtc_post_enable: optc401_phantom_crtc_post_enable,
    disable_phantom_crtc: optc401_disable_phantom_otg,
    is_counter_moving: optc1_is_counter_moving,
    get_position: optc1_get_position,
    get_frame_count: optc1_get_vblank_counter,
    get_scanoutpos: optc1_get_crtc_scanoutpos,
    get_otg_active_size: optc1_get_otg_active_size,
    set_early_control: optc1_set_early_control,
    wait_for_state: optc1_wait_for_state,
    set_blank_color: optc3_program_blank_color,
    did_triggered_reset_occur: optc1_did_triggered_reset_occur,
    triplebuffer_lock: optc3_triplebuffer_lock,
    triplebuffer_unlock: optc2_triplebuffer_unlock,
    enable_reset_trigger: optc1_enable_reset_trigger,
    enable_crtc_reset: optc1_enable_crtc_reset,
    disable_reset_trigger: optc1_disable_reset_trigger,
    lock: optc3_lock,
    unlock: optc1_unlock,
    lock_doublebuffer_enable: optc3_lock_doublebuffer_enable,
    lock_doublebuffer_disable: optc3_lock_doublebuffer_disable,
    enable_optc_clock: optc1_enable_optc_clock,
    set_drr: optc401_set_drr,
    get_last_used_drr_vtotal: optc2_get_last_used_drr_vtotal,
    set_vtotal_min_max: optc401_set_vtotal_min_max,
    set_static_screen_control: optc1_set_static_screen_control,
    program_stereo: optc1_program_stereo,
    is_stereo_left_eye: optc1_is_stereo_left_eye,
    tg_init: optc3_tg_init,
    is_tg_enabled: optc1_is_tg_enabled,
    is_optc_underflow_occurred: optc1_is_optc_underflow_occurred,
    clear_optc_underflow: optc1_clear_optc_underflow,
    setup_global_swap_lock: None,
    get_crc: optc1_get_crc,
    configure_crc: optc1_configure_crc,
    set_dsc_config: optc3_set_dsc_config,
    get_dsc_status: optc2_get_dsc_status,
    set_dwb_source: None,
    set_odm_bypass: optc401_set_odm_bypass,
    set_odm_combine: optc401_set_odm_combine,
    wait_odm_doublebuffer_pending_clear: optc32_wait_odm_doublebuffer_pending_clear,
    set_h_timing_div_manual_mode: optc401_set_h_timing_div_manual_mode,
    get_optc_source: optc2_get_optc_source,
    set_out_mux: optc401_set_out_mux,
    set_drr_trigger_window: optc3_set_drr_trigger_window,
    set_vtotal_change_limit: optc3_set_vtotal_change_limit,
    set_gsl: optc2_set_gsl,
    set_gsl_source_select: optc2_set_gsl_source_select,
    set_vtg_params: optc1_set_vtg_params,
    program_manual_trigger: optc2_program_manual_trigger,
    setup_manual_trigger: optc2_setup_manual_trigger,
    get_hw_timing: optc1_get_hw_timing,
    is_two_pixels_per_container: optc1_is_two_pixels_per_container,
    get_optc_double_buffer_pending: optc3_get_optc_double_buffer_pending,
    get_otg_double_buffer_pending: optc3_get_otg_update_pending,
    get_pipe_update_pending: optc3_get_pipe_update_pending,
    set_vupdate_keepout: optc401_set_vupdate_keepout,
    wait_update_lock_status: optc401_wait_update_lock_status,
    read_otg_state: optc31_read_otg_state,
    optc_read_reg_state: optc31_read_reg_state,
};

pub unsafe fn dcn401_timing_generator_init(optc1: *mut optc) {
    (*optc1).base.funcs = &dcn401_tg_funcs;
    (*optc1).max_h_total = (*optc1).tg_mask.OTG_H_TOTAL + 1;
    (*optc1).max_v_total = (*optc1).tg_mask.OTG_V_TOTAL + 1;
    (*optc1).min_h_blank = 32;
    (*optc1).min_v_blank = 3;
    (*optc1).min_v_blank_interlace = 5;
    (*optc1).min_h_sync_width = 4;
    (*optc1).min_v_sync_width = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
