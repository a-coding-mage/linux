/* Rust translation of dcn31_optc.c. */

// C dependencies supplied by the surrounding translation unit.

unsafe fn optc31_set_odm_combine(optc: *mut timing_generator, opp_id: *mut i32, opp_cnt: i32,
    segment_width: i32, last_segment_width: i32) {
    let _ = last_segment_width;
    let optc1 = DCN10TG_FROM_TG(optc);
    let mut memory_mask: u32 = 0;
    let mem_count_per_opp = (segment_width + 2559) / 2560;

    if opp_cnt == 4 {
        if mem_count_per_opp == 1 { memory_mask = 0xf; }
        else { ASSERT(mem_count_per_opp == 2); memory_mask = 0xff; }
    } else if mem_count_per_opp == 1 {
        memory_mask = 0x1 << ((*opp_id.add(0) * 2) as u32) | 0x1 << ((*opp_id.add(1) * 2) as u32);
    } else if mem_count_per_opp == 2 {
        memory_mask = 0x3 << ((*opp_id.add(0) * 2) as u32) | 0x3 << ((*opp_id.add(1) * 2) as u32);
    } else if mem_count_per_opp == 3 { memory_mask = 0x77; }
    else if mem_count_per_opp == 4 { memory_mask = 0xff; }

    if REG(OPTC_MEMORY_CONFIG) != 0 { REG_SET(OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, memory_mask); }
    if opp_cnt == 2 {
        REG_SET_3(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 1,
            OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1));
    } else if opp_cnt == 4 {
        REG_SET_5(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 3,
            OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1),
            OPTC_SEG2_SRC_SEL, *opp_id.add(2), OPTC_SEG3_SRC_SEL, *opp_id.add(3));
    }
    REG_UPDATE(OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width);
    REG_SET(OTG_H_TIMING_CNTL, 0, OTG_H_TIMING_DIV_MODE, opp_cnt - 1);
    (*optc1).opp_count = opp_cnt;
}

unsafe fn optc31_enable_crtc(optc: *mut timing_generator) -> bool {
    let _optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE(OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, (*optc).inst);
    REG_UPDATE(CONTROL, VTG0_ENABLE, 1);
    REG_UPDATE_2(OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 2, OTG_MASTER_EN, 1);
    true
}

unsafe fn optc31_disable_crtc(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_5(OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, 0xf, OPTC_SEG1_SRC_SEL, 0xf,
        OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf, OPTC_NUM_OF_INPUT_SEGMENT, 0);
    REG_UPDATE(OPTC_MEMORY_CONFIG, OPTC_MEM_SEL, 0);
    REG_UPDATE(OTG_CONTROL, OTG_MASTER_EN, 0);
    REG_UPDATE(CONTROL, VTG0_ENABLE, 0);
    REG_WAIT(OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 100000);
    optc1_clear_optc_underflow(optc);
    true
}

pub unsafe fn optc31_immediate_disable_crtc(optc: *mut timing_generator) -> bool {
    let _optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_2(OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 0, OTG_MASTER_EN, 0);
    REG_UPDATE(CONTROL, VTG0_ENABLE, 0);
    if (*(*optc).ctx).dce_environment != DCE_ENV_DIAG { REG_WAIT(OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 100000); }
    optc1_clear_optc_underflow(optc);
    true
}

