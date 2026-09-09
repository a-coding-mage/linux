/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019, Jeffrey Hugo
 */

// Translated from qcom,gpucc-msm8998.h.

pub const GPUPLL0: u32 = 0;
pub const GPUPLL0_OUT_EVEN: u32 = 1;
pub const RBCPR_CLK_SRC: u32 = 2;
pub const GFX3D_CLK_SRC: u32 = 3;
pub const RBBMTIMER_CLK_SRC: u32 = 4;
pub const GFX3D_ISENSE_CLK_SRC: u32 = 5;
pub const RBCPR_CLK: u32 = 6;
pub const GFX3D_CLK: u32 = 7;
pub const RBBMTIMER_CLK: u32 = 8;
pub const GFX3D_ISENSE_CLK: u32 = 9;
pub const GPUCC_CXO_CLK: u32 = 10;

pub const GPU_CX_BCR: u32 = 0;
pub const RBCPR_BCR: u32 = 1;
pub const GPU_GX_BCR: u32 = 2;
pub const GPU_ISENSE_BCR: u32 = 3;

pub const GPU_CX_GDSC: u32 = 1;
pub const GPU_GX_GDSC: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
