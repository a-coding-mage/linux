// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C dependencies supplied by the surrounding repository are intentionally not
// reimplemented here.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn optc42_get_crc(
    optc: *mut timing_generator,
    idx: u8,
    r_cr: *mut u32,
    g_y: *mut u32,
    b_cb: *mut u32,
) -> bool {
    let mut field: u32 = 0;
    let optc1 = DCN10TG_FROM_TG(optc);

    if idx == 1 && (*(*optc1).tg_mask).OTG_CRC1_EN != 0 {
        REG_GET((*optc1).tg_regs, OTG_CRC_CNTL, OTG_CRC1_EN, &mut field);
    } else {
        REG_GET((*optc1).tg_regs, OTG_CRC_CNTL, OTG_CRC_EN, &mut field);
    }

    // Early return if CRC is not enabled for this CRTC
    if field == 0 {
        return false;
    }

    match idx {
        0 => {
            // OTG_CRC0_DATA_RG has the CRC16 results for the red component
            REG_GET((*optc1).tg_regs, OTG_CRC0_DATA_R, CRC0_R_CR, r_cr);
            // OTG_CRC0_DATA_RG has the CRC16 results for the green component
            REG_GET((*optc1).tg_regs, OTG_CRC0_DATA_G, CRC0_G_Y, g_y);
            // OTG_CRC0_DATA_B has the CRC16 results for the blue component
            REG_GET((*optc1).tg_regs, OTG_CRC0_DATA_B, CRC0_B_CB, b_cb);
        }
        1 => {
            // OTG_CRC1_DATA_RG has the CRC16 results for the red component
            REG_GET((*optc1).tg_regs, OTG_CRC1_DATA_R, CRC0_R_CR, r_cr);
            // OTG_CRC1_DATA_RG has the CRC16 results for the green component
            REG_GET((*optc1).tg_regs, OTG_CRC1_DATA_G, CRC0_G_Y, g_y);
            // OTG_CRC1_DATA_B has the CRC16 results for the blue component
            REG_GET((*optc1).tg_regs, OTG_CRC1_DATA_B, CRC0_B_CB, b_cb);
        }
        _ => return false,
    }
    true
}

pub unsafe fn optc42_enable_pwa(
    optc: *mut timing_generator,
    pwa_sync_param: *mut otc_pwa_frame_sync,
) {
    let optc1 = DCN10TG_FROM_TG(optc);
    // VCOUNT_MODE: 00 counts from VSYNC; 01 counts from VSTARTUP.
    if pwa_sync_param.is_null() {
        return;
    }
    if (*(*(*optc1).base.ctx).dc).debug.enable_otg_frame_sync_pwa {
        // Take mode 1, using the line number from vstartup to get the output frame as early as possible.
        REG_UPDATE_3((*optc1).tg_regs, OTG_PWA_FRAME_SYNC_CONTROL,
            OTG_PWA_FRAME_SYNC_EN, 1,
            OTG_PWA_FRAME_SYNC_VCOUNT_MODE, (*pwa_sync_param).pwa_sync_mode,
            OTG_PWA_FRAME_SYNC_LINE, (*pwa_sync_param).pwa_frame_sync_line_offset);
    }
}

pub unsafe fn optc42_disable_pwa(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE((*optc1).tg_regs, OTG_PWA_FRAME_SYNC_CONTROL, OTG_PWA_FRAME_SYNC_EN, 0);
}

pub unsafe fn optc42_clear_optc_underflow(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE((*optc1).tg_regs, OPTC_INPUT_GLOBAL_CONTROL, OPTC_UNDERFLOW_CLEAR, 1);
    REG_UPDATE((*optc1).tg_regs, OPTC_RSMU_UNDERFLOW, OPTC_RSMU_UNDERFLOW_CLEAR, 1);
}

pub unsafe fn optc42_is_optc_underflow_occurred(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    let mut underflow_occurred = 0u32;
    let mut rsmu_underflow_occurred = 0u32;
    REG_GET((*optc1).tg_regs, OPTC_INPUT_GLOBAL_CONTROL, OPTC_UNDERFLOW_OCCURRED_STATUS, &mut underflow_occurred);
    REG_GET((*optc1).tg_regs, OPTC_RSMU_UNDERFLOW, OPTC_RSMU_UNDERFLOW_OCCURRED_STATUS, &mut rsmu_underflow_occurred);
    underflow_occurred == 1 || rsmu_underflow_occurred != 0
}

pub unsafe fn optc42_disable_crtc(optc: *mut timing_generator) -> bool {
    optc401_disable_crtc(optc);
    optc42_clear_optc_underflow(optc);
    true
}

unsafe fn optc42_set_timing_double_buffer(optc: *mut timing_generator, enable: bool) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let mode: u32 = if enable { 2 } else { 0 };
    // The four modes are retained from the previous dcn3x implementation.
    REG_UPDATE((*optc1).tg_regs, OTG_DOUBLE_BUFFER_CONTROL, OTG_DRR_TIMING_DBUF_UPDATE_MODE, mode);
}

pub unsafe fn optc42_tg_init(optc: *mut timing_generator) {
    optc42_set_timing_double_buffer(optc, true);
    optc42_clear_optc_underflow(optc);
}

