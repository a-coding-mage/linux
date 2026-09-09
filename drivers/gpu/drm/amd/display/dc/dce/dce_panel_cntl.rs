/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translation unit.

unsafe fn dce_get_16_bit_backlight_from_pwm(panel_cntl: *mut panel_cntl) -> u32 {
    let mut current_backlight: u64;
    let (mut bl_period, mut bl_int_count): (u32, u32);
    let (mut bl_pwm, mut fractional_duty_cycle_en): (u32, u32);
    let (mut bl_period_mask, mut bl_pwm_mask): (u32, u32);
    let dce_panel_cntl = TO_DCE_PANEL_CNTL!(panel_cntl);

    REG_READ!(dce_panel_cntl, BL_PWM_PERIOD_CNTL);
    REG_GET!(dce_panel_cntl, BL_PWM_PERIOD_CNTL, BL_PWM_PERIOD, &mut bl_period);
    REG_GET!(dce_panel_cntl, BL_PWM_PERIOD_CNTL, BL_PWM_PERIOD_BITCNT, &mut bl_int_count);

    REG_READ!(dce_panel_cntl, BL_PWM_CNTL);
    REG_GET!(dce_panel_cntl, BL_PWM_CNTL, BL_ACTIVE_INT_FRAC_CNT, &mut bl_pwm);
    REG_GET!(dce_panel_cntl, BL_PWM_CNTL, BL_PWM_FRACTIONAL_EN, &mut fractional_duty_cycle_en);

    if bl_int_count == 0 { bl_int_count = 16; }
    bl_period_mask = (1u32 << bl_int_count) - 1;
    bl_period &= bl_period_mask;
    bl_pwm_mask = bl_period_mask << (16 - bl_int_count);
    if fractional_duty_cycle_en == 0 { bl_pwm &= bl_pwm_mask; } else { bl_pwm &= 0xffff; }

    current_backlight = (bl_pwm as u64) << (1 + bl_int_count);
    if bl_period == 0 { bl_period = 0xffff; }
    current_backlight = div_u64!(current_backlight, bl_period as u64);
    current_backlight = (current_backlight + 1) >> 1;
    current_backlight as u32
}

