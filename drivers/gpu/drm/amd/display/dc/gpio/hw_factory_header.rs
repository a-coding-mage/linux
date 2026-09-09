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
 *
 */

// Declarations supplied by the surrounding translation unit:
// struct hw_gpio_pin;
// struct hw_hpd;
// struct hw_ddc;
// struct hw_generic;
// struct gpio;
// enum gpio_id;
// enum dce_version;
// enum dce_environment;
// struct dc_context;
// GPIO_ID_COUNT

#[repr(C)]
pub struct hw_factory {
    pub number_of_pins: [u32; GPIO_ID_COUNT as usize],
    pub funcs: *mut hw_factory_funcs,
}

#[repr(C)]
pub struct hw_factory_funcs {
    pub init_ddc_data: Option<unsafe extern "C" fn(
        hw_ddc: *mut *mut hw_ddc,
        ctx: *mut dc_context,
        id: gpio_id,
        en: u32,
    )>,
    pub init_generic: Option<unsafe extern "C" fn(
        hw_generic: *mut *mut hw_generic,
        ctx: *mut dc_context,
        id: gpio_id,
        en: u32,
    )>,
    pub init_hpd: Option<unsafe extern "C" fn(
        hw_hpd: *mut *mut hw_hpd,
        ctx: *mut dc_context,
        id: gpio_id,
        en: u32,
    )>,
    pub get_hpd_pin: Option<unsafe extern "C" fn(gpio: *mut gpio) -> *mut hw_gpio_pin>,
    pub get_ddc_pin: Option<unsafe extern "C" fn(gpio: *mut gpio) -> *mut hw_gpio_pin>,
    pub get_generic_pin: Option<unsafe extern "C" fn(gpio: *mut gpio) -> *mut hw_gpio_pin>,
    pub define_hpd_registers: Option<unsafe extern "C" fn(pin: *mut hw_gpio_pin, en: u32)>,
    pub define_ddc_registers: Option<unsafe extern "C" fn(pin: *mut hw_gpio_pin, en: u32)>,
    pub define_generic_registers: Option<unsafe extern "C" fn(pin: *mut hw_gpio_pin, en: u32)>,
}

extern "C" {
    pub fn dal_hw_factory_init(
        factory: *mut hw_factory,
        dce_version: dce_version,
        dce_environment: dce_environment,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
