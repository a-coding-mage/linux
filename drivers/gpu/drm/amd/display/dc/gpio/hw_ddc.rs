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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C dependencies are supplied by the surrounding translation unit.

unsafe fn dal_hw_ddc_destruct(pin: *mut hw_ddc) {
    dal_hw_gpio_destruct(&mut (*pin).base);
}

unsafe fn dal_hw_ddc_destroy(ptr: *mut *mut hw_gpio_pin) {
    let pin = HW_DDC_FROM_BASE(*ptr);
    dal_hw_ddc_destruct(pin);
    kfree(pin);
    *ptr = core::ptr::null_mut();
}

unsafe fn set_config(ptr: *mut hw_gpio_pin, config_data: *const gpio_config_data) -> gpio_result {
    let ddc = HW_DDC_FROM_BASE(ptr);
    let mut hw_gpio: *mut hw_gpio = core::ptr::null_mut();
    let mut regval: u32;
    let mut ddc_data_pd_en: u32 = 0;
    let mut ddc_clk_pd_en: u32 = 0;
    let mut aux_pad_mode: u32 = 0;
    hw_gpio = &mut (*ddc).base;
    if hw_gpio.is_null() {
        ASSERT_CRITICAL(false);
        return GPIO_RESULT_NULL_HANDLE;
    }
    regval = REG_GET_3(gpio.MASK_reg,
        DC_GPIO_DDC1DATA_PD_EN, &mut ddc_data_pd_en,
        DC_GPIO_DDC1CLK_PD_EN, &mut ddc_clk_pd_en,
        AUX_PAD1_MODE, &mut aux_pad_mode);

    match (*config_data).config.ddc.type_ {
    GPIO_DDC_CONFIG_TYPE_MODE_I2C => {
        if (*hw_gpio).base.en != GPIO_DDC_LINE_VIP_PAD {
            if ddc_data_pd_en == 0 || ddc_clk_pd_en == 0 {
                if (*hw_gpio).base.en == GPIO_DDC_LINE_DDC_VGA {
                    REG_SET(gpio.MASK_reg, regval, DC_GPIO_DDC1DATA_PD_EN, 1);
                } else {
                    REG_SET_2(gpio.MASK_reg, regval, DC_GPIO_DDC1DATA_PD_EN, 1,
                        DC_GPIO_DDC1CLK_PD_EN, 1);
                }
                if (*config_data).type == GPIO_CONFIG_TYPE_I2C_AUX_DUAL_MODE { msleep(3); }
            }
        } else {
            let mut sda_pd_dis = 0;
            let mut scl_pd_dis = 0;
            REG_GET_2(gpio.MASK_reg, DC_GPIO_SDA_PD_DIS, &mut sda_pd_dis,
                DC_GPIO_SCL_PD_DIS, &mut scl_pd_dis);
            if sda_pd_dis != 0 {
                REG_SET(gpio.MASK_reg, regval, DC_GPIO_SDA_PD_DIS, 0);
                if (*config_data).type == GPIO_CONFIG_TYPE_I2C_AUX_DUAL_MODE { msleep(3); }
            }
            if scl_pd_dis == 0 {
                REG_SET(gpio.MASK_reg, regval, DC_GPIO_SCL_PD_DIS, 1);
                if (*config_data).type == GPIO_CONFIG_TYPE_I2C_AUX_DUAL_MODE { msleep(3); }
            }
        }
        if aux_pad_mode != 0 {
            if (*config_data).config.ddc.data_en_bit_present || (*config_data).config.ddc.clock_en_bit_present { msleep(2); }
            REG_UPDATE(gpio.MASK_reg, AUX_PAD1_MODE, 0);
        }
        if (*ddc).regs->dc_gpio_aux_ctrl_5 != 0 { REG_UPDATE(dc_gpio_aux_ctrl_5, DDC_PAD_I2CMODE, 1); }
        if (*ddc).regs->phy_aux_cntl != 0 { REG_UPDATE(phy_aux_cntl, AUX_PAD_RXSEL, 1); }
        GPIO_RESULT_OK
    },
    GPIO_DDC_CONFIG_TYPE_MODE_AUX => {
        if aux_pad_mode == 0 { REG_SET(gpio.MASK_reg, regval, AUX_PAD1_MODE, 1); }
        if (*ddc).regs->dc_gpio_aux_ctrl_5 != 0 { REG_UPDATE(dc_gpio_aux_ctrl_5, DDC_PAD_I2CMODE, 0); }
        GPIO_RESULT_OK
    },
    GPIO_DDC_CONFIG_TYPE_POLL_FOR_CONNECT => {
        if (*hw_gpio).base.en <= GPIO_DDC_LINE_DDC_VGA { REG_UPDATE_3(ddc_setup, DC_I2C_DDC1_ENABLE, 1, DC_I2C_DDC1_EDID_DETECT_ENABLE, 1, DC_I2C_DDC1_EDID_DETECT_MODE, 0); return GPIO_RESULT_OK; }
        GPIO_RESULT_NON_SPECIFIC_ERROR
    },
    GPIO_DDC_CONFIG_TYPE_POLL_FOR_DISCONNECT => {
        if (*hw_gpio).base.en <= GPIO_DDC_LINE_DDC_VGA { REG_UPDATE_3(ddc_setup, DC_I2C_DDC1_ENABLE, 1, DC_I2C_DDC1_EDID_DETECT_ENABLE, 1, DC_I2C_DDC1_EDID_DETECT_MODE, 1); return GPIO_RESULT_OK; }
        GPIO_RESULT_NON_SPECIFIC_ERROR
    },
    GPIO_DDC_CONFIG_TYPE_DISABLE_POLLING => {
        if (*hw_gpio).base.en <= GPIO_DDC_LINE_DDC_VGA { REG_UPDATE_2(ddc_setup, DC_I2C_DDC1_ENABLE, 0, DC_I2C_DDC1_EDID_DETECT_ENABLE, 0); return GPIO_RESULT_OK; }
        GPIO_RESULT_NON_SPECIFIC_ERROR
    },
    _ => { BREAK_TO_DEBUGGER(); GPIO_RESULT_NON_SPECIFIC_ERROR }
    }
}

