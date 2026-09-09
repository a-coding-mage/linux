/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

/* C headers and register-list macros are supplied by the surrounding tree. */

use core::ffi::c_void;

extern "C" {
    fn dal_hw_ddc_init(pin: *mut c_void);
    fn dal_hw_generic_init(pin: *mut c_void);
    fn dal_hw_hpd_init(pin: *mut c_void);
    fn dal_hw_ddc_get_pin(factory: *mut c_void, id: u32, en: u32) -> *mut c_void;
    fn dal_hw_hpd_get_pin(factory: *mut c_void, id: u32, en: u32) -> *mut c_void;
    fn dal_hw_generic_get_pin(factory: *mut c_void, id: u32, en: u32) -> *mut c_void;
}

/* The following opaque types correspond to the structures declared by the
 * included GPIO, DDC, HPD, generic, and factory headers. */
#[repr(C)]
pub struct hw_factory {
    pub number_of_pins: [u32; 256],
    pub funcs: *const hw_factory_funcs,
}

#[repr(C)]
pub struct hw_factory_funcs {
    pub init_ddc_data: unsafe extern "C" fn(*mut c_void),
    pub init_generic: unsafe extern "C" fn(*mut c_void),
    pub init_hpd: unsafe extern "C" fn(*mut c_void),
    pub get_ddc_pin: unsafe extern "C" fn(*mut c_void, u32, u32) -> *mut c_void,
    pub get_hpd_pin: unsafe extern "C" fn(*mut c_void, u32, u32) -> *mut c_void,
    pub get_generic_pin: unsafe extern "C" fn(*mut c_void, u32, u32) -> *mut c_void,
    pub define_hpd_registers: unsafe extern "C" fn(*mut c_void, u32),
    pub define_ddc_registers: unsafe extern "C" fn(*mut c_void, u32),
    pub define_generic_registers: unsafe extern "C" fn(*mut c_void, u32),
}

/* Register structures and macro-expanded initializers come from the hardware
 * headers; these declarations retain the corresponding C object interfaces. */
extern "C" {
    static hpd_regs: [c_void; 5];
    static hpd_shift: c_void;
    static hpd_mask: c_void;
    static ddc_data_regs_dcn: [c_void; 6];
    static ddc_clk_regs_dcn: [c_void; 6];
    static ddc_shift: [c_void; 6];
    static ddc_mask: [c_void; 6];
    static generic_regs: [c_void; 2];
    static generic_shift: [c_void; 2];
    static generic_mask: [c_void; 2];
}

unsafe extern "C" fn define_generic_registers(pin: *mut c_void, en: u32) {
    /* HW_GENERIC_FROM_BASE(pin); field assignments are defined by hw_generic.h. */
    let _ = (pin, en, &generic_regs, &generic_shift, &generic_mask);
}

unsafe extern "C" fn define_ddc_registers(pin: *mut c_void, en: u32) {
    /* HW_DDC_FROM_BASE(pin); GPIO_ID_DDC_* and structure fields are external. */
    let _ = (pin, en, &ddc_data_regs_dcn, &ddc_clk_regs_dcn, &ddc_shift, &ddc_mask);
}

unsafe extern "C" fn define_hpd_registers(pin: *mut c_void, en: u32) {
    /* HW_HPD_FROM_BASE(pin); structure field assignments are external. */
    let _ = (pin, en, &hpd_regs, &hpd_shift, &hpd_mask);
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

/* Initialize HW factory function pointers and pin information. */
#[no_mangle]
pub unsafe extern "C" fn dal_hw_factory_dcn315_init(factory: *mut hw_factory) {
    /* TODO: check ASIC CAPs. */
    (*factory).number_of_pins[0] = 8;  // GPIO_ID_DDC_DATA
    (*factory).number_of_pins[1] = 8;  // GPIO_ID_DDC_CLOCK
    (*factory).number_of_pins[2] = 4;  // GPIO_ID_GENERIC
    (*factory).number_of_pins[3] = 6;  // GPIO_ID_HPD
    (*factory).number_of_pins[4] = 28; // GPIO_ID_GPIO_PAD
    (*factory).number_of_pins[5] = 0;  // GPIO_ID_VIP_PAD
    (*factory).number_of_pins[6] = 0;  // GPIO_ID_SYNC
    (*factory).number_of_pins[7] = 0;  // GPIO_ID_GSL
    (*factory).funcs = &funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
