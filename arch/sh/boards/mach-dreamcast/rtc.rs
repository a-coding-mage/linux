// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/dreamcast/rtc.c
 *
 * Dreamcast AICA RTC routines.
 *
 * Copyright (c) 2001, 2002 M. R. Brown <mrbrown@0xd6.org>
 * Copyright (c) 2002 Paul Mundt <lethal@chaoticdreams.org>
 */

// The AICA RTC has an Epoch of 1/1/1950, so we must subtract 20 years (in
// seconds) to get the standard Unix Epoch when getting the time, and add
// 20 years when setting the time.
const TWENTY_YEARS: u64 = ((20 * 365u64 + 5) * 86400);

// The AICA RTC is represented by a 32-bit seconds counter stored in 2 16-bit
// registers.
const AICA_RTC_SECS_H: usize = 0xa0710000;
const AICA_RTC_SECS_L: usize = 0xa0710004;

// External types and functions supplied by the surrounding kernel sources.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rtc_time {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rtc_class_ops {
    pub read_time: Option<unsafe extern "C" fn(*mut device, *mut rtc_time) -> i32>,
    pub set_time: Option<unsafe extern "C" fn(*mut device, *mut rtc_time) -> i32>,
}

extern "C" {
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_writel(value: u32, addr: usize);
    fn rtc_time64_to_tm(time: i64, tm: *mut rtc_time);
    fn rtc_tm_to_time64(tm: *const rtc_time) -> i64;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const u8,
        id: i32,
        data: *const rtc_class_ops,
        size: usize,
    ) -> *mut platform_device;
    fn PTR_ERR_OR_ZERO(ptr: *mut platform_device) -> i32;
}

/**
 * aica_rtc_gettimeofday - Get the time from the AICA RTC
 * @dev: the RTC device (ignored)
 * @tm: pointer to resulting RTC time structure
 *
 * Grabs the current RTC seconds counter and adjusts it to the Unix Epoch.
 */
unsafe extern "C" fn aica_rtc_gettimeofday(_dev: *mut device, tm: *mut rtc_time) -> i32 {
    let mut val1: u64;
    let mut val2: u64;
    let t: i64;

    loop {
        val1 = (((__raw_readl(AICA_RTC_SECS_H) & 0xffff) as u64) << 16)
            | ((__raw_readl(AICA_RTC_SECS_L) & 0xffff) as u64);

        val2 = (((__raw_readl(AICA_RTC_SECS_H) & 0xffff) as u64) << 16)
            | ((__raw_readl(AICA_RTC_SECS_L) & 0xffff) as u64);
        if val1 == val2 {
            break;
        }
    }

    /* normalize to 1970..2106 time range */
    t = (val1.wrapping_sub(TWENTY_YEARS) as u32) as i64;

    rtc_time64_to_tm(t, tm);

    0
}

/**
 * aica_rtc_settimeofday - Set the AICA RTC to the current time
 * @dev: the RTC device (ignored)
 * @tm: pointer to new RTC time structure
 *
 * Adjusts the given @tv to the AICA Epoch and sets the RTC seconds counter.
 */
unsafe extern "C" fn aica_rtc_settimeofday(_dev: *mut device, tm: *mut rtc_time) -> i32 {
    let mut val1: u64;
    let mut val2: u64;
    let secs = rtc_tm_to_time64(tm);
    let adj: u32 = secs.wrapping_add(TWENTY_YEARS as i64) as u32;

    loop {
        __raw_writel((adj & 0xffff0000) >> 16, AICA_RTC_SECS_H);
        __raw_writel(adj & 0xffff, AICA_RTC_SECS_L);

        val1 = (((__raw_readl(AICA_RTC_SECS_H) & 0xffff) as u64) << 16)
            | ((__raw_readl(AICA_RTC_SECS_L) & 0xffff) as u64);

        val2 = (((__raw_readl(AICA_RTC_SECS_H) & 0xffff) as u64) << 16)
            | ((__raw_readl(AICA_RTC_SECS_L) & 0xffff) as u64);
        if val1 == val2 {
            break;
        }
    }

    0
}

static rtc_generic_ops: rtc_class_ops = rtc_class_ops {
    read_time: Some(aica_rtc_gettimeofday),
    set_time: Some(aica_rtc_settimeofday),
};

unsafe extern "C" fn aica_time_init() -> i32 {
    let pdev: *mut platform_device;

    pdev = platform_device_register_data(
        core::ptr::null_mut(),
        b"rtc-generic\0".as_ptr(),
        -1,
        &rtc_generic_ops,
        core::mem::size_of::<rtc_class_ops>(),
    );

    PTR_ERR_OR_ZERO(pdev)
}

// arch_initcall(aica_time_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
