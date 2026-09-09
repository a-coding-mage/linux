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
 */

// Dependency supplied by gpio_regs.h in the original C header.

/*
 * C preprocessor token-pasting macros, preserved as Rust macro forms.  The
 * register, mask, and shift expressions are supplied by the including code.
 */
#[macro_export]
macro_rules! GENERIC_GPIO_REG_LIST_ENTRY {
    ($reg:expr, $mask:expr, $shift:expr) => {
        ($reg, $mask, $shift)
    };
}

#[macro_export]
macro_rules! GENERIC_GPIO_REG_LIST {
    ($mask:expr, $a:expr, $en:expr, $y:expr) => {
        ($mask, $a, $en, $y)
    };
}

#[macro_export]
macro_rules! GENERIC_REG_LIST {
    ($gpio:expr, $mux:expr) => {
        ($gpio, $mux)
    };
}

#[macro_export]
macro_rules! GENERIC_MASK_SH_LIST {
    ($en:expr, $sel:expr) => {
        ($en, $sel)
    };
}

#[repr(C)]
pub struct generic_registers {
    pub gpio: gpio_registers,
    pub mux: u32,
}

#[repr(C)]
pub struct generic_sh_mask {
    /* enable */
    pub GENERIC_EN: u32,
    /* select */
    pub GENERIC_SEL: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
