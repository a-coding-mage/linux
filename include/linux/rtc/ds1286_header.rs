/*
 * Copyright (C) 1998, 1999, 2003 Ralf Baechle
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/**
 * register summary
 */
pub const RTC_HUNDREDTH_SECOND: i32 = 0;
pub const RTC_SECONDS: i32 = 1;
pub const RTC_MINUTES: i32 = 2;
pub const RTC_MINUTES_ALARM: i32 = 3;
pub const RTC_HOURS: i32 = 4;
pub const RTC_HOURS_ALARM: i32 = 5;
pub const RTC_DAY: i32 = 6;
pub const RTC_DAY_ALARM: i32 = 7;
pub const RTC_DATE: i32 = 8;
pub const RTC_MONTH: i32 = 9;
pub const RTC_YEAR: i32 = 10;
pub const RTC_CMD: i32 = 11;
pub const RTC_WHSEC: i32 = 12;
pub const RTC_WSEC: i32 = 13;
pub const RTC_UNUSED: i32 = 14;

/* RTC_*_alarm is always true if 2 MSBs are set */
pub const RTC_ALARM_DONT_CARE: i32 = 0xC0;

/*
 * Bits in the month register
 */
pub const RTC_EOSC: i32 = 0x80;
pub const RTC_ESQW: i32 = 0x40;

/*
 * Bits in the Command register
 */
pub const RTC_TDF: i32 = 0x01;
pub const RTC_WAF: i32 = 0x02;
pub const RTC_TDM: i32 = 0x04;
pub const RTC_WAM: i32 = 0x08;
pub const RTC_PU_LVL: i32 = 0x10;
pub const RTC_IBH_LO: i32 = 0x20;
pub const RTC_IPSW: i32 = 0x40;
pub const RTC_TE: i32 = 0x80;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
