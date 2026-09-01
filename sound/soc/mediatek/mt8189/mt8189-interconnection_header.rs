// SPDX-License-Identifier: GPL-2.0
/*
 * Mediatek MT8189 audio driver interconnection definition
 *
 * Copyright (c) 2025 MediaTek Inc.
 * Author: Darren Ye <darren.ye@mediatek.com>
 */

/* in port define */
pub const I_CONNSYS_I2S_CH1: i32 = 0;
pub const I_CONNSYS_I2S_CH2: i32 = 1;
pub const I_GAIN0_OUT_CH1: i32 = 6;
pub const I_GAIN0_OUT_CH2: i32 = 7;
pub const I_GAIN1_OUT_CH1: i32 = 8;
pub const I_GAIN1_OUT_CH2: i32 = 9;
pub const I_GAIN2_OUT_CH1: i32 = 10;
pub const I_GAIN2_OUT_CH2: i32 = 11;
pub const I_GAIN3_OUT_CH1: i32 = 12;
pub const I_GAIN3_OUT_CH2: i32 = 13;
pub const I_STF_CH1: i32 = 14;
pub const I_ADDA_UL_CH1: i32 = 16;
pub const I_ADDA_UL_CH2: i32 = 17;
pub const I_ADDA_UL_CH3: i32 = 18;
pub const I_ADDA_UL_CH4: i32 = 19;
pub const I_UL_PROX_CH1: i32 = 20;
pub const I_UL_PROX_CH2: i32 = 21;
pub const I_ADDA_UL_CH5: i32 = 24;
pub const I_ADDA_UL_CH6: i32 = 25;
pub const I_DMIC0_CH1: i32 = 28;
pub const I_DMIC0_CH2: i32 = 29;
pub const I_DMIC1_CH1: i32 = 30;
pub const I_DMIC1_CH2: i32 = 31;

/* in port define >= 32 */
pub const I_32_OFFSET: i32 = 32;
pub const I_DL0_CH1: i32 = 32 - I_32_OFFSET;
pub const I_DL0_CH2: i32 = 33 - I_32_OFFSET;
pub const I_DL1_CH1: i32 = 34 - I_32_OFFSET;
pub const I_DL1_CH2: i32 = 35 - I_32_OFFSET;
pub const I_DL2_CH1: i32 = 36 - I_32_OFFSET;
pub const I_DL2_CH2: i32 = 37 - I_32_OFFSET;
pub const I_DL3_CH1: i32 = 38 - I_32_OFFSET;
pub const I_DL3_CH2: i32 = 39 - I_32_OFFSET;
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
pub const I_DL_24CH_CH1: i32 = 54 - I_32_OFFSET;
pub const I_DL_24CH_CH2: i32 = 55 - I_32_OFFSET;
pub const I_DL_24CH_CH3: i32 = 56 - I_32_OFFSET;
pub const I_DL_24CH_CH4: i32 = 57 - I_32_OFFSET;
pub const I_DL_24CH_CH5: i32 = 58 - I_32_OFFSET;
pub const I_DL_24CH_CH6: i32 = 59 - I_32_OFFSET;
pub const I_DL_24CH_CH7: i32 = 60 - I_32_OFFSET;
pub const I_DL_24CH_CH8: i32 = 61 - I_32_OFFSET;

/* in port define >= 64 */
pub const I_64_OFFSET: i32 = 64;
pub const I_DL23_CH1: i32 = 78 - I_64_OFFSET;
pub const I_DL23_CH2: i32 = 79 - I_64_OFFSET;
pub const I_DL24_CH1: i32 = 80 - I_64_OFFSET;
pub const I_DL24_CH2: i32 = 81 - I_64_OFFSET;
pub const I_DL25_CH1: i32 = 82 - I_64_OFFSET;
pub const I_DL25_CH2: i32 = 83 - I_64_OFFSET;

/* in port define >= 128 */
pub const I_128_OFFSET: i32 = 128;
pub const I_PCM_0_CAP_CH1: i32 = 130 - I_128_OFFSET;
pub const I_PCM_0_CAP_CH2: i32 = 131 - I_128_OFFSET;
pub const I_I2SIN0_CH1: i32 = 134 - I_128_OFFSET;
pub const I_I2SIN0_CH2: i32 = 135 - I_128_OFFSET;
pub const I_I2SIN1_CH1: i32 = 136 - I_128_OFFSET;
pub const I_I2SIN1_CH2: i32 = 137 - I_128_OFFSET;

/* in port define >= 192 */
pub const I_192_OFFSET: i32 = 192;
pub const I_SRC_0_OUT_CH1: i32 = 198 - I_192_OFFSET;
pub const I_SRC_0_OUT_CH2: i32 = 199 - I_192_OFFSET;
pub const I_SRC_1_OUT_CH1: i32 = 200 - I_192_OFFSET;
pub const I_SRC_1_OUT_CH2: i32 = 201 - I_192_OFFSET;
pub const I_SRC_2_OUT_CH1: i32 = 202 - I_192_OFFSET;
pub const I_SRC_2_OUT_CH2: i32 = 203 - I_192_OFFSET;
pub const I_SRC_3_OUT_CH1: i32 = 204 - I_192_OFFSET;
pub const I_SRC_3_OUT_CH2: i32 = 205 - I_192_OFFSET;
pub const I_SRC_4_OUT_CH1: i32 = 206 - I_192_OFFSET;
pub const I_SRC_4_OUT_CH2: i32 = 207 - I_192_OFFSET;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
