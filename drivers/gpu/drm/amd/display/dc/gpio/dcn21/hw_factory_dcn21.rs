/*
 * Copyright 2013-15 Advanced Micro Devices, Inc.
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

// Register-list and bit-field initializers are supplied by the corresponding
// hardware headers in the surrounding translation unit.

static HPD_REGISTERS: [hpd_registers; 5] = [
    hpd_regs!(0), hpd_regs!(1), hpd_regs!(2), hpd_regs!(3), hpd_regs!(4),
];

static HPD_SHIFT: hpd_sh_mask = hpd_mask_sh_list!(SHIFT);
static HPD_MASK: hpd_sh_mask = hpd_mask_sh_list!(MASK);

static DDC_DATA_REGS_DCN: [ddc_registers; 5] = [
    ddc_data_regs_dcn2!(1), ddc_data_regs_dcn2!(2), ddc_data_regs_dcn2!(3),
    ddc_data_regs_dcn2!(4), ddc_data_regs_dcn2!(5),
];

static DDC_CLK_REGS_DCN: [ddc_registers; 5] = [
    ddc_clk_regs_dcn2!(1), ddc_clk_regs_dcn2!(2), ddc_clk_regs_dcn2!(3),
    ddc_clk_regs_dcn2!(4), ddc_clk_regs_dcn2!(5),
];

static DDC_SHIFT: [ddc_sh_mask; 6] = [
    ddc_mask_sh_list_dcn2!(SHIFT, 1), ddc_mask_sh_list_dcn2!(SHIFT, 2),
    ddc_mask_sh_list_dcn2!(SHIFT, 3), ddc_mask_sh_list_dcn2!(SHIFT, 4),
    ddc_mask_sh_list_dcn2!(SHIFT, 5), ddc_mask_sh_list_dcn2!(SHIFT, 6),
];

static DDC_MASK: [ddc_sh_mask; 6] = [
    ddc_mask_sh_list_dcn2!(MASK, 1), ddc_mask_sh_list_dcn2!(MASK, 2),
    ddc_mask_sh_list_dcn2!(MASK, 3), ddc_mask_sh_list_dcn2!(MASK, 4),
    ddc_mask_sh_list_dcn2!(MASK, 5), ddc_mask_sh_list_dcn2!(MASK, 6),
];

static GENERIC_REGS: [generic_registers; 1] = [generic_reg_list!(A)];
static GENERIC_SHIFT: [generic_sh_mask; 1] = [generic_mask_sh_list!(SHIFT, A)];
static GENERIC_MASK: [generic_sh_mask; 1] = [generic_mask_sh_list!(MASK, A)];

unsafe fn define_generic_registers(pin: *mut hw_gpio_pin, en: u32) {
    let generic = hw_generic_from_base(pin);
    (*generic).regs = &GENERIC_REGS[en as usize];
    (*generic).shifts = &GENERIC_SHIFT[en as usize];
    (*generic).masks = &GENERIC_MASK[en as usize];
    (*generic).base.regs = &GENERIC_REGS[en as usize].gpio;
}

unsafe fn define_ddc_registers(pin: *mut hw_gpio_pin, en: u32) {
    let ddc = hw_ddc_from_base(pin);
    match (*pin).id {
        GPIO_ID_DDC_DATA => {
            (*ddc).regs = &DDC_DATA_REGS_DCN[en as usize];
            (*ddc).base.regs = &DDC_DATA_REGS_DCN[en as usize].gpio;
        }
        GPIO_ID_DDC_CLOCK => {
            (*ddc).regs = &DDC_CLK_REGS_DCN[en as usize];
            (*ddc).base.regs = &DDC_CLK_REGS_DCN[en as usize].gpio;
        }
        _ => {
            ASSERT_CRITICAL!(false);
            return;
        }
    }
    (*ddc).shifts = &DDC_SHIFT[en as usize];
    (*ddc).masks = &DDC_MASK[en as usize];
}

unsafe fn define_hpd_registers(pin: *mut hw_gpio_pin, en: u32) {
    let hpd = hw_hpd_from_base(pin);
    (*hpd).regs = &HPD_REGISTERS[en as usize];
    (*hpd).shifts = &HPD_SHIFT;
    (*hpd).masks = &HPD_MASK;
    (*hpd).base.regs = &HPD_REGISTERS[en as usize].gpio;
}

static FUNCS: hw_factory_funcs = hw_factory_funcs {
    init_ddc_data: dal_hw_ddc_init,
    init_generic: dal_hw_generic_init,
    init_hpd: dal_hw_hpd_init,
    get_ddc_pin: dal_hw_ddc_get_pin,
    get_hpd_pin: dal_hw_hpd_get_pin,
    get_generic_pin: dal_hw_generic_get_pin,
    define_hpd_registers,
    define_ddc_registers,
    define_generic_registers,
};

pub unsafe fn dal_hw_factory_dcn21_init(factory: *mut hw_factory) {
    // TODO: check ASIC CAPs
    (*factory).number_of_pins[GPIO_ID_DDC_DATA as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_DDC_CLOCK as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_GENERIC as usize] = 4;
    (*factory).number_of_pins[GPIO_ID_HPD as usize] = 6;
    (*factory).number_of_pins[GPIO_ID_GPIO_PAD as usize] = 28;
    (*factory).number_of_pins[GPIO_ID_VIP_PAD as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_SYNC as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_GSL as usize] = 0; // add this
    (*factory).funcs = &FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
