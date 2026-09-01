/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fsl_spdif.h - ALSA S/PDIF interface for the Freescale i.MX SoC
 *
 * Copyright (C) 2013 Freescale Semiconductor, Inc.
 *
 * Author: Nicolin Chen <b42378@freescale.com>
 *
 * Based on fsl_ssi.h
 * Author: Timur Tabi <timur@freescale.com>
 * Copyright 2007-2008 Freescale Semiconductor, Inc.
 */

/* S/PDIF Register Map */
pub const REG_SPDIF_SCR: u32 = 0x0; /* SPDIF Configuration Register */
pub const REG_SPDIF_SRCD: u32 = 0x4; /* CDText Control Register */
pub const REG_SPDIF_SRPC: u32 = 0x8; /* PhaseConfig Register */
pub const REG_SPDIF_SIE: u32 = 0xc; /* InterruptEn Register */
pub const REG_SPDIF_SIS: u32 = 0x10; /* InterruptStat Register */
pub const REG_SPDIF_SIC: u32 = 0x10; /* InterruptClear Register */
pub const REG_SPDIF_SRL: u32 = 0x14; /* SPDIFRxLeft Register */
pub const REG_SPDIF_SRR: u32 = 0x18; /* SPDIFRxRight Register */
pub const REG_SPDIF_SRCSH: u32 = 0x1c; /* SPDIFRxCChannel_h Register */
pub const REG_SPDIF_SRCSL: u32 = 0x20; /* SPDIFRxCChannel_l Register */
pub const REG_SPDIF_SRU: u32 = 0x24; /* UchannelRx Register */
pub const REG_SPDIF_SRQ: u32 = 0x28; /* QchannelRx Register */
pub const REG_SPDIF_STL: u32 = 0x2C; /* SPDIFTxLeft Register */
pub const REG_SPDIF_STR: u32 = 0x30; /* SPDIFTxRight Register */
pub const REG_SPDIF_STCSCH: u32 = 0x34; /* SPDIFTxCChannelCons_h Register */
pub const REG_SPDIF_STCSCL: u32 = 0x38; /* SPDIFTxCChannelCons_l Register */
pub const REG_SPDIF_STCSPH: u32 = 0x3C; /* SPDIFTxCChannel_Prof_h Register */
pub const REG_SPDIF_STCSPL: u32 = 0x40; /* SPDIFTxCChannel_Prof_l Register */
pub const REG_SPDIF_SRFM: u32 = 0x44; /* FreqMeas Register */
pub const REG_SPDIF_STC: u32 = 0x50; /* SPDIFTxClk Register */

pub const REG_SPDIF_SRCCA_31_0: u32 = 0x60; /* SPDIF receive C channel register, bits 31-0 */
pub const REG_SPDIF_SRCCA_63_32: u32 = 0x64; /* SPDIF receive C channel register, bits 63-32 */
pub const REG_SPDIF_SRCCA_95_64: u32 = 0x68; /* SPDIF receive C channel register, bits 95-64 */
pub const REG_SPDIF_SRCCA_127_96: u32 = 0x6C; /* SPDIF receive C channel register, bits 127-96 */
pub const REG_SPDIF_SRCCA_159_128: u32 = 0x70; /* SPDIF receive C channel register, bits 159-128 */
pub const REG_SPDIF_SRCCA_191_160: u32 = 0x74; /* SPDIF receive C channel register, bits 191-160 */
pub const REG_SPDIF_STCCA_31_0: u32 = 0x78; /* SPDIF transmit C channel register, bits 31-0 */
pub const REG_SPDIF_STCCA_63_32: u32 = 0x7C; /* SPDIF transmit C channel register, bits 63-32 */
pub const REG_SPDIF_STCCA_95_64: u32 = 0x80; /* SPDIF transmit C channel register, bits 95-64 */
pub const REG_SPDIF_STCCA_127_96: u32 = 0x84; /* SPDIF transmit C channel register, bits 127-96 */
pub const REG_SPDIF_STCCA_159_128: u32 = 0x88; /* SPDIF transmit C channel register, bits 159-128 */
pub const REG_SPDIF_STCCA_191_160: u32 = 0x8C; /* SPDIF transmit C channel register, bits 191-160 */

