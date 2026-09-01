// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 */

/* Timer Registers */
pub const WC: u32 = 0x1b7000;
pub const TIMR: u32 = 0x1b7004;
pub const TIMR_IE: u32 = 1 << 15;
pub const TIMR_IP: u32 = 1 << 14;
pub const GIP: u32 = 0x1b7010;
pub const GIE: u32 = 0x1b7014;

/* I2C Registers */
pub const I2C_IF_ADDRESS: u32 = 0x1B9000;
pub const I2C_IF_WDATA: u32 = 0x1B9004;
pub const I2C_IF_RDATA: u32 = 0x1B9008;
pub const I2C_IF_STATUS: u32 = 0x1B900C;
pub const I2C_IF_WLOCK: u32 = 0x1B9010;

/* Global Control Registers */
pub const GLOBAL_CNTL_GCTL: u32 = 0x1B7090;

/* PLL Registers */
pub const PLL_CTL: u32 = 0x1B7080;
pub const PLL_STAT: u32 = 0x1B7084;
pub const PLL_ENB: u32 = 0x1B7088;

/* SRC Registers */
pub const SRC_CTL: u32 = 0x1A0000; /* 0x1A0000 + (256 * Chn) */
pub const SRC_CCR: u32 = 0x1A0004; /* 0x1A0004 + (256 * Chn) */
pub const SRC_IMAP: u32 = 0x1A0008; /* 0x1A0008 + (256 * Chn) */
pub const SRC_CA: u32 = 0x1A0010; /* 0x1A0010 + (256 * Chn) */
pub const SRC_CF: u32 = 0x1A0014; /* 0x1A0014 + (256 * Chn) */
pub const SRC_SA: u32 = 0x1A0018; /* 0x1A0018 + (256 * Chn) */
pub const SRC_LA: u32 = 0x1A001C; /* 0x1A001C + (256 * Chn) */
pub const SRC_CTLSWR: u32 = 0x1A0020; /* 0x1A0020 + (256 * Chn) */
pub const SRC_CD: u32 = 0x1A0080; /* 0x1A0080 + (256 * Chn) + (4 * Regn) */
pub const SRC_MCTL: u32 = 0x1A012C;
pub const SRC_IP: u32 = 0x1A102C; /* 0x1A102C + (256 * Regn) */
pub const SRC_ENB: u32 = 0x1A282C; /* 0x1A282C + (256 * Regn) */
pub const SRC_ENBSTAT: u32 = 0x1A202C;
pub const SRC_ENBSA: u32 = 0x1A232C;
pub const SRC_DN0Z: u32 = 0x1A0030;
pub const SRC_DN1Z: u32 = 0x1A0040;
pub const SRC_UPZ: u32 = 0x1A0060;

/* GPIO Registers */
pub const GPIO_DATA: u32 = 0x1B7020;
pub const GPIO_CTRL: u32 = 0x1B7024;
pub const GPIO_EXT_DATA: u32 = 0x1B70A0;

/* Virtual memory registers */
pub const VMEM_PTPAL: u32 = 0x1C6300; /* 0x1C6300 + (16 * Chn) */
pub const VMEM_PTPAH: u32 = 0x1C6304; /* 0x1C6304 + (16 * Chn) */
pub const VMEM_CTL: u32 = 0x1C7000;

/* Transport Registers */
pub const TRANSPORT_ENB: u32 = 0x1B6000;
pub const TRANSPORT_CTL: u32 = 0x1B6004;
pub const TRANSPORT_INT: u32 = 0x1B6008;

/* Audio IO */
pub const AUDIO_IO_AIM: u32 = 0x1B5000; /* 0x1B5000 + (0x04 * Chn) */
pub const AUDIO_IO_TX_CTL: u32 = 0x1B5400; /* 0x1B5400 + (0x40 * Chn) */
pub const AUDIO_IO_TX_CSTAT_L: u32 = 0x1B5408; /* 0x1B5408 + (0x40 * Chn) */
pub const AUDIO_IO_TX_CSTAT_H: u32 = 0x1B540C; /* 0x1B540C + (0x40 * Chn) */
pub const AUDIO_IO_RX_CTL: u32 = 0x1B5410; /* 0x1B5410 + (0x40 * Chn) */
pub const AUDIO_IO_RX_SRT_CTL: u32 = 0x1B5420; /* 0x1B5420 + (0x40 * Chn) */
pub const AUDIO_IO_MCLK: u32 = 0x1B5600;
pub const AUDIO_IO_TX_BLRCLK: u32 = 0x1B5604;
pub const AUDIO_IO_RX_BLRCLK: u32 = 0x1B5608;

/* Mixer */
pub const MIXER_AMOPLO: u32 = 0x130000; /* 0x130000 + (8 * Chn) [4095 : 0] */
pub const MIXER_AMOPHI: u32 = 0x130004; /* 0x130004 + (8 * Chn) [4095 : 0] */
pub const MIXER_PRING_LO_HI: u32 = 0x188000; /* 0x188000 + (4 * Chn) [4095 : 0] */
pub const MIXER_PMOPLO: u32 = 0x138000; /* 0x138000 + (8 * Chn) [4095 : 0] */
pub const MIXER_PMOPHI: u32 = 0x138004; /* 0x138004 + (8 * Chn) [4095 : 0] */
pub const MIXER_AR_ENABLE: u32 = 0x19000C;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
