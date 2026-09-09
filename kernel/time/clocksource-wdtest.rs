// SPDX-License-Identifier: GPL-2.0+
/*
 * Unit test for the clocksource watchdog.
 *
 * Copyright (C) 2021 Facebook, Inc.
 * Copyright (C) 2026 Intel Corp.
 *
 * Author: Paul E. McKenney <paulmck@kernel.org>
 * Author: Thomas Gleixner <tglx@kernel.org>
 */

// Dependencies supplied by the kernel are intentionally left external.
type KtimeT = i64;

extern "C" {
    fn ktime_get_raw_fast_ns() -> KtimeT;
    fn smp_processor_id() -> i32;
    fn udelay(usecs: u32);
    fn msleep(msecs: u32);
    fn kthread_should_stop() -> bool;
    fn schedule_timeout_interruptible(timeout: i64);
    fn clocksource_unregister(cs: *mut Clocksource);
    fn clocksource_register_khz(cs: *mut Clocksource, khz: u32) -> i32;
    fn kthread_run(f: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
                   data: *mut core::ffi::c_void,
                   name: *const core::ffi::c_char) -> *mut TaskStruct;
    fn kthread_stop(task: *mut TaskStruct) -> i32;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn ptr_err<T>(ptr: *mut T) -> i32;
}

#[repr(C)]
pub struct Clocksource {
    pub name: *const core::ffi::c_char,
    pub rating: i32,
    pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> u64>,
    pub mask: u64,
    pub flags: u32,
    pub list: [usize; 2],
    pub wd_cpu: i32,
}

#[repr(C)]
pub struct TaskStruct {
    _private: [u8; 0],
}

const NSEC_PER_MSEC: KtimeT = 1_000_000;
const NSEC_PER_SEC: KtimeT = 1_000_000_000;
const HZ: i64 = 0; // Supplied by the kernel build.
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 0;
const CLOCK_SOURCE_CALIBRATED: u32 = 1 << 1;
const CLOCK_SOURCE_MUST_VERIFY: u32 = 1 << 2;
const CLOCK_SOURCE_WDTEST: u32 = 1 << 3;
const CLOCK_SOURCE_WDTEST_PERCPU: u32 = 1 << 4;
const CLOCK_SOURCE_UNSTABLE: u32 = 1 << 5;
const CLOCK_SOURCE_VALID_FOR_HRES: u32 = 1 << 6;

#[repr(i32)]
enum WdtestStates {
    WDTEST_INJECT_NONE,
    WDTEST_INJECT_DELAY,
    WDTEST_INJECT_POSITIVE,
    WDTEST_INJECT_NEGATIVE,
    WDTEST_INJECT_PERCPU = 0x100,
}

static mut wdtest_state: WdtestStates = WdtestStates::WDTEST_INJECT_NONE;
static mut wdtest_test_count: usize = 0;
static mut wdtest_last_ts: KtimeT = 0;
static mut wdtest_offset: KtimeT = 0;

const SHIFT_4000PPM: u32 = 8;

unsafe fn wdtest_get_offset(cs: *mut Clocksource) -> KtimeT {
    if (wdtest_state as i32) < WdtestStates::WDTEST_INJECT_PERCPU as i32 {
        return if wdtest_test_count & 1 != 0 { 0 } else { wdtest_offset >> SHIFT_4000PPM };
    }
    if (*cs).wd_cpu == smp_processor_id() { 0 } else { NSEC_PER_MSEC }
}

unsafe extern "C" fn wdtest_ktime_read(cs: *mut Clocksource) -> u64 {
    let now = ktime_get_raw_fast_ns();
    let intv = now - wdtest_last_ts;
    if intv > NSEC_PER_SEC / 4 {
        wdtest_test_count = wdtest_test_count.wrapping_add(1);
        wdtest_last_ts = now;
        wdtest_offset = intv;
    }
    match (wdtest_state as i32) & !(WdtestStates::WDTEST_INJECT_PERCPU as i32) {
        x if x == WdtestStates::WDTEST_INJECT_POSITIVE as i32 => (now + wdtest_get_offset(cs)) as u64,
        x if x == WdtestStates::WDTEST_INJECT_NEGATIVE as i32 => (now - wdtest_get_offset(cs)) as u64,
        x if x == WdtestStates::WDTEST_INJECT_DELAY as i32 => { udelay(500); now as u64 },
        _ => now as u64,
    }
}

