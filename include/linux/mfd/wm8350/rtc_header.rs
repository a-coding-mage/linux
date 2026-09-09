/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * rtc.h  --  RTC driver for Wolfson WM8350 PMIC
 *
 * Copyright 2007 Wolfson Microelectronics PLC
 */

// Dependency: declarations from <linux/platform_device.h> are supplied externally.

/* Register values. */
pub const WM8350_RTC_SECONDS_MINUTES: u16 = 0x10;
pub const WM8350_RTC_HOURS_DAY: u16 = 0x11;
pub const WM8350_RTC_DATE_MONTH: u16 = 0x12;
pub const WM8350_RTC_YEAR: u16 = 0x13;
pub const WM8350_ALARM_SECONDS_MINUTES: u16 = 0x14;
pub const WM8350_ALARM_HOURS_DAY: u16 = 0x15;
pub const WM8350_ALARM_DATE_MONTH: u16 = 0x16;
pub const WM8350_RTC_TIME_CONTROL: u16 = 0x17;

/* R16 (0x10) - RTC Seconds/Minutes */
pub const WM8350_RTC_MINS_MASK: u16 = 0x7F00;
pub const WM8350_RTC_MINS_SHIFT: u16 = 8;
pub const WM8350_RTC_SECS_MASK: u16 = 0x007F;
pub const WM8350_RTC_SECS_SHIFT: u16 = 0;

/* R17 (0x11) - RTC Hours/Day */
pub const WM8350_RTC_DAY_MASK: u16 = 0x0700;
pub const WM8350_RTC_DAY_SHIFT: u16 = 8;
pub const WM8350_RTC_HPM_MASK: u16 = 0x0020;
pub const WM8350_RTC_HPM_SHIFT: u16 = 5;
pub const WM8350_RTC_HRS_MASK: u16 = 0x001F;
pub const WM8350_RTC_HRS_SHIFT: u16 = 0;

/* Bit values for R21 (0x15) */
pub const WM8350_RTC_DAY_SUN: i32 = 1;
pub const WM8350_RTC_DAY_MON: i32 = 2;
pub const WM8350_RTC_DAY_TUE: i32 = 3;
pub const WM8350_RTC_DAY_WED: i32 = 4;
pub const WM8350_RTC_DAY_THU: i32 = 5;
pub const WM8350_RTC_DAY_FRI: i32 = 6;
pub const WM8350_RTC_DAY_SAT: i32 = 7;
pub const WM8350_RTC_HPM_AM: i32 = 0;
pub const WM8350_RTC_HPM_PM: i32 = 1;

/* R18 (0x12) - RTC Date/Month */
pub const WM8350_RTC_MTH_MASK: u16 = 0x1F00;
pub const WM8350_RTC_MTH_SHIFT: u16 = 8;
pub const WM8350_RTC_DATE_MASK: u16 = 0x003F;
pub const WM8350_RTC_DATE_SHIFT: u16 = 0;

/* Bit values for R22 (0x16) */
pub const WM8350_RTC_MTH_JAN: i32 = 1;
pub const WM8350_RTC_MTH_FEB: i32 = 2;
pub const WM8350_RTC_MTH_MAR: i32 = 3;
pub const WM8350_RTC_MTH_APR: i32 = 4;
pub const WM8350_RTC_MTH_MAY: i32 = 5;
pub const WM8350_RTC_MTH_JUN: i32 = 6;
pub const WM8350_RTC_MTH_JUL: i32 = 7;
pub const WM8350_RTC_MTH_AUG: i32 = 8;
pub const WM8350_RTC_MTH_SEP: i32 = 9;
pub const WM8350_RTC_MTH_OCT: i32 = 10;
pub const WM8350_RTC_MTH_NOV: i32 = 11;
pub const WM8350_RTC_MTH_DEC: i32 = 12;
pub const WM8350_RTC_MTH_JAN_BCD: u16 = 0x01;
pub const WM8350_RTC_MTH_FEB_BCD: u16 = 0x02;
pub const WM8350_RTC_MTH_MAR_BCD: u16 = 0x03;
pub const WM8350_RTC_MTH_APR_BCD: u16 = 0x04;
pub const WM8350_RTC_MTH_MAY_BCD: u16 = 0x05;
pub const WM8350_RTC_MTH_JUN_BCD: u16 = 0x06;
pub const WM8350_RTC_MTH_JUL_BCD: u16 = 0x07;
pub const WM8350_RTC_MTH_AUG_BCD: u16 = 0x08;
pub const WM8350_RTC_MTH_SEP_BCD: u16 = 0x09;
pub const WM8350_RTC_MTH_OCT_BCD: u16 = 0x10;
pub const WM8350_RTC_MTH_NOV_BCD: u16 = 0x11;
pub const WM8350_RTC_MTH_DEC_BCD: u16 = 0x12;

