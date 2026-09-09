/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies and register-helper macros are supplied by the surrounding
// kernel/display implementation.

const MCP_ABM_LEVEL_SET: u32 = 0x65;
const MCP_ABM_PIPE_SET: u32 = 0x66;
const MCP_BL_SET: u32 = 0x67;
const MCP_DISABLE_ABM_IMMEDIATELY: u32 = 255;

#[repr(C)]
pub struct dce_abm {
    pub base: abm,
    pub regs: *const dce_abm_registers,
    pub abm_shift: *const dce_abm_shift,
    pub abm_mask: *const dce_abm_mask,
}

unsafe fn to_dce_abm(abm: *mut abm) -> *mut dce_abm {
    abm as *mut dce_abm
}

unsafe fn dce_abm_set_pipe(abm: *mut abm, controller_id: u32, panel_inst: u32) -> bool {
    let _ = panel_inst;
    let abm_dce = &mut *to_dce_abm(abm);
    let ramping_boundary: u32 = 0xffff;

    if !(*abm).dmcu_is_running {
        return true;
    }

    REG_WAIT!(abm_dce, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 0, 1, 80000);
    REG_WRITE!(abm_dce, MASTER_COMM_DATA_REG1, ramping_boundary);
    REG_UPDATE_2!(abm_dce, MASTER_COMM_CMD_REG,
        MASTER_COMM_CMD_REG_BYTE0, MCP_ABM_PIPE_SET,
        MASTER_COMM_CMD_REG_BYTE1, controller_id);
    REG_UPDATE!(abm_dce, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 1);
    REG_WAIT!(abm_dce, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 0, 1, 80000);
    true
}

unsafe fn dmcu_set_backlight_level(
    abm_dce: *mut dce_abm,
    backlight_pwm_u16_16: u32,
    mut frame_ramp: u32,
    controller_id: u32,
    panel_id: u32,
) {
    let mut backlight_8_bit: u32 = 0;
    let mut s2: u32;

    if backlight_pwm_u16_16 & 0x10000 != 0 {
        backlight_8_bit = 0xff;
    } else {
        backlight_8_bit = (backlight_pwm_u16_16 >> 8) & 0xff;
    }

    dce_abm_set_pipe(&mut (*abm_dce).base, controller_id, panel_id);
    REG_WAIT!(&mut *abm_dce, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 0, 1, 80000);
    REG_UPDATE!(&mut *abm_dce, BL1_PWM_USER_LEVEL, BL1_PWM_USER_LEVEL, backlight_pwm_u16_16);

    if controller_id == 0 {
        frame_ramp = 0;
    }
    REG_WRITE!(&mut *abm_dce, MASTER_COMM_DATA_REG1, frame_ramp);
    REG_UPDATE!(&mut *abm_dce, MASTER_COMM_CMD_REG, MASTER_COMM_CMD_REG_BYTE0, MCP_BL_SET);
    REG_UPDATE!(&mut *abm_dce, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 1);

    s2 = REG_READ!(&mut *abm_dce, BIOS_SCRATCH_2);
    s2 &= !ATOM_S2_CURRENT_BL_LEVEL_MASK;
    backlight_8_bit &= ATOM_S2_CURRENT_BL_LEVEL_MASK >> ATOM_S2_CURRENT_BL_LEVEL_SHIFT;
    s2 |= backlight_8_bit << ATOM_S2_CURRENT_BL_LEVEL_SHIFT;
    REG_WRITE!(&mut *abm_dce, BIOS_SCRATCH_2, s2);
    REG_WAIT!(&mut *abm_dce, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 0, 1, 80000);
}

unsafe fn dce_abm_init(abm: *mut abm, backlight: u32, user_level: u32) {
    let abm_dce = &mut *to_dce_abm(abm);
    REG_WRITE!(abm_dce, DC_ABM1_HG_SAMPLE_RATE, 0x103);
    REG_WRITE!(abm_dce, DC_ABM1_HG_SAMPLE_RATE, 0x101);
    REG_WRITE!(abm_dce, DC_ABM1_LS_SAMPLE_RATE, 0x103);
    REG_WRITE!(abm_dce, DC_ABM1_LS_SAMPLE_RATE, 0x101);
    REG_WRITE!(abm_dce, BL1_PWM_BL_UPDATE_SAMPLE_RATE, 0x101);
    REG_SET_3!(abm_dce, DC_ABM1_HG_MISC_CTRL, 0,
        ABM1_HG_NUM_OF_BINS_SEL, 0, ABM1_HG_VMAX_SEL, 1,
        ABM1_HG_BIN_BITWIDTH_SIZE_SEL, 0);
    REG_SET_3!(abm_dce, DC_ABM1_IPCSC_COEFF_SEL, 0,
        ABM1_IPCSC_COEFF_SEL_R, 2, ABM1_IPCSC_COEFF_SEL_G, 4,
        ABM1_IPCSC_COEFF_SEL_B, 2);
    REG_UPDATE!(abm_dce, BL1_PWM_CURRENT_ABM_LEVEL, BL1_PWM_CURRENT_ABM_LEVEL, backlight);
    REG_UPDATE!(abm_dce, BL1_PWM_TARGET_ABM_LEVEL, BL1_PWM_TARGET_ABM_LEVEL, backlight);
    REG_UPDATE!(abm_dce, BL1_PWM_USER_LEVEL, BL1_PWM_USER_LEVEL, user_level);
    REG_UPDATE_2!(abm_dce, DC_ABM1_LS_MIN_MAX_PIXEL_VALUE_THRES,
        ABM1_LS_MIN_PIXEL_VALUE_THRES, 0, ABM1_LS_MAX_PIXEL_VALUE_THRES, 1000);
    REG_SET_3!(abm_dce, DC_ABM1_HGLS_REG_READ_PROGRESS, 0,
        ABM1_HG_REG_READ_MISSED_FRAME_CLEAR, 1,
        ABM1_LS_REG_READ_MISSED_FRAME_CLEAR, 1,
        ABM1_BL_REG_READ_MISSED_FRAME_CLEAR, 1);
}

