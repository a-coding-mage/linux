/*
 * Copyright 2025 Advanced Micro Devices, Inc.
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
 */

// addressBlock: aigc_grbma_grbma_grbmadec
// base address: 0x18000
pub const regGRBMA_GFX_INDEX: u32 = 0x0011;
pub const regGRBMA_GFX_INDEX_BASE_IDX: u32 = 1;

// addressBlock: aigc_grbma_grbma_perfddec
// base address: 0x19200
pub const regGRBMA_PERFCOUNTER0_LO: u32 = 0x0480;
pub const regGRBMA_PERFCOUNTER0_LO_BASE_IDX: u32 = 1;
pub const regGRBMA_PERFCOUNTER0_HI: u32 = 0x0481;
pub const regGRBMA_PERFCOUNTER0_HI_BASE_IDX: u32 = 1;
pub const regGRBMA_PERFCOUNTER1_LO: u32 = 0x0482;
pub const regGRBMA_PERFCOUNTER1_LO_BASE_IDX: u32 = 1;
pub const regGRBMA_PERFCOUNTER1_HI: u32 = 0x0483;
pub const regGRBMA_PERFCOUNTER1_HI_BASE_IDX: u32 = 1;

// addressBlock: aigc_grbma_grbma_perfsdec
// base address: 0x19300
pub const regGRBMA_PERFCOUNTER0_SELECT: u32 = 0x04c0;
pub const regGRBMA_PERFCOUNTER0_SELECT_BASE_IDX: u32 = 1;
pub const regGRBMA_PERFCOUNTER1_SELECT: u32 = 0x04c1;
pub const regGRBMA_PERFCOUNTER1_SELECT_BASE_IDX: u32 = 1;
pub const regAID_PERFMON_CNTL: u32 = 0x04c2;
pub const regAID_PERFMON_CNTL_BASE_IDX: u32 = 1;

// addressBlock: aigc_gl2x_gfx_se_perfsdec
// base address: 0x19300
pub const regGL2C_PERFCOUNTER0_SELECT: u32 = 0x04e8;
pub const regGL2C_PERFCOUNTER0_SELECT_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER0_SELECT1: u32 = 0x04e9;
pub const regGL2C_PERFCOUNTER0_SELECT1_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER1_SELECT: u32 = 0x04ea;
pub const regGL2C_PERFCOUNTER1_SELECT_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER1_SELECT1: u32 = 0x04eb;
pub const regGL2C_PERFCOUNTER1_SELECT1_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER2_SELECT: u32 = 0x04ec;
pub const regGL2C_PERFCOUNTER2_SELECT_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER2_SELECT1: u32 = 0x04ed;
pub const regGL2C_PERFCOUNTER2_SELECT1_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER3_SELECT: u32 = 0x04ee;
pub const regGL2C_PERFCOUNTER3_SELECT_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER3_SELECT1: u32 = 0x04ef;
pub const regGL2C_PERFCOUNTER3_SELECT1_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER0_SELECT: u32 = 0x04f0;
pub const regGL2A_PERFCOUNTER0_SELECT_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER0_SELECT1: u32 = 0x04f1;
pub const regGL2A_PERFCOUNTER0_SELECT1_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER1_SELECT: u32 = 0x04f2;
pub const regGL2A_PERFCOUNTER1_SELECT_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER1_SELECT1: u32 = 0x04f3;
pub const regGL2A_PERFCOUNTER1_SELECT1_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER2_SELECT: u32 = 0x04f4;
pub const regGL2A_PERFCOUNTER2_SELECT_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER2_SELECT1: u32 = 0x04f5;
pub const regGL2A_PERFCOUNTER2_SELECT1_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER3_SELECT: u32 = 0x04f6;
pub const regGL2A_PERFCOUNTER3_SELECT_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER3_SELECT1: u32 = 0x04f7;
pub const regGL2A_PERFCOUNTER3_SELECT1_BASE_IDX: u32 = 1;

// addressBlock: aigc_gl2x_gfx_se_perfddec
// base address: 0x19200
pub const regGL2C_PERFCOUNTER0_LO: u32 = 0x04a0;
pub const regGL2C_PERFCOUNTER0_LO_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER0_HI: u32 = 0x04a1;
pub const regGL2C_PERFCOUNTER0_HI_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER1_LO: u32 = 0x04a2;
pub const regGL2C_PERFCOUNTER1_LO_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER1_HI: u32 = 0x04a3;
pub const regGL2C_PERFCOUNTER1_HI_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER2_LO: u32 = 0x04a4;
pub const regGL2C_PERFCOUNTER2_LO_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER2_HI: u32 = 0x04a5;
pub const regGL2C_PERFCOUNTER2_HI_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER3_LO: u32 = 0x04a6;
pub const regGL2C_PERFCOUNTER3_LO_BASE_IDX: u32 = 1;
pub const regGL2C_PERFCOUNTER3_HI: u32 = 0x04a7;
pub const regGL2C_PERFCOUNTER3_HI_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER0_LO: u32 = 0x04a8;
pub const regGL2A_PERFCOUNTER0_LO_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER0_HI: u32 = 0x04a9;
pub const regGL2A_PERFCOUNTER0_HI_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER1_LO: u32 = 0x04aa;
pub const regGL2A_PERFCOUNTER1_LO_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER1_HI: u32 = 0x04ab;
pub const regGL2A_PERFCOUNTER1_HI_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER2_LO: u32 = 0x04ac;
pub const regGL2A_PERFCOUNTER2_LO_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER2_HI: u32 = 0x04ad;
pub const regGL2A_PERFCOUNTER2_HI_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER3_LO: u32 = 0x04ae;
pub const regGL2A_PERFCOUNTER3_LO_BASE_IDX: u32 = 1;
pub const regGL2A_PERFCOUNTER3_HI: u32 = 0x04af;
pub const regGL2A_PERFCOUNTER3_HI_BASE_IDX: u32 = 1;

// addressBlock: aigc_gfx_gcea_se_gfx_se_perfsdec
// base address: 0x19320
pub const regGC_EA_SE_PERFCOUNTER0_SELECT: u32 = 0x04c8;
pub const regGC_EA_SE_PERFCOUNTER0_SELECT_BASE_IDX: u32 = 1;
pub const regGC_EA_SE_PERFCOUNTER0_SELECT1: u32 = 0x04c9;
pub const regGC_EA_SE_PERFCOUNTER0_SELECT1_BASE_IDX: u32 = 1;
pub const regGC_EA_SE_PERFCOUNTER1_SELECT: u32 = 0x04ca;
pub const regGC_EA_SE_PERFCOUNTER1_SELECT_BASE_IDX: u32 = 1;

// addressBlock: aigc_gfx_gcea_se_gfx_se_perfddec
// base address: 0x19240
pub const regGC_EA_SE_PERFCOUNTER0_LO: u32 = 0x0490;
pub const regGC_EA_SE_PERFCOUNTER0_LO_BASE_IDX: u32 = 1;
pub const regGC_EA_SE_PERFCOUNTER0_HI: u32 = 0x0491;
pub const regGC_EA_SE_PERFCOUNTER0_HI_BASE_IDX: u32 = 1;
pub const regGC_EA_SE_PERFCOUNTER1_LO: u32 = 0x0492;
pub const regGC_EA_SE_PERFCOUNTER1_LO_BASE_IDX: u32 = 1;
pub const regGC_EA_SE_PERFCOUNTER1_HI: u32 = 0x0493;
pub const regGC_EA_SE_PERFCOUNTER1_HI_BASE_IDX: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
