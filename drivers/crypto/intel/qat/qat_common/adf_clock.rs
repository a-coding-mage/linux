// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

use core::ffi::{c_char, c_int, c_void};

const MEASURE_CLOCK_RETRIES: u32 = 10;
const MEASURE_CLOCK_DELAY_US: u32 = 10000;
const ME_CLK_DIVIDER: u64 = 16;
const MEASURE_CLOCK_DELTA_THRESHOLD_US: u32 = 100;

const NSEC_PER_USEC: u64 = 1000;
const NSEC_PER_MSEC: u64 = 1_000_000;
const HZ_PER_MHZ: u32 = 1_000_000;
const ETIMEDOUT: c_int = 110;
const EIO: c_int = 5;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

extern "C" {
    fn ktime_get_real_ts64(ts: *mut timespec64);
    fn adf_get_fw_timestamp(accel_dev: *mut adf_accel_dev, timestamp: *mut u64) -> c_int;
    fn fsleep(usecs: u32);
    fn GET_DEV(accel_dev: *mut adf_accel_dev) -> *mut c_void;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
}

#[inline]
unsafe fn timespec_to_us(ts: *const timespec64) -> u64 {
    let ns = ((*ts).tv_sec as u64).wrapping_mul(1_000_000_000)
        .wrapping_add((*ts).tv_nsec as u64);
    ns.wrapping_add(NSEC_PER_USEC / 2) / NSEC_PER_USEC
}

#[inline]
unsafe fn timespec_to_ms(ts: *const timespec64) -> u64 {
    let ns = ((*ts).tv_sec as u64).wrapping_mul(1_000_000_000)
        .wrapping_add((*ts).tv_nsec as u64);
    ns.wrapping_add(NSEC_PER_MSEC / 2) / NSEC_PER_MSEC
}

pub unsafe fn adf_clock_get_current_time() -> u64 {
    let mut ts = timespec64 { tv_sec: 0, tv_nsec: 0 };

    ktime_get_real_ts64(&mut ts);
    timespec_to_ms(&ts)
}

unsafe fn measure_clock(accel_dev: *mut adf_accel_dev, frequency: *mut u32) -> c_int {
    let mut ts1 = timespec64 { tv_sec: 0, tv_nsec: 0 };
    let mut ts2 = timespec64 { tv_sec: 0, tv_nsec: 0 };
    let mut ts3 = timespec64 { tv_sec: 0, tv_nsec: 0 };
    let mut ts4 = timespec64 { tv_sec: 0, tv_nsec: 0 };
    let mut timestamp1: u64 = 0;
    let mut timestamp2: u64 = 0;
    let mut temp: u64;
    let mut delta_us: u32;
    let mut tries: u32;
    let mut ret: c_int;

    tries = MEASURE_CLOCK_RETRIES;
    loop {
        ktime_get_real_ts64(&mut ts1);
        ret = adf_get_fw_timestamp(accel_dev, &mut timestamp1);
        if ret != 0 {
            dev_err(GET_DEV(accel_dev), b"Failed to get fw timestamp\n\0".as_ptr() as *const c_char);
            return ret;
        }
        ktime_get_real_ts64(&mut ts2);
        delta_us = timespec_to_us(&ts2).wrapping_sub(timespec_to_us(&ts1)) as u32;
        if !(delta_us > MEASURE_CLOCK_DELTA_THRESHOLD_US && { tries = tries.wrapping_sub(1); tries != 0 }) { break; }
    }

    if tries == 0 {
        dev_err(GET_DEV(accel_dev), b"Excessive clock measure delay\n\0".as_ptr() as *const c_char);
        return -ETIMEDOUT;
    }

    fsleep(MEASURE_CLOCK_DELAY_US);

    tries = MEASURE_CLOCK_RETRIES;
    loop {
        ktime_get_real_ts64(&mut ts3);
        if adf_get_fw_timestamp(accel_dev, &mut timestamp2) != 0 {
            dev_err(GET_DEV(accel_dev), b"Failed to get fw timestamp\n\0".as_ptr() as *const c_char);
            return -EIO;
        }
        ktime_get_real_ts64(&mut ts4);
        delta_us = timespec_to_us(&ts4).wrapping_sub(timespec_to_us(&ts3)) as u32;
        if !(delta_us > MEASURE_CLOCK_DELTA_THRESHOLD_US && { tries = tries.wrapping_sub(1); tries != 0 }) { break; }
    }

    if tries == 0 {
        dev_err(GET_DEV(accel_dev), b"Excessive clock measure delay\n\0".as_ptr() as *const c_char);
        return -ETIMEDOUT;
    }

    delta_us = timespec_to_us(&ts3).wrapping_sub(timespec_to_us(&ts1)) as u32;
    if delta_us == 0 { return -EINVAL; }

    temp = timestamp2.wrapping_sub(timestamp1).wrapping_mul(ME_CLK_DIVIDER).wrapping_mul(10);
    temp = temp.wrapping_add((delta_us as u64) / 2) / delta_us as u64;
    *frequency = (temp as u32).wrapping_mul(HZ_PER_MHZ / 10);
    0
}

pub unsafe fn adf_dev_measure_clock(accel_dev: *mut adf_accel_dev, frequency: *mut u32, min: u32, max: u32) -> c_int {
    let mut freq: u32 = 0;
    let ret = measure_clock(accel_dev, &mut freq);
    if ret != 0 { return ret; }
    *frequency = freq.clamp(min, max);
    if *frequency != freq {
        dev_warn(GET_DEV(accel_dev), b"Measured clock %d Hz is out of range, assuming %d\n\0".as_ptr() as *const c_char, freq, *frequency);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
