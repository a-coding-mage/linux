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
 */

// Dependencies supplied by gpio_types.h, gpio_interface.h, and hw/gpio.h.

#[repr(C)]
pub struct gpio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_service {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ddc {
    _private: [u8; 0],
}

extern "C" {
    pub fn dal_gpio_create(
        service: *mut gpio_service,
        id: gpio_id,
        en: u32,
        output_state: gpio_pin_output_state,
    ) -> *mut gpio;

    pub fn dal_gpio_destroy(ptr: *mut *mut gpio);

    pub fn dal_gpio_service_create(
        dce_version: dce_version,
        dce_environment: dce_environment,
        ctx: *mut dc_context,
    ) -> *mut gpio_service;

    pub fn dal_gpio_service_create_irq(
        service: *mut gpio_service,
        offset: u32,
        mask: u32,
    ) -> *mut gpio;

    pub fn dal_gpio_service_create_generic_mux(
        service: *mut gpio_service,
        offset: u32,
        mask: u32,
    ) -> *mut gpio;

    pub fn dal_gpio_destroy_generic_mux(mux: *mut *mut gpio);

    pub fn dal_mux_setup_config(
        mux: *mut gpio,
        config: *mut gpio_generic_mux_config,
    ) -> gpio_result;

    pub fn dal_gpio_get_generic_pin_info(
        service: *mut gpio_service,
        id: gpio_id,
        en: u32,
    ) -> gpio_pin_info;

    pub fn dal_gpio_create_ddc(
        service: *mut gpio_service,
        offset: u32,
        mask: u32,
        info: *mut gpio_ddc_hw_info,
    ) -> *mut ddc;

    pub fn dal_gpio_destroy_ddc(ddc: *mut *mut ddc);

    pub fn dal_gpio_service_destroy(ptr: *mut *mut gpio_service);

    pub fn dal_irq_get_source(irq: *const gpio) -> dc_irq_source;

    pub fn dal_irq_get_rx_source(irq: *const gpio) -> dc_irq_source;

    pub fn dal_irq_get_read_request(irq: *const gpio) -> dc_irq_source;

    pub fn dal_irq_setup_hpd_filter(
        irq: *mut gpio,
        config: *mut gpio_hpd_config,
    ) -> gpio_result;

    pub fn dal_gpio_create_irq(
        service: *mut gpio_service,
        id: gpio_id,
        en: u32,
    ) -> *mut gpio;

    pub fn dal_gpio_destroy_irq(ptr: *mut *mut gpio);

    pub fn dal_ddc_open(
        ddc: *mut ddc,
        mode: gpio_mode,
        config_type: gpio_ddc_config_type,
    ) -> gpio_result;

    pub fn dal_ddc_change_mode(ddc: *mut ddc, mode: gpio_mode) -> gpio_result;

    pub fn dal_ddc_get_line(ddc: *const ddc) -> gpio_ddc_line;

    pub fn dal_ddc_set_config(
        ddc: *mut ddc,
        config_type: gpio_ddc_config_type,
    ) -> gpio_result;

    pub fn dal_ddc_close(ddc: *mut ddc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
