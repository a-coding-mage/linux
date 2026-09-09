/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
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

// Dependencies supplied by the surrounding display-driver translation.

unsafe fn dcn301_get_16_bit_backlight_from_pwm(panel_cntl: *mut panel_cntl) -> u32 {
    let mut current_backlight: u64;
    let mut round_result: u32;
    let mut bl_period: u32 = 0;
    let mut bl_int_count: u32 = 0;
    let mut bl_pwm: u32 = 0;
    let mut fractional_duty_cycle_en: u32 = 0;
    let mut bl_period_mask: u32;
    let mut bl_pwm_mask: u32;
    let dcn301_panel_cntl: *mut dcn301_panel_cntl = unsafe { container_of!(panel_cntl, dcn301_panel_cntl, base) };

    REG_GET!(dcn301_panel_cntl, BL_PWM_PERIOD_CNTL, BL_PWM_PERIOD, &mut bl_period);
    REG_GET!(dcn301_panel_cntl, BL_PWM_PERIOD_CNTL, BL_PWM_PERIOD_BITCNT, &mut bl_int_count);
    REG_GET!(dcn301_panel_cntl, BL_PWM_CNTL, BL_ACTIVE_INT_FRAC_CNT, &mut bl_pwm);
    REG_GET!(dcn301_panel_cntl, BL_PWM_CNTL, BL_PWM_FRACTIONAL_EN, &mut fractional_duty_cycle_en);

    if bl_int_count == 0 {
        bl_int_count = 16;
    }

    bl_period_mask = (1u32 << bl_int_count).wrapping_sub(1);
    bl_period &= bl_period_mask;
    bl_pwm_mask = bl_period_mask << (16 - bl_int_count);

    if fractional_duty_cycle_en == 0 {
        bl_pwm &= bl_pwm_mask;
    } else {
        bl_pwm &= 0xffff;
    }

    current_backlight = (bl_pwm as u64) << (1 + bl_int_count);

    if bl_period == 0 {
        bl_period = 0xffff;
    }

    current_backlight = div_u64!(current_backlight, bl_period);
    current_backlight = (current_backlight + 1) >> 1;
    current_backlight = current_backlight * bl_period as u64;
    round_result = (current_backlight & 0xffff_ffff) as u32;
    round_result = (round_result >> (bl_int_count - 1)) & 1;
    current_backlight >>= bl_int_count;
    current_backlight += round_result as u64;

    current_backlight as u32
}

