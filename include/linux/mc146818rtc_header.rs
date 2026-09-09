/* mc146818rtc.h - register definitions for the Real-Time-Clock / CMOS RAM
 * Copyright Torsten Duwe <duwe@informatik.uni-erlangen.de> 1993
 * derived from Data Sheet, Copyright Motorola 1984 (!).
 * It was written to be part of the Linux operating system.
 */
/* permission is hereby granted to copy, modify and redistribute this code
 * in terms of the GNU Library General Public License, Version 2 or later,
 * at your option.
 */

// C dependencies: asm/io.h, linux/rtc.h, asm/mc146818rtc.h, linux/bcd.h,
// linux/delay.h, and linux/pm-trace.h.

#[cfg(feature = "kernel")]
extern "C" {
    pub static mut rtc_lock: spinlock_t;
}

#[cfg(feature = "kernel")]
#[repr(C)]
pub struct cmos_rtc_board_info {
    pub wake_on: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub wake_off: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub flags: u32,
    pub address_space: ::core::ffi::c_int,
    pub rtc_day_alarm: u8,
    pub rtc_mon_alarm: u8,
    pub rtc_century: u8,
}

#[cfg(feature = "kernel")]
pub const CMOS_RTC_FLAGS_NOFREQ: u32 = 1 << 0;

/**********************************************************************
 * register summary
 **********************************************************************/
pub const RTC_SECONDS: u32 = 0;
pub const RTC_SECONDS_ALARM: u32 = 1;
pub const RTC_MINUTES: u32 = 2;
pub const RTC_MINUTES_ALARM: u32 = 3;
pub const RTC_HOURS: u32 = 4;
pub const RTC_HOURS_ALARM: u32 = 5;
/* RTC_*_alarm is always true if 2 MSBs are set */
pub const RTC_ALARM_DONT_CARE: u32 = 0xC0;

pub const RTC_DAY_OF_WEEK: u32 = 6;
pub const RTC_DAY_OF_MONTH: u32 = 7;
pub const RTC_MONTH: u32 = 8;
pub const RTC_YEAR: u32 = 9;

/* control registers - Moto names
 */
pub const RTC_REG_A: u32 = 10;
pub const RTC_REG_B: u32 = 11;
pub const RTC_REG_C: u32 = 12;
pub const RTC_REG_D: u32 = 13;

/**********************************************************************
 * register details
 **********************************************************************/
pub const RTC_FREQ_SELECT: u32 = RTC_REG_A;

/* update-in-progress  - set to "1" 244 microsecs before RTC goes off the bus,
 * reset after update (may take 1.984ms @ 32768Hz RefClock) is complete,
 * totalling to a max high interval of 2.228 ms.
 */
pub const RTC_UIP: u32 = 0x80;
pub const RTC_DIV_CTL: u32 = 0x70;
/* divider control: refclock values 4.194 / 1.049 MHz / 32.768 kHz */
pub const RTC_REF_CLCK_4MHZ: u32 = 0x00;
pub const RTC_REF_CLCK_1MHZ: u32 = 0x10;
pub const RTC_REF_CLCK_32KHZ: u32 = 0x20;
/* 2 values for divider stage reset, others for "testing purposes only" */
pub const RTC_DIV_RESET1: u32 = 0x60;
pub const RTC_DIV_RESET2: u32 = 0x70;
/* In AMD BKDG bit 5 and 6 are reserved, bit 4 is for select dv0 bank */
pub const RTC_AMD_BANK_SELECT: u32 = 0x10;
/* Periodic intr. / Square wave rate select. 0=none, 1=32.8kHz,... 15=2Hz */
pub const RTC_RATE_SELECT: u32 = 0x0F;

/**********************************************************************/
pub const RTC_CONTROL: u32 = RTC_REG_B;
pub const RTC_SET: u32 = 0x80; // disable updates for clock setting
pub const RTC_PIE: u32 = 0x40; // periodic interrupt enable
pub const RTC_AIE: u32 = 0x20; // alarm interrupt enable
pub const RTC_UIE: u32 = 0x10; // update-finished interrupt enable
pub const RTC_SQWE: u32 = 0x08; // enable square-wave output
pub const RTC_DM_BINARY: u32 = 0x04; // all time/date values are BCD if clear
pub const RTC_24H: u32 = 0x02; // 24 hour mode - else hours bit 7 means pm
pub const RTC_DST_EN: u32 = 0x01; // auto switch DST - works f. USA only

/**********************************************************************/
pub const RTC_INTR_FLAGS: u32 = RTC_REG_C;
/* caution - cleared by read */
pub const RTC_IRQF: u32 = 0x80; // any of the following 3 is active
pub const RTC_PF: u32 = 0x40;
pub const RTC_AF: u32 = 0x20;
pub const RTC_UF: u32 = 0x10;

/**********************************************************************/
pub const RTC_VALID: u32 = RTC_REG_D;
pub const RTC_VRT: u32 = 0x80; // valid RAM and time
/**********************************************************************/

// ARCH_RTC_LOCATION may override these definitions in asm/mc146818rtc.h.
#[cfg(not(feature = "arch_rtc_location"))]
pub const RTC_IO_EXTENT: u32 = 0x8;
#[cfg(not(feature = "arch_rtc_location"))]
pub const RTC_IO_EXTENT_USED: u32 = 0x2;
#[cfg(not(feature = "arch_rtc_location"))]
pub const RTC_IOMAPPED: u32 = 1; // Default to I/O mapping.

#[cfg(feature = "arch_rtc_location")]
pub const RTC_IO_EXTENT_USED: u32 = RTC_IO_EXTENT;

extern "C" {
    pub fn mc146818_does_rtc_work() -> bool;
    pub fn mc146818_get_time(time: *mut rtc_time, timeout: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn mc146818_set_time(time: *mut rtc_time) -> ::core::ffi::c_int;
    pub fn mc146818_avoid_UIP(
        callback: Option<unsafe extern "C" fn(seconds: u8, param: *mut ::core::ffi::c_void)>,
        timeout: ::core::ffi::c_int,
        param: *mut ::core::ffi::c_void,
    ) -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
