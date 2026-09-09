// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1991, 1992, 1995  Linus Torvalds
 *
 * Adapted for PowerPC (PReP) by Gary Thomas
 * Modified by Cort Dougan (cort@cs.nmt.edu).
 * Copied and modified from arch/i386/kernel/time.c
 */

// Linux and architecture headers provide the declarations and constants used
// below; they are intentionally left as external dependencies of this unit.

const NVRAM_AS0: ::core::ffi::c_int = 0x74;
const NVRAM_AS1: ::core::ffi::c_int = 0x75;
const NVRAM_DATA: ::core::ffi::c_int = 0x77;

static mut nvram_as1: ::core::ffi::c_int = NVRAM_AS1;
static mut nvram_as0: ::core::ffi::c_int = NVRAM_AS0;
static mut nvram_data: ::core::ffi::c_int = NVRAM_DATA;

extern "C" {
    type device_node;
    type resource;
    type rtc_time;

    static rtc_lock: spinlock_t;

    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const ::core::ffi::c_char,
        compatible: *const ::core::ffi::c_char,
    ) -> *mut device_node;
    fn of_address_to_resource(
        np: *mut device_node,
        index: ::core::ffi::c_int,
        r: *mut resource,
    ) -> ::core::ffi::c_int;
    fn of_node_put(np: *mut device_node);
    fn outb(value: ::core::ffi::c_int, port: ::core::ffi::c_int);
    fn inb(port: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn spin_lock(lock: *const spinlock_t);
    fn spin_unlock(lock: *const spinlock_t);
    fn bin2bcd(value: u32) -> u32;
    fn bcd2bin(value: u32) -> u32;
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    static RTC_CONTROL: ::core::ffi::c_int;
    static RTC_FREQ_SELECT: ::core::ffi::c_int;
    static RTC_SET: u32;
    static RTC_DIV_RESET2: u32;
    static RTC_DM_BINARY: u32;
    static RTC_ALWAYS_BCD: bool;
    static RTC_SECONDS: ::core::ffi::c_int;
    static RTC_MINUTES: ::core::ffi::c_int;
    static RTC_HOURS: ::core::ffi::c_int;
    static RTC_MONTH: ::core::ffi::c_int;
    static RTC_DAY_OF_MONTH: ::core::ffi::c_int;
    static RTC_YEAR: ::core::ffi::c_int;
}

pub unsafe extern "C" fn chrp_time_init() -> i64 {
    let mut rtcs: *mut device_node;
    let mut r: resource = ::core::mem::zeroed();
    let base: ::core::ffi::c_int;

    rtcs = of_find_compatible_node(::core::ptr::null_mut(), b"rtc\0".as_ptr() as _, b"pnpPNP,b00\0".as_ptr() as _);
    if rtcs.is_null() {
        rtcs = of_find_compatible_node(::core::ptr::null_mut(), b"rtc\0".as_ptr() as _, b"ds1385-rtc\0".as_ptr() as _);
    }
    if rtcs.is_null() {
        return 0;
    }
    if of_address_to_resource(rtcs, 0, &mut r) != 0 {
        of_node_put(rtcs);
        return 0;
    }
    of_node_put(rtcs);

    base = r.start;
    nvram_as1 = 0;
    nvram_as0 = base;
    nvram_data = base + 1;
    0
}

unsafe fn chrp_cmos_clock_read(addr: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if nvram_as1 != 0 {
        outb(addr >> 8, nvram_as1);
    }
    outb(addr, nvram_as0);
    inb(nvram_data)
}

unsafe fn chrp_cmos_clock_write(val: u64, addr: ::core::ffi::c_int) {
    if nvram_as1 != 0 {
        outb(addr >> 8, nvram_as1);
    }
    outb(addr, nvram_as0);
    outb(val as ::core::ffi::c_int, nvram_data);
}

/* Set the hardware clock. -- Cort */
pub unsafe extern "C" fn chrp_set_rtc_time(tmarg: *mut rtc_time) -> ::core::ffi::c_int {
    let mut save_control: u8;
    let mut save_freq_select: u8;
    let mut tm: rtc_time = *tmarg;

    spin_lock(&rtc_lock);
    save_control = chrp_cmos_clock_read(RTC_CONTROL) as u8;
    chrp_cmos_clock_write((save_control | RTC_SET as u8) as u64, RTC_CONTROL);
    save_freq_select = chrp_cmos_clock_read(RTC_FREQ_SELECT) as u8;
    chrp_cmos_clock_write((save_freq_select | RTC_DIV_RESET2 as u8) as u64, RTC_FREQ_SELECT);

    if (save_control as u32 & RTC_DM_BINARY) == 0 || RTC_ALWAYS_BCD {
        tm.tm_sec = bin2bcd(tm.tm_sec);
        tm.tm_min = bin2bcd(tm.tm_min);
        tm.tm_hour = bin2bcd(tm.tm_hour);
        tm.tm_mon = bin2bcd(tm.tm_mon);
        tm.tm_mday = bin2bcd(tm.tm_mday);
        tm.tm_year = bin2bcd(tm.tm_year);
    }
    chrp_cmos_clock_write(tm.tm_sec as u64, RTC_SECONDS);
    chrp_cmos_clock_write(tm.tm_min as u64, RTC_MINUTES);
    chrp_cmos_clock_write(tm.tm_hour as u64, RTC_HOURS);
    chrp_cmos_clock_write(tm.tm_mon as u64, RTC_MONTH);
    chrp_cmos_clock_write(tm.tm_mday as u64, RTC_DAY_OF_MONTH);
    chrp_cmos_clock_write(tm.tm_year as u64, RTC_YEAR);

    chrp_cmos_clock_write(save_control as u64, RTC_CONTROL);
    chrp_cmos_clock_write(save_freq_select as u64, RTC_FREQ_SELECT);
    spin_unlock(&rtc_lock);
    0
}

pub unsafe extern "C" fn chrp_get_rtc_time(tm: *mut rtc_time) {
    let (mut year, mut mon, mut day, mut hour, mut min, mut sec): (u32, u32, u32, u32, u32, u32);
    loop {
        sec = chrp_cmos_clock_read(RTC_SECONDS) as u32;
        min = chrp_cmos_clock_read(RTC_MINUTES) as u32;
        hour = chrp_cmos_clock_read(RTC_HOURS) as u32;
        day = chrp_cmos_clock_read(RTC_DAY_OF_MONTH) as u32;
        mon = chrp_cmos_clock_read(RTC_MONTH) as u32;
        year = chrp_cmos_clock_read(RTC_YEAR) as u32;
        if sec == chrp_cmos_clock_read(RTC_SECONDS) as u32 { break; }
    }
    if (chrp_cmos_clock_read(RTC_CONTROL) as u32 & RTC_DM_BINARY) == 0 || RTC_ALWAYS_BCD {
        sec = bcd2bin(sec); min = bcd2bin(min); hour = bcd2bin(hour);
        day = bcd2bin(day); mon = bcd2bin(mon); year = bcd2bin(year);
    }
    if year < 70 { year += 100; }
    (*tm).tm_sec = sec; (*tm).tm_min = min; (*tm).tm_hour = hour;
    (*tm).tm_mday = day; (*tm).tm_mon = mon; (*tm).tm_year = year;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