/* SPDIF Configuration register */
pub const SCR_RXFIFO_CTL_OFFSET: u32 = 23;
pub const SCR_RXFIFO_CTL_MASK: u32 = 1 << SCR_RXFIFO_CTL_OFFSET;
pub const SCR_RXFIFO_CTL_ZERO: u32 = 1 << SCR_RXFIFO_CTL_OFFSET;
pub const SCR_RXFIFO_OFF_OFFSET: u32 = 22;
pub const SCR_RXFIFO_OFF_MASK: u32 = 1 << SCR_RXFIFO_OFF_OFFSET;
pub const SCR_RXFIFO_OFF: u32 = 1 << SCR_RXFIFO_OFF_OFFSET;
pub const SCR_RXFIFO_RST_OFFSET: u32 = 21;
pub const SCR_RXFIFO_RST_MASK: u32 = 1 << SCR_RXFIFO_RST_OFFSET;
pub const SCR_RXFIFO_RST: u32 = 1 << SCR_RXFIFO_RST_OFFSET;
pub const SCR_RXFIFO_FSEL_OFFSET: u32 = 19;
pub const SCR_RXFIFO_FSEL_MASK: u32 = 0x3 << SCR_RXFIFO_FSEL_OFFSET;
pub const SCR_RXFIFO_FSEL_IF0: u32 = 0x0 << SCR_RXFIFO_FSEL_OFFSET;
pub const SCR_RXFIFO_FSEL_IF4: u32 = 0x1 << SCR_RXFIFO_FSEL_OFFSET;
pub const SCR_RXFIFO_FSEL_IF8: u32 = 0x2 << SCR_RXFIFO_FSEL_OFFSET;
pub const SCR_RXFIFO_FSEL_IF12: u32 = 0x3 << SCR_RXFIFO_FSEL_OFFSET;
pub const SCR_RXFIFO_AUTOSYNC_OFFSET: u32 = 18;
pub const SCR_RXFIFO_AUTOSYNC_MASK: u32 = 1 << SCR_RXFIFO_AUTOSYNC_OFFSET;
pub const SCR_RXFIFO_AUTOSYNC: u32 = 1 << SCR_RXFIFO_AUTOSYNC_OFFSET;
pub const SCR_TXFIFO_AUTOSYNC_OFFSET: u32 = 17;
pub const SCR_TXFIFO_AUTOSYNC_MASK: u32 = 1 << SCR_TXFIFO_AUTOSYNC_OFFSET;
pub const SCR_TXFIFO_AUTOSYNC: u32 = 1 << SCR_TXFIFO_AUTOSYNC_OFFSET;
pub const SCR_TXFIFO_FSEL_OFFSET: u32 = 15;
pub const SCR_TXFIFO_FSEL_MASK: u32 = 0x3 << SCR_TXFIFO_FSEL_OFFSET;
pub const SCR_TXFIFO_FSEL_IF0: u32 = 0x0 << SCR_TXFIFO_FSEL_OFFSET;
pub const SCR_TXFIFO_FSEL_IF4: u32 = 0x1 << SCR_TXFIFO_FSEL_OFFSET;
pub const SCR_TXFIFO_FSEL_IF8: u32 = 0x2 << SCR_TXFIFO_FSEL_OFFSET;
pub const SCR_TXFIFO_FSEL_IF12: u32 = 0x3 << SCR_TXFIFO_FSEL_OFFSET;
pub const SCR_RAW_CAPTURE_MODE: u32 = 1 << 14;
pub const SCR_LOW_POWER: u32 = 1 << 13;
pub const SCR_SOFT_RESET: u32 = 1 << 12;
pub const SCR_TXFIFO_CTRL_OFFSET: u32 = 10;
pub const SCR_TXFIFO_CTRL_MASK: u32 = 0x3 << SCR_TXFIFO_CTRL_OFFSET;
pub const SCR_TXFIFO_CTRL_ZERO: u32 = 0x0 << SCR_TXFIFO_CTRL_OFFSET;
pub const SCR_TXFIFO_CTRL_NORMAL: u32 = 0x1 << SCR_TXFIFO_CTRL_OFFSET;
pub const SCR_TXFIFO_CTRL_ONESAMPLE: u32 = 0x2 << SCR_TXFIFO_CTRL_OFFSET;
pub const SCR_DMA_RX_EN_OFFSET: u32 = 9;
pub const SCR_DMA_RX_EN_MASK: u32 = 1 << SCR_DMA_RX_EN_OFFSET;
pub const SCR_DMA_RX_EN: u32 = 1 << SCR_DMA_RX_EN_OFFSET;
pub const SCR_DMA_TX_EN_OFFSET: u32 = 8;
pub const SCR_DMA_TX_EN_MASK: u32 = 1 << SCR_DMA_TX_EN_OFFSET;
pub const SCR_DMA_TX_EN: u32 = 1 << SCR_DMA_TX_EN_OFFSET;
pub const SCR_VAL_OFFSET: u32 = 5;
pub const SCR_VAL_MASK: u32 = 1 << SCR_VAL_OFFSET;
pub const SCR_VAL_CLEAR: u32 = 1 << SCR_VAL_OFFSET;
pub const SCR_TXSEL_OFFSET: u32 = 2;
pub const SCR_TXSEL_MASK: u32 = 0x7 << SCR_TXSEL_OFFSET;
pub const SCR_TXSEL_OFF: u32 = 0 << SCR_TXSEL_OFFSET;
pub const SCR_TXSEL_RX: u32 = 1 << SCR_TXSEL_OFFSET;
pub const SCR_TXSEL_NORMAL: u32 = 0x5 << SCR_TXSEL_OFFSET;
pub const SCR_USRC_SEL_OFFSET: u32 = 0x0;
pub const SCR_USRC_SEL_MASK: u32 = 0x3 << SCR_USRC_SEL_OFFSET;
pub const SCR_USRC_SEL_NONE: u32 = 0x0 << SCR_USRC_SEL_OFFSET;
pub const SCR_USRC_SEL_RECV: u32 = 0x1 << SCR_USRC_SEL_OFFSET;
pub const SCR_USRC_SEL_CHIP: u32 = 0x3 << SCR_USRC_SEL_OFFSET;

