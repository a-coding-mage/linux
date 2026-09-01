/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5670-dsp.h  --  RT5670 ALSA SoC DSP driver
 *
 * Copyright 2014 Realtek Microelectronics
 * Author: Bard Liao <bardliao@realtek.com>
 */

pub const RT5670_DSP_CTRL1: u32 = 0xe0;
pub const RT5670_DSP_CTRL2: u32 = 0xe1;
pub const RT5670_DSP_CTRL3: u32 = 0xe2;
pub const RT5670_DSP_CTRL4: u32 = 0xe3;
pub const RT5670_DSP_CTRL5: u32 = 0xe4;

/* DSP Control 1 (0xe0) */
pub const RT5670_DSP_CMD_MASK: u32 = 0xff << 8;
pub const RT5670_DSP_CMD_PE: u32 = 0x0d << 8; /* Patch Entry */
pub const RT5670_DSP_CMD_MW: u32 = 0x3b << 8; /* Memory Write */
pub const RT5670_DSP_CMD_MR: u32 = 0x37 << 8; /* Memory Read */
pub const RT5670_DSP_CMD_RR: u32 = 0x60 << 8; /* Register Read */
pub const RT5670_DSP_CMD_RW: u32 = 0x68 << 8; /* Register Write */
pub const RT5670_DSP_REG_DATHI: u32 = 0x26 << 8; /* High Data Addr */
pub const RT5670_DSP_REG_DATLO: u32 = 0x25 << 8; /* Low Data Addr */
pub const RT5670_DSP_CLK_MASK: u32 = 0x3 << 6;
pub const RT5670_DSP_CLK_SFT: u32 = 6;
pub const RT5670_DSP_CLK_768K: u32 = 0x0 << 6;
pub const RT5670_DSP_CLK_384K: u32 = 0x1 << 6;
pub const RT5670_DSP_CLK_192K: u32 = 0x2 << 6;
pub const RT5670_DSP_CLK_96K: u32 = 0x3 << 6;
pub const RT5670_DSP_BUSY_MASK: u32 = 0x1 << 5;
pub const RT5670_DSP_RW_MASK: u32 = 0x1 << 4;
pub const RT5670_DSP_DL_MASK: u32 = 0x3 << 2;
pub const RT5670_DSP_DL_0: u32 = 0x0 << 2;
pub const RT5670_DSP_DL_1: u32 = 0x1 << 2;
pub const RT5670_DSP_DL_2: u32 = 0x2 << 2;
pub const RT5670_DSP_DL_3: u32 = 0x3 << 2;
pub const RT5670_DSP_I2C_AL_16: u32 = 0x1 << 1;
pub const RT5670_DSP_CMD_EN: u32 = 0x1;

#[repr(C)]
pub struct rt5670_dsp_param {
    pub cmd_fmt: u16,
    pub addr: u16,
    pub data: u16,
    pub cmd: u8,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