unsafe fn dal_hw_ddc_construct(ddc: *mut hw_ddc, id: gpio_id, en: u32, ctx: *mut dc_context) {
    dal_hw_gpio_construct(&mut (*ddc).base, id, en, ctx);
    (*ddc).base.base.funcs = &funcs;
}

static funcs: hw_gpio_pin_funcs = hw_gpio_pin_funcs {
    destroy: Some(dal_hw_ddc_destroy), open: Some(dal_hw_gpio_open),
    get_value: Some(dal_hw_gpio_get_value), set_value: Some(dal_hw_gpio_set_value),
    set_config: Some(set_config), change_mode: Some(dal_hw_gpio_change_mode),
    close: Some(dal_hw_gpio_close),
};

pub unsafe fn dal_hw_ddc_init(hw_ddc: *mut *mut hw_ddc, ctx: *mut dc_context, id: gpio_id, en: u32) {
    if en > GPIO_DDC_LINE_MAX { ASSERT_CRITICAL(false); *hw_ddc = core::ptr::null_mut(); }
    *hw_ddc = kzalloc_obj::<hw_ddc>();
    if (*hw_ddc).is_null() { ASSERT_CRITICAL(false); return; }
    dal_hw_ddc_construct(*hw_ddc, id, en, ctx);
}

pub unsafe fn dal_hw_ddc_get_pin(gpio: *mut gpio) -> *mut hw_gpio_pin {
    let hw_ddc = dal_gpio_get_ddc(gpio);
    &mut (*hw_ddc).base.base
}

unsafe fn store_registers_ddc_i3cpad(ddc: *mut hw_ddc) {
    match (*ddc).base.base.id {
    GPIO_ID_DDC_DATA => { REG_GET(dc_i3cpad_control0, DC_I3CPAD_DDCDATA_MASK, &mut (*ddc).base.store.mask); REG_GET(dc_i3cpad_control0, DC_I3CPAD_DATA_A, &mut (*ddc).base.store.a); REG_GET(dc_i3cpad_control0, DC_I3CPAD_DATA_EN, &mut (*ddc).base.store.en); }
    GPIO_ID_DDC_CLOCK => { REG_GET(dc_i3cpad_control0, DC_I3CPAD_DDCCLK_MASK, &mut (*ddc).base.store.mask); REG_GET(dc_i3cpad_control0, DC_I3CPAD_CLK_A, &mut (*ddc).base.store.a); REG_GET(dc_i3cpad_control0, DC_I3CPAD_CLK_EN, &mut (*ddc).base.store.en); }
    _ => {}
    }
}