pub const fn SCR_DMA_xX_EN(tx: bool) -> u32 {
    if tx {
        SCR_DMA_TX_EN
    } else {
        SCR_DMA_RX_EN
    }
}

/* SPDIF CDText control */
pub const SRCD_CD_USER_OFFSET: u32 = 1;
pub const SRCD_CD_USER: u32 = 1 << SRCD_CD_USER_OFFSET;

/* SPDIF Phase Configuration register */
pub const SRPC_DPLL_LOCKED: u32 = 1 << 6;
pub const SRPC_CLKSRC_SEL_OFFSET: u32 = 7;
pub const SRPC_CLKSRC_SEL_MASK: u32 = 0xf << SRPC_CLKSRC_SEL_OFFSET;
pub const fn SRPC_CLKSRC_SEL_SET(x: u32) -> u32 {
    (x << SRPC_CLKSRC_SEL_OFFSET) & SRPC_CLKSRC_SEL_MASK
}
pub const SRPC_CLKSRC_SEL_LOCKED_OFFSET1: u32 = 5;
pub const SRPC_CLKSRC_SEL_LOCKED_OFFSET2: u32 = 2;
pub const SRPC_GAINSEL_OFFSET: u32 = 3;
pub const SRPC_GAINSEL_MASK: u32 = 0x7 << SRPC_GAINSEL_OFFSET;
pub const fn SRPC_GAINSEL_SET(x: u32) -> u32 {
    (x << SRPC_GAINSEL_OFFSET) & SRPC_GAINSEL_MASK
}

pub const SRPC_CLKSRC_MAX: u32 = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum spdif_gainsel {
    GAINSEL_MULTI_24 = 0,
    GAINSEL_MULTI_16,
    GAINSEL_MULTI_12,
    GAINSEL_MULTI_8,
    GAINSEL_MULTI_6,
    GAINSEL_MULTI_4,
    GAINSEL_MULTI_3,
}
pub const GAINSEL_MULTI_MAX: u32 = spdif_gainsel::GAINSEL_MULTI_3 as u32 + 1;
pub const SPDIF_DEFAULT_GAINSEL: spdif_gainsel = spdif_gainsel::GAINSEL_MULTI_8;

