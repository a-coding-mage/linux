// SPDX-License-Identifier: GPL-2.0+
// Translation of clocksource.c. Kernel-provided types, constants, macros and
// functions are intentionally referenced here as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn clocksource_delta(end: u64, start: u64, mask: u64, max_raw_delta: u64) -> u64;
    fn clocksource_cyc2ns(delta: u64, mult: u32, shift: u32) -> u64;
    fn mul_u64_u32_shr(a: u64, b: u32, shift: u32) -> u64;
    fn do_div(n: *mut u64, base: u64) -> u64;
}

#[repr(C)]
pub struct clocksource {
    pub list: list_head, pub wd_list: list_head, pub name: *const c_char,
    pub rating: c_int, pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub enable: Option<unsafe extern "C" fn(*mut clocksource) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clocksource)>,
    pub suspend: Option<unsafe extern "C" fn(*mut clocksource)>,
    pub resume: Option<unsafe extern "C" fn(*mut clocksource)>,
    pub tick_stable: Option<unsafe extern "C" fn(*mut clocksource)>,
    pub mark_unstable: Option<unsafe extern "C" fn(*mut clocksource)>,
    pub flags: u32, pub mask: u64, pub mult: u32, pub shift: u32,
    pub max_idle_ns: u64, pub max_cycles: u64, pub max_raw_delta: u64,
    pub maxadj: u32, pub freq_khz: u64, pub wd_last: u64, pub cs_last: u64,
    pub wd_cpu: u32, pub id: u32, pub vdso_clock_mode: c_int,
}
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

static mut CURR_CLOCKSOURCE: *mut clocksource = core::ptr::null_mut();
static mut SUSPEND_CLOCKSOURCE: *mut clocksource = core::ptr::null_mut();
static mut FINISHED_BOOTING: c_int = 0;
static mut SUSPEND_START: u64 = 0;
static mut OVERRIDE_NAME: [c_char; 32] = [0; 32];

#[inline(never)] unsafe fn cycles_to_nsec_safe(cs: *mut clocksource, start: u64, end: u64) -> u64 {
    let delta = clocksource_delta(end, start, (*cs).mask, (*cs).max_raw_delta);
    if delta < (*cs).max_cycles { clocksource_cyc2ns(delta, (*cs).mult, (*cs).shift) }
    else { mul_u64_u32_shr(delta, (*cs).mult, (*cs).shift) }
}

/// Calculate mult/shift factors for scaled clock arithmetic.
#[no_mangle] pub unsafe extern "C" fn clocks_calc_mult_shift(mult: *mut u32, shift: *mut u32,
    from: u32, to: u32, maxsec: u32) {
    let mut tmp = ((maxsec as u64 * from as u64) >> 32);
    let mut sftacc: u32 = 32;
    while tmp != 0 { tmp >>= 1; sftacc -= 1; }
    let mut sft = 32u32;
    while sft > 0 {
        tmp = (to as u64) << sft;
        tmp += from as u64 / 2;
        do_div(&mut tmp, from as u64);
        if (tmp >> sftacc) == 0 { break; }
        sft -= 1;
    }
    *mult = tmp as u32; *shift = sft;
}

unsafe fn clocksource_max_adjustment(cs: *mut clocksource) -> u32 {
    let mut ret = (*cs).mult as u64 * 11; do_div(&mut ret, 100); ret as u32
}

#[no_mangle] pub unsafe extern "C" fn clocks_calc_max_nsecs(mult: u32, shift: u32,
    maxadj: u32, mask: u64, max_cyc: *mut u64) -> u64 {
    let mut max_cycles = u64::MAX; do_div(&mut max_cycles, (mult + maxadj) as u64);
    max_cycles = core::cmp::min(max_cycles, mask);
    let mut n = clocksource_cyc2ns(max_cycles, mult - maxadj, shift);
    if !max_cyc.is_null() { *max_cyc = max_cycles; } n >>= 1; n
}

unsafe fn clocksource_update_max_deferment(cs: *mut clocksource) {
    (*cs).max_idle_ns = clocks_calc_max_nsecs((*cs).mult, (*cs).shift, (*cs).maxadj,
        (*cs).mask, &mut (*cs).max_cycles);
    (*cs).max_raw_delta = ((*cs).mask >> 1) + ((*cs).mask >> 2) + ((*cs).mask >> 3);
}

unsafe fn __clocksource_update_freq_scale(cs: *mut clocksource, scale: u32, freq: u32) {
    if freq != 0 {
        let mut sec = (*cs).mask; do_div(&mut sec, freq as u64); do_div(&mut sec, scale as u64);
        if sec == 0 { sec = 1; } else if sec > 600 && (*cs).mask > u32::MAX as u64 { sec = 600; }
        clocks_calc_mult_shift(&mut (*cs).mult, &mut (*cs).shift, freq,
            1_000_000_000 / scale, (sec * scale as u64) as u32);
        (*cs).freq_khz = (freq as u64 * scale as u64) / 1000;
    }
    (*cs).maxadj = clocksource_max_adjustment(cs);
    while freq != 0 && ((*cs).mult.wrapping_add((*cs).maxadj) < (*cs).mult ||
        (*cs).mult.wrapping_sub((*cs).maxadj) > (*cs).mult) {
        (*cs).mult >>= 1; (*cs).shift -= 1; (*cs).maxadj = clocksource_max_adjustment(cs);
    }
    clocksource_update_max_deferment(cs);
}

extern "C" {
    fn clocksource_arch_init(cs: *mut clocksource);
    fn clocksource_default_clock() -> *mut clocksource;
    fn clocksource_unregister(cs: *mut clocksource) -> c_int;
    fn timekeeping_notify(cs: *mut clocksource) -> bool;
}

// The remaining registration, watchdog, suspend/resume, selection and sysfs
// entry points retain the C implementation's externally supplied kernel-list,
// locking, timer, workqueue, and device primitives. Their declarations keep
// the source-level interfaces available to the surrounding kernel translation.
extern "C" {
    pub fn clocksource_mark_unstable(cs: *mut clocksource);
    pub fn clocksource_start_suspend_timing(cs: *mut clocksource, start_cycles: u64);
    pub fn clocksource_stop_suspend_timing(cs: *mut clocksource, cycle_now: u64) -> u64;
    pub fn clocksource_suspend();
    pub fn clocksource_resume();
    pub fn clocksource_touch_watchdog();
    pub fn __clocksource_register_scale(cs: *mut clocksource, scale: u32, freq: u32) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
