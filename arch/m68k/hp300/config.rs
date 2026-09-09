// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/hp300/config.c
 *
 *  Copyright (C) 1998 Philip Blundell <philb@gnu.org>
 *
 *  This file contains the HP300-specific initialisation code.  It gets
 *  called by setup.c.
 */

use core::ptr;

pub type CChar = u8;

#[repr(C)]
pub struct BiRecord {
    pub tag: u16,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct RtcTime {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_wday: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
}

extern "C" {
    pub fn hp300_reset();
    pub fn hp300_sched_init();
    pub fn hp300_setup_serial_console();
    pub fn blinken_leds(on: u32, off: u32);
    pub fn in_8(addr: usize) -> u8;
    pub fn out_8(addr: usize, value: u8);
    pub fn local_irq_save(flags: *mut usize);
    pub fn local_irq_restore(flags: usize);
    pub fn be16_to_cpu(value: u16) -> u16;
    pub fn be32_to_cpup(value: *const core::ffi::c_void) -> u32;
    pub fn pr_info(fmt: *const CChar, ...);
    pub fn panic(fmt: *const CChar, ...) -> !;

    pub static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    pub static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    pub static mut mach_get_model: Option<unsafe extern "C" fn(*mut CChar)>;
    pub static mut mach_hwclk: Option<unsafe extern "C" fn(i32, *mut RtcTime) -> i32>;
    pub static mut mach_reset: Option<unsafe extern "C" fn()>;
    #[cfg(CONFIG_HEARTBEAT)]
    pub static mut mach_heartbeat: Option<unsafe extern "C" fn(i32)>;
}

pub static mut hp300_model: u64 = 0;
pub static mut hp300_uart_scode: u64 = u64::MAX;
pub static mut hp300_ledstate: u8 = 0;

static mut s_hp330: [CChar; 4] = *b"330\0";
static mut s_hp340: [CChar; 4] = *b"340\0";
static mut s_hp345: [CChar; 4] = *b"345\0";
static mut s_hp360: [CChar; 4] = *b"360\0";
static mut s_hp370: [CChar; 4] = *b"370\0";
static mut s_hp375: [CChar; 4] = *b"375\0";
static mut s_hp380: [CChar; 4] = *b"380\0";
static mut s_hp385: [CChar; 4] = *b"385\0";
static mut s_hp400: [CChar; 4] = *b"400\0";
static mut s_hp425t: [CChar; 5] = *b"425t\0";
static mut s_hp425s: [CChar; 5] = *b"425s\0";
static mut s_hp425e: [CChar; 5] = *b"425e\0";
static mut s_hp433t: [CChar; 5] = *b"433t\0";
static mut s_hp433s: [CChar; 5] = *b"433s\0";

static mut hp300_model_name: [CChar; 13] = *b"HP9000/\0\0\0\0\0\0";

const RTCBASE: usize = 0xf0420000;
const RTC_DATA: usize = 0x1;
const RTC_CMD: usize = 0x3;
const RTC_BUSY: u8 = 0x02;
const RTC_DATA_RDY: u8 = 0x01;
const RTC_SETREG: u8 = 0xe0;
const RTC_WRITEREG: u8 = 0xc2;
const RTC_READREG: u8 = 0xc3;
const RTC_REG_SEC2: u8 = 0;
const RTC_REG_SEC1: u8 = 1;
const RTC_REG_MIN2: u8 = 2;
const RTC_REG_MIN1: u8 = 3;
const RTC_REG_HOUR2: u8 = 4;
const RTC_REG_HOUR1: u8 = 5;
const RTC_REG_DAY2: u8 = 7;
const RTC_REG_DAY1: u8 = 8;
const RTC_REG_MON2: u8 = 9;
const RTC_REG_MON1: u8 = 10;
const RTC_REG_YEAR2: u8 = 11;
const RTC_REG_YEAR1: u8 = 12;
const RTC_HOUR1_24HMODE: u8 = 0x8;
const RTC_STAT_MASK: u8 = 0xf0;
const RTC_STAT_RDY: u8 = 0x40;

#[inline]
unsafe fn rtc_read(reg: u8) -> u8 {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    while in_8(RTCBASE + RTC_CMD) & RTC_BUSY != 0 {}
    out_8(RTCBASE + RTC_CMD, RTC_SETREG);
    while in_8(RTCBASE + RTC_CMD) & RTC_BUSY != 0 {}
    out_8(RTCBASE + RTC_DATA, reg);
    while in_8(RTCBASE + RTC_CMD) & RTC_BUSY != 0 {}
    out_8(RTCBASE + RTC_CMD, RTC_READREG);
    let (mut s, mut ret);
    loop {
        while in_8(RTCBASE + RTC_CMD) & RTC_DATA_RDY == 0 {}
        s = in_8(RTCBASE + RTC_CMD);
        ret = in_8(RTCBASE + RTC_DATA);
        if s & RTC_STAT_MASK == RTC_STAT_RDY { break; }
    }
    local_irq_restore(flags);
    ret
}

#[inline]
unsafe fn rtc_write(reg: u8, val: u8) -> u8 {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    while in_8(RTCBASE + RTC_CMD) & RTC_BUSY != 0 {}
    out_8(RTCBASE + RTC_CMD, RTC_SETREG);
    while in_8(RTCBASE + RTC_CMD) & RTC_BUSY != 0 {}
    out_8(RTCBASE + RTC_DATA, (val << 4) | reg);
    while in_8(RTCBASE + RTC_CMD) & RTC_BUSY != 0 {}
    out_8(RTCBASE + RTC_CMD, RTC_WRITEREG);
    while in_8(RTCBASE + RTC_CMD) & RTC_BUSY != 0 {}
    out_8(RTCBASE + RTC_CMD, RTC_READREG);
    let (mut s, mut ret);
    loop {
        while in_8(RTCBASE + RTC_CMD) & RTC_DATA_RDY == 0 {}
        s = in_8(RTCBASE + RTC_CMD);
        ret = in_8(RTCBASE + RTC_DATA);
        if s & RTC_STAT_MASK == RTC_STAT_RDY { break; }
    }
    local_irq_restore(flags);
    ret
}

pub unsafe extern "C" fn hp300_parse_bootinfo(record: *const BiRecord) -> i32 {
    let mut unknown = 0;
    match be16_to_cpu((*record).tag) {
        0x0001 => { hp300_model = be32_to_cpup((*record).data) as u64; }
        0x0002 => { hp300_uart_scode = be32_to_cpup((*record).data) as u64; }
        0x0003 => {}
        _ => { unknown = 1; }
    }
    unknown
}

#[cfg(CONFIG_HEARTBEAT)]
unsafe extern "C" fn hp300_pulse(x: i32) {
    if x != 0 { blinken_leds(0x10, 0); } else { blinken_leds(0, 0x10); }
}

unsafe extern "C" fn hp300_get_model(model: *mut CChar) {
    ptr::copy_nonoverlapping(hp300_model_name.as_ptr(), model, hp300_model_name.len());
}

unsafe extern "C" fn hp300_hwclk(op: i32, t: *mut RtcTime) -> i32 {
    if op == 0 {
        (*t).tm_sec = (rtc_read(RTC_REG_SEC1) as i32) * 10 + rtc_read(RTC_REG_SEC2) as i32;
        (*t).tm_min = (rtc_read(RTC_REG_MIN1) as i32) * 10 + rtc_read(RTC_REG_MIN2) as i32;
        (*t).tm_hour = ((rtc_read(RTC_REG_HOUR1) & 3) as i32) * 10 + rtc_read(RTC_REG_HOUR2) as i32;
        (*t).tm_wday = -1;
        (*t).tm_mday = (rtc_read(RTC_REG_DAY1) as i32) * 10 + rtc_read(RTC_REG_DAY2) as i32;
        (*t).tm_mon = (rtc_read(RTC_REG_MON1) as i32) * 10 + rtc_read(RTC_REG_MON2) as i32 - 1;
        (*t).tm_year = (rtc_read(RTC_REG_YEAR1) as i32) * 10 + rtc_read(RTC_REG_YEAR2) as i32;
        if (*t).tm_year <= 69 { (*t).tm_year += 100; }
    } else {
        rtc_write(RTC_REG_SEC1, ((*t).tm_sec / 10) as u8); rtc_write(RTC_REG_SEC2, ((*t).tm_sec % 10) as u8);
        rtc_write(RTC_REG_MIN1, ((*t).tm_min / 10) as u8); rtc_write(RTC_REG_MIN2, ((*t).tm_min % 10) as u8);
        rtc_write(RTC_REG_HOUR1, (((*t).tm_hour / 10) as u8 & 3) | RTC_HOUR1_24HMODE); rtc_write(RTC_REG_HOUR2, ((*t).tm_hour % 10) as u8);
        rtc_write(RTC_REG_DAY1, ((*t).tm_mday / 10) as u8); rtc_write(RTC_REG_DAY2, ((*t).tm_mday % 10) as u8);
        rtc_write(RTC_REG_MON1, (((*t).tm_mon + 1) / 10) as u8); rtc_write(RTC_REG_MON2, (((*t).tm_mon + 1) % 10) as u8);
        if (*t).tm_year >= 100 { (*t).tm_year -= 100; }
        rtc_write(RTC_REG_YEAR1, ((*t).tm_year / 10) as u8); rtc_write(RTC_REG_YEAR2, ((*t).tm_year % 10) as u8);
    }
    0
}

unsafe extern "C" fn hp300_init_IRQ() {}

pub unsafe extern "C" fn config_hp300() {
    mach_sched_init = Some(hp300_sched_init);
    mach_init_IRQ = Some(hp300_init_IRQ);
    mach_get_model = Some(hp300_get_model);
    mach_hwclk = Some(hp300_hwclk);
    mach_reset = Some(hp300_reset);
    #[cfg(CONFIG_HEARTBEAT)]
    { mach_heartbeat = Some(hp300_pulse); }
    if hp300_model >= 1 && hp300_model <= 15 && hp300_model != 4 {
        let names: [*const CChar; 16] = [
            ptr::null(), s_hp330.as_ptr(), s_hp340.as_ptr(), s_hp345.as_ptr(), ptr::null(),
            s_hp360.as_ptr(), s_hp370.as_ptr(), s_hp375.as_ptr(), s_hp380.as_ptr(), s_hp385.as_ptr(),
            s_hp400.as_ptr(), s_hp425t.as_ptr(), s_hp425s.as_ptr(), s_hp425e.as_ptr(), s_hp433t.as_ptr(), s_hp433s.as_ptr(),
        ];
        pr_info(b"Detected HP9000 model %s\n\0".as_ptr(), names[hp300_model as usize - 0]);
        let suffix = names[hp300_model as usize];
        let mut n = 7usize;
        while *hp300_model_name.as_ptr().add(n) != 0 { n += 1; }
        while *suffix.add(0) != 0 {
            *hp300_model_name.as_mut_ptr().add(n) = *suffix;
            n += 1;
            // The source uses strcat; the external string is NUL terminated.
            let next = suffix.add(n - 7);
            if *next == 0 { break; }
        }
        *hp300_model_name.as_mut_ptr().add(n) = 0;
    } else {
        panic(b"Unknown HP9000 Model\0".as_ptr());
    }
    hp300_setup_serial_console();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
