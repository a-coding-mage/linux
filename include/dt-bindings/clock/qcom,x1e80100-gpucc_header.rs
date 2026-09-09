/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 */

/* GPU_CC clocks */
pub const GPU_CC_AHB_CLK: u32 = 0;
pub const GPU_CC_CB_CLK: u32 = 1;
pub const GPU_CC_CRC_AHB_CLK: u32 = 2;
pub const GPU_CC_CX_FF_CLK: u32 = 3;
pub const GPU_CC_CX_GMU_CLK: u32 = 4;
pub const GPU_CC_CXO_AON_CLK: u32 = 5;
pub const GPU_CC_CXO_CLK: u32 = 6;
pub const GPU_CC_DEMET_CLK: u32 = 7;
pub const GPU_CC_DEMET_DIV_CLK_SRC: u32 = 8;
pub const GPU_CC_FF_CLK_SRC: u32 = 9;
pub const GPU_CC_FREQ_MEASURE_CLK: u32 = 10;
pub const GPU_CC_GMU_CLK_SRC: u32 = 11;
pub const GPU_CC_GX_GMU_CLK: u32 = 12;
pub const GPU_CC_GX_VSENSE_CLK: u32 = 13;
pub const GPU_CC_HLOS1_VOTE_GPU_SMMU_CLK: u32 = 14;
pub const GPU_CC_HUB_AON_CLK: u32 = 15;
pub const GPU_CC_HUB_CLK_SRC: u32 = 16;
pub const GPU_CC_HUB_CX_INT_CLK: u32 = 17;
pub const GPU_CC_MEMNOC_GFX_CLK: u32 = 18;
pub const GPU_CC_MND1X_0_GFX3D_CLK: u32 = 19;
pub const GPU_CC_MND1X_1_GFX3D_CLK: u32 = 20;
pub const GPU_CC_PLL0: u32 = 21;
pub const GPU_CC_PLL1: u32 = 22;
pub const GPU_CC_SLEEP_CLK: u32 = 23;
pub const GPU_CC_XO_CLK_SRC: u32 = 24;
pub const GPU_CC_XO_DIV_CLK_SRC: u32 = 25;
pub const GPU_CC_CX_ACCU_SHIFT_CLK: u32 = 26;
pub const GPU_CC_GX_ACCU_SHIFT_CLK: u32 = 27;

/* GDSCs */
pub const GPU_CX_GDSC: u32 = 0;
pub const GPU_GX_GDSC: u32 = 1;

/* GPU_CC resets */
pub const GPU_CC_ACD_BCR: u32 = 0;
pub const GPU_CC_CB_BCR: u32 = 1;
pub const GPU_CC_CX_BCR: u32 = 2;
pub const GPU_CC_FAST_HUB_BCR: u32 = 3;
pub const GPU_CC_FF_BCR: u32 = 4;
pub const GPU_CC_GFX3D_AON_BCR: u32 = 5;
pub const GPU_CC_GMU_BCR: u32 = 6;
pub const GPU_CC_GX_BCR: u32 = 7;
pub const GPU_CC_XO_BCR: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
