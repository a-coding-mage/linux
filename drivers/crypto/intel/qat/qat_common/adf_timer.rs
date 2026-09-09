// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// The Linux headers and driver headers included by the C source provide these
// types and functions in the surrounding translation unit.

use core::ffi::c_void;

const ADF_DEFAULT_TIMER_PERIOD_MS: u64 = 200;

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_accel_dev {
    pub timer: *mut adf_timer,
}

#[repr(C)]
pub struct adf_timer {
    pub work_ctx: delayed_work,
    pub accel_dev: *mut adf_accel_dev,
    pub initial_ktime: i64,
}

extern "C" {
    fn to_delayed_work(work: *mut work_struct) -> *mut delayed_work;
    fn container_of_timer(work: *mut delayed_work) -> *mut adf_timer;
    fn ktime_get_real() -> i64;
    fn ktime_ms_delta(later: i64, earlier: i64) -> i64;
    fn msecs_to_jiffies(milliseconds: u64) -> u64;
    fn adf_misc_wq_queue_delayed_work(work: *mut delayed_work, delay: u64);
    fn adf_send_admin_tim_sync(accel_dev: *mut adf_accel_dev, time_periods: u64) -> i32;
    fn dev_err(dev: *mut c_void, format: *const u8, ...);
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn init_delayed_work(work: *mut delayed_work, handler: unsafe extern "C" fn(*mut work_struct));
    fn cancel_delayed_work_sync(work: *mut delayed_work);
    fn get_dev(accel_dev: *mut adf_accel_dev) -> *mut c_void;
}

/* This periodic update is used to trigger HB, RL & TL fw events */
unsafe extern "C" fn work_handler(work: *mut work_struct) {
    let timer_ctx: *mut adf_timer = container_of_timer(to_delayed_work(work));
    let accel_dev = (*timer_ctx).accel_dev;

    adf_misc_wq_queue_delayed_work(
        &mut (*timer_ctx).work_ctx,
        msecs_to_jiffies(ADF_DEFAULT_TIMER_PERIOD_MS),
    );

    let time_periods = (ktime_ms_delta(ktime_get_real(), (*timer_ctx).initial_ktime) as u64)
        / ADF_DEFAULT_TIMER_PERIOD_MS;

    if adf_send_admin_tim_sync(accel_dev, time_periods) != 0 {
        dev_err(
            get_dev(accel_dev),
            b"Failed to synchronize qat timer\0".as_ptr(),
        );
    }
}

pub unsafe extern "C" fn adf_timer_start(accel_dev: *mut adf_accel_dev) -> i32 {
    let timer_ctx = kzalloc(core::mem::size_of::<adf_timer>()) as *mut adf_timer;
    if timer_ctx.is_null() {
        return -12; // -ENOMEM
    }

    (*timer_ctx).accel_dev = accel_dev;
    (*accel_dev).timer = timer_ctx;
    (*timer_ctx).initial_ktime = ktime_get_real();

    init_delayed_work(&mut (*timer_ctx).work_ctx, work_handler);
    adf_misc_wq_queue_delayed_work(
        &mut (*timer_ctx).work_ctx,
        msecs_to_jiffies(ADF_DEFAULT_TIMER_PERIOD_MS),
    );

    0
}

pub unsafe extern "C" fn adf_timer_stop(accel_dev: *mut adf_accel_dev) {
    let timer_ctx = (*accel_dev).timer;

    if timer_ctx.is_null() {
        return;
    }

    cancel_delayed_work_sync(&mut (*timer_ctx).work_ctx);

    kfree(timer_ctx as *mut c_void);
    (*accel_dev).timer = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
