/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 */

/* LPASS_AUDIO_CC clocks */
pub const LPASS_AUDIO_CC_PLL: u32 = 0;
pub const LPASS_AUDIO_CC_PLL_OUT_AUX2: u32 = 1;
pub const LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC: u32 = 2;
pub const LPASS_AUDIO_CC_PLL_OUT_MAIN_DIV_CLK_SRC: u32 = 3;
pub const LPASS_AUDIO_CC_CDIV_RX_MCLK_DIV_CLK_SRC: u32 = 4;
pub const LPASS_AUDIO_CC_CODEC_MEM0_CLK: u32 = 5;
pub const LPASS_AUDIO_CC_CODEC_MEM1_CLK: u32 = 6;
pub const LPASS_AUDIO_CC_CODEC_MEM2_CLK: u32 = 7;
pub const LPASS_AUDIO_CC_CODEC_MEM_CLK: u32 = 8;
pub const LPASS_AUDIO_CC_EXT_MCLK0_CLK: u32 = 9;
pub const LPASS_AUDIO_CC_EXT_MCLK0_CLK_SRC: u32 = 10;
pub const LPASS_AUDIO_CC_EXT_MCLK1_CLK: u32 = 11;
pub const LPASS_AUDIO_CC_EXT_MCLK1_CLK_SRC: u32 = 12;
pub const LPASS_AUDIO_CC_RX_MCLK_2X_CLK: u32 = 13;
pub const LPASS_AUDIO_CC_RX_MCLK_CLK: u32 = 14;
pub const LPASS_AUDIO_CC_RX_MCLK_CLK_SRC: u32 = 15;

/* LPASS AUDIO CC CSR */
pub const LPASS_AUDIO_SWR_RX_CGCR: u32 = 0;
pub const LPASS_AUDIO_SWR_TX_CGCR: u32 = 1;
pub const LPASS_AUDIO_SWR_WSA_CGCR: u32 = 2;

/* LPASS_AON_CC clocks */
pub const LPASS_AON_CC_PLL: u32 = 0;
pub const LPASS_AON_CC_PLL_OUT_EVEN: u32 = 1;
pub const LPASS_AON_CC_PLL_OUT_MAIN_CDIV_DIV_CLK_SRC: u32 = 2;
pub const LPASS_AON_CC_PLL_OUT_ODD: u32 = 3;
pub const LPASS_AON_CC_AUDIO_HM_H_CLK: u32 = 4;
pub const LPASS_AON_CC_CDIV_TX_MCLK_DIV_CLK_SRC: u32 = 5;
pub const LPASS_AON_CC_MAIN_RCG_CLK_SRC: u32 = 6;
pub const LPASS_AON_CC_TX_MCLK_2X_CLK: u32 = 7;
pub const LPASS_AON_CC_TX_MCLK_CLK: u32 = 8;
pub const LPASS_AON_CC_TX_MCLK_RCG_CLK_SRC: u32 = 9;
pub const LPASS_AON_CC_VA_MEM0_CLK: u32 = 10;

/* LPASS_AON_CC power domains */
pub const LPASS_AON_CC_LPASS_AUDIO_HM_GDSC: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