/* R19 (0x13) - RTC Year */
pub const WM8350_RTC_YHUNDREDS_MASK: u16 = 0x3F00;
pub const WM8350_RTC_YHUNDREDS_SHIFT: u16 = 8;
pub const WM8350_RTC_YUNITS_MASK: u16 = 0x00FF;
pub const WM8350_RTC_YUNITS_SHIFT: u16 = 0;

/* R20 (0x14) - Alarm Seconds/Minutes */
pub const WM8350_RTC_ALMMINS_MASK: u16 = 0x7F00;
pub const WM8350_RTC_ALMMINS_SHIFT: u16 = 8;
pub const WM8350_RTC_ALMSECS_MASK: u16 = 0x007F;
pub const WM8350_RTC_ALMSECS_SHIFT: u16 = 0;
pub const WM8350_RTC_ALMMINS_DONT_CARE: i32 = -1;
pub const WM8350_RTC_ALMSECS_DONT_CARE: i32 = -1;

/* R21 (0x15) - Alarm Hours/Day */
pub const WM8350_RTC_ALMDAY_MASK: u16 = 0x0F00;
pub const WM8350_RTC_ALMDAY_SHIFT: u16 = 8;
pub const WM8350_RTC_ALMHPM_MASK: u16 = 0x0020;
pub const WM8350_RTC_ALMHPM_SHIFT: u16 = 5;
pub const WM8350_RTC_ALMHRS_MASK: u16 = 0x001F;
pub const WM8350_RTC_ALMHRS_SHIFT: u16 = 0;
pub const WM8350_RTC_ALMDAY_DONT_CARE: i32 = -1;
pub const WM8350_RTC_ALMDAY_SUN: i32 = 1;
pub const WM8350_RTC_ALMDAY_MON: i32 = 2;
pub const WM8350_RTC_ALMDAY_TUE: i32 = 3;
pub const WM8350_RTC_ALMDAY_WED: i32 = 4;
pub const WM8350_RTC_ALMDAY_THU: i32 = 5;
pub const WM8350_RTC_ALMDAY_FRI: i32 = 6;
pub const WM8350_RTC_ALMDAY_SAT: i32 = 7;
pub const WM8350_RTC_ALMHPM_AM: i32 = 0;
pub const WM8350_RTC_ALMHPM_PM: i32 = 1;
pub const WM8350_RTC_ALMHRS_DONT_CARE: i32 = -1;

/* R22 (0x16) - Alarm Date/Month */
pub const WM8350_RTC_ALMMTH_MASK: u16 = 0x1F00;
pub const WM8350_RTC_ALMMTH_SHIFT: u16 = 8;
pub const WM8350_RTC_ALMDATE_MASK: u16 = 0x003F;
pub const WM8350_RTC_ALMDATE_SHIFT: u16 = 0;
pub const WM8350_RTC_ALMDATE_DONT_CARE: i32 = -1;
pub const WM8350_RTC_ALMMTH_DONT_CARE: i32 = -1;
pub const WM8350_RTC_ALMMTH_JAN: i32 = 1;
pub const WM8350_RTC_ALMMTH_FEB: i32 = 2;
pub const WM8350_RTC_ALMMTH_MAR: i32 = 3;
pub const WM8350_RTC_ALMMTH_APR: i32 = 4;
pub const WM8350_RTC_ALMMTH_MAY: i32 = 5;
pub const WM8350_RTC_ALMMTH_JUN: i32 = 6;
pub const WM8350_RTC_ALMMTH_JUL: i32 = 7;
pub const WM8350_RTC_ALMMTH_AUG: i32 = 8;
pub const WM8350_RTC_ALMMTH_SEP: i32 = 9;
pub const WM8350_RTC_ALMMTH_OCT: i32 = 10;
pub const WM8350_RTC_ALMMTH_NOV: i32 = 11;
pub const WM8350_RTC_ALMMTH_DEC: i32 = 12;
pub const WM8350_RTC_ALMMTH_JAN_BCD: u16 = 0x01;
pub const WM8350_RTC_ALMMTH_FEB_BCD: u16 = 0x02;
pub const WM8350_RTC_ALMMTH_MAR_BCD: u16 = 0x03;
pub const WM8350_RTC_ALMMTH_APR_BCD: u16 = 0x04;
pub const WM8350_RTC_ALMMTH_MAY_BCD: u16 = 0x05;
pub const WM8350_RTC_ALMMTH_JUN_BCD: u16 = 0x06;
pub const WM8350_RTC_ALMMTH_JUL_BCD: u16 = 0x07;
pub const WM8350_RTC_ALMMTH_AUG_BCD: u16 = 0x08;
pub const WM8350_RTC_ALMMTH_SEP_BCD: u16 = 0x09;
pub const WM8350_RTC_ALMMTH_OCT_BCD: u16 = 0x10;
pub const WM8350_RTC_ALMMTH_NOV_BCD: u16 = 0x11;
pub const WM8350_RTC_ALMMTH_DEC_BCD: u16 = 0x12;

