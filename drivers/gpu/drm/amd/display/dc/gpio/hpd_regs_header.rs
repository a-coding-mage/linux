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

// Dependency supplied by the translated gpio_regs header.

pub const ONE_MORE_0: u32 = 1;
pub const ONE_MORE_1: u32 = 2;
pub const ONE_MORE_2: u32 = 3;
pub const ONE_MORE_3: u32 = 4;
pub const ONE_MORE_4: u32 = 5;
pub const ONE_MORE_5: u32 = 6;

// The following C preprocessor macros use token pasting and designated-field
// initializers. Their expansion is retained as source-level intent here;
// REG, REGI, and SF_HPD are supplied by the gpio register definitions.
//
// #define HPD_GPIO_REG_LIST_ENTRY(type, cd, id) \
//     .type ## _reg = REG(DC_GPIO_HPD_## type), \
//     .type ## _mask = DC_GPIO_HPD_ ## type ## __DC_GPIO_HPD ## id ## _ ## type ## _MASK, \
//     .type ## _shift = DC_GPIO_HPD_ ## type ## __DC_GPIO_HPD ## id ## _ ## type ## __SHIFT
//
// #define HPD_GPIO_REG_LIST(id) \
//     { HPD_GPIO_REG_LIST_ENTRY(MASK, cd, id), \
//       HPD_GPIO_REG_LIST_ENTRY(A, cd, id), \
//       HPD_GPIO_REG_LIST_ENTRY(EN, cd, id), \
//       HPD_GPIO_REG_LIST_ENTRY(Y, cd, id) }
//
// #define HPD_REG_LIST(id) \
//     HPD_GPIO_REG_LIST(ONE_MORE_## id), \
//     .int_status = REGI(DC_HPD_INT_STATUS, HPD, id), \
//     .toggle_filt_cntl = REGI(DC_HPD_TOGGLE_FILT_CNTL, HPD, id)
//
// #define HPD_MASK_SH_LIST(mask_sh) \
//     SF_HPD(DC_HPD_INT_STATUS, DC_HPD_SENSE_DELAYED, mask_sh), \
//     SF_HPD(DC_HPD_INT_STATUS, DC_HPD_SENSE, mask_sh), \
//     SF_HPD(DC_HPD_TOGGLE_FILT_CNTL, DC_HPD_CONNECT_INT_DELAY, mask_sh), \
//     SF_HPD(DC_HPD_TOGGLE_FILT_CNTL, DC_HPD_DISCONNECT_INT_DELAY, mask_sh)

#[repr(C)]
pub struct hpd_registers {
    pub gpio: gpio_registers,
    pub int_status: u32,
    pub toggle_filt_cntl: u32,
}

#[repr(C)]
pub struct hpd_sh_mask {
    /* int_status */
    pub DC_HPD_SENSE_DELAYED: u32,
    pub DC_HPD_SENSE: u32,
    /* toggle_filt_cntl */
    pub DC_HPD_CONNECT_INT_DELAY: u32,
    pub DC_HPD_DISCONNECT_INT_DELAY: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
