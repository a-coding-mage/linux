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

// Dependencies supplied by the surrounding translation unit:
// generic_regs.h, hw_gpio.h

#[repr(C)]
pub struct hw_generic {
    pub base: hw_gpio,
    pub regs: *const generic_registers,
    pub shifts: *const generic_sh_mask,
    pub masks: *const generic_sh_mask,
}

// C equivalent:
// container_of((HW_GPIO_FROM_BASE(hw_gpio)), struct hw_generic, base)
#[macro_export]
macro_rules! HW_GENERIC_FROM_BASE {
    ($hw_gpio:expr) => {
        container_of!(HW_GPIO_FROM_BASE!($hw_gpio), hw_generic, base)
    };
}

extern "C" {
    pub fn dal_hw_generic_init(
        hw_generic: *mut *mut hw_generic,
        ctx: *mut dc_context,
        id: gpio_id,
        en: u32,
    );

    pub fn dal_hw_generic_get_pin(gpio: *mut gpio) -> *mut hw_gpio_pin;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
