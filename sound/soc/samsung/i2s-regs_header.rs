/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Samsung I2S driver's register header
 */

pub const I2SCON: u32 = 0x0;
pub const I2SMOD: u32 = 0x4;
pub const I2SFIC: u32 = 0x8;
pub const I2SPSR: u32 = 0xc;
pub const I2STXD: u32 = 0x10;
pub const I2SRXD: u32 = 0x14;
pub const I2SFICS: u32 = 0x18;
pub const I2STXDS: u32 = 0x1c;
pub const I2SAHB: u32 = 0x20;
pub const I2SSTR0: u32 = 0x24;
pub const I2SSIZE: u32 = 0x28;
pub const I2STRNCNT: u32 = 0x2c;
pub const I2SLVL0ADDR: u32 = 0x30;
pub const I2SLVL1ADDR: u32 = 0x34;
pub const I2SLVL2ADDR: u32 = 0x38;
pub const I2SLVL3ADDR: u32 = 0x3c;
pub const I2SSTR1: u32 = 0x40;
pub const I2SVER: u32 = 0x44;
pub const I2SFIC1: u32 = 0x48;
pub const I2STDM: u32 = 0x4c;
pub const I2SFSTA: u32 = 0x50;

pub const CON_RSTCLR: u32 = 1 << 31;
pub const CON_FRXOFSTATUS: u32 = 1 << 26;
pub const CON_FRXORINTEN: u32 = 1 << 25;
pub const CON_FTXSURSTAT: u32 = 1 << 24;
pub const CON_FTXSURINTEN: u32 = 1 << 23;
pub const CON_TXSDMA_PAUSE: u32 = 1 << 20;
pub const CON_TXSDMA_ACTIVE: u32 = 1 << 18;

pub const CON_FTXURSTATUS: u32 = 1 << 17;
pub const CON_FTXURINTEN: u32 = 1 << 16;
pub const CON_TXFIFO2_EMPTY: u32 = 1 << 15;
pub const CON_TXFIFO1_EMPTY: u32 = 1 << 14;
pub const CON_TXFIFO2_FULL: u32 = 1 << 13;
pub const CON_TXFIFO1_FULL: u32 = 1 << 12;

pub const CON_LRINDEX: u32 = 1 << 11;
pub const CON_TXFIFO_EMPTY: u32 = 1 << 10;
pub const CON_RXFIFO_EMPTY: u32 = 1 << 9;
pub const CON_TXFIFO_FULL: u32 = 1 << 8;
pub const CON_RXFIFO_FULL: u32 = 1 << 7;
pub const CON_TXDMA_PAUSE: u32 = 1 << 6;
pub const CON_RXDMA_PAUSE: u32 = 1 << 5;
pub const CON_TXCH_PAUSE: u32 = 1 << 4;
pub const CON_RXCH_PAUSE: u32 = 1 << 3;
pub const CON_TXDMA_ACTIVE: u32 = 1 << 2;
pub const CON_RXDMA_ACTIVE: u32 = 1 << 1;
pub const CON_ACTIVE: u32 = 1 << 0;

pub const MOD_OPCLK_SHIFT: u32 = 30;
pub const MOD_OPCLK_CDCLK_OUT: u32 = 0 << MOD_OPCLK_SHIFT;
pub const MOD_OPCLK_CDCLK_IN: u32 = 1 << MOD_OPCLK_SHIFT;
pub const MOD_OPCLK_BCLK_OUT: u32 = 2 << MOD_OPCLK_SHIFT;
pub const MOD_OPCLK_PCLK: u32 = 3 << MOD_OPCLK_SHIFT;
pub const MOD_OPCLK_MASK: u32 = 3 << MOD_OPCLK_SHIFT;
pub const MOD_TXS_IDMA: u32 = 1 << 28; /* Sec_TXFIFO use I-DMA */

pub const MOD_BLCS_SHIFT: u32 = 26;
pub const MOD_BLCS_16BIT: u32 = 0 << MOD_BLCS_SHIFT;
pub const MOD_BLCS_8BIT: u32 = 1 << MOD_BLCS_SHIFT;
pub const MOD_BLCS_24BIT: u32 = 2 << MOD_BLCS_SHIFT;
pub const MOD_BLCS_MASK: u32 = 3 << MOD_BLCS_SHIFT;
pub const MOD_BLCP_SHIFT: u32 = 24;
pub const MOD_BLCP_16BIT: u32 = 0 << MOD_BLCP_SHIFT;
pub const MOD_BLCP_8BIT: u32 = 1 << MOD_BLCP_SHIFT;
pub const MOD_BLCP_24BIT: u32 = 2 << MOD_BLCP_SHIFT;
pub const MOD_BLCP_MASK: u32 = 3 << MOD_BLCP_SHIFT;

