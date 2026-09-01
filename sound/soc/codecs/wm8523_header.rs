/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8523.h  --  WM8523 ASoC driver
 *
 * Copyright 2009 Wolfson Microelectronics, plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 * Based on wm8753.h
 */

/*
 * Register values.
 */
pub const WM8523_DEVICE_ID: u32 = 0x00;
pub const WM8523_REVISION: u32 = 0x01;
pub const WM8523_PSCTRL1: u32 = 0x02;
pub const WM8523_AIF_CTRL1: u32 = 0x03;
pub const WM8523_AIF_CTRL2: u32 = 0x04;
pub const WM8523_DAC_CTRL3: u32 = 0x05;
pub const WM8523_DAC_GAINL: u32 = 0x06;
pub const WM8523_DAC_GAINR: u32 = 0x07;
pub const WM8523_ZERO_DETECT: u32 = 0x08;

pub const WM8523_REGISTER_COUNT: u32 = 9;
pub const WM8523_MAX_REGISTER: u32 = 0x08;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - DEVICE_ID
 */
pub const WM8523_CHIP_ID_MASK: u32 = 0xFFFF; /* CHIP_ID - [15:0] */
pub const WM8523_CHIP_ID_SHIFT: u32 = 0; /* CHIP_ID - [15:0] */
pub const WM8523_CHIP_ID_WIDTH: u32 = 16; /* CHIP_ID - [15:0] */

/*
 * R1 (0x01) - REVISION
 */
pub const WM8523_CHIP_REV_MASK: u32 = 0x0007; /* CHIP_REV - [2:0] */
pub const WM8523_CHIP_REV_SHIFT: u32 = 0; /* CHIP_REV - [2:0] */
pub const WM8523_CHIP_REV_WIDTH: u32 = 3; /* CHIP_REV - [2:0] */

/*
 * R2 (0x02) - PSCTRL1
 */
pub const WM8523_SYS_ENA_MASK: u32 = 0x0003; /* SYS_ENA - [1:0] */
pub const WM8523_SYS_ENA_SHIFT: u32 = 0; /* SYS_ENA - [1:0] */
pub const WM8523_SYS_ENA_WIDTH: u32 = 2; /* SYS_ENA - [1:0] */

/*
 * R3 (0x03) - AIF_CTRL1
 */
pub const WM8523_TDM_MODE_MASK: u32 = 0x1800; /* TDM_MODE - [12:11] */
pub const WM8523_TDM_MODE_SHIFT: u32 = 11; /* TDM_MODE - [12:11] */
pub const WM8523_TDM_MODE_WIDTH: u32 = 2; /* TDM_MODE - [12:11] */
pub const WM8523_TDM_SLOT_MASK: u32 = 0x0600; /* TDM_SLOT - [10:9] */
pub const WM8523_TDM_SLOT_SHIFT: u32 = 9; /* TDM_SLOT - [10:9] */
pub const WM8523_TDM_SLOT_WIDTH: u32 = 2; /* TDM_SLOT - [10:9] */
pub const WM8523_DEEMPH: u32 = 0x0100; /* DEEMPH  */
pub const WM8523_DEEMPH_MASK: u32 = 0x0100; /* DEEMPH  */
pub const WM8523_DEEMPH_SHIFT: u32 = 8; /* DEEMPH  */
pub const WM8523_DEEMPH_WIDTH: u32 = 1; /* DEEMPH  */
pub const WM8523_AIF_MSTR: u32 = 0x0080; /* AIF_MSTR  */
pub const WM8523_AIF_MSTR_MASK: u32 = 0x0080; /* AIF_MSTR  */
pub const WM8523_AIF_MSTR_SHIFT: u32 = 7; /* AIF_MSTR  */
pub const WM8523_AIF_MSTR_WIDTH: u32 = 1; /* AIF_MSTR  */
pub const WM8523_LRCLK_INV: u32 = 0x0040; /* LRCLK_INV  */
pub const WM8523_LRCLK_INV_MASK: u32 = 0x0040; /* LRCLK_INV  */
pub const WM8523_LRCLK_INV_SHIFT: u32 = 6; /* LRCLK_INV  */
pub const WM8523_LRCLK_INV_WIDTH: u32 = 1; /* LRCLK_INV  */
pub const WM8523_BCLK_INV: u32 = 0x0020; /* BCLK_INV  */
pub const WM8523_BCLK_INV_MASK: u32 = 0x0020; /* BCLK_INV  */
pub const WM8523_BCLK_INV_SHIFT: u32 = 5; /* BCLK_INV  */
pub const WM8523_BCLK_INV_WIDTH: u32 = 1; /* BCLK_INV  */
pub const WM8523_WL_MASK: u32 = 0x0018; /* WL - [4:3] */
pub const WM8523_WL_SHIFT: u32 = 3; /* WL - [4:3] */
pub const WM8523_WL_WIDTH: u32 = 2; /* WL - [4:3] */
pub const WM8523_FMT_MASK: u32 = 0x0007; /* FMT - [2:0] */
pub const WM8523_FMT_SHIFT: u32 = 0; /* FMT - [2:0] */
pub const WM8523_FMT_WIDTH: u32 = 3; /* FMT - [2:0] */

