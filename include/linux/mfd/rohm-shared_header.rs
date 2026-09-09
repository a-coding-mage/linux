/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2020 ROHM Semiconductors */

/* RTC definitions shared between BD70528 and BD71828 */

pub const BD70528_MASK_RTC_SEC: u32 = 0x7f;
pub const BD70528_MASK_RTC_MINUTE: u32 = 0x7f;
pub const BD70528_MASK_RTC_HOUR_24H: u32 = 0x80;
pub const BD70528_MASK_RTC_HOUR_PM: u32 = 0x20;
pub const BD70528_MASK_RTC_HOUR: u32 = 0x3f;
pub const BD70528_MASK_RTC_DAY: u32 = 0x3f;
pub const BD70528_MASK_RTC_WEEK: u32 = 0x07;
pub const BD70528_MASK_RTC_MONTH: u32 = 0x1f;
pub const BD70528_MASK_RTC_YEAR: u32 = 0xff;
pub const BD70528_MASK_ALM_EN: u32 = 0x7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
