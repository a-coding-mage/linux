// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/sh03/rtc.c -- CTP/PCI-SH03 on-chip RTC support
 *
 *  Copyright (C) 2004  Saito.K & Jeanne(ksaito@interface.co.jp)
 *
 */

// Linux headers supplied by the surrounding kernel translation.

const RTC_BASE: usize = 0xb0000000;
const RTC_SEC1: usize = RTC_BASE + 0;
const RTC_SEC10: usize = RTC_BASE + 1;
const RTC_MIN1: usize = RTC_BASE + 2;
const RTC_MIN10: usize = RTC_BASE + 3;
const RTC_HOU1: usize = RTC_BASE + 4;
const RTC_HOU10: usize = RTC_BASE + 5;
const RTC_WEE1: usize = RTC_BASE + 6;
const RTC_DAY1: usize = RTC_BASE + 7;
const RTC_DAY10: usize = RTC_BASE + 8;
const RTC_MON1: usize = RTC_BASE + 9;
const RTC_MON10: usize = RTC_BASE + 10;
const RTC_YEA1: usize = RTC_BASE + 11;
const RTC_YEA10: usize = RTC_BASE + 12;
const RTC_YEA100: usize = RTC_BASE + 13;
const RTC_YEA1000: usize = RTC_BASE + 14;
const RTC_CTL: usize = RTC_BASE + 15;
const RTC_BUSY: u8 = 1;
const RTC_STOP: u8 = 2;

extern "C" {
    static mut sh03_rtc_lock: SpinLock;
    fn __raw_readb(addr: usize) -> u8;
    fn __raw_writeb(value: u8, addr: usize);
    fn spin_lock(lock: *mut SpinLock);
    fn spin_unlock(lock: *mut SpinLock);
    fn printk(format: *const u8, ...);
    fn printk_once(format: *const u8, ...);
    fn platform_device_register_data(
        parent: *mut Device,
        name: *const u8,
        id: i32,
        data: *const RtcClassOps,
        size: usize,
    ) -> *mut PlatformDevice;
    fn PTR_ERR_OR_ZERO(ptr: *mut PlatformDevice) -> i32;
}

#[repr(C)]
struct SpinLock;
#[repr(C)]
struct Device;
#[repr(C)]
struct PlatformDevice;

#[repr(C)]
struct RtcTime {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
}

#[repr(C)]
struct RtcClassOps {
    read_time: Option<unsafe extern "C" fn(*mut Device, *mut RtcTime) -> i32>,
    set_time: Option<unsafe extern "C" fn(*mut Device, *mut RtcTime) -> i32>,
}

