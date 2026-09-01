// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm8971.h  --  audio driver for WM8971
 *
 * Copyright 2005 Lab126, Inc.
 *
 * Author: Kenneth Kiraly <kiraly@lab126.com>
 */

pub const WM8971_LINVOL: u32 = 0x00;
pub const WM8971_RINVOL: u32 = 0x01;
pub const WM8971_LOUT1V: u32 = 0x02;
pub const WM8971_ROUT1V: u32 = 0x03;
pub const WM8971_ADCDAC: u32 = 0x05;
pub const WM8971_IFACE: u32 = 0x07;
pub const WM8971_SRATE: u32 = 0x08;
pub const WM8971_LDAC: u32 = 0x0a;
pub const WM8971_RDAC: u32 = 0x0b;
pub const WM8971_BASS: u32 = 0x0c;
pub const WM8971_TREBLE: u32 = 0x0d;
pub const WM8971_RESET: u32 = 0x0f;
pub const WM8971_ALC1: u32 = 0x11;
pub const WM8971_ALC2: u32 = 0x12;
pub const WM8971_ALC3: u32 = 0x13;
pub const WM8971_NGATE: u32 = 0x14;
pub const WM8971_LADC: u32 = 0x15;
pub const WM8971_RADC: u32 = 0x16;
pub const WM8971_ADCTL1: u32 = 0x17;
pub const WM8971_ADCTL2: u32 = 0x18;
pub const WM8971_PWR1: u32 = 0x19;
pub const WM8971_PWR2: u32 = 0x1a;
pub const WM8971_ADCTL3: u32 = 0x1b;
pub const WM8971_ADCIN: u32 = 0x1f;
pub const WM8971_LADCIN: u32 = 0x20;
pub const WM8971_RADCIN: u32 = 0x21;
pub const WM8971_LOUTM1: u32 = 0x22;
pub const WM8971_LOUTM2: u32 = 0x23;
pub const WM8971_ROUTM1: u32 = 0x24;
pub const WM8971_ROUTM2: u32 = 0x25;
pub const WM8971_MOUTM1: u32 = 0x26;
pub const WM8971_MOUTM2: u32 = 0x27;
pub const WM8971_LOUT2V: u32 = 0x28;
pub const WM8971_ROUT2V: u32 = 0x29;
pub const WM8971_MOUTV: u32 = 0x2A;

pub const WM8971_SYSCLK: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