pub unsafe fn optc31_set_drr(optc: *mut timing_generator, params: *const drr_params) {
    let _optc1 = DCN10TG_FROM_TG(optc);
    if !params.is_null() && (*params).vertical_total_max > 0 && (*params).vertical_total_min > 0 {
        if (*params).vertical_total_mid != 0 {
            REG_SET(OTG_V_TOTAL_MID, 0, OTG_V_TOTAL_MID, (*params).vertical_total_mid - 1);
            REG_UPDATE_2(OTG_V_TOTAL_CONTROL, OTG_VTOTAL_MID_REPLACING_MAX_EN, 1,
                OTG_VTOTAL_MID_FRAME_NUM, (*params).vertical_total_mid_frame_num as u8);
        }
        (*(*optc).funcs).set_vtotal_min_max(optc, (*params).vertical_total_min - 1, (*params).vertical_total_max - 1);
        REG_UPDATE_4(OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MIN_SEL, 1, OTG_V_TOTAL_MAX_SEL, 1,
            OTG_FORCE_LOCK_ON_EVENT, 0, OTG_SET_V_TOTAL_MIN_MASK, 1 << 1);
        (*(*optc).funcs).setup_manual_trigger(optc);
    } else {
        REG_UPDATE_4(OTG_V_TOTAL_CONTROL, OTG_SET_V_TOTAL_MIN_MASK, 0, OTG_V_TOTAL_MIN_SEL, 0,
            OTG_V_TOTAL_MAX_SEL, 0, OTG_FORCE_LOCK_ON_EVENT, 0);
        (*(*optc).funcs).set_vtotal_min_max(optc, 0, 0);
    }
}

pub unsafe fn optc3_init_odm(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_SET_5(OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 0, OPTC_SEG0_SRC_SEL, (*optc).inst,
        OPTC_SEG1_SRC_SEL, 0xf, OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf);
    REG_SET(OTG_H_TIMING_CNTL, 0, OTG_H_TIMING_DIV_MODE, 0);
    REG_SET(OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, 0);
    (*optc1).opp_count = 1;
}

pub unsafe fn optc31_read_otg_state(optc: *mut timing_generator, s: *mut dcn_otg_state) {
    let _optc1 = DCN10TG_FROM_TG(optc);
    REG_GET(OTG_CONTROL, OTG_MASTER_EN, &mut (*s).otg_enabled);
    REG_GET_2(OTG_V_BLANK_START_END, OTG_V_BLANK_START, &mut (*s).v_blank_start, OTG_V_BLANK_END, &mut (*s).v_blank_end);
    REG_GET(OTG_V_SYNC_A_CNTL, OTG_V_SYNC_A_POL, &mut (*s).v_sync_a_pol);
    REG_GET(OTG_V_TOTAL, OTG_V_TOTAL, &mut (*s).v_total);
    REG_GET(OTG_V_TOTAL_MAX, OTG_V_TOTAL_MAX, &mut (*s).v_total_max);
    REG_GET(OTG_V_TOTAL_MIN, OTG_V_TOTAL_MIN, &mut (*s).v_total_min);
    REG_GET(OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MAX_SEL, &mut (*s).v_total_max_sel);
    REG_GET(OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MIN_SEL, &mut (*s).v_total_min_sel);
    REG_GET_2(OTG_V_SYNC_A, OTG_V_SYNC_A_START, &mut (*s).v_sync_a_start, OTG_V_SYNC_A_END, &mut (*s).v_sync_a_end);
    REG_GET_2(OTG_H_BLANK_START_END, OTG_H_BLANK_START, &mut (*s).h_blank_start, OTG_H_BLANK_END, &mut (*s).h_blank_end);
    REG_GET_2(OTG_H_SYNC_A, OTG_H_SYNC_A_START, &mut (*s).h_sync_a_start, OTG_H_SYNC_A_END, &mut (*s).h_sync_a_end);
    REG_GET(OTG_H_SYNC_A_CNTL, OTG_H_SYNC_A_POL, &mut (*s).h_sync_a_pol);
    REG_GET(OTG_H_TOTAL, OTG_H_TOTAL, &mut (*s).h_total);
    REG_GET(OPTC_INPUT_GLOBAL_CONTROL, OPTC_UNDERFLOW_OCCURRED_STATUS, &mut (*s).underflow_occurred_status);
    REG_GET(OTG_VERTICAL_INTERRUPT1_CONTROL, OTG_VERTICAL_INTERRUPT1_INT_ENABLE, &mut (*s).vertical_interrupt1_en);
    REG_GET(OTG_VERTICAL_INTERRUPT1_POSITION, OTG_VERTICAL_INTERRUPT1_LINE_START, &mut (*s).vertical_interrupt1_line);
    REG_GET(OTG_VERTICAL_INTERRUPT2_CONTROL, OTG_VERTICAL_INTERRUPT2_INT_ENABLE, &mut (*s).vertical_interrupt2_en);
    REG_GET(OTG_VERTICAL_INTERRUPT2_POSITION, OTG_VERTICAL_INTERRUPT2_LINE_START, &mut (*s).vertical_interrupt2_line);
    REG_GET(INTERRUPT_DEST, OTG0_IHC_OTG_VERTICAL_INTERRUPT2_DEST, &mut (*s).vertical_interrupt2_dest);
    (*s).otg_master_update_lock = REG_READ(OTG_MASTER_UPDATE_LOCK);
    (*s).otg_double_buffer_control = REG_READ(OTG_DOUBLE_BUFFER_CONTROL);
}

