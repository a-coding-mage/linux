// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding hardware abstraction and register headers.

const DCN_BASE__INST0_SEG2: u32 = 0x0000_34C0;

// Register-list macro expansion points from the C headers are intentionally retained.

const hpd_regs: [hpd_registers; 4] = [
    hpd_regs!(0),
    hpd_regs!(1),
    hpd_regs!(2),
    hpd_regs!(3),
];

const hpd_shift: hpd_sh_mask = hpd_mask_sh_list!(__SHIFT);
const hpd_mask: hpd_sh_mask = hpd_mask_sh_list!(_MASK);

const ddc_data_regs_dcn: [ddc_registers; 7] = [
    ddc_data_regs_dcn2!(1),
    ddc_data_regs_dcn2!(2),
    ddc_data_regs_dcn2!(3),
    ddc_data_regs_dcn2!(4),
    ddc_registers {
        gpio: [0; 12],
        ddc_setup: 0,
        phy_aux_cntl: 0,
        dc_gpio_aux_ctrl_5: 0,
    },
    ddc_registers {
        gpio: [0; 12],
        ddc_setup: 0,
        phy_aux_cntl: 0,
        dc_gpio_aux_ctrl_5: 0,
    },
    ddc_registers {
        gpio: ddc_gpio_vga_reg_list!(DATA),
        ddc_setup: 0,
        phy_aux_cntl: 0,
        dc_gpio_aux_ctrl_5: 0,
    },
];

const ddc_clk_regs_dcn: [ddc_registers; 7] = [
    ddc_clk_regs_dcn2!(1),
    ddc_clk_regs_dcn2!(2),
    ddc_clk_regs_dcn2!(3),
    ddc_clk_regs_dcn2!(4),
    ddc_registers {
        gpio: [0; 12],
        ddc_setup: 0,
        phy_aux_cntl: 0,
        dc_gpio_aux_ctrl_5: 0,
    },
    ddc_registers {
        gpio: [0; 12],
        ddc_setup: 0,
        phy_aux_cntl: 0,
        dc_gpio_aux_ctrl_5: 0,
    },
    ddc_registers {
        gpio: ddc_gpio_vga_reg_list!(CLK),
        ddc_setup: 0,
        phy_aux_cntl: 0,
        dc_gpio_aux_ctrl_5: 0,
    },
];

const ddc_shift: [ddc_sh_mask; 7] = [
    ddc_mask_sh_list_dcn2!(__SHIFT, 1),
    ddc_mask_sh_list_dcn2!(__SHIFT, 2),
    ddc_mask_sh_list_dcn2!(__SHIFT, 3),
    ddc_mask_sh_list_dcn2!(__SHIFT, 4),
    ddc_mask_sh_list_dcn2!(__SHIFT, 5),
    ddc_mask_sh_list_dcn2!(__SHIFT, 6),
    ddc_mask_sh_list_dcn2_vga!(__SHIFT),
];

const ddc_mask: [ddc_sh_mask; 7] = [
    ddc_mask_sh_list_dcn2!(_MASK, 1),
    ddc_mask_sh_list_dcn2!(_MASK, 2),
    ddc_mask_sh_list_dcn2!(_MASK, 3),
    ddc_mask_sh_list_dcn2!(_MASK, 4),
    ddc_mask_sh_list_dcn2!(_MASK, 5),
    ddc_mask_sh_list_dcn2!(_MASK, 6),
    ddc_mask_sh_list_dcn2_vga!(_MASK),
];

const generic_regs: [generic_registers; 2] = [
    generic_regs!(A),
    generic_regs!(B),
];

const generic_shift: [generic_sh_mask; 2] = [
    generic_mask_sh_list!(__SHIFT, A),
    generic_mask_sh_list!(__SHIFT, B),
];

const generic_mask: [generic_sh_mask; 2] = [
    generic_mask_sh_list!(_MASK, A),
    generic_mask_sh_list!(_MASK, B),
];

unsafe fn define_generic_registers(pin: *mut hw_gpio_pin, en: u32) {
    let generic = HW_GENERIC_FROM_BASE(pin);

    (*generic).regs = &generic_regs[en as usize];
    (*generic).shifts = &generic_shift[en as usize];
    (*generic).masks = &generic_mask[en as usize];
    (*generic).base.regs = &generic_regs[en as usize].gpio;
}

unsafe fn define_ddc_registers(pin: *mut hw_gpio_pin, en: u32) {
    let ddc = HW_DDC_FROM_BASE(pin);

    match (*pin).id {
        GPIO_ID_DDC_DATA => {
            (*ddc).regs = &ddc_data_regs_dcn[en as usize];
            (*ddc).base.regs = &ddc_data_regs_dcn[en as usize].gpio;
        }
        GPIO_ID_DDC_CLOCK => {
            (*ddc).regs = &ddc_clk_regs_dcn[en as usize];
            (*ddc).base.regs = &ddc_clk_regs_dcn[en as usize].gpio;
        }
        _ => {
            ASSERT_CRITICAL!(false);
            return;
        }
    }

    (*ddc).shifts = &ddc_shift[en as usize];
    (*ddc).masks = &ddc_mask[en as usize];
}

unsafe fn define_hpd_registers(pin: *mut hw_gpio_pin, en: u32) {
    let hpd = HW_HPD_FROM_BASE(pin);

    (*hpd).regs = &hpd_regs[en as usize];
    (*hpd).shifts = &hpd_shift;
    (*hpd).masks = &hpd_mask;
    (*hpd).base.regs = &hpd_regs[en as usize].gpio;
}

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
 * dal_hw_factory_dcn401_init
 *
 * @brief
 * Initialize HW factory function pointers and pin info
 *
 * @param
 * struct hw_factory *factory - [out] struct of function pointers
 */
pub unsafe fn dal_hw_factory_dcn401_init(factory: *mut hw_factory) {
    (*factory).number_of_pins[GPIO_ID_DDC_DATA as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_DDC_CLOCK as usize] = 8;
    (*factory).number_of_pins[GPIO_ID_GENERIC as usize] = 4;
    (*factory).number_of_pins[GPIO_ID_HPD as usize] = 5;
    (*factory).number_of_pins[GPIO_ID_GPIO_PAD as usize] = 28;
    (*factory).number_of_pins[GPIO_ID_VIP_PAD as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_SYNC as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_GSL as usize] = 0; // add this

    (*factory).funcs = &funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
