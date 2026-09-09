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

// Dependency supplied by hpd_regs.h.

#[repr(C)]
pub struct hw_hpd {
    pub base: hw_gpio,
    pub regs: *const hpd_registers,
    pub shifts: *const hpd_sh_mask,
    pub masks: *const hpd_sh_mask,
}

// C macro translated literally; HW_GPIO_FROM_BASE and container_of are
// supplied by the surrounding dependency set.
#[macro_export]
macro_rules! HW_HPD_FROM_BASE {
    ($hw_gpio:expr) => {
        container_of!(HW_GPIO_FROM_BASE!($hw_gpio), hw_hpd, base)
    };
}

extern "C" {
    pub fn dal_hw_hpd_init(
        hw_hpd: *mut *mut hw_hpd,
        ctx: *mut dc_context,
        id: gpio_id,
        en: u32,
    );

    pub fn dal_hw_hpd_get_pin(gpio: *mut gpio) -> *mut hw_gpio_pin;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