const KTIME_FLAGS: u32 = CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_CALIBRATED |
    CLOCK_SOURCE_MUST_VERIFY | CLOCK_SOURCE_WDTEST;

static mut clocksource_wdtest_ktime: Clocksource = Clocksource {
    name: b"wdtest-ktime\0".as_ptr() as *const _, rating: 10,
    read: Some(wdtest_ktime_read), mask: u64::MAX, flags: KTIME_FLAGS,
    list: [0; 2], wd_cpu: 0,
};

unsafe fn wdtest_clocksource_reset(which: WdtestStates, percpu: bool) {
    clocksource_unregister(&raw mut clocksource_wdtest_ktime);
    wdtest_state = which;
    if percpu { wdtest_state = core::mem::transmute(wdtest_state as i32 | 0x100); }
    wdtest_test_count = 0; wdtest_last_ts = 0;
    clocksource_wdtest_ktime.rating = 10;
    clocksource_wdtest_ktime.flags = KTIME_FLAGS;
    if percpu { clocksource_wdtest_ktime.flags |= CLOCK_SOURCE_WDTEST_PERCPU; }
    clocksource_register_khz(&raw mut clocksource_wdtest_ktime, 1000 * 1000);
}

unsafe fn wdtest_execute(which: WdtestStates, percpu: bool, expect: u32, calls: usize) -> bool {
    wdtest_clocksource_reset(which, percpu);
    while wdtest_test_count < calls {
        let flags = clocksource_wdtest_ktime.flags;
        if kthread_should_stop() { return false; }
        if flags & CLOCK_SOURCE_UNSTABLE != 0 { return expect & CLOCK_SOURCE_UNSTABLE != 0; }
        if flags & CLOCK_SOURCE_VALID_FOR_HRES != 0 { return expect & CLOCK_SOURCE_VALID_FOR_HRES != 0; }
        msleep(100);
    }
    expect == 0
}

unsafe fn wdtest_run(percpu: bool) -> bool {
    wdtest_execute(WdtestStates::WDTEST_INJECT_NONE, percpu, CLOCK_SOURCE_VALID_FOR_HRES, 8) &&
    wdtest_execute(WdtestStates::WDTEST_INJECT_DELAY, percpu, 0, 4) &&
    wdtest_execute(WdtestStates::WDTEST_INJECT_POSITIVE, percpu, CLOCK_SOURCE_UNSTABLE, 8) &&
    wdtest_execute(WdtestStates::WDTEST_INJECT_NEGATIVE, percpu, CLOCK_SOURCE_UNSTABLE, 8)
}

unsafe extern "C" fn wdtest_func(_arg: *mut core::ffi::c_void) -> i32 {
    clocksource_register_khz(&raw mut clocksource_wdtest_ktime, 1000 * 1000);
    let _ = wdtest_run(false) && wdtest_run(true);
    clocksource_unregister(&raw mut clocksource_wdtest_ktime);
    while !kthread_should_stop() { schedule_timeout_interruptible(3600 * HZ); }
    0
}

static mut wdtest_thread: *mut TaskStruct = core::ptr::null_mut();

unsafe extern "C" fn clocksource_wdtest_init() -> i32 {
    let t = kthread_run(wdtest_func, core::ptr::null_mut(), b"wdtest\0".as_ptr() as *const _);
    if t.is_null() { return -1; }
    wdtest_thread = t; 0
}

unsafe extern "C" fn clocksource_wdtest_cleanup() {
    if !wdtest_thread.is_null() { kthread_stop(wdtest_thread); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
