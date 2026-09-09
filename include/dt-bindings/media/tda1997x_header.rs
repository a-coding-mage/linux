/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2017 Gateworks Corporation
 */

/* TDA19973 36bit Video Port control registers */
pub const TDA1997X_VP36_35_32: u32 = 0;
pub const TDA1997X_VP36_31_28: u32 = 1;
pub const TDA1997X_VP36_27_24: u32 = 2;
pub const TDA1997X_VP36_23_20: u32 = 3;
pub const TDA1997X_VP36_19_16: u32 = 4;
pub const TDA1997X_VP36_15_12: u32 = 5;
pub const TDA1997X_VP36_11_08: u32 = 6;
pub const TDA1997X_VP36_07_04: u32 = 7;
pub const TDA1997X_VP36_03_00: u32 = 8;

/* TDA19971 24bit Video Port control registers */
pub const TDA1997X_VP24_V23_20: u32 = 0;
pub const TDA1997X_VP24_V19_16: u32 = 1;
pub const TDA1997X_VP24_V15_12: u32 = 3;
pub const TDA1997X_VP24_V11_08: u32 = 4;
pub const TDA1997X_VP24_V07_04: u32 = 6;
pub const TDA1997X_VP24_V03_00: u32 = 7;

/* Pin groups */
pub const TDA1997X_VP_OUT_EN: u32 = 0x80; /* enable output group */
pub const TDA1997X_VP_HIZ: u32 = 0x40; /* hi-Z output group when not used */
pub const TDA1997X_VP_SWP: u32 = 0x10; /* pin-swap output group */
pub const TDA1997X_R_CR_CBCR_3_0: u32 = 0 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
pub const TDA1997X_R_CR_CBCR_7_4: u32 = 1 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
pub const TDA1997X_R_CR_CBCR_11_8: u32 = 2 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
pub const TDA1997X_B_CB_3_0: u32 = 3 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
pub const TDA1997X_B_CB_7_4: u32 = 4 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
pub const TDA1997X_B_CB_11_8: u32 = 5 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
pub const TDA1997X_G_Y_3_0: u32 = 6 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
pub const TDA1997X_G_Y_7_4: u32 = 7 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
pub const TDA1997X_G_Y_11_8: u32 = 8 | TDA1997X_VP_OUT_EN | TDA1997X_VP_HIZ;
/* pinswapped groups */
pub const TDA1997X_R_CR_CBCR_3_0_S: u32 = TDA1997X_R_CR_CBCR_3_0 | TDA1997X_VP_SWAP;
pub const TDA1997X_R_CR_CBCR_7_4_S: u32 = TDA1997X_R_CR_CBCR_7_4 | TDA1997X_VP_SWAP;
pub const TDA1997X_R_CR_CBCR_11_8_S: u32 = TDA1997X_R_CR_CBCR_11_8 | TDA1997X_VP_SWAP;
pub const TDA1997X_B_CB_3_0_S: u32 = TDA1997X_B_CB_3_0 | TDA1997X_VP_SWAP;
pub const TDA1997X_B_CB_7_4_S: u32 = TDA1997X_B_CB_7_4 | TDA1997X_VP_SWAP;
pub const TDA1997X_B_CB_11_8_S: u32 = TDA1997X_B_CB_11_8 | TDA1997X_VP_SWAP;
pub const TDA1997X_G_Y_3_0_S: u32 = TDA1997X_G_Y_3_0 | TDA1997X_VP_SWAP;
pub const TDA1997X_G_Y_7_4_S: u32 = TDA1997X_G_Y_7_4 | TDA1997X_VP_SWAP;
pub const TDA1997X_G_Y_11_8_S: u32 = TDA1997X_G_Y_11_8 | TDA1997X_VP_SWAP;

/* Audio bus DAI format */
pub const TDA1997X_I2S16: u32 = 1; /* I2S 16bit */
pub const TDA1997X_I2S32: u32 = 2; /* I2S 32bit */
pub const TDA1997X_SPDIF: u32 = 3; /* SPDIF */
pub const TDA1997X_OBA: u32 = 4; /* One Bit Audio */
pub const TDA1997X_DST: u32 = 5; /* Direct Stream Transfer */
pub const TDA1997X_I2S16_HBR: u32 = 6; /* HBR straight in I2S 16bit mode */
pub const TDA1997X_I2S16_HBR_DEMUX: u32 = 7; /* HBR demux in I2S 16bit mode */
pub const TDA1997X_I2S32_HBR_DEMUX: u32 = 8; /* HBR demux in I2S 32bit mode */
pub const TDA1997X_SPDIF_HBR_DEMUX: u32 = 9; /* HBR demux in SPDIF mode */

/* Audio bus channel layout */
pub const TDA1997X_LAYOUT0: u32 = 0; /* 2-channel */
pub const TDA1997X_LAYOUT1: u32 = 1; /* 8-channel */

/* Audio bus clock */
pub const TDA1997X_ACLK_16FS: u32 = 0;
pub const TDA1997X_ACLK_32FS: u32 = 1;
pub const TDA1997X_ACLK_64FS: u32 = 2;
pub const TDA1997X_ACLK_128FS: u32 = 3;
pub const TDA1997X_ACLK_256FS: u32 = 4;
pub const TDA1997X_ACLK_512FS: u32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
