/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 Linaro Ltd.
 * Copyright (c) 2014 Hisilicon Limited.
 */

/* fixed rate */
pub const HIX5HD2_FIXED_1200M: u32 = 1;
pub const HIX5HD2_FIXED_400M: u32 = 2;
pub const HIX5HD2_FIXED_48M: u32 = 3;
pub const HIX5HD2_FIXED_24M: u32 = 4;
pub const HIX5HD2_FIXED_600M: u32 = 5;
pub const HIX5HD2_FIXED_300M: u32 = 6;
pub const HIX5HD2_FIXED_75M: u32 = 7;
pub const HIX5HD2_FIXED_200M: u32 = 8;
pub const HIX5HD2_FIXED_100M: u32 = 9;
pub const HIX5HD2_FIXED_40M: u32 = 10;
pub const HIX5HD2_FIXED_150M: u32 = 11;
pub const HIX5HD2_FIXED_1728M: u32 = 12;
pub const HIX5HD2_FIXED_28P8M: u32 = 13;
pub const HIX5HD2_FIXED_432M: u32 = 14;
pub const HIX5HD2_FIXED_345P6M: u32 = 15;
pub const HIX5HD2_FIXED_288M: u32 = 16;
pub const HIX5HD2_FIXED_60M: u32 = 17;
pub const HIX5HD2_FIXED_750M: u32 = 18;
pub const HIX5HD2_FIXED_500M: u32 = 19;
pub const HIX5HD2_FIXED_54M: u32 = 20;
pub const HIX5HD2_FIXED_27M: u32 = 21;
pub const HIX5HD2_FIXED_1500M: u32 = 22;
pub const HIX5HD2_FIXED_375M: u32 = 23;
pub const HIX5HD2_FIXED_187M: u32 = 24;
pub const HIX5HD2_FIXED_250M: u32 = 25;
pub const HIX5HD2_FIXED_125M: u32 = 26;
pub const HIX5HD2_FIXED_2P02M: u32 = 27;
pub const HIX5HD2_FIXED_50M: u32 = 28;
pub const HIX5HD2_FIXED_25M: u32 = 29;
pub const HIX5HD2_FIXED_83M: u32 = 30;

/* mux clocks */
pub const HIX5HD2_SFC_MUX: u32 = 64;
pub const HIX5HD2_MMC_MUX: u32 = 65;
pub const HIX5HD2_FEPHY_MUX: u32 = 66;
pub const HIX5HD2_SD_MUX: u32 = 67;

/* gate clocks */
pub const HIX5HD2_SFC_RST: u32 = 128;
pub const HIX5HD2_SFC_CLK: u32 = 129;
pub const HIX5HD2_MMC_CIU_CLK: u32 = 130;
pub const HIX5HD2_MMC_BIU_CLK: u32 = 131;
pub const HIX5HD2_MMC_CIU_RST: u32 = 132;
pub const HIX5HD2_FWD_BUS_CLK: u32 = 133;
pub const HIX5HD2_FWD_SYS_CLK: u32 = 134;
pub const HIX5HD2_MAC0_PHY_CLK: u32 = 135;
pub const HIX5HD2_SD_CIU_CLK: u32 = 136;
pub const HIX5HD2_SD_BIU_CLK: u32 = 137;
pub const HIX5HD2_SD_CIU_RST: u32 = 138;
pub const HIX5HD2_WDG0_CLK: u32 = 139;
pub const HIX5HD2_WDG0_RST: u32 = 140;
pub const HIX5HD2_I2C0_CLK: u32 = 141;
pub const HIX5HD2_I2C0_RST: u32 = 142;
pub const HIX5HD2_I2C1_CLK: u32 = 143;
pub const HIX5HD2_I2C1_RST: u32 = 144;
pub const HIX5HD2_I2C2_CLK: u32 = 145;
pub const HIX5HD2_I2C2_RST: u32 = 146;
pub const HIX5HD2_I2C3_CLK: u32 = 147;
pub const HIX5HD2_I2C3_RST: u32 = 148;
pub const HIX5HD2_I2C4_CLK: u32 = 149;
pub const HIX5HD2_I2C4_RST: u32 = 150;
pub const HIX5HD2_I2C5_CLK: u32 = 151;
pub const HIX5HD2_I2C5_RST: u32 = 152;

/* complex */
pub const HIX5HD2_MAC0_CLK: u32 = 192;
pub const HIX5HD2_MAC1_CLK: u32 = 193;
pub const HIX5HD2_SATA_CLK: u32 = 194;
pub const HIX5HD2_USB_CLK: u32 = 195;

pub const HIX5HD2_NR_CLKS: u32 = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
