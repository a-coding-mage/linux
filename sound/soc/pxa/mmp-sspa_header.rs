// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * linux/sound/soc/pxa/mmp-sspa.h
 *
 * Copyright (C) 2011 Marvell International Ltd.
 */

/*
 * SSPA Registers
 */
pub const SSPA_D: u32 = 0x00;
pub const SSPA_ID: u32 = 0x04;
pub const SSPA_CTL: u32 = 0x08;
pub const SSPA_SP: u32 = 0x0c;
pub const SSPA_FIFO_UL: u32 = 0x10;
pub const SSPA_INT_MASK: u32 = 0x14;
pub const SSPA_C: u32 = 0x18;
pub const SSPA_FIFO_NOFS: u32 = 0x1c;
pub const SSPA_FIFO_SIZE: u32 = 0x20;

/* SSPA Control Register */
pub const SSPA_CTL_XPH: u32 = 1 << 31; /* Read Phase */
pub const SSPA_CTL_XFIG: u32 = 1 << 15; /* Transmit Zeros when FIFO Empty */
pub const SSPA_CTL_JST: u32 = 1 << 3; /* Audio Sample Justification */
pub const SSPA_CTL_XFRLEN2_MASK: u32 = 7 << 24;
pub const fn SSPA_CTL_XFRLEN2(x: u32) -> u32 {
    x << 24
} /* Transmit Frame Length in Phase 2 */
pub const SSPA_CTL_XWDLEN2_MASK: u32 = 7 << 21;
pub const fn SSPA_CTL_XWDLEN2(x: u32) -> u32 {
    x << 21
} /* Transmit Word Length in Phase 2 */
pub const fn SSPA_CTL_XDATDLY(x: u32) -> u32 {
    x << 19
} /* Transmit Data Delay */
pub const SSPA_CTL_XSSZ2_MASK: u32 = 7 << 16;
pub const fn SSPA_CTL_XSSZ2(x: u32) -> u32 {
    x << 16
} /* Transmit Sample Audio Size */
pub const SSPA_CTL_XFRLEN1_MASK: u32 = 7 << 8;
pub const fn SSPA_CTL_XFRLEN1(x: u32) -> u32 {
    x << 8
} /* Transmit Frame Length in Phase 1 */
pub const SSPA_CTL_XWDLEN1_MASK: u32 = 7 << 5;
pub const fn SSPA_CTL_XWDLEN1(x: u32) -> u32 {
    x << 5
} /* Transmit Word Length in Phase 1 */
pub const SSPA_CTL_XSSZ1_MASK: u32 = 7 << 0;
pub const fn SSPA_CTL_XSSZ1(x: u32) -> u32 {
    x << 0
} /* XSSZ1 */

pub const SSPA_CTL_8_BITS: u32 = 0x0; /* Sample Size */
pub const SSPA_CTL_12_BITS: u32 = 0x1;
pub const SSPA_CTL_16_BITS: u32 = 0x2;
pub const SSPA_CTL_20_BITS: u32 = 0x3;
pub const SSPA_CTL_24_BITS: u32 = 0x4;
pub const SSPA_CTL_32_BITS: u32 = 0x5;

/* SSPA Serial Port Register */
pub const SSPA_SP_WEN: u32 = 1 << 31; /* Write Configuration Enable */
pub const SSPA_SP_MSL: u32 = 1 << 18; /* Master Slave Configuration */
pub const SSPA_SP_CLKP: u32 = 1 << 17; /* CLKP Polarity Clock Edge Select */
pub const SSPA_SP_FSP: u32 = 1 << 16; /* FSP Polarity Clock Edge Select */
pub const SSPA_SP_FFLUSH: u32 = 1 << 2; /* FIFO Flush */
pub const SSPA_SP_S_RST: u32 = 1 << 1; /* Active High Reset Signal */
pub const SSPA_SP_S_EN: u32 = 1 << 0; /* Serial Clock Domain Enable */
pub const SSPA_SP_FWID_MASK: u32 = 0x3f << 20;
pub const fn SSPA_SP_FWID(x: u32) -> u32 {
    x << 20
} /* Frame-Sync Width */
pub const SSPA_TXSP_FPER_MASK: u32 = 0x3f << 4;
pub const fn SSPA_TXSP_FPER(x: u32) -> u32 {
    x << 4
} /* Frame-Sync Active */

/* sspa clock sources */
pub const MMP_SSPA_CLK_PLL: u32 = 0;
pub const MMP_SSPA_CLK_VCXO: u32 = 1;
pub const MMP_SSPA_CLK_AUDIO: u32 = 3;

/* sspa pll id */
pub const MMP_SYSCLK: u32 = 0;
pub const MMP_SSPA_CLK: u32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