unsafe fn dce_panel_cntl_hw_init(panel_cntl: *mut panel_cntl) -> u32 {
    let dce_panel_cntl = TO_DCE_PANEL_CNTL!(panel_cntl);
    let mut value: u32;
    let current_backlight: u32;
    REG_GET!(dce_panel_cntl, BL_PWM_CNTL, BL_ACTIVE_INT_FRAC_CNT, &mut value);

    if (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL != 0 {
        REG_WRITE!(dce_panel_cntl, BL_PWM_CNTL, (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL);
        REG_WRITE!(dce_panel_cntl, BL_PWM_CNTL2, (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL2);
        REG_WRITE!(dce_panel_cntl, BL_PWM_PERIOD_CNTL, (*panel_cntl).stored_backlight_registers.BL_PWM_PERIOD_CNTL);
        REG_UPDATE!(dce_panel_cntl, PWRSEQ_REF_DIV, BL_PWM_REF_DIV, (*panel_cntl).stored_backlight_registers.LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV);
    } else if value != 0 && value != 1 {
        (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL = REG_READ!(dce_panel_cntl, BL_PWM_CNTL);
        (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL2 = REG_READ!(dce_panel_cntl, BL_PWM_CNTL2);
        (*panel_cntl).stored_backlight_registers.BL_PWM_PERIOD_CNTL = REG_READ!(dce_panel_cntl, BL_PWM_PERIOD_CNTL);
        REG_GET!(dce_panel_cntl, PWRSEQ_REF_DIV, BL_PWM_REF_DIV, &mut (*panel_cntl).stored_backlight_registers.LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV);
    } else {
        REG_WRITE!(dce_panel_cntl, BL_PWM_CNTL, 0x8000FA00);
        REG_WRITE!(dce_panel_cntl, BL_PWM_PERIOD_CNTL, 0x000C0FA0);
    }
    value = REG_READ!(dce_panel_cntl, BIOS_SCRATCH_2);
    value |= ATOM_S2_VRI_BRIGHT_ENABLE;
    REG_WRITE!(dce_panel_cntl, BIOS_SCRATCH_2, value);
    REG_UPDATE!(dce_panel_cntl, BL_PWM_CNTL, BL_PWM_EN, 1);
    REG_UPDATE!(dce_panel_cntl, BL_PWM_GRP1_REG_LOCK, BL_PWM_GRP1_REG_LOCK, 0);
    current_backlight = dce_get_16_bit_backlight_from_pwm(panel_cntl);
    current_backlight
}

unsafe fn dce_is_panel_backlight_on(panel_cntl: *mut panel_cntl) -> bool {
    let dce_panel_cntl = TO_DCE_PANEL_CNTL!(panel_cntl);
    let (mut blon, mut blon_ovrd, mut pwrseq_target_state) = (0u32, 0u32, 0u32);
    REG_GET_2!(dce_panel_cntl, PWRSEQ_CNTL, LVTMA_BLON, &mut blon, LVTMA_BLON_OVRD, &mut blon_ovrd);
    REG_GET!(dce_panel_cntl, PWRSEQ_CNTL, LVTMA_PWRSEQ_TARGET_STATE, &mut pwrseq_target_state);
    if blon_ovrd != 0 { blon != 0 } else { pwrseq_target_state != 0 }
}

unsafe fn dce_is_panel_powered_on(panel_cntl: *mut panel_cntl) -> bool {
    let dce_panel_cntl = TO_DCE_PANEL_CNTL!(panel_cntl);
    let (mut pwr_seq_state, mut dig_on, mut dig_on_ovrd) = (0u32, 0u32, 0u32);
    REG_GET!(dce_panel_cntl, PWRSEQ_STATE, LVTMA_PWRSEQ_TARGET_STATE_R, &mut pwr_seq_state);
    REG_GET_2!(dce_panel_cntl, PWRSEQ_CNTL, LVTMA_DIGON, &mut dig_on, LVTMA_DIGON_OVRD, &mut dig_on_ovrd);
    pwr_seq_state == 1 || (dig_on == 1 && dig_on_ovrd == 1)
}

unsafe fn dce_store_backlight_level(panel_cntl: *mut panel_cntl) {
    let dce_panel_cntl = TO_DCE_PANEL_CNTL!(panel_cntl);
    (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL = REG_READ!(dce_panel_cntl, BL_PWM_CNTL);
    (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL2 = REG_READ!(dce_panel_cntl, BL_PWM_CNTL2);
    (*panel_cntl).stored_backlight_registers.BL_PWM_PERIOD_CNTL = REG_READ!(dce_panel_cntl, BL_PWM_PERIOD_CNTL);
    REG_GET!(dce_panel_cntl, PWRSEQ_REF_DIV, BL_PWM_REF_DIV, &mut (*panel_cntl).stored_backlight_registers.LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV);
}

unsafe fn dce_driver_set_backlight(panel_cntl: *mut panel_cntl, backlight_pwm_u16_16: u32) {
    let dce_panel_cntl = TO_DCE_PANEL_CNTL!(panel_cntl);
    let (mut backlight_16bit, mut masked_pwm_period, mut pwm_period_bitcnt) = (0u32, 0u32, 0u32);
    let mut bit_count: u8;
    let active_duty_cycle: u64;
    REG_GET_2!(dce_panel_cntl, BL_PWM_PERIOD_CNTL, BL_PWM_PERIOD_BITCNT, &mut pwm_period_bitcnt, BL_PWM_PERIOD, &mut masked_pwm_period);
    if pwm_period_bitcnt == 0 { bit_count = 16; } else { bit_count = pwm_period_bitcnt as u8; }
    masked_pwm_period &= (1u32 << bit_count) - 1;
    active_duty_cycle = backlight_pwm_u16_16 as u64 * masked_pwm_period as u64;
    backlight_16bit = (active_duty_cycle >> bit_count) as u32;
    backlight_16bit &= 0xffff;
    backlight_16bit += ((active_duty_cycle >> (bit_count - 1)) & 1) as u32;
    REG_UPDATE_2!(dce_panel_cntl, BL_PWM_GRP1_REG_LOCK, BL_PWM_GRP1_IGNORE_MASTER_LOCK_EN, 1, BL_PWM_GRP1_REG_LOCK, 1);
    REG_UPDATE!(dce_panel_cntl, BL_PWM_CNTL, BL_ACTIVE_INT_FRAC_CNT, backlight_16bit);
    REG_UPDATE!(dce_panel_cntl, BL_PWM_GRP1_REG_LOCK, BL_PWM_GRP1_REG_LOCK, 0);
    REG_WAIT!(dce_panel_cntl, BL_PWM_GRP1_REG_LOCK, BL_PWM_GRP1_REG_UPDATE_PENDING, 0, 1, 10000);
}

unsafe fn dce_panel_cntl_destroy(panel_cntl: *mut *mut panel_cntl) {
    let dce_panel_cntl = TO_DCE_PANEL_CNTL!(*panel_cntl);
    kfree!(dce_panel_cntl);
    *panel_cntl = core::ptr::null_mut();
}

static DCE_LINK_PANEL_CNTL_FUNCS: panel_cntl_funcs = panel_cntl_funcs {
    destroy: Some(dce_panel_cntl_destroy), hw_init: Some(dce_panel_cntl_hw_init),
    is_panel_backlight_on: Some(dce_is_panel_backlight_on), is_panel_powered_on: Some(dce_is_panel_powered_on),
    store_backlight_level: Some(dce_store_backlight_level), driver_set_backlight: Some(dce_driver_set_backlight),
    get_current_backlight: Some(dce_get_16_bit_backlight_from_pwm),
};

pub unsafe fn dce_panel_cntl_construct(dce_panel_cntl: *mut dce_panel_cntl, init_data: *const panel_cntl_init_data, regs: *const dce_panel_cntl_registers, shift: *const dce_panel_cntl_shift, mask: *const dce_panel_cntl_mask) {
    let base = &mut (*dce_panel_cntl).base;
    base.stored_backlight_registers.BL_PWM_CNTL = 0;
    base.stored_backlight_registers.BL_PWM_CNTL2 = 0;
    base.stored_backlight_registers.BL_PWM_PERIOD_CNTL = 0;
    base.stored_backlight_registers.LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV = 0;
    (*dce_panel_cntl).regs = regs;
    (*dce_panel_cntl).shift = shift;
    (*dce_panel_cntl).mask = mask;
    (*dce_panel_cntl).base.funcs = &DCE_LINK_PANEL_CNTL_FUNCS;
    (*dce_panel_cntl).base.ctx = (*init_data).ctx;
    (*dce_panel_cntl).base.inst = (*init_data).inst;
    (*dce_panel_cntl).base.pwrseq_inst = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
