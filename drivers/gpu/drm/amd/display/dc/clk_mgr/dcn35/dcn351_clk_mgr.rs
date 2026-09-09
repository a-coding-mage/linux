/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

pub const DCN_BASE__INST0_SEG1: u32 = 0x000000C0;
pub const mmCLK1_CLK_PLL_REQ: u32 = 0x16E37;

pub const mmCLK1_CLK0_DFS_CNTL: u32 = 0x16E69;
pub const mmCLK1_CLK1_DFS_CNTL: u32 = 0x16E6C;
pub const mmCLK1_CLK2_DFS_CNTL: u32 = 0x16E6F;
pub const mmCLK1_CLK3_DFS_CNTL: u32 = 0x16E72;
pub const mmCLK1_CLK4_DFS_CNTL: u32 = 0x16E75;
pub const mmCLK1_CLK5_DFS_CNTL: u32 = 0x16E78;

pub const mmCLK1_CLK0_CURRENT_CNT: u32 = 0x16EFC;
pub const mmCLK1_CLK1_CURRENT_CNT: u32 = 0x16EFD;
pub const mmCLK1_CLK2_CURRENT_CNT: u32 = 0x16EFE;
pub const mmCLK1_CLK3_CURRENT_CNT: u32 = 0x16EFF;
pub const mmCLK1_CLK4_CURRENT_CNT: u32 = 0x16F00;
pub const mmCLK1_CLK5_CURRENT_CNT: u32 = 0x16F01;

pub const mmCLK1_CLK0_BYPASS_CNTL: u32 = 0x16E8A;
pub const mmCLK1_CLK1_BYPASS_CNTL: u32 = 0x16E93;
pub const mmCLK1_CLK2_BYPASS_CNTL: u32 = 0x16E9C;
pub const mmCLK1_CLK3_BYPASS_CNTL: u32 = 0x16EA5;
pub const mmCLK1_CLK4_BYPASS_CNTL: u32 = 0x16EAE;
pub const mmCLK1_CLK5_BYPASS_CNTL: u32 = 0x16EB7;

pub const mmCLK1_CLK0_DS_CNTL: u32 = 0x16E83;
pub const mmCLK1_CLK1_DS_CNTL: u32 = 0x16E8C;
pub const mmCLK1_CLK2_DS_CNTL: u32 = 0x16E95;
pub const mmCLK1_CLK3_DS_CNTL: u32 = 0x16E9E;
pub const mmCLK1_CLK4_DS_CNTL: u32 = 0x16EA7;
pub const mmCLK1_CLK5_DS_CNTL: u32 = 0x16EB0;

pub const mmCLK1_CLK0_ALLOW_DS: u32 = 0x16E84;
pub const mmCLK1_CLK1_ALLOW_DS: u32 = 0x16E8D;
pub const mmCLK1_CLK2_ALLOW_DS: u32 = 0x16E96;
pub const mmCLK1_CLK3_ALLOW_DS: u32 = 0x16E9F;
pub const mmCLK1_CLK4_ALLOW_DS: u32 = 0x16EA8;
pub const mmCLK1_CLK5_ALLOW_DS: u32 = 0x16EB1;

pub const mmCLK5_spll_field_8: u32 = 0x1B04B;
pub const mmCLK6_spll_field_8: u32 = 0x1B24B;
pub const mmDENTIST_DISPCLK_CNTL: u32 = 0x0124;
pub const regDENTIST_DISPCLK_CNTL: u32 = 0x0064;
pub const regDENTIST_DISPCLK_CNTL_BASE_IDX: u32 = 1;

pub const CLK1_CLK_PLL_REQ__FbMult_int__SHIFT: u32 = 0x0;
pub const CLK1_CLK_PLL_REQ__PllSpineDiv__SHIFT: u32 = 0xc;
pub const CLK1_CLK_PLL_REQ__FbMult_frac__SHIFT: u32 = 0x10;
pub const CLK1_CLK_PLL_REQ__FbMult_int_MASK: u32 = 0x000001FF;
pub const CLK1_CLK_PLL_REQ__PllSpineDiv_MASK: u32 = 0x0000F000;
pub const CLK1_CLK_PLL_REQ__FbMult_frac_MASK: u32 = 0xFFFF0000;
pub const CLK1_CLK2_BYPASS_CNTL__CLK2_BYPASS_SEL_MASK: u32 = 0x00000007;

pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_WDIVIDER__SHIFT: u32 = 0x0;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_RDIVIDER__SHIFT: u32 = 0x8;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_CHG_DONE__SHIFT: u32 = 0x13;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DPPCLK_CHG_DONE__SHIFT: u32 = 0x14;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DPPCLK_WDIVIDER__SHIFT: u32 = 0x18;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_WDIVIDER_MASK: u32 = 0x0000007F;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_RDIVIDER_MASK: u32 = 0x00007F00;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_CHG_DONE_MASK: u32 = 0x00080000;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DPPCLK_CHG_DONE_MASK: u32 = 0x00100000;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DPPCLK_WDIVIDER_MASK: u32 = 0x7F000000;
pub const CLK5_spll_field_8__spll_ssc_en_MASK: u32 = 0x00002000;

// The following macro initializers expand through declarations supplied by the
// clock-manager headers; retain the C definitions as external data dependencies.
extern "C" {
    static clk_mgr_regs_dcn351: clk_mgr_registers;
    static clk_mgr_shift_dcn351: clk_mgr_shift;
    static clk_mgr_mask_dcn351: clk_mgr_mask;
    fn dcn35_clk_mgr_construct(
        ctx: *mut dc_context,
        clk_mgr: *mut clk_mgr_dcn35,
        pp_smu: *mut pp_smu_funcs,
        dccg: *mut dccg,
    );
}

pub unsafe fn dcn351_clk_mgr_construct(
    ctx: *mut dc_context,
    clk_mgr: *mut clk_mgr_dcn35,
    pp_smu: *mut pp_smu_funcs,
    dccg: *mut dccg,
) {
    (*clk_mgr).base.regs = &clk_mgr_regs_dcn351;
    (*clk_mgr).base.clk_mgr_shift = &clk_mgr_shift_dcn351;
    (*clk_mgr).base.clk_mgr_mask = &clk_mgr_mask_dcn351;
    dcn35_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
