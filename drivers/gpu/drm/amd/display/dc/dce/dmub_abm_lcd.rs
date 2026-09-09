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

// Dependencies supplied by the surrounding translation unit.

const DISABLE_ABM_IMMEDIATELY: u32 = 255;

unsafe fn dmub_abm_enable_fractional_pwm(dc: *mut dc_context) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let fractional_pwm: u32 = if (*(*dc).dc).config.disable_fractional_pwm == false { 1 } else { 0 };
    let edp_id_count = (*(*dc).dc).dc_edp_id_count;
    let mut panel_mask: u8 = 0;

    for i in 0..edp_id_count {
        panel_mask |= 0x01u8 << i;
    }

    cmd.abm_set_pwm_frac.header.type_ = DMUB_CMD__ABM;
    cmd.abm_set_pwm_frac.header.sub_type = DMUB_CMD__ABM_SET_PWM_FRAC;
    cmd.abm_set_pwm_frac.abm_set_pwm_frac_data.fractional_pwm = fractional_pwm;
    cmd.abm_set_pwm_frac.abm_set_pwm_frac_data.version = DMUB_CMD_ABM_CONTROL_VERSION_1;
    cmd.abm_set_pwm_frac.abm_set_pwm_frac_data.panel_mask = panel_mask;
    cmd.abm_set_pwm_frac.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_set_pwm_frac_data>();

    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

pub unsafe fn dmub_abm_init(abm: *mut abm, backlight: u32, user_level: u32) {
    let dce_abm = TO_DMUB_ABM(abm);

    REG_WRITE!(dce_abm, DC_ABM1_HG_SAMPLE_RATE, 0x3);
    REG_WRITE!(dce_abm, DC_ABM1_HG_SAMPLE_RATE, 0x1);
    REG_WRITE!(dce_abm, DC_ABM1_LS_SAMPLE_RATE, 0x3);
    REG_WRITE!(dce_abm, DC_ABM1_LS_SAMPLE_RATE, 0x1);
    REG_WRITE!(dce_abm, BL1_PWM_BL_UPDATE_SAMPLE_RATE, 0x1);

    REG_SET_3!(dce_abm, DC_ABM1_HG_MISC_CTRL, 0,
        ABM1_HG_NUM_OF_BINS_SEL, 0, ABM1_HG_VMAX_SEL, 1,
        ABM1_HG_BIN_BITWIDTH_SIZE_SEL, 0);
    REG_SET_3!(dce_abm, DC_ABM1_IPCSC_COEFF_SEL, 0,
        ABM1_IPCSC_COEFF_SEL_R, 2, ABM1_IPCSC_COEFF_SEL_G, 4,
        ABM1_IPCSC_COEFF_SEL_B, 2);
    REG_UPDATE!(dce_abm, BL1_PWM_CURRENT_ABM_LEVEL, BL1_PWM_CURRENT_ABM_LEVEL, backlight);
    REG_UPDATE!(dce_abm, BL1_PWM_TARGET_ABM_LEVEL, BL1_PWM_TARGET_ABM_LEVEL, backlight);
    REG_UPDATE!(dce_abm, BL1_PWM_USER_LEVEL, BL1_PWM_USER_LEVEL, user_level);
    REG_UPDATE_2!(dce_abm, DC_ABM1_LS_MIN_MAX_PIXEL_VALUE_THRES,
        ABM1_LS_MIN_PIXEL_VALUE_THRES, 0, ABM1_LS_MAX_PIXEL_VALUE_THRES, 1000);
    REG_SET_3!(dce_abm, DC_ABM1_HGLS_REG_READ_PROGRESS, 0,
        ABM1_HG_REG_READ_MISSED_FRAME_CLEAR, 1,
        ABM1_LS_REG_READ_MISSED_FRAME_CLEAR, 1,
        ABM1_BL_REG_READ_MISSED_FRAME_CLEAR, 1);

    dmub_abm_enable_fractional_pwm((*abm).ctx);
}

