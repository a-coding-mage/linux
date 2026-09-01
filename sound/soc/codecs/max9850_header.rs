// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * max9850.h  --  codec driver for max9850
 *
 * Copyright (C) 2011 taskit GmbH
 * Author: Christian Glindkamp <christian.glindkamp@taskit.de>
 */

pub const MAX9850_STATUSA: u32 = 0x00;
pub const MAX9850_STATUSB: u32 = 0x01;
pub const MAX9850_VOLUME: u32 = 0x02;
pub const MAX9850_GENERAL_PURPOSE: u32 = 0x03;
pub const MAX9850_INTERRUPT: u32 = 0x04;
pub const MAX9850_ENABLE: u32 = 0x05;
pub const MAX9850_CLOCK: u32 = 0x06;
pub const MAX9850_CHARGE_PUMP: u32 = 0x07;
pub const MAX9850_LRCLK_MSB: u32 = 0x08;
pub const MAX9850_LRCLK_LSB: u32 = 0x09;
pub const MAX9850_DIGITAL_AUDIO: u32 = 0x0a;

pub const MAX9850_CACHEREGNUM: u32 = 11;

/* MAX9850_DIGITAL_AUDIO */
pub const MAX9850_MASTER: u32 = 1 << 7;
pub const MAX9850_INV: u32 = 1 << 6;
pub const MAX9850_BCINV: u32 = 1 << 5;
pub const MAX9850_DLY: u32 = 1 << 3;
pub const MAX9850_RTJ: u32 = 1 << 2;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
