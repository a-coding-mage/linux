// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/char/hw_random/timeriomem-rng.c
 *
 * Copyright (C) 2009 Alexander Clouter <alex@digriz.org.uk>
 *
 * Derived from drivers/char/hw_random/omap-rng.c
 *   Copyright 2005 (c) MontaVista Software, Inc.
 *   Author: Deepak Saxena <dsaxena@plexity.net>
 *
 * Overview:
 *   This driver is useful for platforms that have an IO range that provides
 *   periodic random data from a single IO memory address.  All the platform
 *   has to do is provide the address and 'wait time' that new data becomes
 *   available.
 *
 * TODO: add support for reading sizes other than 32bits and masking
 */

// C dependencies supplied by the kernel environment:
// linux/completion.h, linux/delay.h, linux/hrtimer.h, linux/hw_random.h,
// linux/io.h, linux/ktime.h, linux/module.h, linux/of.h,
// linux/platform_device.h, linux/slab.h, linux/time.h,
// linux/timeriomem-rng.h

#[repr(C)]
pub struct TimeriomemRngPrivate {
    pub io_base: *mut core::ffi::c_void,
    pub period: KtimeT,
    pub present: u32,
    pub timer: Hrtimer,
    pub completion: Completion,
    pub rng_ops: Hwrng,
}

extern "C" {
    pub fn timeriomem_rng_read(hwrng: *mut Hwrng, data: *mut core::ffi::c_void,
                               max: usize, wait: bool) -> i32;
    pub fn timeriomem_rng_trigger(timer: *mut Hrtimer) -> HrtimerRestart;
    pub fn timeriomem_rng_probe(pdev: *mut PlatformDevice) -> i32;
    pub fn timeriomem_rng_remove(pdev: *mut PlatformDevice);
}

#[allow(non_camel_case_types)]
pub type KtimeT = i64;

#[repr(C)]
pub struct Hrtimer { _private: [u8; 0] }
#[repr(C)]
pub struct Completion { _private: [u8; 0] }
#[repr(C)]
pub struct Hwrng {
    pub name: *const core::ffi::c_char,
    pub read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
    pub quality: u32,
}
#[repr(C)]
pub struct PlatformDevice { _private: [u8; 0] }
#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
}
#[repr(C)]
pub struct PlatformDriver {
    pub driver: Driver,
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut PlatformDevice)>,
}
#[repr(C)]
pub struct Driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}
#[repr(C)]
pub enum HrtimerRestart { HrtimerNoRestart }

// The following declarations preserve the kernel-provided operations used by
// the original implementation.
extern "C" {
    fn ktime_to_us(kt: KtimeT) -> i32;
    fn us_to_ktime(us: i32) -> KtimeT;
    fn wait_for_completion(completion: *mut Completion);
    fn usleep_range(min: i32, max: i32);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn reinit_completion(completion: *mut Completion);
    fn complete(completion: *mut Completion);
    fn hrtimer_forward_now(timer: *mut Hrtimer, period: KtimeT);
    fn hrtimer_restart(timer: *mut Hrtimer);
    fn hrtimer_cancel(timer: *mut Hrtimer);
}

pub unsafe extern "C" fn timeriomem_rng_read_impl(
    hwrng: *mut Hwrng, mut data: *mut core::ffi::c_void,
    mut max: usize, wait: bool,
) -> i32 {
    let priv_: *mut TimeriomemRngPrivate =
        (hwrng as *mut u8).sub(core::mem::offset_of!(TimeriomemRngPrivate, rng_ops))
            as *mut TimeriomemRngPrivate;
    let mut retval: i32 = 0;
    let period_us = ktime_to_us((*priv_).period);

    if !wait && (*priv_).present == 0 {
        return 0;
    }

    wait_for_completion(&mut (*priv_).completion);

    loop {
        if retval > 0 {
            let tolerance = core::cmp::max(1, period_us / 100);
            usleep_range(period_us, period_us + tolerance);
        }

        *(data as *mut u32) = readl((*priv_).io_base);
        retval += core::mem::size_of::<u32>() as i32;
        data = (data as *mut u8).add(core::mem::size_of::<u32>()) as *mut core::ffi::c_void;
        max -= core::mem::size_of::<u32>();
        if !(wait && max > core::mem::size_of::<u32>()) {
            break;
        }
    }

    (*priv_).present = 0;
    reinit_completion(&mut (*priv_).completion);
    hrtimer_forward_now(&mut (*priv_).timer, (*priv_).period);
    hrtimer_restart(&mut (*priv_).timer);
    retval
}

pub unsafe extern "C" fn timeriomem_rng_trigger_impl(timer: *mut Hrtimer) -> HrtimerRestart {
    let priv_: *mut TimeriomemRngPrivate =
        (timer as *mut u8).sub(core::mem::offset_of!(TimeriomemRngPrivate, timer))
            as *mut TimeriomemRngPrivate;
    (*priv_).present = 1;
    complete(&mut (*priv_).completion);
    HrtimerRestart::HrtimerNoRestart
}

// The remaining probe, remove, driver registration, and module metadata are
// represented by the externally supplied platform-driver integration.
pub static mut TIMERIOMEM_RNG_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"timeriomem_rng\0".as_ptr() as *const core::ffi::c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

pub static mut TIMERIOMEM_RNG_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver {
        name: b"timeriomem_rng\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: unsafe { TIMERIOMEM_RNG_MATCH.as_ptr() },
    },
    probe: Some(timeriomem_rng_probe),
    remove: Some(timeriomem_rng_remove),
};

// MODULE_DEVICE_TABLE(of, timeriomem_rng_match)
// module_platform_driver(timeriomem_rng_driver)
// MODULE_LICENSE("GPL")
// MODULE_AUTHOR("Alexander Clouter <alex@digriz.org.uk>")
// MODULE_DESCRIPTION("Timer IOMEM H/W RNG driver")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