pub unsafe fn dmub_abm_get_current_backlight(abm: *mut abm) -> u32 {
    let dce_abm = TO_DMUB_ABM(abm);
    let backlight = REG_READ!(dce_abm, BL1_PWM_CURRENT_ABM_LEVEL);
    // Return backlight in hardware format: unsigned 17 bits, with 1 integer
    // bit and 16 fractional bits.
    backlight
}

pub unsafe fn dmub_abm_get_target_backlight(abm: *mut abm) -> u32 {
    let dce_abm = TO_DMUB_ABM(abm);
    let backlight = REG_READ!(dce_abm, BL1_PWM_TARGET_ABM_LEVEL);
    // Return backlight in hardware format: unsigned 17 bits, with 1 integer
    // bit and 16 fractional bits.
    backlight
}

pub unsafe fn dmub_abm_set_level(abm: *mut abm, level: u32, panel_mask: u8) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*abm).ctx;
    cmd.abm_set_level.header.type_ = DMUB_CMD__ABM;
    cmd.abm_set_level.header.sub_type = DMUB_CMD__ABM_SET_LEVEL;
    cmd.abm_set_level.abm_set_level_data.level = level;
    cmd.abm_set_level.abm_set_level_data.version = DMUB_CMD_ABM_CONTROL_VERSION_1;
    cmd.abm_set_level.abm_set_level_data.panel_mask = panel_mask;
    cmd.abm_set_level.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_set_level_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_abm_init_config(abm: *mut abm, src: *const i8, bytes: u32, inst: u32) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*abm).ctx;
    let panel_mask = 0x01u8 << inst;
    dmub_srv_flush_buffer_mem((*dc).dmub_srv.dmub, &mut (*(*dc).dmub_srv.dmub).scratch_mem_fb);
    // TODO: Optimize by only reading back final 4 bytes.
    core::ptr::copy_nonoverlapping(src as *const u8,
        (*(*dc).dmub_srv.dmub).scratch_mem_fb.cpu_addr as *mut u8, bytes as usize);
    // Firmware will copy from cw7 to fw_state.
    cmd.abm_init_config.header.type_ = DMUB_CMD__ABM;
    cmd.abm_init_config.header.sub_type = DMUB_CMD__ABM_INIT_CONFIG;
    cmd.abm_init_config.abm_init_config_data.src.quad_part = (*(*dc).dmub_srv.dmub).scratch_mem_fb.gpu_addr;
    cmd.abm_init_config.abm_init_config_data.bytes = bytes as u16;
    cmd.abm_init_config.abm_init_config_data.version = DMUB_CMD_ABM_CONTROL_VERSION_1;
    cmd.abm_init_config.abm_init_config_data.panel_mask = panel_mask;
    cmd.abm_init_config.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_init_config_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

