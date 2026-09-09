/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency supplied by dcn32/dcn32_dpp.h.

macro_rules! DPP_REG_LIST_SH_MASK_DCN35 {
    ($mask_sh:expr) => {
        DPP_REG_LIST_SH_MASK_DCN30_COMMON!($mask_sh),
        TF_SF!(DPP_TOP0_DPP_CONTROL, DPP_FGCG_REP_DIS, $mask_sh),
        TF_SF!(DPP_TOP0_DPP_CONTROL, DPP_FGCG_REP_DIS, $mask_sh),
        TF_SF!(DPP_TOP0_DPP_CONTROL, DISPCLK_R_GATE_DISABLE, $mask_sh)
    };
}

macro_rules! DPP_REG_FIELD_LIST_DCN35 {
    ($type:ty) => {
        DPP_REG_FIELD_LIST_DCN3!($type);
        $type DPP_FGCG_REP_DIS;
    };
}

#[repr(C)]
pub struct dcn35_dpp_shift {
    DPP_REG_FIELD_LIST_DCN3!(u8);
    pub DPP_FGCG_REP_DIS: u8,
}

#[repr(C)]
pub struct dcn35_dpp_mask {
    DPP_REG_FIELD_LIST_DCN3!(u32);
    pub DPP_FGCG_REP_DIS: u32,
}

extern "C" {
    pub fn dpp35_dppclk_control(
        dpp_base: *mut dpp,
        dppclk_div: bool,
        enable: bool,
    );

    pub fn dpp35_construct(
        dpp3: *mut dcn3_dpp,
        ctx: *mut dc_context,
        inst: u32,
        tf_regs: *const dcn3_dpp_registers,
        tf_shift: *const dcn35_dpp_shift,
        tf_mask: *const dcn35_dpp_mask,
    ) -> bool;

    pub fn dpp35_set_fgcg(dpp: *mut dcn3_dpp, enable: bool);

    pub fn dpp35_program_bias_and_scale_fcnv(
        dpp_base: *mut dpp,
        bias_and_scale: *mut dc_bias_and_scale,
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
