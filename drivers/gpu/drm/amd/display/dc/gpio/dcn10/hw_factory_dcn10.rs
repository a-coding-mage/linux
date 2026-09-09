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

// Register-list and bit-mask initializers are supplied by the corresponding
// DCN10 headers, as in the original C translation unit.

extern "C" {
    static hpd_regs: [hpd_registers; 6];
    static hpd_shift: hpd_sh_mask;
    static hpd_mask: hpd_sh_mask;
    static ddc_data_regs: [ddc_registers; 8];
    static ddc_clk_regs: [ddc_registers; 8];
    static ddc_shift: ddc_sh_mask;
    static ddc_mask: ddc_sh_mask;
    static generic_regs: [generic_registers; 2];
    static generic_shift: [generic_sh_mask; 2];
    static generic_mask: [generic_sh_mask; 2];
}

unsafe fn define_generic_registers(pin: *mut hw_gpio_pin, en: u32) {
    let generic: *mut hw_generic = HW_GENERIC_FROM_BASE(pin);

    (*generic).regs = &generic_regs[en as usize];
    (*generic).shifts = &generic_shift[en as usize];
    (*generic).masks = &generic_mask[en as usize];
    (*generic).base.regs = &generic_regs[en as usize].gpio;
}

unsafe fn define_ddc_registers(pin: *mut hw_gpio_pin, en: u32) {
    let ddc: *mut hw_ddc = HW_DDC_FROM_BASE(pin);

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
            ASSERT_CRITICAL(false);
            return;
        }
    }

    (*ddc).shifts = &ddc_shift;
    (*ddc).masks = &ddc_mask;
}

unsafe fn define_hpd_registers(pin: *mut hw_gpio_pin, en: u32) {
    let hpd: *mut hw_hpd = HW_HPD_FROM_BASE(pin);

    (*hpd).regs = &hpd_regs[en as usize];
    (*hpd).shifts = &hpd_shift;
    (*hpd).masks = &hpd_mask;
    (*hpd).base.regs = &hpd_regs[en as usize].gpio;
}

/* function table */
static funcs: hw_factory_funcs = hw_factory_funcs {
    init_ddc_data: dal_hw_ddc_init,
    init_generic: dal_hw_generic_init,
    init_hpd: dal_hw_hpd_init,
    get_ddc_pin: dal_hw_ddc_get_pin,
    get_hpd_pin: dal_hw_hpd_get_pin,
    get_generic_pin: dal_hw_generic_get_pin,
    define_hpd_registers: define_hpd_registers,
    define_ddc_registers: define_ddc_registers,
    define_generic_registers: define_generic_registers,
};

/*
 * dal_hw_factory_dcn10_init
 *
 * @brief
 * Initialize HW factory function pointers and pin info
 *
 * @param
 * struct hw_factory *factory - [out] struct of function pointers
 */
pub unsafe fn dal_hw_factory_dcn10_init(factory: *mut hw_factory) {
    /* TODO check ASIC CAPs */
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