pub const MOD_C2DD_HHALF: u32 = 1 << 21; /* Discard Higher-half */
pub const MOD_C2DD_LHALF: u32 = 1 << 20; /* Discard Lower-half */
pub const MOD_C1DD_HHALF: u32 = 1 << 19;
pub const MOD_C1DD_LHALF: u32 = 1 << 18;
pub const MOD_DC2_EN: u32 = 1 << 17;
pub const MOD_DC1_EN: u32 = 1 << 16;
pub const MOD_BLC_16BIT: u32 = 0 << 13;
pub const MOD_BLC_8BIT: u32 = 1 << 13;
pub const MOD_BLC_24BIT: u32 = 2 << 13;
pub const MOD_BLC_MASK: u32 = 3 << 13;

pub const MOD_TXONLY: u32 = 0 << 8;
pub const MOD_RXONLY: u32 = 1 << 8;
pub const MOD_TXRX: u32 = 2 << 8;
pub const MOD_MASK: u32 = 3 << 8;
pub const MOD_LRP_SHIFT: u32 = 7;
pub const MOD_LR_LLOW: u32 = 0;
pub const MOD_LR_RLOW: u32 = 1;
pub const MOD_SDF_SHIFT: u32 = 5;
pub const MOD_SDF_IIS: u32 = 0;
pub const MOD_SDF_MSB: u32 = 1;
pub const MOD_SDF_LSB: u32 = 2;
pub const MOD_SDF_MASK: u32 = 3;
pub const MOD_RCLK_SHIFT: u32 = 3;
pub const MOD_RCLK_256FS: u32 = 0;
pub const MOD_RCLK_512FS: u32 = 1;
pub const MOD_RCLK_384FS: u32 = 2;
pub const MOD_RCLK_768FS: u32 = 3;
pub const MOD_RCLK_MASK: u32 = 3;
pub const MOD_BCLK_SHIFT: u32 = 1;
pub const MOD_BCLK_32FS: u32 = 0;
pub const MOD_BCLK_48FS: u32 = 1;
pub const MOD_BCLK_16FS: u32 = 2;
pub const MOD_BCLK_24FS: u32 = 3;
pub const MOD_BCLK_MASK: u32 = 3;
pub const MOD_8BIT: u32 = 1 << 0;

pub const EXYNOS5420_MOD_LRP_SHIFT: u32 = 15;
pub const EXYNOS5420_MOD_SDF_SHIFT: u32 = 6;
pub const EXYNOS5420_MOD_RCLK_SHIFT: u32 = 4;
pub const EXYNOS5420_MOD_BCLK_SHIFT: u32 = 0;
pub const EXYNOS5420_MOD_BCLK_64FS: u32 = 4;
pub const EXYNOS5420_MOD_BCLK_96FS: u32 = 5;
pub const EXYNOS5420_MOD_BCLK_128FS: u32 = 6;
pub const EXYNOS5420_MOD_BCLK_192FS: u32 = 7;
pub const EXYNOS5420_MOD_BCLK_256FS: u32 = 8;
pub const EXYNOS5420_MOD_BCLK_MASK: u32 = 0xf;

pub const EXYNOS7_MOD_RCLK_64FS: u32 = 4;
pub const EXYNOS7_MOD_RCLK_128FS: u32 = 5;
pub const EXYNOS7_MOD_RCLK_96FS: u32 = 6;
pub const EXYNOS7_MOD_RCLK_192FS: u32 = 7;

pub const PSR_PSREN: u32 = 1 << 15;
pub const fn PSR_PSVAL(x: u32) -> u32 {
    (((x).wrapping_sub(1)) << 8) & 0x3f00
}

pub const fn FIC_TX2COUNT(x: u32) -> u32 {
    ((x) >> 24) & 0xf
}

pub const fn FIC_TX1COUNT(x: u32) -> u32 {
    ((x) >> 16) & 0xf
}

pub const FIC_TXFLUSH: u32 = 1 << 15;
pub const FIC_RXFLUSH: u32 = 1 << 7;

pub const fn FIC_TXCOUNT(x: u32) -> u32 {
    ((x) >> 8) & 0xf
}

pub const fn FIC_RXCOUNT(x: u32) -> u32 {
    ((x) >> 0) & 0xf
}

pub const fn FICS_TXCOUNT(x: u32) -> u32 {
    ((x) >> 8) & 0x7f
}

pub const AHB_INTENLVL0: u32 = 1 << 24;
pub const AHB_LVL0INT: u32 = 1 << 20;
pub const AHB_CLRLVL0INT: u32 = 1 << 16;
pub const AHB_DMARLD: u32 = 1 << 5;
pub const AHB_INTMASK: u32 = 1 << 3;
pub const AHB_DMAEN: u32 = 1 << 0;
pub const AHB_LVLINTMASK: u32 = 0xf << 20;

pub const I2SSIZE_TRNMSK: u32 = 0xffff;
pub const I2SSIZE_SHIFT: u32 = 16;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