unsafe fn dcn301_panel_cntl_hw_init(panel_cntl: *mut panel_cntl) -> u32 {
    let dcn301_panel_cntl: *mut dcn301_panel_cntl = unsafe { container_of!(panel_cntl, dcn301_panel_cntl, base) };
    let mut value: u32 = 0;
    let current_backlight: u32;

    REG_GET!(dcn301_panel_cntl, BL_PWM_CNTL, BL_ACTIVE_INT_FRAC_CNT, &mut value);

    if value == 0 || value == 1 {
        if (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL != 0 {
            REG_WRITE!(dcn301_panel_cntl, BL_PWM_CNTL, (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL);
            REG_WRITE!(dcn301_panel_cntl, BL_PWM_CNTL2, (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL2);
            REG_WRITE!(dcn301_panel_cntl, BL_PWM_PERIOD_CNTL, (*panel_cntl).stored_backlight_registers.BL_PWM_PERIOD_CNTL);
            REG_UPDATE!(dcn301_panel_cntl, PWRSEQ_REF_DIV, BL_PWM_REF_DIV, (*panel_cntl).stored_backlight_registers.LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV);
        } else {
            // TODO: VBIOS should have initialized PWM registers on boot.
            REG_WRITE!(dcn301_panel_cntl, BL_PWM_CNTL, 0xC000FA00);
            REG_WRITE!(dcn301_panel_cntl, BL_PWM_PERIOD_CNTL, 0x000C0FA0);
        }
    } else {
        (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL = REG_READ!(dcn301_panel_cntl, BL_PWM_CNTL);
        (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL2 = REG_READ!(dcn301_panel_cntl, BL_PWM_CNTL2);
        (*panel_cntl).stored_backlight_registers.BL_PWM_PERIOD_CNTL = REG_READ!(dcn301_panel_cntl, BL_PWM_PERIOD_CNTL);
        REG_GET!(dcn301_panel_cntl, PWRSEQ_REF_DIV, BL_PWM_REF_DIV, &mut (*panel_cntl).stored_backlight_registers.LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV);
    }

    REG_UPDATE!(dcn301_panel_cntl, BL_PWM_CNTL, BL_PWM_EN, 1);
    REG_UPDATE!(dcn301_panel_cntl, BL_PWM_GRP1_REG_LOCK, BL_PWM_GRP1_REG_LOCK, 0);
    current_backlight = dcn301_get_16_bit_backlight_from_pwm(panel_cntl);
    current_backlight
}

unsafe fn dcn301_panel_cntl_destroy(panel_cntl: *mut *mut panel_cntl) {
    let dcn301_panel_cntl: *mut dcn301_panel_cntl = container_of!(*panel_cntl, dcn301_panel_cntl, base);
    kfree!(dcn301_panel_cntl);
    *panel_cntl = core::ptr::null_mut();
}

unsafe fn dcn301_is_panel_backlight_on(panel_cntl: *mut panel_cntl) -> bool {
    let dcn301_panel_cntl: *mut dcn301_panel_cntl = container_of!(panel_cntl, dcn301_panel_cntl, base);
    let mut value: u32 = 0;
    REG_GET!(dcn301_panel_cntl, PWRSEQ_CNTL, PANEL_BLON, &mut value);
    value != 0
}

unsafe fn dcn301_is_panel_powered_on(panel_cntl: *mut panel_cntl) -> bool {
    let dcn301_panel_cntl: *mut dcn301_panel_cntl = container_of!(panel_cntl, dcn301_panel_cntl, base);
    let (mut pwr_seq_state, mut dig_on, mut dig_on_ovrd) = (0u32, 0u32, 0u32);
    REG_GET!(dcn301_panel_cntl, PWRSEQ_STATE, PANEL_PWRSEQ_TARGET_STATE_R, &mut pwr_seq_state);
    REG_GET_2!(dcn301_panel_cntl, PWRSEQ_CNTL, PANEL_DIGON, &mut dig_on, PANEL_DIGON_OVRD, &mut dig_on_ovrd);
    pwr_seq_state == 1 || (dig_on == 1 && dig_on_ovrd == 1)
}

unsafe fn dcn301_store_backlight_level(panel_cntl: *mut panel_cntl) {
    let dcn301_panel_cntl: *mut dcn301_panel_cntl = container_of!(panel_cntl, dcn301_panel_cntl, base);
    (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL = REG_READ!(dcn301_panel_cntl, BL_PWM_CNTL);
    (*panel_cntl).stored_backlight_registers.BL_PWM_CNTL2 = REG_READ!(dcn301_panel_cntl, BL_PWM_CNTL2);
    (*panel_cntl).stored_backlight_registers.BL_PWM_PERIOD_CNTL = REG_READ!(dcn301_panel_cntl, BL_PWM_PERIOD_CNTL);
    REG_GET!(dcn301_panel_cntl, PWRSEQ_REF_DIV, BL_PWM_REF_DIV, &mut (*panel_cntl).stored_backlight_registers.LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV);
}

static dcn301_link_panel_cntl_funcs: panel_cntl_funcs = panel_cntl_funcs {
    destroy: Some(dcn301_panel_cntl_destroy),
    hw_init: Some(dcn301_panel_cntl_hw_init),
    is_panel_backlight_on: Some(dcn301_is_panel_backlight_on),
    is_panel_powered_on: Some(dcn301_is_panel_powered_on),
    store_backlight_level: Some(dcn301_store_backlight_level),
    get_current_backlight: Some(dcn301_get_16_bit_backlight_from_pwm),
};

unsafe fn dcn301_panel_cntl_construct(
    dcn301_panel_cntl: *mut dcn301_panel_cntl,
    init_data: *const panel_cntl_init_data,
    regs: *const dce_panel_cntl_registers,
    shift: *const dcn301_panel_cntl_shift,
    mask: *const dcn301_panel_cntl_mask,
) {
    (*dcn301_panel_cntl).regs = regs;
    (*dcn301_panel_cntl).shift = shift;
    (*dcn301_panel_cntl).mask = mask;
    (*dcn301_panel_cntl).base.funcs = &dcn301_link_panel_cntl_funcs;
    (*dcn301_panel_cntl).base.ctx = (*init_data).ctx;
    (*dcn301_panel_cntl).base.inst = (*init_data).inst;
    (*dcn301_panel_cntl).base.pwrseq_inst = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
