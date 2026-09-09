// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// C dependencies are supplied by the surrounding translation unit.

const DCN_BASE__INST0_SEG2: u32 = 0x000034C0;

// DCN_BASE__INST0_SEG2 is the only segment used by this register table.
// Register and field constants below are supplied by the generated register
// headers in the surrounding translation.

static HPD_REGS: [hpd_registers; 4] = [
    hpd_registers {
        gpio: Default::default(),
        int_status: regHPD0_DC_HPD_INT_STATUS,
        toggle_filt_cntl: regHPD0_DC_HPD_TOGGLE_FILT_CNTL,
    },
    hpd_registers {
        gpio: Default::default(),
        int_status: regHPD1_DC_HPD_INT_STATUS,
        toggle_filt_cntl: regHPD1_DC_HPD_TOGGLE_FILT_CNTL,
    },
    hpd_registers {
        gpio: Default::default(),
        int_status: regHPD2_DC_HPD_INT_STATUS,
        toggle_filt_cntl: regHPD2_DC_HPD_TOGGLE_FILT_CNTL,
    },
    hpd_registers {
        gpio: Default::default(),
        int_status: regHPD3_DC_HPD_INT_STATUS,
        toggle_filt_cntl: regHPD3_DC_HPD_TOGGLE_FILT_CNTL,
    },
];

static HPD_SHIFT: hpd_sh_mask = hpd_sh_mask { HPD_MASK_SH_LIST_SHIFT };
static HPD_MASK: hpd_sh_mask = hpd_sh_mask { HPD_MASK_SH_LIST_MASK };

static DDC_REGS: [ddc_registers; 2] = [
    ddc_registers {
        gpio: Default::default(),
        ddc_setup: DC_I2C_DDC1_SETUP,
        phy_aux_cntl: PHY_AUX_CNTL,
        dc_gpio_aux_ctrl_5: DC_GPIO_AUX_CTRL_5,
        dc_i3cpad_control0: DC_I3C0_DC_I3CPAD_CONTROL0,
        dc_i3cpad_control1: DC_I3C0_DC_I3CPAD_CONTROL1,
    },
    ddc_registers {
        gpio: Default::default(),
        ddc_setup: DC_I2C_DDC2_SETUP,
        phy_aux_cntl: PHY_AUX_CNTL,
        dc_gpio_aux_ctrl_5: DC_GPIO_AUX_CTRL_5,
        dc_i3cpad_control0: DC_I3C1_DC_I3CPAD_CONTROL0,
        dc_i3cpad_control1: DC_I3C1_DC_I3CPAD_CONTROL1,
    },
];

static DDC_SHIFT: [ddc_sh_mask; 2] = [
    DDC_MASK_SH_LIST_DCN6_SHIFT,
    DDC_MASK_SH_LIST_DCN6_SHIFT,
];

static DDC_MASK: [ddc_sh_mask; 2] = [
    DDC_MASK_SH_LIST_DCN6_MASK,
    DDC_MASK_SH_LIST_DCN6_MASK,
];

static GENERIC_REGS: [generic_registers; 2] = [
    generic_registers { gpio: Default::default() },
    generic_registers { gpio: Default::default() },
];

static GENERIC_SHIFT: [generic_sh_mask; 2] = [
    generic_sh_mask { value: 0 },
    generic_sh_mask { value: 0 },
];

static GENERIC_MASK: [generic_sh_mask; 2] = [
    generic_sh_mask { value: 0 },
    generic_sh_mask { value: 0 },
];

unsafe fn dcn60_define_generic_registers(pin: *mut hw_gpio_pin, en: u32) {
    let generic = HW_GENERIC_FROM_BASE(pin);
    (*generic).regs = &GENERIC_REGS[en as usize];
    (*generic).shifts = &GENERIC_SHIFT[en as usize];
    (*generic).masks = &GENERIC_MASK[en as usize];
    (*generic).base.regs = &GENERIC_REGS[en as usize].gpio;
}

unsafe fn dcn60_define_ddc_registers(pin: *mut hw_gpio_pin, en: u32) {
    let ddc = HW_DDC_FROM_BASE(pin);
    match (*pin).id {
        GPIO_ID_DDC_DATA | GPIO_ID_DDC_CLOCK => {
            (*ddc).regs = &DDC_REGS[en as usize];
            (*ddc).base.regs = &DDC_REGS[en as usize].gpio;
        }
        _ => {
            ASSERT_CRITICAL(false);
            return;
        }
    }
    (*ddc).shifts = &DDC_SHIFT[en as usize];
    (*ddc).masks = &DDC_MASK[en as usize];
}

unsafe fn dcn60_define_hpd_registers(pin: *mut hw_gpio_pin, en: u32) {
    let hpd = HW_HPD_FROM_BASE(pin);
    (*hpd).regs = &HPD_REGS[en as usize];
    (*hpd).shifts = &HPD_SHIFT;
    (*hpd).masks = &HPD_MASK;
    (*hpd).base.regs = &HPD_REGS[en as usize].gpio;
}

static FUNCS: hw_factory_funcs = hw_factory_funcs {
    init_ddc_data: dal_hw_ddc_init_i3cpad,
    init_generic: dal_hw_generic_init,
    init_hpd: dal_hw_hpd_init,
    get_ddc_pin: dal_hw_ddc_get_pin,
    get_hpd_pin: dal_hw_hpd_get_pin,
    get_generic_pin: dal_hw_generic_get_pin,
    define_hpd_registers: dcn60_define_hpd_registers,
    define_ddc_registers: dcn60_define_ddc_registers,
    define_generic_registers: dcn60_define_generic_registers,
};

pub unsafe fn dal_hw_factory_dcn60_init(factory: *mut hw_factory) {
    (*factory).number_of_pins[GPIO_ID_DDC_DATA as usize] = 2;
    (*factory).number_of_pins[GPIO_ID_DDC_CLOCK as usize] = 2;
    (*factory).number_of_pins[GPIO_ID_GENERIC as usize] = 2;
    (*factory).number_of_pins[GPIO_ID_HPD as usize] = 4;
    (*factory).number_of_pins[GPIO_ID_GPIO_PAD as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_VIP_PAD as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_SYNC as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_GSL as usize] = 0;
    (*factory).funcs = &FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
