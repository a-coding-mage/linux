/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/rtc/m48t59.h
 *
 * Definitions for the platform data of m48t59 RTC chip driver.
 *
 * Copyright (c) 2007 Wind River Systems, Inc.
 *
 * Mark Zhan <rongkai.zhan@windriver.com>
 */

/*
 * M48T59 Register Offset
 */
pub const M48T59_YEAR: u32 = 0xf;
pub const M48T59_MONTH: u32 = 0xe;
pub const M48T59_MDAY: u32 = 0xd; /* Day of Month */
pub const M48T59_WDAY: u32 = 0xc; /* Day of Week */
pub const M48T59_WDAY_CB: u32 = 0x20; /* Century Bit */
pub const M48T59_WDAY_CEB: u32 = 0x10; /* Century Enable Bit */
pub const M48T59_HOUR: u32 = 0xb;
pub const M48T59_MIN: u32 = 0xa;
pub const M48T59_SEC: u32 = 0x9;
pub const M48T59_CNTL: u32 = 0x8;
pub const M48T59_CNTL_READ: u32 = 0x40;
pub const M48T59_CNTL_WRITE: u32 = 0x80;
pub const M48T59_WATCHDOG: u32 = 0x7;
pub const M48T59_INTR: u32 = 0x6;
pub const M48T59_INTR_AFE: u32 = 0x80; /* Alarm Interrupt Enable */
pub const M48T59_INTR_ABE: u32 = 0x20;
pub const M48T59_ALARM_DATE: u32 = 0x5;
pub const M48T59_ALARM_HOUR: u32 = 0x4;
pub const M48T59_ALARM_MIN: u32 = 0x3;
pub const M48T59_ALARM_SEC: u32 = 0x2;
pub const M48T59_UNUSED: u32 = 0x1;
pub const M48T59_FLAGS: u32 = 0x0;
pub const M48T59_FLAGS_WDT: u32 = 0x80; /* watchdog timer expired */
pub const M48T59_FLAGS_AF: u32 = 0x40; /* alarm */
pub const M48T59_FLAGS_BF: u32 = 0x10; /* low battery */

pub const M48T59RTC_TYPE_M48T59: i32 = 0; /* to keep compatibility */
pub const M48T59RTC_TYPE_M48T02: i32 = 1;
pub const M48T59RTC_TYPE_M48T08: i32 = 2;

/* External dependency supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct m48t59_plat_data {
    /* The method to access M48T59 registers */
    pub write_byte: Option<unsafe extern "C" fn(dev: *mut device, ofs: u32, val: u8)>,
    pub read_byte: Option<unsafe extern "C" fn(dev: *mut device, ofs: u32) -> u8>,

    pub type_: i32, /* RTC model */

    /* ioaddr mapped externally */
    pub ioaddr: *mut core::ffi::c_void,
    /* offset to RTC registers, automatically set according to the type */
    pub offset: u32,

    /* YY digits (in RTC) are offset, i.e. year is 1900 + yy_offset + YY */
    pub yy_offset: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