pub unsafe fn dmub_abm_set_pause(abm: *mut abm, pause: bool, panel_inst: u32, stream_inst: u32) -> bool {
    let _ = stream_inst;
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*abm).ctx;
    let panel_mask = 0x01u8 << panel_inst;
    cmd.abm_pause.header.type_ = DMUB_CMD__ABM;
    cmd.abm_pause.header.sub_type = DMUB_CMD__ABM_PAUSE;
    cmd.abm_pause.abm_pause_data.enable = pause;
    cmd.abm_pause.abm_pause_data.panel_mask = panel_mask;
    cmd.abm_set_level.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_pause_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_abm_save_restore(dc: *mut dc_context, panel_inst: u32, p_data: *mut abm_save_restore) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let panel_mask = 0x01u8 << panel_inst;
    let bytes = core::mem::size_of::<abm_save_restore>();
    dmub_srv_flush_buffer_mem((*dc).dmub_srv.dmub, &mut (*(*dc).dmub_srv.dmub).scratch_mem_fb);
    core::ptr::copy_nonoverlapping(p_data as *const u8,
        (*(*dc).dmub_srv.dmub).scratch_mem_fb.cpu_addr as *mut u8, bytes);
    cmd.abm_save_restore.header.type_ = DMUB_CMD__ABM;
    cmd.abm_save_restore.header.sub_type = DMUB_CMD__ABM_SAVE_RESTORE;
    cmd.abm_save_restore.abm_init_config_data.src.quad_part = (*(*dc).dmub_srv.dmub).scratch_mem_fb.gpu_addr;
    cmd.abm_save_restore.abm_init_config_data.bytes = bytes as u16;
    cmd.abm_save_restore.abm_init_config_data.version = DMUB_CMD_ABM_CONTROL_VERSION_1;
    cmd.abm_save_restore.abm_init_config_data.panel_mask = panel_mask;
    cmd.abm_save_restore.header.payload_bytes = core::mem::size_of::<dmub_rb_cmd_abm_save_restore>() - core::mem::size_of::<dmub_cmd_header>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    core::ptr::copy_nonoverlapping((*(*dc).dmub_srv.dmub).scratch_mem_fb.cpu_addr as *const u8, p_data as *mut u8, bytes);
    true
}

pub unsafe fn dmub_abm_set_pipe(abm: *mut abm, otg_inst: u32, option: u32, panel_inst: u32, pwrseq_inst: u32) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*abm).ctx;
    cmd.abm_set_pipe.header.type_ = DMUB_CMD__ABM;
    cmd.abm_set_pipe.header.sub_type = DMUB_CMD__ABM_SET_PIPE;
    cmd.abm_set_pipe.abm_set_pipe_data.otg_inst = otg_inst as u8;
    cmd.abm_set_pipe.abm_set_pipe_data.pwrseq_inst = pwrseq_inst as u8;
    cmd.abm_set_pipe.abm_set_pipe_data.set_pipe_option = option as u8;
    cmd.abm_set_pipe.abm_set_pipe_data.panel_inst = panel_inst as u8;
    cmd.abm_set_pipe.abm_set_pipe_data.ramping_boundary = 0xFF;
    cmd.abm_set_pipe.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_set_pipe_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_abm_set_backlight_level(abm: *mut abm, backlight_pwm_u16_16: u32, frame_ramp: u32, panel_inst: u32) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*abm).ctx;
    cmd.abm_set_backlight.header.type_ = DMUB_CMD__ABM;
    cmd.abm_set_backlight.header.sub_type = DMUB_CMD__ABM_SET_BACKLIGHT;
    cmd.abm_set_backlight.abm_set_backlight_data.frame_ramp = frame_ramp;
    cmd.abm_set_backlight.abm_set_backlight_data.backlight_user_level = backlight_pwm_u16_16;
    cmd.abm_set_backlight.abm_set_backlight_data.version = DMUB_CMD_ABM_CONTROL_VERSION_1;
    cmd.abm_set_backlight.abm_set_backlight_data.panel_mask = 0x01u8 << panel_inst;
    cmd.abm_set_backlight.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_set_backlight_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

pub unsafe fn dmub_abm_set_event(abm: *mut abm, scaling_enable: u32, scaling_strength_map: u32, panel_inst: u32) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*abm).ctx;
    cmd.abm_set_event.header.type_ = DMUB_CMD__ABM;
    cmd.abm_set_event.header.sub_type = DMUB_CMD__ABM_SET_EVENT;
    cmd.abm_set_event.abm_set_event_data.vb_scaling_enable = scaling_enable as u8;
    cmd.abm_set_event.abm_set_event_data.vb_scaling_strength_mapping = scaling_strength_map;
    cmd.abm_set_event.abm_set_event_data.panel_mask = 1u8 << panel_inst;
    cmd.abm_set_event.header.payload_bytes = core::mem::size_of::<dmub_cmd_abm_set_event_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
