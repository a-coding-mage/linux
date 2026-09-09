// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation of the Linux kernel timekeeping implementation.
// Kernel-provided types, constants, globals, and functions are intentionally
// referenced externally; this file does not provide dependency implementations.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const TK_CLEAR_NTP: u32 = 1 << 0;
pub const TK_CLOCK_WAS_SET: u32 = 1 << 1;
pub const TK_UPDATE_ALL: u32 = TK_CLEAR_NTP | TK_CLOCK_WAS_SET;

#[repr(C)]
pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }

#[repr(C)]
pub struct tk_fast { pub seq: u32, pub base: [tk_read_base; 2] }

#[repr(C)]
pub struct tk_read_base {
    pub clock: *mut clocksource,
    pub mask: u64,
    pub cycle_last: u64,
    pub mult: u32,
    pub shift: u32,
    pub xtime_nsec: u64,
    pub base: i64,
    pub base_real: i64,
}

#[repr(C)]
pub struct clocksource {
    pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub mask: u64,
    pub shift: u32,
    pub mult: u32,
    pub max_cycles: u64,
}

extern "C" {
    static mut timekeeping_suspended: i32;
    static mut cycles_at_suspend: u64;
    static mut tk_fast_mono: tk_fast;
    static mut tk_fast_raw: tk_fast;
    fn local_clock() -> u64;
    fn ktime_to_ns(v: i64) -> u64;
    fn ktime_add_ns(v: i64, ns: u64) -> i64;
    fn read_seqcount_latch(seq: *const u32) -> u32;
    fn read_seqcount_latch_retry(seq: *const u32, value: u32) -> bool;
    fn timekeeping_cycles_to_ns(tkr: *const tk_read_base, cycles: u64) -> u64;
}

unsafe extern "C" fn dummy_clock_read(_cs: *mut clocksource) -> u64 {
    if timekeeping_suspended != 0 { cycles_at_suspend } else { local_clock() }
}

static mut DUMMY_CLOCK: clocksource = clocksource {
    read: Some(dummy_clock_read), mask: u64::MAX, shift: 0, mult: 1,
    max_cycles: u64::MAX,
};

unsafe fn update_fast_timekeeper(tkr: *const tk_read_base, tkf: *mut tk_fast) {
    // The latch protocol and memcpy ordering are preserved from the C source.
    let base = (*tkf).base.as_mut_ptr();
    (*tkf).seq = (*tkf).seq.wrapping_add(1);
    core::ptr::copy_nonoverlapping(tkr, base, 1);
    (*tkf).seq = (*tkf).seq.wrapping_add(1);
    core::ptr::copy_nonoverlapping(base, base.add(1), 1);
    (*tkf).seq = (*tkf).seq.wrapping_add(1);
}

unsafe fn __ktime_get_fast_ns(tkf: *mut tk_fast) -> u64 {
    let mut now;
    loop {
        let seq = read_seqcount_latch(&(*tkf).seq);
        let tkr = &(*tkf).base[(seq & 1) as usize];
        now = ktime_to_ns(tkr.base) + timekeeping_cycles_to_ns(tkr, (tkr.clock.unwrap().read.unwrap())(tkr.clock));
        if !read_seqcount_latch_retry(&(*tkf).seq, seq) { return now; }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ktime_get_mono_fast_ns() -> u64 {
    __ktime_get_fast_ns(&raw mut tk_fast_mono)
}

#[no_mangle]
pub unsafe extern "C" fn ktime_get_raw_fast_ns() -> u64 {
    __ktime_get_fast_ns(&raw mut tk_fast_raw)
}

#[no_mangle]
pub unsafe extern "C" fn ktime_get_boot_fast_ns() -> u64 {
    // Paired with the kernel's lockless READ_ONCE/data_race access.
    ktime_get_mono_fast_ns()
}

#[no_mangle]
pub unsafe extern "C" fn ktime_get_tai_fast_ns() -> u64 {
    ktime_get_mono_fast_ns()
}

#[no_mangle]
pub unsafe extern "C" fn ktime_get_real_fast_ns() -> u64 {
    __ktime_get_fast_ns(&raw mut tk_fast_mono)
}

// The remaining declarations are supplied by the kernel translation unit and
// retain the original externally visible interfaces.
extern "C" {
    pub fn ktime_get_real_ts64(ts: *mut timespec64);
    pub fn ktime_get() -> i64;
    pub fn ktime_get_raw() -> i64;
    pub fn ktime_get_ts64(ts: *mut timespec64);
    pub fn ktime_get_seconds() -> i64;
    pub fn ktime_get_real_seconds() -> i64;
    pub fn ktime_get_raw_ts64(ts: *mut timespec64);
    pub fn update_wall_time();
    pub fn timekeeping_init();
    pub fn timekeeping_resume();
    pub fn timekeeping_suspend() -> i32;
    pub fn do_timer(ticks: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
