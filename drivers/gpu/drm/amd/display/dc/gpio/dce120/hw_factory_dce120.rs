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

// External types, constants, macros, register definitions, and functions are
// supplied by the corresponding kernel/display dependencies.

#[allow(non_upper_case_globals)]
static HPD_REGS: [hpd_registers; 6] = [
    hpd_regs!(0),
    hpd_regs!(1),
    hpd_regs!(2),
    hpd_regs!(3),
    hpd_regs!(4),
    hpd_regs!(5),
];

static HPD_SHIFT: hpd_sh_mask = HPD_MASK_SH_LIST!(__SHIFT);
static HPD_MASK: hpd_sh_mask = HPD_MASK_SH_LIST!(_MASK);

static DDC_DATA_REGS: [ddc_registers; 8] = [
    ddc_data_regs!(1),
    ddc_data_regs!(2),
    ddc_data_regs!(3),
    ddc_data_regs!(4),
    ddc_data_regs!(5),
    ddc_data_regs!(6),
    ddc_vga_data_regs!,
    ddc_i2c_data_regs!,
];

static DDC_CLK_REGS: [ddc_registers; 8] = [
    ddc_clk_regs!(1),
    ddc_clk_regs!(2),
    ddc_clk_regs!(3),
    ddc_clk_regs!(4),
    ddc_clk_regs!(5),
    ddc_clk_regs!(6),
    ddc_vga_clk_regs!,
    ddc_i2c_clk_regs!,
];

static DDC_SHIFT: ddc_sh_mask = DDC_MASK_SH_LIST!(__SHIFT);
static DDC_MASK: ddc_sh_mask = DDC_MASK_SH_LIST!(_MASK);

unsafe fn define_ddc_registers(pin: *mut hw_gpio_pin, en: u32) {
    let ddc: *mut hw_ddc = HW_DDC_FROM_BASE!(pin);

    match (*pin).id {
        GPIO_ID_DDC_DATA => {
            (*ddc).regs = &DDC_DATA_REGS[en as usize];
            (*ddc).base.regs = &DDC_DATA_REGS[en as usize].gpio;
        }
        GPIO_ID_DDC_CLOCK => {
            (*ddc).regs = &DDC_CLK_REGS[en as usize];
            (*ddc).base.regs = &DDC_CLK_REGS[en as usize].gpio;
        }
        _ => {
            ASSERT_CRITICAL!(false);
            return;
        }
    }

    (*ddc).shifts = &DDC_SHIFT;
    (*ddc).masks = &DDC_MASK;
}

unsafe fn define_hpd_registers(pin: *mut hw_gpio_pin, en: u32) {
    let hpd: *mut hw_hpd = HW_HPD_FROM_BASE!(pin);

    (*hpd).regs = &HPD_REGS[en as usize];
    (*hpd).shifts = &HPD_SHIFT;
    (*hpd).masks = &HPD_MASK;
    (*hpd).base.regs = &HPD_REGS[en as usize].gpio;
}

/* function table */
static FUNCS: hw_factory_funcs = hw_factory_funcs {
    init_ddc_data: Some(dal_hw_ddc_init),
    init_generic: None,
    init_hpd: Some(dal_hw_hpd_init),
    get_ddc_pin: Some(dal_hw_ddc_get_pin),
    get_hpd_pin: Some(dal_hw_hpd_get_pin),
    get_generic_pin: None,
    define_hpd_registers: Some(define_hpd_registers),
    define_ddc_registers: Some(define_ddc_registers),
};

/*
 * dal_hw_factory_dce120_init
 *
 * @brief
 * Initialize HW factory function pointers and pin info
 *
 * @param
 * struct hw_factory *factory - [out] struct of function pointers
 */
pub unsafe fn dal_hw_factory_dce120_init(factory: *mut hw_factory) {
    /* TODO check ASIC CAPs */
    (*factory).number_of_pins[GPIO_ID_DDC_DATA as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_DDC_CLOCK as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_GENERIC as usize] = 7;
    (*factory).number_of_pins[GPIO_ID_HPD as usize] = 6;
    (*factory).number_of_pins[GPIO_ID_GPIO_PAD as usize] = 31;
    (*factory).number_of_pins[GPIO_ID_VIP_PAD as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_SYNC as usize] = 2;
    (*factory).number_of_pins[GPIO_ID_GSL as usize] = 4;

    (*factory).funcs = &FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
