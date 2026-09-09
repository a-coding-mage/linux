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

// Dependency: declarations from ddc_regs.h and related headers are supplied externally.

#[repr(C)]
pub struct hw_ddc {
    pub base: hw_gpio,
    pub regs: *const ddc_registers,
    pub shifts: *const ddc_sh_mask,
    pub masks: *const ddc_sh_mask,
}

// C macro equivalent; container_of and HW_GPIO_FROM_BASE are external dependencies.
#[macro_export]
macro_rules! HW_DDC_FROM_BASE {
    ($hw_gpio:expr) => {
        container_of!(HW_GPIO_FROM_BASE!($hw_gpio), hw_ddc, base)
    };
}

extern "C" {
    pub fn dal_hw_ddc_init(
        hw_ddc: *mut *mut hw_ddc,
        ctx: *mut dc_context,
        id: gpio_id,
        en: u32,
    );

    pub fn dal_hw_ddc_get_pin(gpio: *mut gpio) -> *mut hw_gpio_pin;

    pub fn dal_hw_ddc_init_i3cpad(
        hw_ddc: *mut *mut hw_ddc,
        ctx: *mut dc_context,
        id: gpio_id,
        en: u32,
    );

    pub fn dal_hw_ddc_open_i3cpad(
        ptr: *mut hw_gpio_pin,
        mode: gpio_mode,
    ) -> bool;

    pub fn dal_hw_ddc_get_value_i3cpad(
        ptr: *const hw_gpio_pin,
        value: *mut u32,
    ) -> gpio_result;

    pub fn dal_hw_ddc_set_value_i3cpad(
        ptr: *const hw_gpio_pin,
        value: u32,
    ) -> gpio_result;

    pub fn dal_hw_ddc_config_mode_i3cpad(
        ddc: *mut hw_ddc,
        mode: gpio_mode,
    ) -> gpio_result;

    pub fn dal_hw_ddc_change_mode_i3cpad(
        ptr: *mut hw_gpio_pin,
        mode: gpio_mode,
    ) -> gpio_result;

    pub fn dal_hw_ddc_close_i3cpad(ptr: *mut hw_gpio_pin);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
