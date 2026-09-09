// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Register definitions and types are supplied by the corresponding C/Rust
// hardware headers.

const DCN_BASE_INST0_SEG2: u32 = 0x0000_34C0;

// The following register-list macros expand to the platform register values
// supplied by the generated register headers.

#[allow(non_upper_case_globals)]
static hpd_regs: [hpd_registers; 3] = [
    hpd_registers { int_status: REGI!(DC_HPD_INT_STATUS, HPD, 0), toggle_filt_cntl: REGI!(DC_HPD_TOGGLE_FILT_CNTL, HPD, 0), ..unsafe { core::mem::zeroed() } },
    hpd_registers { int_status: REGI!(DC_HPD_INT_STATUS, HPD, 1), toggle_filt_cntl: REGI!(DC_HPD_TOGGLE_FILT_CNTL, HPD, 1), ..unsafe { core::mem::zeroed() } },
    hpd_registers { int_status: REGI!(DC_HPD_INT_STATUS, HPD, 2), toggle_filt_cntl: REGI!(DC_HPD_TOGGLE_FILT_CNTL, HPD, 2), ..unsafe { core::mem::zeroed() } },
];

static hpd_shift: hpd_sh_mask = hpd_sh_mask { HPD_MASK_SH_LIST!(__SHIFT) };
static hpd_mask: hpd_sh_mask = hpd_sh_mask { HPD_MASK_SH_LIST!(_MASK) };

// The first seven entries are dummy ports with no corresponding DDC GPIO.
static ddc_data_regs_dcn: [ddc_registers; 8] = [
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: DDC_GPIO_VGA_REG_LIST!(DATA), ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
];

static ddc_clk_regs_dcn: [ddc_registers; 7] = [
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: [0; 12], ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
    ddc_registers { gpio: DDC_GPIO_VGA_REG_LIST!(CLK), ddc_setup: 0, phy_aux_cntl: 0, dc_gpio_aux_ctrl_5: 0 },
];

static generic_regs: [generic_registers; 2] = [generic_registers { gpio: 0 }, generic_registers { gpio: 0 }];
static generic_shift: [generic_sh_mask; 2] = [generic_sh_mask { value: 0 }, generic_sh_mask { value: 0 }];
static generic_mask: [generic_sh_mask; 2] = [generic_sh_mask { value: 0 }, generic_sh_mask { value: 0 }];

unsafe fn define_generic_registers(pin: *mut hw_gpio_pin, en: u32) {
    let generic = HW_GENERIC_FROM_BASE!(pin);
    (*generic).regs = &generic_regs[en as usize];
    (*generic).shifts = &generic_shift[en as usize];
    (*generic).masks = &generic_mask[en as usize];
    (*generic).base.regs = &generic_regs[en as usize].gpio;
}

unsafe fn define_ddc_registers(pin: *mut hw_gpio_pin, en: u32) {
    let ddc = HW_DDC_FROM_BASE!(pin);
    match (*pin).id {
        GPIO_ID_DDC_DATA => { (*ddc).regs = &ddc_data_regs_dcn[en as usize]; (*ddc).base.regs = &ddc_data_regs_dcn[en as usize].gpio; }
        GPIO_ID_DDC_CLOCK => { (*ddc).regs = &ddc_clk_regs_dcn[en as usize]; (*ddc).base.regs = &ddc_clk_regs_dcn[en as usize].gpio; }
        _ => { ASSERT_CRITICAL!(false); return; }
    }
    (*ddc).shifts = core::ptr::null();
    (*ddc).masks = core::ptr::null();
}

unsafe fn define_hpd_registers(pin: *mut hw_gpio_pin, en: u32) {
    let hpd = HW_HPD_FROM_BASE!(pin);
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
    define_hpd_registers,
    define_ddc_registers,
    define_generic_registers,
};

pub unsafe fn dal_hw_factory_dcn42b_init(factory: *mut hw_factory) {
    (*factory).number_of_pins[GPIO_ID_DDC_DATA as usize] = 1; // VGA
    (*factory).number_of_pins[GPIO_ID_DDC_CLOCK as usize] = 1; // VGA
    (*factory).number_of_pins[GPIO_ID_GENERIC as usize] = 4;
    (*factory).number_of_pins[GPIO_ID_HPD as usize] = 3; // only 3 HPD
    (*factory).number_of_pins[GPIO_ID_GPIO_PAD as usize] = 28;
    (*factory).number_of_pins[GPIO_ID_VIP_PAD as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_SYNC as usize] = 0;
    (*factory).number_of_pins[GPIO_ID_GSL as usize] = 0; // add this
    (*factory).funcs = &funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
