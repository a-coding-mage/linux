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
 */

// C dependencies supplied by the surrounding translation unit:
// #include "mcif_wb.h"
// #include "dcn32/dcn32_mmhubbub.h"

// MCIF_WB_REG_VARIABLE_LIST_DCN3_5 expands the DCN3.0 register list and adds
// uint32_t MMHUBBUB_CLOCK_CNTL.
// MCIF_WB_COMMON_MASK_SH_LIST_DCN3_5(mask_sh) expands the DCN32 mask list and
// adds fields MMHUBBUB_TEST_CLK_SEL, DISPCLK_R_MMHUBBUB_GATE_DIS,
// DISPCLK_G_WBIF0_GATE_DIS, SOCCLK_G_WBIF0_GATE_DIS, and
// MMHUBBUB_FGCG_REP_DIS.
// MCIF_WB_REG_FIELD_LIST_DCN3_5(type) expands the DCN3.0 field list and adds
// the five fields above with the supplied type.

#[repr(C)]
pub struct dcn35_mmhubbub_registers {
    pub MMHUBBUB_CLOCK_CNTL: u32,
}

#[repr(C)]
pub struct dcn35_mmhubbub_mask {
    pub MMHUBBUB_TEST_CLK_SEL: u32,
    pub DISPCLK_R_MMHUBBUB_GATE_DIS: u32,
    pub DISPCLK_G_WBIF0_GATE_DIS: u32,
    pub SOCCLK_G_WBIF0_GATE_DIS: u32,
    pub MMHUBBUB_FGCG_REP_DIS: u32,
}

#[repr(C)]
pub struct dcn35_mmhubbub_shift {
    pub MMHUBBUB_TEST_CLK_SEL: u8,
    pub DISPCLK_R_MMHUBBUB_GATE_DIS: u8,
    pub DISPCLK_G_WBIF0_GATE_DIS: u8,
    pub SOCCLK_G_WBIF0_GATE_DIS: u8,
    pub MMHUBBUB_FGCG_REP_DIS: u8,
}

extern "C" {
    pub fn dcn35_mmhubbub_construct(
        mcif_wb30: *mut dcn30_mmhubbub,
        ctx: *mut dc_context,
        mcif_wb_regs: *const dcn35_mmhubbub_registers,
        mcif_wb_shift: *const dcn35_mmhubbub_shift,
        mcif_wb_mask: *const dcn35_mmhubbub_mask,
        inst: i32,
    );

    pub fn dcn35_mmhubbub_set_fgcg(mcif_wb30: *mut dcn30_mmhubbub, enabled: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
