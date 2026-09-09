// SPDX-License-Identifier: GPL-2.0-only
/*
 * Reference tracker self test.
 *
 * Copyright (c) 2021 Eric Dumazet <edumazet@google.com>
 */

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct ref_tracker_dir {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ref_tracker {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    counter: c_int,
}

unsafe extern "C" {
    fn ref_tracker_alloc(
        dir: *mut ref_tracker_dir,
        trackerp: *mut *mut ref_tracker,
        gfp: c_ulong,
    );
    fn ref_tracker_free(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker);
    fn ref_tracker_dir_init(dir: *mut ref_tracker_dir, limit: c_int, name: *const c_char);
    fn ref_tracker_dir_exit(dir: *mut ref_tracker_dir);
    fn timer_setup(
        timer: *mut timer_list,
        function: Option<unsafe extern "C" fn(*mut timer_list)>,
        flags: c_ulong,
    );
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn msleep(msecs: c_ulong);
}

const GFP_KERNEL: c_ulong = 0;
const GFP_ATOMIC: c_ulong = 0;
const JIFFIES: c_ulong = 0;

static mut ref_dir: ref_tracker_dir = ref_tracker_dir { _private: [] };
static mut tracker: [*mut ref_tracker; 20] = [core::ptr::null_mut(); 20];

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc1(
    dir: *mut ref_tracker_dir,
    trackerp: *mut *mut ref_tracker,
) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc2(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc3(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc4(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc5(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc6(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc7(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc8(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc9(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc10(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc11(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc12(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc13(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc14(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc15(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc16(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc17(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc18(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_alloc19(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);
}

#[inline(never)]
unsafe fn alloctest_ref_tracker_free(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) {
    ref_tracker_free(dir, trackerp);
}

static mut test_ref_tracker_timer: timer_list = timer_list { _private: [] };
static mut test_ref_timer_done: atomic_t = atomic_t { counter: 0 };

unsafe extern "C" fn test_ref_tracker_timer_func(_t: *mut timer_list) {
    ref_tracker_alloc(&raw mut ref_dir, (&raw mut tracker).cast::<*mut ref_tracker>(), GFP_ATOMIC);
    atomic_set(&raw mut test_ref_timer_done, 1);
}

unsafe fn test_ref_tracker_init() -> c_int {
    ref_tracker_dir_init(&raw mut ref_dir, 100, c"selftest".as_ptr());

    timer_setup(&raw mut test_ref_tracker_timer, Some(test_ref_tracker_timer_func), 0);
    mod_timer(&raw mut test_ref_tracker_timer, JIFFIES + 1);

    alloctest_ref_tracker_alloc1(&raw mut ref_dir, (&raw mut tracker[1]));
    alloctest_ref_tracker_alloc2(&raw mut ref_dir, (&raw mut tracker[2]));
    alloctest_ref_tracker_alloc3(&raw mut ref_dir, (&raw mut tracker[3]));
    alloctest_ref_tracker_alloc4(&raw mut ref_dir, (&raw mut tracker[4]));
    alloctest_ref_tracker_alloc5(&raw mut ref_dir, (&raw mut tracker[5]));
    alloctest_ref_tracker_alloc6(&raw mut ref_dir, (&raw mut tracker[6]));
    alloctest_ref_tracker_alloc7(&raw mut ref_dir, (&raw mut tracker[7]));
    alloctest_ref_tracker_alloc8(&raw mut ref_dir, (&raw mut tracker[8]));
    alloctest_ref_tracker_alloc9(&raw mut ref_dir, (&raw mut tracker[9]));
    alloctest_ref_tracker_alloc10(&raw mut ref_dir, (&raw mut tracker[10]));
    alloctest_ref_tracker_alloc11(&raw mut ref_dir, (&raw mut tracker[11]));
    alloctest_ref_tracker_alloc12(&raw mut ref_dir, (&raw mut tracker[12]));
    alloctest_ref_tracker_alloc13(&raw mut ref_dir, (&raw mut tracker[13]));
    alloctest_ref_tracker_alloc14(&raw mut ref_dir, (&raw mut tracker[14]));
    alloctest_ref_tracker_alloc15(&raw mut ref_dir, (&raw mut tracker[15]));
    alloctest_ref_tracker_alloc16(&raw mut ref_dir, (&raw mut tracker[16]));
    alloctest_ref_tracker_alloc17(&raw mut ref_dir, (&raw mut tracker[17]));
    alloctest_ref_tracker_alloc18(&raw mut ref_dir, (&raw mut tracker[18]));
    alloctest_ref_tracker_alloc19(&raw mut ref_dir, (&raw mut tracker[19]));

    /* free all trackers but first 0 and 1. */
    let mut i = 2;
    while i < tracker.len() {
        alloctest_ref_tracker_free(&raw mut ref_dir, (&raw mut tracker[i]));
        i += 1;
    }

    /* Attempt to free an already freed tracker. */
    alloctest_ref_tracker_free(&raw mut ref_dir, (&raw mut tracker[2]));

    while atomic_read(&raw const test_ref_timer_done) == 0 {
        msleep(1);
    }

    /* This should warn about tracker[0] & tracker[1] being not freed. */
    ref_tracker_dir_exit(&raw mut ref_dir);

    0
}

unsafe fn test_ref_tracker_exit() {}

// module_init(test_ref_tracker_init);
// module_exit(test_ref_tracker_exit);

// MODULE_DESCRIPTION("Reference tracker self test");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
