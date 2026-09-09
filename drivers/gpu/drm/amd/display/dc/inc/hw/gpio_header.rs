/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
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

// Dependency supplied by gpio_types.h.

#[repr(C)]
pub union gpio_hw_container {
    pub ddc: *mut hw_ddc,
    pub generic: *mut hw_generic,
    pub hpd: *mut hw_hpd,
}

#[repr(C)]
pub struct gpio {
    pub service: *mut gpio_service,
    pub pin: *mut hw_gpio_pin,
    pub id: gpio_id,
    pub en: u32,

    pub hw_container: gpio_hw_container,
    pub mode: gpio_mode,

    // when GPIO comes from VBIOS, it has defined output state
    pub output_state: gpio_pin_output_state,
}

/*
 * The original C declaration is disabled by #if 0 and is intentionally kept
 * disabled here as well.  Its members are function-pointer declarations for
 * the GPIO factory and hardware-translation interfaces.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
