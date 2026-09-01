/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Audio driver for AK4458
 *
 * Copyright (C) 2016 Asahi Kasei Microdevices Corporation
 * Copyright 2018 NXP
 */

/* C header dependency: <linux/regmap.h> */

/* Settings */

pub const AK4458_00_CONTROL1: u32 = 0x00;
pub const AK4458_01_CONTROL2: u32 = 0x01;
pub const AK4458_02_CONTROL3: u32 = 0x02;
pub const AK4458_03_LCHATT: u32 = 0x03;
pub const AK4458_04_RCHATT: u32 = 0x04;
pub const AK4458_05_CONTROL4: u32 = 0x05;
pub const AK4458_06_DSD1: u32 = 0x06;
pub const AK4458_07_CONTROL5: u32 = 0x07;
pub const AK4458_08_SOUND_CONTROL: u32 = 0x08;
pub const AK4458_09_DSD2: u32 = 0x09;
pub const AK4458_0A_CONTROL6: u32 = 0x0A;
pub const AK4458_0B_CONTROL7: u32 = 0x0B;
pub const AK4458_0C_CONTROL8: u32 = 0x0C;
pub const AK4458_0D_CONTROL9: u32 = 0x0D;
pub const AK4458_0E_CONTROL10: u32 = 0x0E;
pub const AK4458_0F_L2CHATT: u32 = 0x0F;
pub const AK4458_10_R2CHATT: u32 = 0x10;
pub const AK4458_11_L3CHATT: u32 = 0x11;
pub const AK4458_12_R3CHATT: u32 = 0x12;
pub const AK4458_13_L4CHATT: u32 = 0x13;
pub const AK4458_14_R4CHATT: u32 = 0x14;

/* Bitfield Definitions */

/* AK4458_00_CONTROL1 (0x00) Fields
 * Addr Register Name  D7     D6    D5    D4    D3    D2    D1    D0
 * 00H  Control 1      ACKS   0     0     0     DIF2  DIF1  DIF0  RSTN
 */

/* Digital Filter (SD, SLOW, SSLOW) */
pub const AK4458_SD_MASK: u32 = 0x20;
pub const AK4458_SLOW_MASK: u32 = 0x01;
pub const AK4458_SSLOW_MASK: u32 = 0x01;

/* DIF2	1 0
 *  x	1 0 MSB justified  Figure 3 (default)
 *  x	1 1 I2S Compliment  Figure 4
 */
pub const AK4458_DIF_SHIFT: u32 = 1;
pub const AK4458_DIF_MASK: u32 = 0x0e;

pub const AK4458_DIF_16BIT_LSB: u32 = 0 << 1;
pub const AK4458_DIF_24BIT_I2S: u32 = 3 << 1;
pub const AK4458_DIF_32BIT_LSB: u32 = 5 << 1;
pub const AK4458_DIF_32BIT_MSB: u32 = 6 << 1;
pub const AK4458_DIF_32BIT_I2S: u32 = 7 << 1;

/* AK4458_00_CONTROL1 (0x00) D0 bit */
pub const AK4458_RSTN_MASK: u32 = 0x01;
pub const AK4458_RSTN: u32 = 0x1 << 0;

/* AK4458_0A_CONTROL6 Mode bits */
pub const AK4458_MODE_SHIFT: u32 = 6;
pub const AK4458_MODE_MASK: u32 = 0xc0;
pub const AK4458_MODE_NORMAL: u32 = 0 << AK4458_MODE_SHIFT;
pub const AK4458_MODE_TDM128: u32 = 1 << AK4458_MODE_SHIFT;
pub const AK4458_MODE_TDM256: u32 = 2 << AK4458_MODE_SHIFT;
pub const AK4458_MODE_TDM512: u32 = 3 << AK4458_MODE_SHIFT;

/* DAC Digital attenuator transition time setting
 * Table 19
 * Mode	ATS1	ATS2	ATT speed
 * 0	0	0	4080/fs
 * 1	0	1	2040/fs
 * 2	1	0	510/fs
 * 3	1	1	255/fs
 * */
pub const AK4458_ATS_SHIFT: u32 = 6;
pub const AK4458_ATS_MASK: u32 = 0xc0;
pub const AK4458_DCHAIN_MASK: u32 = 0x1 << 1;

pub const AK4458_DSDSEL_MASK: u32 = 0x1 << 0;
pub const AK4458_DP_MASK: u32 = 0x1 << 7;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