/* SPDIF interrupt mask define */
pub const INT_DPLL_LOCKED: u32 = 1 << 20;
pub const INT_TXFIFO_UNOV: u32 = 1 << 19;
pub const INT_TXFIFO_RESYNC: u32 = 1 << 18;
pub const INT_CNEW: u32 = 1 << 17;
pub const INT_VAL_NOGOOD: u32 = 1 << 16;
pub const INT_SYM_ERR: u32 = 1 << 15;
pub const INT_BIT_ERR: u32 = 1 << 14;
pub const INT_URX_FUL: u32 = 1 << 10;
pub const INT_URX_OV: u32 = 1 << 9;
pub const INT_QRX_FUL: u32 = 1 << 8;
pub const INT_QRX_OV: u32 = 1 << 7;
pub const INT_UQ_SYNC: u32 = 1 << 6;
pub const INT_UQ_ERR: u32 = 1 << 5;
pub const INT_RXFIFO_UNOV: u32 = 1 << 4;
pub const INT_RXFIFO_RESYNC: u32 = 1 << 3;
pub const INT_LOSS_LOCK: u32 = 1 << 2;
pub const INT_TX_EM: u32 = 1 << 1;
pub const INT_RXFIFO_FUL: u32 = 1 << 0;

/* SPDIF Clock register */
pub const STC_SYSCLK_DF_OFFSET: u32 = 11;
pub const STC_SYSCLK_DF_MASK: u32 = 0x1ff << STC_SYSCLK_DF_OFFSET;
pub const fn STC_SYSCLK_DF(x: u32) -> u32 {
    (((x).wrapping_sub(1)) << STC_SYSCLK_DF_OFFSET) & STC_SYSCLK_DF_MASK
}
pub const STC_TXCLK_SRC_OFFSET: u32 = 8;
pub const STC_TXCLK_SRC_MASK: u32 = 0x7 << STC_TXCLK_SRC_OFFSET;
pub const fn STC_TXCLK_SRC_SET(x: u32) -> u32 {
    (x << STC_TXCLK_SRC_OFFSET) & STC_TXCLK_SRC_MASK
}
pub const STC_TXCLK_ALL_EN_OFFSET: u32 = 7;
pub const STC_TXCLK_ALL_EN_MASK: u32 = 1 << STC_TXCLK_ALL_EN_OFFSET;
pub const STC_TXCLK_ALL_EN: u32 = 1 << STC_TXCLK_ALL_EN_OFFSET;
pub const STC_TXCLK_DF_OFFSET: u32 = 0;
pub const STC_TXCLK_DF_MASK: u32 = 0x7f << STC_TXCLK_DF_OFFSET;
pub const fn STC_TXCLK_DF(x: u32) -> u32 {
    (((x).wrapping_sub(1)) << STC_TXCLK_DF_OFFSET) & STC_TXCLK_DF_MASK
}
pub const STC_TXCLK_SRC_MAX: u32 = 8;

pub const STC_TXCLK_SPDIF_ROOT: u32 = 1;

/* SPDIF tx rate */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum spdif_txrate {
    SPDIF_TXRATE_22050 = 0,
    SPDIF_TXRATE_32000,
    SPDIF_TXRATE_44100,
    SPDIF_TXRATE_48000,
    SPDIF_TXRATE_88200,
    SPDIF_TXRATE_96000,
    SPDIF_TXRATE_176400,
    SPDIF_TXRATE_192000,
}
pub const SPDIF_TXRATE_MAX: u32 = spdif_txrate::SPDIF_TXRATE_192000 as u32 + 1;

pub const SPDIF_CSTATUS_BYTE: u32 = 6;
pub const SPDIF_UBITS_SIZE: u32 = 96;
pub const SPDIF_QSUB_SIZE: u32 = SPDIF_UBITS_SIZE / 8;

pub const FSL_SPDIF_RATES_PLAYBACK: u32 = SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

pub const FSL_SPDIF_RATES_CAPTURE: u32 = SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_64000
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

pub const FSL_SPDIF_FORMATS_PLAYBACK: u32 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE;

pub const FSL_SPDIF_FORMATS_CAPTURE: u32 = SNDRV_PCM_FMTBIT_S24_LE;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
