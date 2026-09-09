/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency supplied by dcn20/dcn20_dccg.h.

macro_rules! DCCG_REG_LIST_DCN30 {
    () => {
        DCCG_REG_LIST_DCN2!(),
        DCCG_SRII!(CLOCK_CNTL, HDMICHARCLK, 0),
        DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 2),
        DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 3),
        DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 4),
        DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 5),
        SR!(PHYASYMCLK_CLOCK_CNTL),
        SR!(PHYBSYMCLK_CLOCK_CNTL),
        SR!(PHYCSYMCLK_CLOCK_CNTL)
    };
}

macro_rules! DCCG_MASK_SH_LIST_DCN3 {
    ($mask_sh:tt) => {
        DCCG_MASK_SH_LIST_DCN2!($mask_sh),
        DCCG_SF!(HDMICHARCLK0_CLOCK_CNTL, HDMICHARCLK0_EN, $mask_sh),
        DCCG_SF!(HDMICHARCLK0_CLOCK_CNTL, HDMICHARCLK0_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYASYMCLK_CLOCK_CNTL, PHYASYMCLK_FORCE_EN, $mask_sh),
        DCCG_SF!(PHYASYMCLK_CLOCK_CNTL, PHYASYMCLK_FORCE_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYBSYMCLK_CLOCK_CNTL, PHYBSYMCLK_FORCE_EN, $mask_sh),
        DCCG_SF!(PHYBSYMCLK_CLOCK_CNTL, PHYBSYMCLK_FORCE_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYCSYMCLK_CLOCK_CNTL, PHYCSYMCLK_FORCE_EN, $mask_sh),
        DCCG_SF!(PHYCSYMCLK_CLOCK_CNTL, PHYCSYMCLK_FORCE_SRC_SEL, $mask_sh)
    };
}

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dccg_registers {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dccg_shift {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dccg_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dccg {
    _private: [u8; 0],
}

extern "C" {
    pub fn dccg3_create(
        ctx: *mut dc_context,
        regs: *const dccg_registers,
        dccg_shift: *const dccg_shift,
        dccg_mask: *const dccg_mask,
    ) -> *mut dccg;

    pub fn dccg30_create(
        ctx: *mut dc_context,
        regs: *const dccg_registers,
        dccg_shift: *const dccg_shift,
        dccg_mask: *const dccg_mask,
    ) -> *mut dccg;

    pub fn dccg3_enable_hdmicharclk(
        dccg: *mut dccg,
        hpo_inst: ::core::ffi::c_int,
        phypll_inst: ::core::ffi::c_int,
    );

    pub fn dccg3_disable_hdmicharclk(
        dccg: *mut dccg,
        hpo_inst: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