/* R23 (0x17) - RTC Time Control */
pub const WM8350_RTC_BCD: u16 = 0x8000;
pub const WM8350_RTC_BCD_MASK: u16 = 0x8000;
pub const WM8350_RTC_BCD_SHIFT: u16 = 15;
pub const WM8350_RTC_12HR: u16 = 0x4000;
pub const WM8350_RTC_12HR_MASK: u16 = 0x4000;
pub const WM8350_RTC_12HR_SHIFT: u16 = 14;
pub const WM8350_RTC_DST: u16 = 0x2000;
pub const WM8350_RTC_DST_MASK: u16 = 0x2000;
pub const WM8350_RTC_DST_SHIFT: u16 = 13;
pub const WM8350_RTC_SET: u16 = 0x0800;
pub const WM8350_RTC_SET_MASK: u16 = 0x0800;
pub const WM8350_RTC_SET_SHIFT: u16 = 11;
pub const WM8350_RTC_STS: u16 = 0x0400;
pub const WM8350_RTC_STS_MASK: u16 = 0x0400;
pub const WM8350_RTC_STS_SHIFT: u16 = 10;
pub const WM8350_RTC_ALMSET: u16 = 0x0200;
pub const WM8350_RTC_ALMSET_MASK: u16 = 0x0200;
pub const WM8350_RTC_ALMSET_SHIFT: u16 = 9;
pub const WM8350_RTC_ALMSTS: u16 = 0x0100;
pub const WM8350_RTC_ALMSTS_MASK: u16 = 0x0100;
pub const WM8350_RTC_ALMSTS_SHIFT: u16 = 8;
pub const WM8350_RTC_PINT: u16 = 0x0070;
pub const WM8350_RTC_PINT_MASK: u16 = 0x0070;
pub const WM8350_RTC_PINT_SHIFT: u16 = 4;
pub const WM8350_RTC_DSW: u16 = 0x000F;
pub const WM8350_RTC_DSW_MASK: u16 = 0x000F;
pub const WM8350_RTC_DSW_SHIFT: u16 = 0;

/* Bit values for R23 (0x17) */
pub const WM8350_RTC_BCD_BINARY: i32 = 0;
pub const WM8350_RTC_BCD_BCD: i32 = 1;
pub const WM8350_RTC_12HR_24HR: i32 = 0;
pub const WM8350_RTC_12HR_12HR: i32 = 1;
pub const WM8350_RTC_DST_DISABLED: i32 = 0;
pub const WM8350_RTC_DST_ENABLED: i32 = 1;
pub const WM8350_RTC_SET_RUN: i32 = 0;
pub const WM8350_RTC_SET_SET: i32 = 1;
pub const WM8350_RTC_STS_RUNNING: i32 = 0;
pub const WM8350_RTC_STS_STOPPED: i32 = 1;
pub const WM8350_RTC_ALMSET_RUN: i32 = 0;
pub const WM8350_RTC_ALMSET_SET: i32 = 1;
pub const WM8350_RTC_ALMSTS_RUNNING: i32 = 0;
pub const WM8350_RTC_ALMSTS_STOPPED: i32 = 1;
pub const WM8350_RTC_PINT_DISABLED: i32 = 0;
pub const WM8350_RTC_PINT_SECS: i32 = 1;
pub const WM8350_RTC_PINT_MINS: i32 = 2;
pub const WM8350_RTC_PINT_HRS: i32 = 3;
pub const WM8350_RTC_PINT_DAYS: i32 = 4;
pub const WM8350_RTC_PINT_MTHS: i32 = 5;
pub const WM8350_RTC_DSW_DISABLED: i32 = 0;
pub const WM8350_RTC_DSW_1HZ: i32 = 1;
pub const WM8350_RTC_DSW_2HZ: i32 = 2;
pub const WM8350_RTC_DSW_4HZ: i32 = 3;
pub const WM8350_RTC_DSW_8HZ: i32 = 4;
pub const WM8350_RTC_DSW_16HZ: i32 = 5;
pub const WM8350_RTC_DSW_32HZ: i32 = 6;
pub const WM8350_RTC_DSW_64HZ: i32 = 7;
pub const WM8350_RTC_DSW_128HZ: i32 = 8;
pub const WM8350_RTC_DSW_256HZ: i32 = 9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
