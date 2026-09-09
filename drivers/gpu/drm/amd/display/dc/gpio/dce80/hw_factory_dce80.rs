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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C dependencies supplied by the surrounding translation unit:
// dm_services.h, gpio_types.h, hw_factory.h, hw_factory_dce80.h,
// hw_gpio.h, hw_ddc.h, hw_hpd.h, hw_generic.h, dce_8_0_d.h,
// dce_8_0_sh_mask.h, reg_helper.h, hpd_regs.h, and ddc_regs.h.

use core::ffi::c_void;

// Register and bit-field values are supplied by the DCE8 headers.
extern "C" {
    static mut hpd_regs: [hpd_registers; 6];
    static mut hpd_shift: hpd_sh_mask;
    static mut hpd_mask: hpd_sh_mask;
    static mut ddc_data_regs: [ddc_registers; 8];
    static mut ddc_clk_regs: [ddc_registers; 8];
    static mut ddc_shift: ddc_sh_mask;
    static mut ddc_mask: ddc_sh_mask;

    fn dal_hw_ddc_init(base: *mut hw_gpio_pin);
    fn dal_hw_hpd_init(base: *mut hw_gpio_pin);
    fn dal_hw_ddc_get_pin(factory: *mut hw_factory, id: u32, en: u32) -> *mut hw_gpio_pin;
    fn dal_hw_hpd_get_pin(factory: *mut hw_factory, id: u32, en: u32) -> *mut hw_gpio_pin;
}

#[repr(C)]
pub struct gpio_registers { pub value: u32 }
#[repr(C)]
pub struct hpd_registers { pub gpio: gpio_registers, pub int_status: u32, pub toggle_filt_cntl: u32 }
#[repr(C)]
pub struct hpd_sh_mask { pub dc_hpd_sense_delayed: u32, pub dc_hpd_sense: u32, pub dc_hpd_connect_int_delay: u32, pub dc_hpd_disconnect_int_delay: u32 }
#[repr(C)]
pub struct ddc_registers { pub gpio: gpio_registers }
#[repr(C)]
pub struct ddc_sh_mask { pub value: u32 }
#[repr(C)]
pub struct hw_gpio_pin { pub id: u32 }
#[repr(C)]
pub struct hw_ddc { pub base: hw_gpio_base, pub regs: *const ddc_registers, pub shifts: *const ddc_sh_mask, pub masks: *const ddc_sh_mask }
#[repr(C)]
pub struct hw_hpd { pub base: hw_gpio_base, pub regs: *const hpd_registers, pub shifts: *const hpd_sh_mask, pub masks: *const hpd_sh_mask }
#[repr(C)]
pub struct hw_gpio_base { pub regs: *const gpio_registers }
#[repr(C)]
pub struct hw_factory_funcs {
    pub init_ddc_data: Option<unsafe extern "C" fn(*mut hw_gpio_pin)>,
    pub init_generic: Option<unsafe extern "C" fn()>,
    pub init_hpd: Option<unsafe extern "C" fn(*mut hw_gpio_pin)>,
    pub get_ddc_pin: Option<unsafe extern "C" fn(*mut hw_factory, u32, u32) -> *mut hw_gpio_pin>,
    pub get_hpd_pin: Option<unsafe extern "C" fn(*mut hw_factory, u32, u32) -> *mut hw_gpio_pin>,
    pub get_generic_pin: Option<unsafe extern "C" fn()>,
    pub define_hpd_registers: Option<unsafe extern "C" fn(*mut hw_gpio_pin, u32)>,
    pub define_ddc_registers: Option<unsafe extern "C" fn(*mut hw_gpio_pin, u32)>,
}
#[repr(C)]
pub struct hw_factory { pub number_of_pins: [u32; 8], pub funcs: *const hw_factory_funcs }

const GPIO_ID_DDC_DATA: u32 = 0;
const GPIO_ID_DDC_CLOCK: u32 = 1;
const GPIO_ID_GENERIC: u32 = 2;
const GPIO_ID_HPD: u32 = 3;
const GPIO_ID_GPIO_PAD: u32 = 4;
const GPIO_ID_VIP_PAD: u32 = 5;
const GPIO_ID_SYNC: u32 = 6;
const GPIO_ID_GSL: u32 = 7;

unsafe fn define_ddc_registers(pin: *mut hw_gpio_pin, en: u32) {
    let ddc = pin as *mut hw_ddc;
    match (*pin).id {
        GPIO_ID_DDC_DATA => { (*ddc).regs = &ddc_data_regs[en as usize]; (*ddc).base.regs = &ddc_data_regs[en as usize].gpio; }
        GPIO_ID_DDC_CLOCK => { (*ddc).regs = &ddc_clk_regs[en as usize]; (*ddc).base.regs = &ddc_clk_regs[en as usize].gpio; }
        _ => { debug_assert!(false); return; }
    }
    (*ddc).shifts = &ddc_shift;
    (*ddc).masks = &ddc_mask;
}

unsafe fn define_hpd_registers(pin: *mut hw_gpio_pin, en: u32) {
    let hpd = pin as *mut hw_hpd;
    (*hpd).regs = &hpd_regs[en as usize];
    (*hpd).shifts = &hpd_shift;
    (*hpd).masks = &hpd_mask;
    (*hpd).base.regs = &hpd_regs[en as usize].gpio;
}

static funcs: hw_factory_funcs = hw_factory_funcs {
    init_ddc_data: Some(dal_hw_ddc_init), init_generic: None, init_hpd: Some(dal_hw_hpd_init),
    get_ddc_pin: Some(dal_hw_ddc_get_pin), get_hpd_pin: Some(dal_hw_hpd_get_pin), get_generic_pin: None,
    define_hpd_registers: Some(define_hpd_registers), define_ddc_registers: Some(define_ddc_registers),
};

pub unsafe extern "C" fn dal_hw_factory_dce80_init(factory: *mut hw_factory) {
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
