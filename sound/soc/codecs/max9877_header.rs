/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * max9877.h  --  amp driver for max9877
 *
 * Copyright (C) 2009 Samsung Electronics Co.Ltd
 * Author: Joonyoung Shim <jy0922.shim@samsung.com>
 */

pub const MAX9877_INPUT_MODE: u32 = 0x00;
pub const MAX9877_SPK_VOLUME: u32 = 0x01;
pub const MAX9877_HPL_VOLUME: u32 = 0x02;
pub const MAX9877_HPR_VOLUME: u32 = 0x03;
pub const MAX9877_OUTPUT_MODE: u32 = 0x04;

/* MAX9877_INPUT_MODE */
pub const MAX9877_INB: u32 = 1 << 4;
pub const MAX9877_INA: u32 = 1 << 5;
pub const MAX9877_ZCD: u32 = 1 << 6;

/* MAX9877_OUTPUT_MODE */
pub const MAX9877_OUTMODE_MASK: u32 = 15 << 0;
pub const MAX9877_OSC_MASK: u32 = 3 << 4;
pub const MAX9877_OSC_OFFSET: u32 = 4;
pub const MAX9877_BYPASS: u32 = 1 << 6;
pub const MAX9877_SHDN: u32 = 1 << 7;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
