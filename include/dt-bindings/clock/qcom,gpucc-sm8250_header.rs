/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2017-2020, The Linux Foundation. All rights reserved.
 */

/* Header guard: _DT_BINDINGS_CLK_QCOM_GPU_CC_SM8250_H */

/* GPU_CC clock registers */
pub const GPU_CC_AHB_CLK: u32 = 0;
pub const GPU_CC_CRC_AHB_CLK: u32 = 1;
pub const GPU_CC_CX_APB_CLK: u32 = 2;
pub const GPU_CC_CX_GMU_CLK: u32 = 3;
pub const GPU_CC_CX_SNOC_DVM_CLK: u32 = 4;
pub const GPU_CC_CXO_AON_CLK: u32 = 5;
pub const GPU_CC_CXO_CLK: u32 = 6;
pub const GPU_CC_GMU_CLK_SRC: u32 = 7;
pub const GPU_CC_GX_GMU_CLK: u32 = 8;
pub const GPU_CC_PLL1: u32 = 9;
pub const GPU_CC_HLOS1_VOTE_GPU_SMMU_CLK: u32 = 10;

/* GPU_CC Resets */
pub const GPUCC_GPU_CC_ACD_BCR: u32 = 0;
pub const GPUCC_GPU_CC_CX_BCR: u32 = 1;
pub const GPUCC_GPU_CC_GFX3D_AON_BCR: u32 = 2;
pub const GPUCC_GPU_CC_GMU_BCR: u32 = 3;
pub const GPUCC_GPU_CC_GX_BCR: u32 = 4;
pub const GPUCC_GPU_CC_XO_BCR: u32 = 5;

/* GPU_CC GDSCRs */
pub const GPU_CX_GDSC: u32 = 0;
pub const GPU_GX_GDSC: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