pub unsafe fn optc42_lock_doublebuffer_enable(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let (mut v_blank_start, mut v_blank_end, mut h_blank_start, mut h_blank_end) = (0u32, 0u32, 0u32, 0u32);
    REG_GET_2((*optc1).tg_regs, OTG_V_BLANK_START_END, OTG_V_BLANK_START, &mut v_blank_start, OTG_V_BLANK_END, &mut v_blank_end);
    REG_GET_2((*optc1).tg_regs, OTG_H_BLANK_START_END, OTG_H_BLANK_START, &mut h_blank_start, OTG_H_BLANK_END, &mut h_blank_end);
    REG_UPDATE_2((*optc1).tg_regs, OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_START_Y, v_blank_start, MASTER_UPDATE_LOCK_DB_END_Y, v_blank_start);
    REG_UPDATE_2((*optc1).tg_regs, OTG_GLOBAL_CONTROL4, DIG_UPDATE_POSITION_X, 20, DIG_UPDATE_POSITION_Y, v_blank_start);
    REG_UPDATE_3((*optc1).tg_regs, OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_START_X, h_blank_start.wrapping_sub(200).wrapping_sub(1), MASTER_UPDATE_LOCK_DB_END_X, h_blank_end, MASTER_UPDATE_LOCK_DB_EN, 1);
    REG_UPDATE((*optc1).tg_regs, OTG_GLOBAL_CONTROL2, GLOBAL_UPDATE_LOCK_EN, 1);
    REG_SET_3((*optc1).tg_regs, OTG_VUPDATE_KEEPOUT, 0, MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_START_OFFSET, 0, MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_END_OFFSET, 100, OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN, 1);
    TRACE_OPTC_LOCK_UNLOCK_STATE(optc1, (*optc).inst, true);
}

static mut dcn42_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: optc1_validate_timing,
    program_timing: optc1_program_timing,
    setup_vertical_interrupt0: optc1_setup_vertical_interrupt0,
    setup_vertical_interrupt1: optc1_setup_vertical_interrupt1,
    setup_vertical_interrupt2: optc1_setup_vertical_interrupt2,
    program_global_sync: optc401_program_global_sync,
    enable_crtc: optc401_enable_crtc,
    disable_crtc: optc42_disable_crtc,
    phantom_crtc_post_enable: optc401_phantom_crtc_post_enable,
    disable_phantom_crtc: optc401_disable_phantom_otg,
    // used by enable_timing_synchronization. Not needed for FPGA
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
    lock_doublebuffer_enable: optc42_lock_doublebuffer_enable,
    lock_doublebuffer_disable: optc3_lock_doublebuffer_disable,
    enable_optc_clock: optc1_enable_optc_clock,
    set_drr: optc401_set_drr,
    get_last_used_drr_vtotal: optc2_get_last_used_drr_vtotal,
    set_vtotal_min_max: optc401_set_vtotal_min_max,
    set_static_screen_control: optc1_set_static_screen_control,
    program_stereo: optc1_program_stereo,
    is_stereo_left_eye: optc1_is_stereo_left_eye,
    tg_init: optc42_tg_init,
    is_tg_enabled: optc1_is_tg_enabled,
    is_optc_underflow_occurred: optc42_is_optc_underflow_occurred,
    clear_optc_underflow: optc42_clear_optc_underflow,
    setup_global_swap_lock: None,
    get_crc: optc42_get_crc,
    configure_crc: optc35_configure_crc,
    set_dsc_config: optc3_set_dsc_config,
    get_dsc_status: optc2_get_dsc_status,
    set_dwb_source: None,
    set_odm_bypass: optc401_set_odm_bypass,
    set_odm_combine: optc401_set_odm_combine,
    wait_odm_doublebuffer_pending_clear: optc32_wait_odm_doublebuffer_pending_clear,
    set_h_timing_div_manual_mode: optc401_set_h_timing_div_manual_mode,
    get_optc_source: optc2_get_optc_source,
    wait_otg_disable: optc35_wait_otg_disable,
    set_out_mux: optc401_set_out_mux,
    set_drr_trigger_window: optc3_set_drr_trigger_window,
    set_vtotal_change_limit: optc3_set_vtotal_change_limit,
    set_gsl: optc2_set_gsl,
    set_gsl_source_select: optc2_set_gsl_source_select,
    set_vtg_params: optc1_set_vtg_params,
    program_manual_trigger: optc2_program_manual_trigger,
    setup_manual_trigger: optc2_setup_manual_trigger,
    get_hw_timing: optc1_get_hw_timing,
    init_odm: optc3_init_odm,
    set_long_vtotal: optc35_set_long_vtotal,
    is_two_pixels_per_container: optc1_is_two_pixels_per_container,
    get_optc_double_buffer_pending: optc3_get_optc_double_buffer_pending,
    get_otg_double_buffer_pending: optc3_get_otg_update_pending,
    get_pipe_update_pending: optc3_get_pipe_update_pending,
    set_vupdate_keepout: optc401_set_vupdate_keepout,
    wait_update_lock_status: optc401_wait_update_lock_status,
    optc_read_reg_state: optc31_read_reg_state,
    read_otg_state: optc31_read_otg_state,
    enable_otg_pwa: optc42_enable_pwa,
    disable_otg_pwa: optc42_disable_pwa,
};

pub unsafe fn dcn42_timing_generator_init(optc1: *mut optc) {
    (*optc1).base.funcs = &dcn42_tg_funcs;
    (*optc1).max_h_total = (*(*optc1).tg_mask).OTG_H_TOTAL + 1;
    (*optc1).max_v_total = (*(*optc1).tg_mask).OTG_V_TOTAL + 1;
    (*optc1).min_h_blank = 32;
    (*optc1).min_v_blank = 3;
    (*optc1).min_v_blank_interlace = 5;
    (*optc1).min_h_sync_width = 4;
    (*optc1).min_v_sync_width = 1;
    dcn35_timing_generator_set_fgcg(optc1, (*(*(*optc1).base.ctx).dc).debug.enable_fine_grain_clock_gating.bits.optc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
