/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2005 Openedhand Ltd.
 *
 * Author: Richard Purdie <richard@openedhand.com>
 *
 * Based on WM8753.h
 */

/* WM8988 register space */

pub const WM8988_LINVOL: u32 = 0x00;
pub const WM8988_RINVOL: u32 = 0x01;
pub const WM8988_LOUT1V: u32 = 0x02;
pub const WM8988_ROUT1V: u32 = 0x03;
pub const WM8988_ADCDAC: u32 = 0x05;
pub const WM8988_IFACE: u32 = 0x07;
pub const WM8988_SRATE: u32 = 0x08;
pub const WM8988_LDAC: u32 = 0x0a;
pub const WM8988_RDAC: u32 = 0x0b;
pub const WM8988_BASS: u32 = 0x0c;
pub const WM8988_TREBLE: u32 = 0x0d;
pub const WM8988_RESET: u32 = 0x0f;
pub const WM8988_3D: u32 = 0x10;
pub const WM8988_ALC1: u32 = 0x11;
pub const WM8988_ALC2: u32 = 0x12;
pub const WM8988_ALC3: u32 = 0x13;
pub const WM8988_NGATE: u32 = 0x14;
pub const WM8988_LADC: u32 = 0x15;
pub const WM8988_RADC: u32 = 0x16;
pub const WM8988_ADCTL1: u32 = 0x17;
pub const WM8988_ADCTL2: u32 = 0x18;
pub const WM8988_PWR1: u32 = 0x19;
pub const WM8988_PWR2: u32 = 0x1a;
pub const WM8988_ADCTL3: u32 = 0x1b;
pub const WM8988_ADCIN: u32 = 0x1f;
pub const WM8988_LADCIN: u32 = 0x20;
pub const WM8988_RADCIN: u32 = 0x21;
pub const WM8988_LOUTM1: u32 = 0x22;
pub const WM8988_LOUTM2: u32 = 0x23;
pub const WM8988_ROUTM1: u32 = 0x24;
pub const WM8988_ROUTM2: u32 = 0x25;
pub const WM8988_LOUT2V: u32 = 0x28;
pub const WM8988_ROUT2V: u32 = 0x29;
pub const WM8988_LPPB: u32 = 0x43;
pub const WM8988_NUM_REG: u32 = 0x44;

pub const WM8988_SYSCLK: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