/*
 * R4 (0x04) - AIF_CTRL2
 */
pub const WM8523_DAC_OP_MUX_MASK: u32 = 0x00C0; /* DAC_OP_MUX - [7:6] */
pub const WM8523_DAC_OP_MUX_SHIFT: u32 = 6; /* DAC_OP_MUX - [7:6] */
pub const WM8523_DAC_OP_MUX_WIDTH: u32 = 2; /* DAC_OP_MUX - [7:6] */
pub const WM8523_BCLKDIV_MASK: u32 = 0x0038; /* BCLKDIV - [5:3] */
pub const WM8523_BCLKDIV_SHIFT: u32 = 3; /* BCLKDIV - [5:3] */
pub const WM8523_BCLKDIV_WIDTH: u32 = 3; /* BCLKDIV - [5:3] */
pub const WM8523_SR_MASK: u32 = 0x0007; /* SR - [2:0] */
pub const WM8523_SR_SHIFT: u32 = 0; /* SR - [2:0] */
pub const WM8523_SR_WIDTH: u32 = 3; /* SR - [2:0] */

/*
 * R5 (0x05) - DAC_CTRL3
 */
pub const WM8523_ZC: u32 = 0x0010; /* ZC  */
pub const WM8523_ZC_MASK: u32 = 0x0010; /* ZC  */
pub const WM8523_ZC_SHIFT: u32 = 4; /* ZC  */
pub const WM8523_ZC_WIDTH: u32 = 1; /* ZC  */
pub const WM8523_DACR: u32 = 0x0008; /* DACR  */
pub const WM8523_DACR_MASK: u32 = 0x0008; /* DACR  */
pub const WM8523_DACR_SHIFT: u32 = 3; /* DACR  */
pub const WM8523_DACR_WIDTH: u32 = 1; /* DACR  */
pub const WM8523_DACL: u32 = 0x0004; /* DACL  */
pub const WM8523_DACL_MASK: u32 = 0x0004; /* DACL  */
pub const WM8523_DACL_SHIFT: u32 = 2; /* DACL  */
pub const WM8523_DACL_WIDTH: u32 = 1; /* DACL  */
pub const WM8523_VOL_UP_RAMP: u32 = 0x0002; /* VOL_UP_RAMP  */
pub const WM8523_VOL_UP_RAMP_MASK: u32 = 0x0002; /* VOL_UP_RAMP  */
pub const WM8523_VOL_UP_RAMP_SHIFT: u32 = 1; /* VOL_UP_RAMP  */
pub const WM8523_VOL_UP_RAMP_WIDTH: u32 = 1; /* VOL_UP_RAMP  */
pub const WM8523_VOL_DOWN_RAMP: u32 = 0x0001; /* VOL_DOWN_RAMP  */
pub const WM8523_VOL_DOWN_RAMP_MASK: u32 = 0x0001; /* VOL_DOWN_RAMP  */
pub const WM8523_VOL_DOWN_RAMP_SHIFT: u32 = 0; /* VOL_DOWN_RAMP  */
pub const WM8523_VOL_DOWN_RAMP_WIDTH: u32 = 1; /* VOL_DOWN_RAMP  */

/*
 * R6 (0x06) - DAC_GAINL
 */
pub const WM8523_DACL_VU: u32 = 0x0200; /* DACL_VU  */
pub const WM8523_DACL_VU_MASK: u32 = 0x0200; /* DACL_VU  */
pub const WM8523_DACL_VU_SHIFT: u32 = 9; /* DACL_VU  */
pub const WM8523_DACL_VU_WIDTH: u32 = 1; /* DACL_VU  */
pub const WM8523_DACL_VOL_MASK: u32 = 0x01FF; /* DACL_VOL - [8:0] */
pub const WM8523_DACL_VOL_SHIFT: u32 = 0; /* DACL_VOL - [8:0] */
pub const WM8523_DACL_VOL_WIDTH: u32 = 9; /* DACL_VOL - [8:0] */

/*
 * R7 (0x07) - DAC_GAINR
 */
pub const WM8523_DACR_VU: u32 = 0x0200; /* DACR_VU  */
pub const WM8523_DACR_VU_MASK: u32 = 0x0200; /* DACR_VU  */
pub const WM8523_DACR_VU_SHIFT: u32 = 9; /* DACR_VU  */
pub const WM8523_DACR_VU_WIDTH: u32 = 1; /* DACR_VU  */
pub const WM8523_DACR_VOL_MASK: u32 = 0x01FF; /* DACR_VOL - [8:0] */
pub const WM8523_DACR_VOL_SHIFT: u32 = 0; /* DACR_VOL - [8:0] */
pub const WM8523_DACR_VOL_WIDTH: u32 = 9; /* DACR_VOL - [8:0] */

/*
 * R8 (0x08) - ZERO_DETECT
 */
pub const WM8523_ZD_COUNT_MASK: u32 = 0x0003; /* ZD_COUNT - [1:0] */
pub const WM8523_ZD_COUNT_SHIFT: u32 = 0; /* ZD_COUNT - [1:0] */
pub const WM8523_ZD_COUNT_WIDTH: u32 = 2; /* ZD_COUNT - [1:0] */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
