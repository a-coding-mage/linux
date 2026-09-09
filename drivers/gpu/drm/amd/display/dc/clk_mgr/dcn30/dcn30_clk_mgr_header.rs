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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// CLK1_CLK_PLL_REQ
pub const CLK11_CLK1_CLK_PLL_REQ__FbMult_int__SHIFT: u32 = 0x0;
pub const CLK11_CLK1_CLK_PLL_REQ__PllSpineDiv__SHIFT: u32 = 0xc;
pub const CLK11_CLK1_CLK_PLL_REQ__FbMult_frac__SHIFT: u32 = 0x10;
pub const CLK11_CLK1_CLK_PLL_REQ__FbMult_int_MASK: u32 = 0x000001FF;
pub const CLK11_CLK1_CLK_PLL_REQ__PllSpineDiv_MASK: u32 = 0x0000F000;
pub const CLK11_CLK1_CLK_PLL_REQ__FbMult_frac_MASK: u32 = 0xFFFF0000;

// CLK1_CLK0_DFS_CNTL
pub const CLK11_CLK1_CLK0_DFS_CNTL__CLK0_DIVIDER__SHIFT: u32 = 0x0;
pub const CLK11_CLK1_CLK0_DFS_CNTL__CLK0_DIVIDER_MASK: u32 = 0x0000007F;
/* DPREF clock related */
pub const CLK0_CLK3_DFS_CNTL__CLK3_DIVIDER__SHIFT: u32 = 0x0;
pub const CLK0_CLK3_DFS_CNTL__CLK3_DIVIDER_MASK: u32 = 0x0000007F;
pub const CLK1_CLK3_DFS_CNTL__CLK3_DIVIDER__SHIFT: u32 = 0x0;
pub const CLK1_CLK3_DFS_CNTL__CLK3_DIVIDER_MASK: u32 = 0x0000007F;
pub const CLK2_CLK3_DFS_CNTL__CLK3_DIVIDER__SHIFT: u32 = 0x0;
pub const CLK2_CLK3_DFS_CNTL__CLK3_DIVIDER_MASK: u32 = 0x0000007F;
pub const CLK3_CLK3_DFS_CNTL__CLK3_DIVIDER__SHIFT: u32 = 0x0;
pub const CLK3_CLK3_DFS_CNTL__CLK3_DIVIDER_MASK: u32 = 0x0000007F;

// CLK3_0_CLK3_CLK_PLL_REQ
pub const CLK3_0_CLK3_CLK_PLL_REQ__FbMult_int__SHIFT: u32 = 0x0;
pub const CLK3_0_CLK3_CLK_PLL_REQ__PllSpineDiv__SHIFT: u32 = 0xc;
pub const CLK3_0_CLK3_CLK_PLL_REQ__FbMult_frac__SHIFT: u32 = 0x10;
pub const CLK3_0_CLK3_CLK_PLL_REQ__FbMult_int_MASK: u32 = 0x000001FF;
pub const CLK3_0_CLK3_CLK_PLL_REQ__PllSpineDiv_MASK: u32 = 0x0000F000;
pub const CLK3_0_CLK3_CLK_PLL_REQ__FbMult_frac_MASK: u32 = 0xFFFF0000;

pub const mmCLK0_CLK2_DFS_CNTL: u32 = 0x16C55;
pub const mmCLK00_CLK0_CLK2_DFS_CNTL: u32 = 0x16C55;
pub const mmCLK01_CLK0_CLK2_DFS_CNTL: u32 = 0x16E55;
pub const mmCLK02_CLK0_CLK2_DFS_CNTL: u32 = 0x17055;

pub const mmCLK0_CLK3_DFS_CNTL: u32 = 0x16C60;
pub const mmCLK00_CLK0_CLK3_DFS_CNTL: u32 = 0x16C60;
pub const mmCLK01_CLK0_CLK3_DFS_CNTL: u32 = 0x16E60;
pub const mmCLK02_CLK0_CLK3_DFS_CNTL: u32 = 0x17060;
pub const mmCLK03_CLK0_CLK3_DFS_CNTL: u32 = 0x17260;

pub const mmCLK0_CLK_PLL_REQ: u32 = 0x16C10;
pub const mmCLK00_CLK0_CLK_PLL_REQ: u32 = 0x16C10;
pub const mmCLK01_CLK0_CLK_PLL_REQ: u32 = 0x16E10;
pub const mmCLK02_CLK0_CLK_PLL_REQ: u32 = 0x17010;
pub const mmCLK03_CLK0_CLK_PLL_REQ: u32 = 0x17210;

pub const mmCLK1_CLK_PLL_REQ: u32 = 0x1B00D;
pub const mmCLK10_CLK1_CLK_PLL_REQ: u32 = 0x1B00D;
pub const mmCLK11_CLK1_CLK_PLL_REQ: u32 = 0x1B20D;
pub const mmCLK12_CLK1_CLK_PLL_REQ: u32 = 0x1B40D;
pub const mmCLK13_CLK1_CLK_PLL_REQ: u32 = 0x1B60D;

pub const mmCLK2_CLK_PLL_REQ: u32 = 0x17E0D;

/* AMCLK */
pub const mmCLK11_CLK1_CLK0_DFS_CNTL: u32 = 0x1B23F;
pub const mmCLK11_CLK1_CLK_PLL_REQ: u32 = 0x1B20D;

extern "C" {
    pub fn dcn3_init_clocks(clk_mgr_base: *mut clk_mgr);

    pub fn dcn3_clk_mgr_construct(
        ctx: *mut dc_context,
        clk_mgr: *mut clk_mgr_internal,
        pp_smu: *mut pp_smu_funcs,
        dccg: *mut dccg,
    );

    pub fn dcn3_clk_mgr_destroy(clk_mgr: *mut clk_mgr_internal);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
