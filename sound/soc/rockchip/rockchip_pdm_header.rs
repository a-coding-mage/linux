/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rockchip PDM ALSA SoC Digital Audio Interface(DAI)  driver
 *
 * Copyright (C) 2017 Fuzhou Rockchip Electronics Co., Ltd
 */

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

/* PDM REGS */
pub const PDM_SYSCONFIG: u32 = 0x0000;
pub const PDM_CTRL0: u32 = 0x0004;
pub const PDM_CTRL1: u32 = 0x0008;
pub const PDM_CLK_CTRL: u32 = 0x000c;
pub const PDM_HPF_CTRL: u32 = 0x0010;
pub const PDM_FIFO_CTRL: u32 = 0x0014;
pub const PDM_DMA_CTRL: u32 = 0x0018;
pub const PDM_INT_EN: u32 = 0x001c;
pub const PDM_INT_CLR: u32 = 0x0020;
pub const PDM_INT_ST: u32 = 0x0024;
pub const PDM_RXFIFO_DATA: u32 = 0x0030;
pub const PDM_DATA_VALID: u32 = 0x0054;
pub const PDM_VERSION: u32 = 0x0058;

/* PDM_SYSCONFIG */
pub const PDM_RX_MASK: u32 = 0x1 << 2;
pub const PDM_RX_START: u32 = 0x1 << 2;
pub const PDM_RX_STOP: u32 = 0x0 << 2;
pub const PDM_RX_CLR_MASK: u32 = 0x1 << 0;
pub const PDM_RX_CLR_WR: u32 = 0x1 << 0;
pub const PDM_RX_CLR_DONE: u32 = 0x0 << 0;

/* PDM CTRL0 */
pub const PDM_PATH_MSK: u32 = 0xf << 27;
pub const PDM_MODE_MSK: u32 = BIT(31);
pub const PDM_MODE_RJ: u32 = 0;
pub const PDM_MODE_LJ: u32 = BIT(31);
pub const PDM_PATH3_EN: u32 = BIT(30);
pub const PDM_PATH2_EN: u32 = BIT(29);
pub const PDM_PATH1_EN: u32 = BIT(28);
pub const PDM_PATH0_EN: u32 = BIT(27);
pub const PDM_HWT_EN: u32 = BIT(26);
pub const PDM_SAMPLERATE_MSK: u32 = GENMASK(7, 5);
pub const fn PDM_SAMPLERATE(x: u32) -> u32 {
    x << 5
}
pub const PDM_VDW_MSK: u32 = 0x1f << 0;
pub const fn PDM_VDW(X: u32) -> u32 {
    (X - 1) << 0
}

/* PDM CTRL1 */
pub const PDM_FD_NUMERATOR_SFT: u32 = 16;
pub const PDM_FD_NUMERATOR_MSK: u32 = GENMASK(31, 16);
pub const PDM_FD_DENOMINATOR_SFT: u32 = 0;
pub const PDM_FD_DENOMINATOR_MSK: u32 = GENMASK(15, 0);

/* PDM CLK CTRL */
pub const fn PDM_PATH_SHIFT(x: u32) -> u32 {
    8 + x * 2
}
pub const fn PDM_PATH_MASK(x: u32) -> u32 {
    0x3 << PDM_PATH_SHIFT(x)
}
pub const fn PDM_PATH(x: u32, v: u32) -> u32 {
    v << PDM_PATH_SHIFT(x)
}
pub const PDM_CLK_FD_RATIO_MSK: u32 = BIT(6);
pub const PDM_CLK_FD_RATIO_40: u32 = 0x0 << 6;
pub const PDM_CLK_FD_RATIO_35: u32 = BIT(6);
pub const PDM_CLK_MSK: u32 = BIT(5);
pub const PDM_CLK_EN: u32 = BIT(5);
pub const PDM_CLK_DIS: u32 = 0x0 << 5;
pub const PDM_CKP_MSK: u32 = BIT(3);
pub const PDM_CKP_NORMAL: u32 = 0x0 << 3;
pub const PDM_CKP_INVERTED: u32 = BIT(3);
pub const PDM_DS_RATIO_MSK: u32 = 0x7 << 0;
pub const PDM_CLK_320FS: u32 = 0x0 << 0;
pub const PDM_CLK_640FS: u32 = 0x1 << 0;
pub const PDM_CLK_1280FS: u32 = 0x2 << 0;
pub const PDM_CLK_2560FS: u32 = 0x3 << 0;
pub const PDM_CLK_5120FS: u32 = 0x4 << 0;
pub const PDM_CIC_RATIO_MSK: u32 = 0x3 << 0;

/* PDM HPF CTRL */
pub const PDM_HPF_LE: u32 = BIT(3);
pub const PDM_HPF_RE: u32 = BIT(2);
pub const PDM_HPF_CF_MSK: u32 = 0x3 << 0;
pub const PDM_HPF_3P79HZ: u32 = 0x0 << 0;
pub const PDM_HPF_60HZ: u32 = 0x1 << 0;
pub const PDM_HPF_243HZ: u32 = 0x2 << 0;
pub const PDM_HPF_493HZ: u32 = 0x3 << 0;

/* PDM DMA CTRL */
pub const PDM_DMA_RD_MSK: u32 = BIT(8);
pub const PDM_DMA_RD_EN: u32 = BIT(8);
pub const PDM_DMA_RD_DIS: u32 = 0x0 << 8;
pub const PDM_DMA_RDL_MSK: u32 = 0x7f << 0;
pub const fn PDM_DMA_RDL(X: u32) -> u32 {
    (X - 1) << 0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