pub unsafe fn optc31_read_reg_state(optc: *mut timing_generator, r: *mut dcn_optc_reg_state) {
    let _optc1 = DCN10TG_FROM_TG(optc);
    // The C source reads each register into the correspondingly named field.
    macro_rules! rr { ($field:ident, $reg:ident) => { (*r).$field = REG_READ($reg); }; }
    rr!(otg_drr_v_total_reach_range, OTG_DRR_V_TOTAL_REACH_RANGE);
    rr!(optc_bytes_per_pixel, OPTC_BYTES_PER_PIXEL); rr!(optc_data_format_control, OPTC_DATA_FORMAT_CONTROL);
    rr!(optc_data_source_select, OPTC_DATA_SOURCE_SELECT); rr!(optc_input_clock_control, OPTC_INPUT_CLOCK_CONTROL);
    rr!(optc_input_global_control, OPTC_INPUT_GLOBAL_CONTROL); rr!(optc_input_spare_register, OPTC_INPUT_SPARE_REGISTER);
    rr!(optc_memory_config, OPTC_MEMORY_CONFIG); rr!(optc_rsmu_underflow, OPTC_RSMU_UNDERFLOW);
    rr!(optc_underflow_threshold, OPTC_UNDERFLOW_THRESHOLD); rr!(optc_width_control, OPTC_WIDTH_CONTROL);
    rr!(otg_3d_structure_control, OTG_3D_STRUCTURE_CONTROL); rr!(otg_clock_control, OTG_CLOCK_CONTROL);
    rr!(otg_control, OTG_CONTROL); rr!(otg_count_control, OTG_COUNT_CONTROL); rr!(otg_count_reset, OTG_COUNT_RESET);
    rr!(otg_crc_cntl, OTG_CRC_CNTL); rr!(otg_crc_sig_blue_control_mask, OTG_CRC_SIG_BLUE_CONTROL_MASK);
    rr!(otg_crc_sig_red_green_mask, OTG_CRC_SIG_RED_GREEN_MASK); rr!(otg_crc0_data_b, OTG_CRC0_DATA_B); rr!(otg_crc0_data_rg, OTG_CRC0_DATA_RG);
    rr!(otg_crc0_windowa_x_control, OTG_CRC0_WINDOWA_X_CONTROL); rr!(otg_crc0_windowa_x_control_readback, OTG_CRC0_WINDOWA_X_CONTROL_READBACK);
    rr!(otg_crc0_windowa_y_control, OTG_CRC0_WINDOWA_Y_CONTROL); rr!(otg_crc0_windowa_y_control_readback, OTG_CRC0_WINDOWA_Y_CONTROL_READBACK);
    rr!(otg_crc0_windowb_x_control, OTG_CRC0_WINDOWB_X_CONTROL); rr!(otg_crc0_windowb_x_control_readback, OTG_CRC0_WINDOWB_X_CONTROL_READBACK);
    rr!(otg_crc0_windowb_y_control, OTG_CRC0_WINDOWB_Y_CONTROL); rr!(otg_crc0_windowb_y_control_readback, OTG_CRC0_WINDOWB_Y_CONTROL_READBACK);
    rr!(otg_crc1_data_b, OTG_CRC1_DATA_B); rr!(otg_crc1_data_rg, OTG_CRC1_DATA_RG);
    rr!(otg_crc1_windowa_x_control, OTG_CRC1_WINDOWA_X_CONTROL); rr!(otg_crc1_windowa_x_control_readback, OTG_CRC1_WINDOWA_X_CONTROL_READBACK);
    rr!(otg_crc1_windowa_y_control, OTG_CRC1_WINDOWA_Y_CONTROL); rr!(otg_crc1_windowa_y_control_readback, OTG_CRC1_WINDOWA_Y_CONTROL_READBACK);
    rr!(otg_crc1_windowb_x_control, OTG_CRC1_WINDOWB_X_CONTROL); rr!(otg_crc1_windowb_x_control_readback, OTG_CRC1_WINDOWB_X_CONTROL_READBACK);
    rr!(otg_crc1_windowb_y_control, OTG_CRC1_WINDOWB_Y_CONTROL); rr!(otg_crc1_windowb_y_control_readback, OTG_CRC1_WINDOWB_Y_CONTROL_READBACK);
    rr!(otg_crc2_data_b, OTG_CRC2_DATA_B); rr!(otg_crc2_data_rg, OTG_CRC2_DATA_RG); rr!(otg_crc3_data_b, OTG_CRC3_DATA_B); rr!(otg_crc3_data_rg, OTG_CRC3_DATA_RG);
    rr!(otg_dlpc_control, OTG_DLPC_CONTROL); rr!(otg_double_buffer_control, OTG_DOUBLE_BUFFER_CONTROL); rr!(otg_drr_control2, OTG_DRR_CONTOL2); rr!(otg_drr_control, OTG_DRR_CONTROL);
    rr!(otg_drr_timing_int_status, OTG_DRR_TIMING_INT_STATUS); rr!(otg_drr_trigger_window, OTG_DRR_TRIGGER_WINDOW); rr!(otg_drr_v_total_change, OTG_DRR_V_TOTAL_CHANGE); rr!(otg_dsc_start_position, OTG_DSC_START_POSITION);
    rr!(otg_force_count_now_cntl, OTG_FORCE_COUNT_NOW_CNTL); rr!(otg_global_control0, OTG_GLOBAL_CONTROL0); rr!(otg_global_control1, OTG_GLOBAL_CONTROL1); rr!(otg_global_control2, OTG_GLOBAL_CONTROL2); rr!(otg_global_control3, OTG_GLOBAL_CONTROL3); rr!(otg_global_control4, OTG_GLOBAL_CONTROL4); rr!(otg_global_sync_status, OTG_GLOBAL_SYNC_STATUS);
    rr!(otg_gsl_control, OTG_GSL_CONTROL); rr!(otg_gsl_vsync_gap, OTG_GSL_VSYNC_GAP); rr!(otg_gsl_window_x, OTG_GSL_WINDOW_X); rr!(otg_gsl_window_y, OTG_GSL_WINDOW_Y);
    rr!(otg_h_blank_start_end, OTG_H_BLANK_START_END); rr!(otg_h_sync_a, OTG_H_SYNC_A); rr!(otg_h_sync_a_cntl, OTG_H_SYNC_A_CNTL); rr!(otg_h_timing_cntl, OTG_H_TIMING_CNTL); rr!(otg_h_total, OTG_H_TOTAL);
    rr!(otg_interlace_control, OTG_INTERLACE_CONTROL); rr!(otg_interlace_status, OTG_INTERLACE_STATUS); rr!(otg_interrupt_control, OTG_INTERRUPT_CONTROL); rr!(otg_long_vblank_status, OTG_LONG_VBLANK_STATUS); rr!(otg_m_const_dto0, OTG_M_CONST_DTO0); rr!(otg_m_const_dto1, OTG_M_CONST_DTO1); rr!(otg_manual_force_vsync_next_line, OTG_MANUAL_FORCE_VSYNC_NEXT_LINE); rr!(otg_master_en, OTG_MASTER_EN); rr!(otg_master_update_lock, OTG_MASTER_UPDATE_LOCK); rr!(otg_master_update_mode, OTG_MASTER_UPDATE_MODE); rr!(otg_nom_vert_position, OTG_NOM_VERT_POSITION); rr!(otg_pipe_update_status, OTG_PIPE_UPDATE_STATUS);
    rr!(otg_pixel_data_readback0, OTG_PIXEL_DATA_READBACK0); rr!(otg_pixel_data_readback1, OTG_PIXEL_DATA_READBACK1); rr!(otg_request_control, OTG_REQUEST_CONTROL); rr!(otg_snapshot_control, OTG_SNAPSHOT_CONTROL); rr!(otg_snapshot_frame, OTG_SNAPSHOT_FRAME); rr!(otg_snapshot_position, OTG_SNAPSHOT_POSITION); rr!(otg_snapshot_status, OTG_SNAPSHOT_STATUS); rr!(otg_spare_register, OTG_SPARE_REGISTER); rr!(otg_static_screen_control, OTG_STATIC_SCREEN_CONTROL); rr!(otg_status, OTG_STATUS); rr!(otg_status_frame_count, OTG_STATUS_FRAME_COUNT); rr!(otg_status_hv_count, OTG_STATUS_HV_COUNT); rr!(otg_status_position, OTG_STATUS_POSITION); rr!(otg_status_vf_count, OTG_STATUS_VF_COUNT);
    rr!(otg_stereo_control, OTG_STEREO_CONTROL); rr!(otg_stereo_force_next_eye, OTG_STEREO_FORCE_NEXT_EYE); rr!(otg_stereo_status, OTG_STEREO_STATUS); rr!(otg_trig_manual_control, OTG_TRIG_MANUAL_CONTROL); rr!(otg_triga_cntl, OTG_TRIGA_CNTL); rr!(otg_triga_manual_trig, OTG_TRIGA_MANUAL_TRIG); rr!(otg_trigb_cntl, OTG_TRIGB_CNTL); rr!(otg_trigb_manual_trig, OTG_TRIGB_MANUAL_TRIG); rr!(otg_update_lock, OTG_UPDATE_LOCK); rr!(otg_v_blank_start_end, OTG_V_BLANK_START_END); rr!(otg_v_count_stop_control, OTG_V_COUNT_STOP_CONTROL); rr!(otg_v_count_stop_control2, OTG_V_COUNT_STOP_CONTROL2); rr!(otg_v_sync_a, OTG_V_SYNC_A); rr!(otg_v_sync_a_cntl, OTG_V_SYNC_A_CNTL); rr!(otg_v_total, OTG_V_TOTAL); rr!(otg_v_total_control, OTG_V_TOTAL_CONTROL); rr!(otg_v_total_int_status, OTG_V_TOTAL_INT_STATUS); rr!(otg_v_total_max, OTG_V_TOTAL_MAX); rr!(otg_v_total_mid, OTG_V_TOTAL_MID); rr!(otg_v_total_min, OTG_V_TOTAL_MIN); rr!(otg_vert_sync_control, OTG_VERT_SYNC_CONTROL); rr!(otg_vertical_interrupt0_control, OTG_VERTICAL_INTERRUPT0_CONTROL); rr!(otg_vertical_interrupt0_position, OTG_VERTICAL_INTERRUPT0_POSITION); rr!(otg_vertical_interrupt1_control, OTG_VERTICAL_INTERRUPT1_CONTROL); rr!(otg_vertical_interrupt1_position, OTG_VERTICAL_INTERRUPT1_POSITION); rr!(otg_vertical_interrupt2_control, OTG_VERTICAL_INTERRUPT2_CONTROL); rr!(otg_vertical_interrupt2_position, OTG_VERTICAL_INTERRUPT2_POSITION); rr!(otg_vready_param, OTG_VREADY_PARAM); rr!(otg_vstartup_param, OTG_VSTARTUP_PARAM); rr!(otg_vsync_nom_int_status, OTG_VSYNC_NOM_INT_STATUS); rr!(otg_vupdate_keepout, OTG_VUPDATE_KEEPOUT); rr!(otg_vupdate_param, OTG_VUPDATE_PARAM);
}

