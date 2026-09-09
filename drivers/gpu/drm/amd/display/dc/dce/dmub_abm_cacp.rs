/* Copyright (C) 2022 Advanced Micro Devices, Inc. All rights reserved. */

// External dependencies supplied by the surrounding repository:
// dmub_abm.h, dmub_abm_cacp.h, dce_abm.h, dc.h, dc_dmub_srv.h,
// dmub/dmub_srv.h, and core_types.h.

const CACP_LEVEL_NUM: usize = 4;

pub unsafe fn dmub_cacp_init(
    abm: *mut abm,
    src: *const core::ffi::c_char,
    bytes: u32,
    panel_inst: u32,
) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc: *mut dc_context = (*abm).ctx;
    let panel_mask: u8 = (0x01u8).wrapping_shl(panel_inst);
    let mut edp_links: [*mut dc_link; MAX_NUM_EDP] = [core::ptr::null_mut(); MAX_NUM_EDP];
    let mut i: u32;
    let mut edp_num: u32 = 0;

    // TODO: Optimize by only reading back final 4 bytes
    dmub_srv_flush_buffer_mem(
        (*(*dc).dmub_srv).dmub,
        &mut (*(*(*dc).dmub_srv).dmub).scratch_mem_fb,
    );

    // Copy iramtable into cw7
    core::ptr::copy_nonoverlapping(
        src as *const u8,
        (*(*(*dc).dmub_srv).dmub).scratch_mem_fb.cpu_addr as *mut u8,
        bytes as usize,
    );

    // Fw will copy from cw7 to fw_state
    cmd.cacp_init_config.header.type_ = DMUB_CMD__CACP;
    cmd.cacp_init_config.header.sub_type = DMUB_CMD__CACP_INIT_CONFIG;
    cmd.cacp_init_config.cacp_init_config_data.src.quad_part =
        (*(*(*dc).dmub_srv).dmub).scratch_mem_fb.gpu_addr;
    cmd.cacp_init_config.cacp_init_config_data.bytes = bytes as u16;
    cmd.cacp_init_config.cacp_init_config_data.panel_mask = panel_mask;
    cmd.cacp_init_config.cacp_init_config_data.visual_confirm =
        ((*(*dc).dc).debug.visual_confirm == VISUAL_CONFIRM_ABM);

    cmd.cacp_init_config.header.payload_bytes =
        core::mem::size_of::<dmub_cmd_cacp_init_config_data>();

    dc_get_edp_links((*dc).dc, edp_links.as_mut_ptr(), &mut edp_num);
    i = 0;
    while i < edp_num {
        if panel_inst == i {
            break;
        }
        i += 1;
    }

    if i < edp_num {
        cmd.cacp_init_config.cacp_init_config_data.strscl_valid =
            (*edp_links[panel_inst as usize]).panel_config.cacp.strscl_valid as u8;
        cmd.cacp_init_config.cacp_init_config_data.mode =
            if (*edp_links[panel_inst as usize]).panel_config.cacp.cacp_control_mode {
                DMUB_CMD_CACP_CONTROL_MODE_1
            } else {
                DMUB_CMD_CACP_CONTROL_MODE_0
            };
        for j in 0..CACP_LEVEL_NUM {
            cmd.cacp_init_config.cacp_init_config_data.strscl_sdr[j] =
                (*edp_links[panel_inst as usize]).panel_config.cacp.strscl_sdr[j] as u8;
            cmd.cacp_init_config.cacp_init_config_data.strscl_hdr[j] =
                (*edp_links[panel_inst as usize]).panel_config.cacp.strscl_hdr[j] as u8;
        }
    }

    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

