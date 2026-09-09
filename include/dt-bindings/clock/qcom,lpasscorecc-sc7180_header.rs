/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 */

/* LPASS_CORE_CC clocks */
pub const LPASS_LPAAUDIO_DIG_PLL: i32 = 0;
pub const LPASS_LPAAUDIO_DIG_PLL_OUT_ODD: i32 = 1;
pub const CORE_CLK_SRC: i32 = 2;
pub const EXT_MCLK0_CLK_SRC: i32 = 3;
pub const LPAIF_PRI_CLK_SRC: i32 = 4;
pub const LPAIF_SEC_CLK_SRC: i32 = 5;
pub const LPASS_AUDIO_CORE_CORE_CLK: i32 = 6;
pub const LPASS_AUDIO_CORE_EXT_MCLK0_CLK: i32 = 7;
pub const LPASS_AUDIO_CORE_LPAIF_PRI_IBIT_CLK: i32 = 8;
pub const LPASS_AUDIO_CORE_LPAIF_SEC_IBIT_CLK: i32 = 9;
pub const LPASS_AUDIO_CORE_SYSNOC_MPORT_CORE_CLK: i32 = 10;

/* LPASS Core power domains */
pub const LPASS_CORE_HM_GDSCR: i32 = 0;

/* LPASS Audio power domains */
pub const LPASS_AUDIO_HM_GDSCR: i32 = 0;
pub const LPASS_PDC_HM_GDSCR: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
