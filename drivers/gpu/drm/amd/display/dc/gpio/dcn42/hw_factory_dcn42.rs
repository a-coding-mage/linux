// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C dependencies supplied by the surrounding DC/GPIO implementation are
// intentionally referenced here rather than reimplemented in this file.

const DCN_BASE_INST0_SEG2: u32 = 0x0000_34c0;

// The following register-list macros correspond to the generated C register
// definitions. Their Rust equivalents are provided by the surrounding crate.

static HPD_REGS: [hpd_registers; 5] = [
    hpd_regs!(0),
    hpd_regs!(1),
    hpd_regs!(2),
    hpd_regs!(3),
    hpd_regs!(4),
];

static HPD_SHIFT: hpd_sh_mask = hpd_mask_sh_list!(__SHIFT);
static HPD_MASK: hpd_sh_mask = hpd_mask_sh_list!(_MASK);

static DDC_DATA_REGS_DCN: [ddc_registers; 8] = [
    ddc_data_regs_dcn2!(1),
    ddc_data_regs_dcn2!(2),
    ddc_data_regs_dcn2!(3),
    ddc_data_regs_dcn2!(4),
    ddc_data_regs_dcn2!(5),
    ddc_registers {
        gpio: [0; 12],
        ddc_setup: 0,
        phy_aux_cntl: 0,
        dc_gpio_aux_ctrl_5: 0,
    },
    ddc_gpio_vga_reg_list!(DATA),
];

static DDC_CLK_REGS_DCN: [ddc_registers; 8] = [
    ddc_clk_regs_dcn2!(1),
    ddc_clk_regs_dcn2!(2),
    ddc_clk_regs_dcn2!(3),
    ddc_clk_regs_dcn2!(4),
    ddc_clk_regs_dcn2!(5),
    ddc_registers {
        gpio: [0; 12],
        ddc_setup: 0,
        phy_aux_cntl: 0,
        dc_gpio_aux_ctrl_5: 0,
    },
    ddc_gpio_vga_reg_list!(CLK),
];

static DDC_SHIFT: [ddc_sh_mask; 7] = [
    ddc_mask_sh_list_dcn2!(__SHIFT, 1),
    ddc_mask_sh_list_dcn2!(__SHIFT, 2),
    ddc_mask_sh_list_dcn2!(__SHIFT, 3),
    ddc_mask_sh_list_dcn2!(__SHIFT, 4),
    ddc_mask_sh_list_dcn2!(__SHIFT, 5),
    ddc_mask_sh_list_dcn2!(__SHIFT, 6),
    ddc_mask_sh_list_dcn2_vga!(__SHIFT),
];

static DDC_MASK: [ddc_sh_mask; 7] = [
    ddc_mask_sh_list_dcn2!(_MASK, 1),
    ddc_mask_sh_list_dcn2!(_MASK, 2),
    ddc_mask_sh_list_dcn2!(_MASK, 3),
    ddc_mask_sh_list_dcn2!(_MASK, 4),
    ddc_mask_sh_list_dcn2!(_MASK, 5),
    ddc_mask_sh_list_dcn2!(_MASK, 6),
    ddc_mask_sh_list_dcn2_vga!(_MASK),
];

static GENERIC_REGS: [generic_registers; 2] = [generic_registers { gpio: [0; 0] }, generic_registers { gpio: [0; 0] }];
static GENERIC_SHIFT: [generic_sh_mask; 2] = [generic_sh_mask { value: 0 }, generic_sh_mask { value: 0 }];
static GENERIC_MASK: [generic_sh_mask; 2] = [generic_sh_mask { value: 0 }, generic_sh_mask { value: 0 }];

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
    (*hpd).regs = &HPD_REGS[en as usize];
    (*hpd).shifts = &HPD_SHIFT;
    (*hpd).masks = &HPD_MASK;
    (*hpd).base.regs = &HPD_REGS[en as usize].gpio;
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

pub unsafe fn dal_hw_factory_dcn42_init(factory: *mut hw_factory) {
    (*factory).number_of_pins[GPIO_ID_DDC_DATA as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_DDC_CLOCK as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_GENERIC as usize] = 4;
    (*factory).number_of_pins[GPIO_ID_HPD as usize] = 5;
    (*factory).number_of_pins[GPIO_ID_GPIO_PAD as usize] = 28;
    (*factory).number_of_pins[GPIO_ID_VIP_PAD as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_SYNC as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_GSL as usize] = 0; // add this
    (*factory).funcs = &FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
