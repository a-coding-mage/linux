/*
 * Copyright 2020 Mauro Rossi <issor.oruam@gmail.com>
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

// Register definitions, GPIO/DDC/HPD types, helper macros, and constants are
// supplied by the surrounding translation unit.

static hpd_regs: [hpd_registers; 6] = [
    hpd_registers { HPD_GPIO_REG_LIST!(1), int_status: mmDC_HPD1_INT_STATUS, toggle_filt_cntl: mmDC_HPD1_TOGGLE_FILT_CNTL },
    hpd_registers { HPD_GPIO_REG_LIST!(2), int_status: mmDC_HPD2_INT_STATUS, toggle_filt_cntl: mmDC_HPD2_TOGGLE_FILT_CNTL },
    hpd_registers { HPD_GPIO_REG_LIST!(3), int_status: mmDC_HPD3_INT_STATUS, toggle_filt_cntl: mmDC_HPD3_TOGGLE_FILT_CNTL },
    hpd_registers { HPD_GPIO_REG_LIST!(4), int_status: mmDC_HPD4_INT_STATUS, toggle_filt_cntl: mmDC_HPD4_TOGGLE_FILT_CNTL },
    hpd_registers { HPD_GPIO_REG_LIST!(5), int_status: mmDC_HPD5_INT_STATUS, toggle_filt_cntl: mmDC_HPD5_TOGGLE_FILT_CNTL },
    hpd_registers { HPD_GPIO_REG_LIST!(6), int_status: mmDC_HPD6_INT_STATUS, toggle_filt_cntl: mmDC_HPD6_TOGGLE_FILT_CNTL },
];

static hpd_shift: hpd_sh_mask = hpd_sh_mask {
    DC_HPD_SENSE_DELAYED: DC_HPD1_INT_STATUS__DC_HPD1_SENSE_DELAYED_SHIFT,
    DC_HPD_SENSE: DC_HPD1_INT_STATUS__DC_HPD1_SENSE_SHIFT,
    DC_HPD_CONNECT_INT_DELAY: DC_HPD1_TOGGLE_FILT_CNTL__DC_HPD1_CONNECT_INT_DELAY_SHIFT,
    DC_HPD_DISCONNECT_INT_DELAY: DC_HPD1_TOGGLE_FILT_CNTL__DC_HPD1_DISCONNECT_INT_DELAY_SHIFT,
};

static hpd_mask: hpd_sh_mask = hpd_sh_mask {
    DC_HPD_SENSE_DELAYED: DC_HPD1_INT_STATUS__DC_HPD1_SENSE_DELAYED_MASK,
    DC_HPD_SENSE: DC_HPD1_INT_STATUS__DC_HPD1_SENSE_MASK,
    DC_HPD_CONNECT_INT_DELAY: DC_HPD1_TOGGLE_FILT_CNTL__DC_HPD1_CONNECT_INT_DELAY_MASK,
    DC_HPD_DISCONNECT_INT_DELAY: DC_HPD1_TOGGLE_FILT_CNTL__DC_HPD1_DISCONNECT_INT_DELAY_MASK,
};

static ddc_data_regs: [ddc_registers; 8] = [
    DDC_DATA_REGS!(1), DDC_DATA_REGS!(2), DDC_DATA_REGS!(3), DDC_DATA_REGS!(4),
    DDC_DATA_REGS!(5), DDC_DATA_REGS!(6), DDC_VGA_DATA_REGS, DDC_I2C_DATA_REGS,
];

static ddc_clk_regs: [ddc_registers; 8] = [
    DDC_CLK_REGS!(1), DDC_CLK_REGS!(2), DDC_CLK_REGS!(3), DDC_CLK_REGS!(4),
    DDC_CLK_REGS!(5), DDC_CLK_REGS!(6), DDC_VGA_CLK_REGS, DDC_I2C_CLK_REGS,
];

static ddc_shift: ddc_sh_mask = DDC_MASK_SH_LIST!(SHIFT);
static ddc_mask: ddc_sh_mask = DDC_MASK_SH_LIST!(MASK);

unsafe fn define_ddc_registers(pin: *mut hw_gpio_pin, en: u32) {
    let ddc: *mut hw_ddc = HW_DDC_FROM_BASE!(pin);
    match (*pin).id {
        GPIO_ID_DDC_DATA => {
            (*ddc).regs = &ddc_data_regs[en as usize];
            (*ddc).base.regs = &ddc_data_regs[en as usize].gpio;
        }
        GPIO_ID_DDC_CLOCK => {
            (*ddc).regs = &ddc_clk_regs[en as usize];
            (*ddc).base.regs = &ddc_clk_regs[en as usize].gpio;
        }
        _ => {
            ASSERT_CRITICAL!(false);
            return;
        }
    }
    (*ddc).shifts = &ddc_shift;
    (*ddc).masks = &ddc_mask;
}

unsafe fn define_hpd_registers(pin: *mut hw_gpio_pin, en: u32) {
    let hpd: *mut hw_hpd = HW_HPD_FROM_BASE!(pin);
    (*hpd).regs = &hpd_regs[en as usize];
    (*hpd).shifts = &hpd_shift;
    (*hpd).masks = &hpd_mask;
    (*hpd).base.regs = &hpd_regs[en as usize].gpio;
}

static funcs: hw_factory_funcs = hw_factory_funcs {
    init_ddc_data: Some(dal_hw_ddc_init),
    init_generic: None,
    init_hpd: Some(dal_hw_hpd_init),
    get_ddc_pin: Some(dal_hw_ddc_get_pin),
    get_hpd_pin: Some(dal_hw_hpd_get_pin),
    get_generic_pin: None,
    define_hpd_registers: Some(define_hpd_registers),
    define_ddc_registers: Some(define_ddc_registers),
};

pub unsafe fn dal_hw_factory_dce60_init(factory: *mut hw_factory) {
    (*factory).number_of_pins[GPIO_ID_DDC_DATA as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_DDC_CLOCK as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_GENERIC as usize] = 7;
    (*factory).number_of_pins[GPIO_ID_HPD as usize] = 6;
    (*factory).number_of_pins[GPIO_ID_GPIO_PAD as usize] = 31;
    (*factory).number_of_pins[GPIO_ID_VIP_PAD as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_SYNC as usize] = 2;
    (*factory).number_of_pins[GPIO_ID_GSL as usize] = 4;
    (*factory).funcs = &funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