static mut dcn31_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: Some(optc1_validate_timing), program_timing: Some(optc1_program_timing),
    setup_vertical_interrupt0: Some(optc1_setup_vertical_interrupt0), setup_vertical_interrupt1: Some(optc1_setup_vertical_interrupt1),
    setup_vertical_interrupt2: Some(optc1_setup_vertical_interrupt2), program_global_sync: Some(optc1_program_global_sync),
    enable_crtc: Some(optc31_enable_crtc), disable_crtc: Some(optc31_disable_crtc), immediate_disable_crtc: Some(optc31_immediate_disable_crtc),
    is_counter_moving: Some(optc1_is_counter_moving), get_position: Some(optc1_get_position), get_frame_count: Some(optc1_get_vblank_counter),
    get_scanoutpos: Some(optc1_get_crtc_scanoutpos), get_otg_active_size: Some(optc1_get_otg_active_size), set_early_control: Some(optc1_set_early_control),
    wait_for_state: Some(optc1_wait_for_state), set_blank_color: Some(optc3_program_blank_color), did_triggered_reset_occur: Some(optc1_did_triggered_reset_occur),
    triplebuffer_lock: Some(optc3_triplebuffer_lock), triplebuffer_unlock: Some(optc2_triplebuffer_unlock), enable_reset_trigger: Some(optc1_enable_reset_trigger),
    enable_crtc_reset: Some(optc1_enable_crtc_reset), disable_reset_trigger: Some(optc1_disable_reset_trigger), lock: Some(optc3_lock), unlock: Some(optc1_unlock),
    lock_doublebuffer_enable: Some(optc3_lock_doublebuffer_enable), lock_doublebuffer_disable: Some(optc3_lock_doublebuffer_disable), enable_optc_clock: Some(optc1_enable_optc_clock),
    set_drr: Some(optc31_set_drr), get_last_used_drr_vtotal: Some(optc2_get_last_used_drr_vtotal), set_vtotal_min_max: Some(optc1_set_vtotal_min_max),
    set_static_screen_control: Some(optc1_set_static_screen_control), program_stereo: Some(optc1_program_stereo), is_stereo_left_eye: Some(optc1_is_stereo_left_eye),
    tg_init: Some(optc3_tg_init), is_tg_enabled: Some(optc1_is_tg_enabled), is_optc_underflow_occurred: Some(optc1_is_optc_underflow_occurred),
    clear_optc_underflow: Some(optc1_clear_optc_underflow), setup_global_swap_lock: None, get_crc: Some(optc1_get_crc), configure_crc: Some(optc2_configure_crc),
    set_dsc_config: Some(optc3_set_dsc_config), get_dsc_status: Some(optc2_get_dsc_status), set_dwb_source: None, set_odm_bypass: Some(optc3_set_odm_bypass),
    set_odm_combine: Some(optc31_set_odm_combine), get_optc_source: Some(optc2_get_optc_source), set_out_mux: Some(optc3_set_out_mux),
    set_drr_trigger_window: Some(optc3_set_drr_trigger_window), set_vtotal_change_limit: Some(optc3_set_vtotal_change_limit), set_gsl: Some(optc2_set_gsl),
    set_gsl_source_select: Some(optc2_set_gsl_source_select), set_vtg_params: Some(optc1_set_vtg_params), program_manual_trigger: Some(optc2_program_manual_trigger),
    setup_manual_trigger: Some(optc2_setup_manual_trigger), get_hw_timing: Some(optc1_get_hw_timing), init_odm: Some(optc3_init_odm),
    is_two_pixels_per_container: Some(optc1_is_two_pixels_per_container), read_otg_state: Some(optc31_read_otg_state), optc_read_reg_state: Some(optc31_read_reg_state),
};

pub unsafe fn dcn31_timing_generator_init(optc1: *mut optc) {
    (*optc1).base.funcs = &dcn31_tg_funcs;
    (*optc1).max_h_total = (*optc1).tg_mask.OTG_H_TOTAL + 1;
    (*optc1).max_v_total = (*optc1).tg_mask.OTG_V_TOTAL + 1;
    (*optc1).min_h_blank = 32; (*optc1).min_v_blank = 3; (*optc1).min_v_blank_interlace = 5;
    (*optc1).min_h_sync_width = 4; (*optc1).min_v_sync_width = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
