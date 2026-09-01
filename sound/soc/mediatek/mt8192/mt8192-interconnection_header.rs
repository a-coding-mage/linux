/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Mediatek MT8192 audio driver interconnection definition
 *
 * Copyright (c) 2020 MediaTek Inc.
 * Author: Shane Chien <shane.chien@mediatek.com>
 */

/* in port define */
pub const I_I2S0_CH1: i32 = 0;
pub const I_I2S0_CH2: i32 = 1;
pub const I_ADDA_UL_CH1: i32 = 3;
pub const I_ADDA_UL_CH2: i32 = 4;
pub const I_DL1_CH1: i32 = 5;
pub const I_DL1_CH2: i32 = 6;
pub const I_DL2_CH1: i32 = 7;
pub const I_DL2_CH2: i32 = 8;
pub const I_PCM_1_CAP_CH1: i32 = 9;
pub const I_GAIN1_OUT_CH1: i32 = 10;
pub const I_GAIN1_OUT_CH2: i32 = 11;
pub const I_GAIN2_OUT_CH1: i32 = 12;
pub const I_GAIN2_OUT_CH2: i32 = 13;
pub const I_PCM_2_CAP_CH1: i32 = 14;
pub const I_ADDA_UL_CH3: i32 = 17;
pub const I_ADDA_UL_CH4: i32 = 18;
pub const I_DL12_CH1: i32 = 19;
pub const I_DL12_CH2: i32 = 20;
pub const I_PCM_2_CAP_CH2: i32 = 21;
pub const I_PCM_1_CAP_CH2: i32 = 22;
pub const I_DL3_CH1: i32 = 23;
pub const I_DL3_CH2: i32 = 24;
pub const I_I2S2_CH1: i32 = 25;
pub const I_I2S2_CH2: i32 = 26;
pub const I_I2S2_CH3: i32 = 27;
pub const I_I2S2_CH4: i32 = 28;

/* in port define >= 32 */
pub const I_32_OFFSET: i32 = 32;
pub const I_CONNSYS_I2S_CH1: i32 = 34 - I_32_OFFSET;
pub const I_CONNSYS_I2S_CH2: i32 = 35 - I_32_OFFSET;
pub const I_SRC_1_OUT_CH1: i32 = 36 - I_32_OFFSET;
pub const I_SRC_1_OUT_CH2: i32 = 37 - I_32_OFFSET;
pub const I_SRC_2_OUT_CH1: i32 = 38 - I_32_OFFSET;
pub const I_SRC_2_OUT_CH2: i32 = 39 - I_32_OFFSET;
pub const I_DL4_CH1: i32 = 40 - I_32_OFFSET;
pub const I_DL4_CH2: i32 = 41 - I_32_OFFSET;
pub const I_DL5_CH1: i32 = 42 - I_32_OFFSET;
pub const I_DL5_CH2: i32 = 43 - I_32_OFFSET;
pub const I_DL6_CH1: i32 = 44 - I_32_OFFSET;
pub const I_DL6_CH2: i32 = 45 - I_32_OFFSET;
pub const I_DL7_CH1: i32 = 46 - I_32_OFFSET;
pub const I_DL7_CH2: i32 = 47 - I_32_OFFSET;
pub const I_DL8_CH1: i32 = 48 - I_32_OFFSET;
pub const I_DL8_CH2: i32 = 49 - I_32_OFFSET;
pub const I_DL9_CH1: i32 = 50 - I_32_OFFSET;
pub const I_DL9_CH2: i32 = 51 - I_32_OFFSET;
pub const I_I2S6_CH1: i32 = 52 - I_32_OFFSET;
pub const I_I2S6_CH2: i32 = 53 - I_32_OFFSET;
pub const I_I2S8_CH1: i32 = 54 - I_32_OFFSET;
pub const I_I2S8_CH2: i32 = 55 - I_32_OFFSET;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