pub unsafe fn dmub_cacp_set_level(
    abm: *mut abm,
    abm_level: u32,
    panel_mask: u8,
) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc: *mut dc_context = (*abm).ctx;

    cmd.cacp_set_level.header.type_ = DMUB_CMD__CACP;
    cmd.cacp_set_level.header.sub_type = DMUB_CMD__CACP_SET_LEVEL;
    cmd.cacp_set_level.cacp_set_level_data.level = abm_level;
    cmd.cacp_set_level.cacp_set_level_data.version = DMUB_CMD_CACP_CONTROL_VERSION_1;
    cmd.cacp_set_level.cacp_set_level_data.panel_mask = panel_mask;
    cmd.cacp_set_level.header.payload_bytes =
        core::mem::size_of::<dmub_cmd_cacp_set_level_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_cacp_set_pipe(
    abm: *mut abm,
    otg_inst: u32,
    pipe_option: u32,
    panel_inst: u32,
    pwrseq_inst: u32,
) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc: *mut dc_context = (*abm).ctx;
    cmd.cacp_set_pipe.header.type_ = DMUB_CMD__CACP;
    cmd.cacp_set_pipe.header.sub_type = DMUB_CMD__CACP_SET_PIPE;
    cmd.cacp_set_pipe.cacp_set_pipe_data.otg_inst = otg_inst as u8;
    cmd.cacp_set_pipe.cacp_set_pipe_data.panel_inst = panel_inst as u8;
    cmd.cacp_set_pipe.cacp_set_pipe_data.set_pipe_option = pipe_option as u8;
    cmd.cacp_set_pipe.cacp_set_pipe_data.pwrseq_inst = pwrseq_inst as u8;
    cmd.cacp_set_pipe.header.payload_bytes = core::mem::size_of::<dmub_cmd_cacp_set_pipe_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_cacp_set_event(
    abm: *mut abm,
    full_screen: u32,
    trans_info: u32,
    hdr_mode: u32,
    scaling_enable: u32,
    panel_inst: u32,
) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc: *mut dc_context = (*abm).ctx;
    // TODO:
    cmd.cacp_set_event.header.type_ = DMUB_CMD__CACP;
    cmd.cacp_set_event.header.sub_type = DMUB_CMD__CACP_SET_EVENT;
    cmd.cacp_set_event.cacp_set_event_data.full_screen_mode = full_screen as u8;
    cmd.cacp_set_event.cacp_set_event_data.trans_info = trans_info;
    cmd.cacp_set_event.cacp_set_event_data.hdr_mode = hdr_mode as u8;
    cmd.cacp_set_event.cacp_set_event_data.vb_scaling_enable = scaling_enable as u8;
    cmd.cacp_set_event.cacp_set_event_data.panel_mask = 1u32.wrapping_shl(panel_inst);
    cmd.cacp_set_event.header.payload_bytes = core::mem::size_of::<dmub_cmd_cacp_set_event_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_cacp_set_pause(
    abm: *mut abm,
    pause: bool,
    panel_inst: u32,
    otg_inst: u32,
) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc: *mut dc_context = (*abm).ctx;
    cmd.cacp_pause.header.type_ = DMUB_CMD__CACP;
    cmd.cacp_pause.header.sub_type = DMUB_CMD__CACP_PAUSE;
    cmd.cacp_pause.cacp_pause_data.panel_mask = 1u32.wrapping_shl(panel_inst);
    cmd.cacp_pause.cacp_pause_data.otg_inst = otg_inst as u8;
    cmd.cacp_pause.cacp_pause_data.enable = pause;
    cmd.cacp_pause.header.payload_bytes = core::mem::size_of::<dmub_cmd_cacp_pause_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_cacp_set_backlight_level(
    abm: *mut abm,
    backlight_pwm_u16_16: u32,
    frame_ramp: u32,
    panel_inst: u32,
) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc: *mut dc_context = (*abm).ctx;
    cmd.cacp_set_backlight.header.type_ = DMUB_CMD__CACP;
    cmd.cacp_set_backlight.header.sub_type = DMUB_CMD__CACP_SET_BACKLIGHT;
    cmd.cacp_set_backlight.cacp_set_backlight_data.frame_ramp = frame_ramp;
    cmd.cacp_set_backlight.cacp_set_backlight_data.backlight_user_level = backlight_pwm_u16_16;
    cmd.cacp_set_backlight.cacp_set_backlight_data.version = DMUB_CMD_CACP_CONTROL_VERSION_1;
    cmd.cacp_set_backlight.cacp_set_backlight_data.panel_mask = 0x01u32.wrapping_shl(panel_inst);
    cmd.cacp_set_backlight.header.payload_bytes =
        core::mem::size_of::<dmub_cmd_cacp_set_backlight_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_cacp_enable_fractional_pwm(abm: *mut abm, panel_mask: u8) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc: *mut dc_context = (*abm).ctx;
    let fractional_pwm: u32 = if !(*(*dc).dc).config.disable_fractional_pwm { 1 } else { 0 };
    cmd.cacp_set_pwm_frac.header.type_ = DMUB_CMD__CACP;
    cmd.cacp_set_pwm_frac.header.sub_type = DMUB_CMD__CACP_SET_PWM_FRAC;
    cmd.cacp_set_pwm_frac.cacp_set_pwm_frac_data.fractional_pwm = fractional_pwm;
    cmd.cacp_set_pwm_frac.cacp_set_pwm_frac_data.version = DMUB_CMD_CACP_CONTROL_VERSION_1;
    cmd.cacp_set_pwm_frac.cacp_set_pwm_frac_data.panel_mask = panel_mask;
    cmd.cacp_set_pwm_frac.header.payload_bytes =
        core::mem::size_of::<dmub_cmd_cacp_set_pwm_frac_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

pub unsafe fn dmub_cacp_get_histogram(
    dc: *mut dc_context,
    panel_inst: u32,
    histogram: *mut u32,
    histogram_type: dmub_abm_histogram_type,
    size: u32,
) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    dmub_srv_flush_buffer_mem(
        (*(*dc).dmub_srv).dmub,
        &mut (*(*(*dc).dmub_srv).dmub).scratch_mem_fb,
    );
    cmd.cacp_get_histogram.header.type_ = DMUB_CMD__CACP;
    cmd.cacp_get_histogram.header.sub_type = DMUB_CMD__CACP_GET_HISTOGRAM;
    cmd.cacp_get_histogram.dest.quad_part = (*(*(*dc).dmub_srv).dmub).scratch_mem_fb.gpu_addr;
    cmd.cacp_get_histogram.bytes = size as u16;
    cmd.cacp_get_histogram.panel_inst = panel_inst as u8;
    cmd.cacp_get_histogram.histogram_type = histogram_type;
    cmd.cacp_get_histogram.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_cacp_get_histogram>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    core::ptr::copy_nonoverlapping(
        (*(*(*dc).dmub_srv).dmub).scratch_mem_fb.cpu_addr as *const u8,
        histogram as *mut u8,
        size as usize,
    );
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