unsafe extern "C" fn sh03_rtc_gettimeofday(_dev: *mut Device, tm: *mut RtcTime) -> i32 {
    let (mut year, mut mon, mut day, mut hour, mut min, mut sec):
        (u32, u32, u32, u32, u32, u32);

    spin_lock(&raw mut sh03_rtc_lock);
    'again: loop {
        loop {
            sec = (__raw_readb(RTC_SEC1) & 0xf) as u32
                + ((__raw_readb(RTC_SEC10) & 0x7) as u32) * 10;
            min = (__raw_readb(RTC_MIN1) & 0xf) as u32
                + ((__raw_readb(RTC_MIN10) & 0xf) as u32) * 10;
            hour = (__raw_readb(RTC_HOU1) & 0xf) as u32
                + ((__raw_readb(RTC_HOU10) & 0xf) as u32) * 10;
            day = (__raw_readb(RTC_DAY1) & 0xf) as u32
                + ((__raw_readb(RTC_DAY10) & 0xf) as u32) * 10;
            mon = (__raw_readb(RTC_MON1) & 0xf) as u32
                + ((__raw_readb(RTC_MON10) & 0xf) as u32) * 10;
            year = (__raw_readb(RTC_YEA1) & 0xf) as u32
                + ((__raw_readb(RTC_YEA10) & 0xf) as u32) * 10
                + ((__raw_readb(RTC_YEA100) & 0xf) as u32) * 100
                + ((__raw_readb(RTC_YEA1000) & 0xf) as u32) * 1000;
            let sec_again = (__raw_readb(RTC_SEC1) & 0xf) as u32
                + ((__raw_readb(RTC_SEC10) & 0x7) as u32) * 10;
            if sec == sec_again { break; }
        }
        if year == 0 || mon < 1 || mon > 12 || day > 31 || day < 1
            || hour > 23 || min > 59 || sec > 59
        {
            printk(b"SH-03 RTC: invalid value, resetting to 1 Jan 2000\0".as_ptr());
            printk(b"year=%d, mon=%d, day=%d, hour=%d, min=%d, sec=%d\n\0".as_ptr(),
                year, mon, day, hour, min, sec);
            __raw_writeb(0, RTC_SEC1); __raw_writeb(0, RTC_SEC10);
            __raw_writeb(0, RTC_MIN1); __raw_writeb(0, RTC_MIN10);
            __raw_writeb(0, RTC_HOU1); __raw_writeb(0, RTC_HOU10);
            __raw_writeb(6, RTC_WEE1);
            __raw_writeb(1, RTC_DAY1); __raw_writeb(0, RTC_DAY10);
            __raw_writeb(1, RTC_MON1); __raw_writeb(0, RTC_MON10);
            __raw_writeb(0, RTC_YEA1); __raw_writeb(0, RTC_YEA10);
            __raw_writeb(0, RTC_YEA100); __raw_writeb(2, RTC_YEA1000);
            __raw_writeb(0, RTC_CTL);
            continue 'again;
        }
        break;
    }
    spin_unlock(&raw mut sh03_rtc_lock);
    (*tm).tm_sec = sec as i32;
    (*tm).tm_min = min as i32;
    (*tm).tm_hour = hour as i32;
    (*tm).tm_mday = day as i32;
    (*tm).tm_mon = mon as i32;
    (*tm).tm_year = year as i32 - 1900;
    0
}

unsafe fn set_rtc_mmss(tm: *mut RtcTime) -> i32 {
    let mut retval = 0;
    let mut real_seconds: i32;
    let mut real_minutes: i32;
    let cmos_minutes: i32;
    spin_lock(&raw mut sh03_rtc_lock);
    for _i in 0..1_000_000 {
        if __raw_readb(RTC_CTL) & RTC_BUSY == 0 { break; }
    }
    cmos_minutes = ((__raw_readb(RTC_MIN1) & 0xf) as i32)
        + ((__raw_readb(RTC_MIN10) & 0xf) as i32) * 10;
    real_seconds = (*tm).tm_sec;
    real_minutes = (*tm).tm_min;
    if ((real_minutes - cmos_minutes).abs() + 15) / 30 & 1 != 0 { real_minutes += 30; }
    real_minutes %= 60;
    if (real_minutes - cmos_minutes).abs() < 30 {
        __raw_writeb((real_seconds % 10) as u8, RTC_SEC1);
        __raw_writeb((real_seconds / 10) as u8, RTC_SEC10);
        __raw_writeb((real_minutes % 10) as u8, RTC_MIN1);
        __raw_writeb((real_minutes / 10) as u8, RTC_MIN10);
    } else {
        printk_once(b"set_rtc_mmss: can't update from %d to %d\n\0".as_ptr(), cmos_minutes, real_minutes);
        retval = -22;
    }
    spin_unlock(&raw mut sh03_rtc_lock);
    retval
}

unsafe extern "C" fn sh03_rtc_settimeofday(_dev: *mut Device, tm: *mut RtcTime) -> i32 {
    set_rtc_mmss(tm)
}

static rtc_generic_ops: RtcClassOps = RtcClassOps {
    read_time: Some(sh03_rtc_gettimeofday),
    set_time: Some(sh03_rtc_settimeofday),
};

unsafe extern "C" fn sh03_time_init() -> i32 {
    let pdev = platform_device_register_data(
        core::ptr::null_mut(), b"rtc-generic\0".as_ptr(), -1,
        &raw const rtc_generic_ops, core::mem::size_of::<RtcClassOps>());
    PTR_ERR_OR_ZERO(pdev)
}

// arch_initcall(sh03_time_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
