/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 */

/* GPU_CC clocks */
pub const GPU_CC_PLL0: u32 = 0;
pub const GPU_CC_PLL1: u32 = 1;
pub const GPU_CC_AHB_CLK: u32 = 2;
pub const GPU_CC_CB_CLK: u32 = 3;
pub const GPU_CC_CRC_AHB_CLK: u32 = 4;
pub const GPU_CC_CX_GMU_CLK: u32 = 5;
pub const GPU_CC_CX_SNOC_DVM_CLK: u32 = 6;
pub const GPU_CC_CXO_AON_CLK: u32 = 7;
pub const GPU_CC_CXO_CLK: u32 = 8;
pub const GPU_CC_FREQ_MEASURE_CLK: u32 = 9;
pub const GPU_CC_GMU_CLK_SRC: u32 = 10;
pub const GPU_CC_GX_GMU_CLK: u32 = 11;
pub const GPU_CC_GX_VSENSE_CLK: u32 = 12;
pub const GPU_CC_HUB_AHB_DIV_CLK_SRC: u32 = 13;
pub const GPU_CC_HUB_AON_CLK: u32 = 14;
pub const GPU_CC_HUB_CLK_SRC: u32 = 15;
pub const GPU_CC_HUB_CX_INT_CLK: u32 = 16;
pub const GPU_CC_HUB_CX_INT_DIV_CLK_SRC: u32 = 17;
pub const GPU_CC_SLEEP_CLK: u32 = 18;
pub const GPU_CC_HLOS1_VOTE_GPU_SMMU_CLK: u32 = 19;

/* GPU_CC power domains */
pub const GPU_CC_CX_GDSC: u32 = 0;
pub const GPU_CC_GX_GDSC: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