unsafe fn dce_abm_get_current_backlight(abm: *mut abm) -> u32 {
    REG_READ!(&mut *to_dce_abm(abm), BL1_PWM_CURRENT_ABM_LEVEL)
}

unsafe fn dce_abm_get_target_backlight(abm: *mut abm) -> u32 {
    REG_READ!(&mut *to_dce_abm(abm), BL1_PWM_TARGET_ABM_LEVEL)
}

unsafe fn dce_abm_set_level(abm: *mut abm, level: u32) -> bool {
    let abm_dce = &mut *to_dce_abm(abm);
    if !(*abm).dmcu_is_running { return true; }
    REG_WAIT!(abm_dce, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 0, 1, 80000);
    REG_UPDATE_2!(abm_dce, MASTER_COMM_CMD_REG,
        MASTER_COMM_CMD_REG_BYTE0, MCP_ABM_LEVEL_SET,
        MASTER_COMM_CMD_REG_BYTE2, level);
    REG_UPDATE!(abm_dce, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 1);
    true
}

unsafe fn dce_abm_immediate_disable(abm: *mut abm, panel_inst: u32) -> bool {
    if !(*abm).dmcu_is_running { return true; }
    dce_abm_set_pipe(abm, MCP_DISABLE_ABM_IMMEDIATELY, panel_inst);
    true
}

unsafe fn dce_abm_set_backlight_level_pwm(
    abm: *mut abm, backlight_pwm_u16_16: u32, frame_ramp: u32,
    controller_id: u32, panel_inst: u32,
) -> bool {
    dmcu_set_backlight_level(to_dce_abm(abm), backlight_pwm_u16_16, frame_ramp,
        controller_id, panel_inst);
    true
}

static dce_funcs: abm_funcs = abm_funcs {
    abm_init: Some(dce_abm_init),
    set_abm_level: Some(dce_abm_set_level),
    set_pipe: Some(dce_abm_set_pipe),
    set_backlight_level_pwm: Some(dce_abm_set_backlight_level_pwm),
    get_current_backlight: Some(dce_abm_get_current_backlight),
    get_target_backlight: Some(dce_abm_get_target_backlight),
    init_abm_config: None,
    set_abm_immediate_disable: Some(dce_abm_immediate_disable),
};

unsafe fn dce_abm_construct(
    abm_dce: *mut dce_abm, ctx: *mut dc_context,
    regs: *const dce_abm_registers, abm_shift: *const dce_abm_shift,
    abm_mask: *const dce_abm_mask,
) {
    (*abm_dce).base.ctx = ctx;
    (*abm_dce).base.funcs = &dce_funcs;
    (*abm_dce).base.dmcu_is_running = false;
    (*abm_dce).regs = regs;
    (*abm_dce).abm_shift = abm_shift;
    (*abm_dce).abm_mask = abm_mask;
}

pub unsafe fn dce_abm_create(
    ctx: *mut dc_context, regs: *const dce_abm_registers,
    abm_shift: *const dce_abm_shift, abm_mask: *const dce_abm_mask,
) -> *mut abm {
    let abm_dce = kzalloc_obj::<dce_abm>();
    if abm_dce.is_null() {
        BREAK_TO_DEBUGGER!();
        return core::ptr::null_mut();
    }
    dce_abm_construct(abm_dce, ctx, regs, abm_shift, abm_mask);
    (*abm_dce).base.funcs = &dce_funcs;
    &mut (*abm_dce).base
}

pub unsafe fn dce_abm_destroy(abm: *mut *mut abm) {
    let abm_dce = to_dce_abm(*abm);
    kfree(abm_dce);
    *abm = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