unsafe fn restore_registers_ddc_i3cpad(ddc: *mut hw_ddc) {
    match (*ddc).base.base.id {
    GPIO_ID_DDC_DATA => { REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DDCDATA_MASK, (*ddc).base.store.mask); REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DATA_A, (*ddc).base.store.a); REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DATA_EN, (*ddc).base.store.en); }
    GPIO_ID_DDC_CLOCK => { REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DDCCLK_MASK, (*ddc).base.store.mask); REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_CLK_A, (*ddc).base.store.a); REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_CLK_EN, (*ddc).base.store.en); }
    _ => {}
    }
}

pub unsafe fn dal_hw_ddc_open_i3cpad(ptr: *mut hw_gpio_pin, mode: gpio_mode) -> bool { let ddc = HW_DDC_FROM_BASE(ptr); store_registers_ddc_i3cpad(ddc); (*ptr).opened = dal_hw_ddc_config_mode_i3cpad(ddc, mode) == GPIO_RESULT_OK; (*ptr).opened }

pub unsafe fn dal_hw_ddc_get_value_i3cpad(ptr: *const hw_gpio_pin, value: *mut u32) -> gpio_result {
    let ddc = HW_DDC_FROM_BASE(ptr as *mut hw_gpio_pin);
    match (*ptr).mode { GPIO_MODE_INPUT | GPIO_MODE_OUTPUT | GPIO_MODE_HARDWARE | GPIO_MODE_FAST_OUTPUT => { match (*ddc).base.base.id { GPIO_ID_DDC_DATA => REG_GET(dc_i3cpad_control0, DC_I3CPAD_DATA_Y, value), GPIO_ID_DDC_CLOCK => REG_GET(dc_i3cpad_control0, DC_I3CPAD_CLK_Y, value), _ => {} } GPIO_RESULT_OK }, _ => GPIO_RESULT_NON_SPECIFIC_ERROR }
}

pub unsafe fn dal_hw_ddc_set_value_i3cpad(ptr: *const hw_gpio_pin, value: u32) -> gpio_result {
    let ddc = HW_DDC_FROM_BASE(ptr as *mut hw_gpio_pin);
    match (*ptr).mode { GPIO_MODE_OUTPUT => { match (*ddc).base.base.id { GPIO_ID_DDC_DATA => REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DATA_A, value), GPIO_ID_DDC_CLOCK => REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_CLK_A, value), _ => {} } GPIO_RESULT_OK }, GPIO_MODE_FAST_OUTPUT => { match (*ddc).base.base.id { GPIO_ID_DDC_DATA => REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DATA_EN, value), GPIO_ID_DDC_CLOCK => REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_CLK_EN, value), _ => {} } GPIO_RESULT_OK }, _ => GPIO_RESULT_NON_SPECIFIC_ERROR }
}

pub unsafe fn dal_hw_ddc_change_mode_i3cpad(ptr: *mut hw_gpio_pin, mode: gpio_mode) -> gpio_result { dal_hw_ddc_config_mode_i3cpad(HW_DDC_FROM_BASE(ptr), mode) }

