// SPDX-License-Identifier: GPL-2.0-only
/*
 * ak4535.h  --  AK4535 Soc Audio driver
 *
 * Copyright 2005 Openedhand Ltd.
 *
 * Author: Richard Purdie <richard@openedhand.com>
 *
 * Based on wm8753.h
 */

/* AK4535 register space */

pub const AK4535_PM1: u32 = 0x0;
pub const AK4535_PM2: u32 = 0x1;
pub const AK4535_SIG1: u32 = 0x2;
pub const AK4535_SIG2: u32 = 0x3;
pub const AK4535_MODE1: u32 = 0x4;
pub const AK4535_MODE2: u32 = 0x5;
pub const AK4535_DAC: u32 = 0x6;
pub const AK4535_MIC: u32 = 0x7;
pub const AK4535_TIMER: u32 = 0x8;
pub const AK4535_ALC1: u32 = 0x9;
pub const AK4535_ALC2: u32 = 0xa;
pub const AK4535_PGA: u32 = 0xb;
pub const AK4535_LATT: u32 = 0xc;
pub const AK4535_RATT: u32 = 0xd;
pub const AK4535_VOL: u32 = 0xe;
pub const AK4535_STATUS: u32 = 0xf;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
