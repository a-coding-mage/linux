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

// Dependency: dcn20/dcn20_dsc.h

// C preprocessor equivalent. The referenced DCN20 definitions are supplied by
// the corresponding dependency translation.
macro_rules! DSC_REG_LIST_SH_MASK_DCN35 {
    ($mask_sh:expr) => {
        DSC_REG_LIST_SH_MASK_DCN20!($mask_sh),
        DSC_SF!(DSC_TOP0_DSC_TOP_CONTROL, DSC_FGCG_REP_DIS, $mask_sh)
    };
}

macro_rules! DSC_FIELD_LIST_DCN35 {
    ($ty:ty) => {
        DSC_FIELD_LIST_DCN20!($ty);
        $ty DSC_FGCG_REP_DIS;
    };
}

#[repr(C)]
pub struct dcn35_dsc_shift {
    // Expansion of DSC_FIELD_LIST_DCN20(uint8_t)
    pub dcn20: dcn20_dsc_shift,
    pub DSC_FGCG_REP_DIS: u8,
}

#[repr(C)]
pub struct dcn35_dsc_mask {
    // Expansion of DSC_FIELD_LIST_DCN20(uint32_t)
    pub dcn20: dcn20_dsc_mask,
    pub DSC_FGCG_REP_DIS: u32,
}

extern "C" {
    pub fn dsc35_construct(
        dsc: *mut dcn20_dsc,
        ctx: *mut dc_context,
        inst: ::core::ffi::c_int,
        dsc_regs: *const dcn20_dsc_registers,
        dsc_shift: *const dcn35_dsc_shift,
        dsc_mask: *const dcn35_dsc_mask,
    );

    pub fn dsc35_set_fgcg(dsc20: *mut dcn20_dsc, enable: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