pub unsafe fn dal_hw_ddc_config_mode_i3cpad(ddc: *mut hw_ddc, mode: gpio_mode) -> gpio_result {
    (*ddc).base.base.mode = mode;
    match mode {
    GPIO_MODE_INPUT => { match (*ddc).base.base.id { GPIO_ID_DDC_DATA => { REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DATA_EN, 0); REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DDCDATA_MASK, 1); }, GPIO_ID_DDC_CLOCK => { REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_CLK_EN, 0); REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DDCCLK_MASK, 1); }, _ => {} } GPIO_RESULT_OK },
    GPIO_MODE_OUTPUT | GPIO_MODE_FAST_OUTPUT => { match (*ddc).base.base.id { GPIO_ID_DDC_DATA => { REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DATA_A, 0); REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DDCDATA_MASK, 1); }, GPIO_ID_DDC_CLOCK => { REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_CLK_A, 0); REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DDCCLK_MASK, 1); }, _ => {} } GPIO_RESULT_OK },
    GPIO_MODE_HARDWARE => { match (*ddc).base.base.id { GPIO_ID_DDC_DATA => REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DDCDATA_MASK, 0), GPIO_ID_DDC_CLOCK => REG_UPDATE(dc_i3cpad_control0, DC_I3CPAD_DDCCLK_MASK, 0), _ => {} } GPIO_RESULT_OK },
    _ => GPIO_RESULT_NON_SPECIFIC_ERROR
    }
}

unsafe fn dal_hw_ddc_set_config_i3cpad(ptr: *mut hw_gpio_pin, config_data: *const gpio_config_data) -> gpio_result {
    match (*config_data).config.ddc.type_ {
    GPIO_DDC_CONFIG_TYPE_MODE_I2C => { REG_UPDATE(dc_i3cpad_control1, DC_I3CPAD_RXSEL, 0); GPIO_RESULT_OK },
    GPIO_DDC_CONFIG_TYPE_MODE_AUX => GPIO_RESULT_OK,
    GPIO_DDC_CONFIG_TYPE_POLL_FOR_CONNECT => { REG_UPDATE_3(ddc_setup, DC_I2C_DDC1_ENABLE, 1, DC_I2C_DDC1_EDID_DETECT_ENABLE, 1, DC_I2C_DDC1_EDID_DETECT_MODE, 0); GPIO_RESULT_OK },
    GPIO_DDC_CONFIG_TYPE_POLL_FOR_DISCONNECT => { REG_UPDATE_3(ddc_setup, DC_I2C_DDC1_ENABLE, 1, DC_I2C_DDC1_EDID_DETECT_ENABLE, 1, DC_I2C_DDC1_EDID_DETECT_MODE, 1); GPIO_RESULT_OK },
    GPIO_DDC_CONFIG_TYPE_DISABLE_POLLING => { REG_UPDATE_2(ddc_setup, DC_I2C_DDC1_ENABLE, 0, DC_I2C_DDC1_EDID_DETECT_ENABLE, 0); GPIO_RESULT_OK },
    _ => { BREAK_TO_DEBUGGER(); GPIO_RESULT_NON_SPECIFIC_ERROR }
    }
}

pub unsafe fn dal_hw_ddc_close_i3cpad(ptr: *mut hw_gpio_pin) { restore_registers_ddc_i3cpad(HW_DDC_FROM_BASE(ptr)); (*ptr).mode = GPIO_MODE_UNKNOWN; (*ptr).opened = false; }

static funcs_i3cpad: hw_gpio_pin_funcs = hw_gpio_pin_funcs {
    destroy: Some(dal_hw_ddc_destroy), open: Some(dal_hw_ddc_open_i3cpad),
    get_value: Some(dal_hw_ddc_get_value_i3cpad), set_value: Some(dal_hw_ddc_set_value_i3cpad),
    set_config: Some(dal_hw_ddc_set_config_i3cpad), change_mode: Some(dal_hw_ddc_change_mode_i3cpad),
    close: Some(dal_hw_ddc_close_i3cpad),
};

unsafe fn dal_hw_ddc_construct_i3cpad(ddc: *mut hw_ddc, id: gpio_id, en: u32, ctx: *mut dc_context) { dal_hw_gpio_construct(&mut (*ddc).base, id, en, ctx); (*ddc).base.base.funcs = &funcs_i3cpad; }

pub unsafe fn dal_hw_ddc_init_i3cpad(hw_ddc: *mut *mut hw_ddc, ctx: *mut dc_context, id: gpio_id, en: u32) { if en > GPIO_DDC_LINE_MAX { ASSERT_CRITICAL(false); *hw_ddc = core::ptr::null_mut(); } *hw_ddc = kzalloc::<hw_ddc>(GFP_KERNEL); if (*hw_ddc).is_null() { ASSERT_CRITICAL(false); return; } dal_hw_ddc_construct_i3cpad(*hw_ddc, id, en, ctx); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
