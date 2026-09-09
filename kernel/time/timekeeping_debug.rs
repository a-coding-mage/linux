// SPDX-License-Identifier: GPL-2.0+
/*
 * debugfs file to track time spent in suspend
 *
 * Copyright (c) 2011, Google, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const NUM_BINS: usize = 32;
const NSEC_PER_MSEC: c_ulong = 1_000_000;

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: c_long,
}

type c_long = isize;

extern "C" {
    fn seq_puts(s: *mut seq_file, text: *const c_char) -> c_int;
    fn seq_printf(s: *mut seq_file, format: *const c_char, ...);
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut c_void,
        data: *mut c_void,
        fops: *const c_void,
    ) -> *mut c_void;
    fn pm_deferred_pr_dbg(format: *const c_char, ...);
    fn fls(x: i64) -> c_int;
    fn for_each_possible_cpu_next(cpu: c_int) -> c_int;
    fn per_cpu_timekeeping_mg_floor_swaps(cpu: c_int) -> c_ulong;
}

// Incremented every time mg_floor is updated.
#[no_mangle]
pub static mut timekeeping_mg_floor_swaps: c_ulong = 0;

static mut sleep_time_bin: [c_uint; NUM_BINS] = [0; NUM_BINS];

unsafe extern "C" fn tk_debug_sleep_time_show(
    s: *mut seq_file,
    _data: *mut c_void,
) -> c_int {
    let _ = seq_puts(s, b"      time (secs)        count\n\0".as_ptr() as *const c_char);
    let _ = seq_puts(s, b"------------------------------\n\0".as_ptr() as *const c_char);

    let mut bin: c_uint = 0;
    while bin < 32 {
        if sleep_time_bin[bin as usize] == 0 {
            bin += 1;
            continue;
        }

        let lower = if bin != 0 { 1u32 << (bin - 1) } else { 0 };
        let upper = 1u32 << bin;
        let _ = seq_printf(
            s,
            b"%10u - %-10u %4u\n\0".as_ptr() as *const c_char,
            lower,
            upper,
            sleep_time_bin[bin as usize],
        );
        bin += 1;
    }
    0
}

// Equivalent to DEFINE_SHOW_ATTRIBUTE(tk_debug_sleep_time).
extern "C" {
    static tk_debug_sleep_time_fops: c_void;
}

unsafe extern "C" fn tk_debug_sleep_time_init() -> c_int {
    let _ = debugfs_create_file(
        b"sleep_time\0".as_ptr() as *const c_char,
        0o444,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &tk_debug_sleep_time_fops,
    );
    0
}

// late_initcall(tk_debug_sleep_time_init);

pub unsafe extern "C" fn tk_debug_account_sleep_time(t: *const timespec64) {
    // Cap bin index so we don't overflow the array.
    let raw_bin = fls((*t).tv_sec);
    let bin = core::cmp::min(raw_bin, (NUM_BINS - 1) as c_int) as usize;

    sleep_time_bin[bin] = sleep_time_bin[bin].wrapping_add(1);
    pm_deferred_pr_dbg(
        b"Timekeeping suspended for %lld.%03lu seconds\n\0".as_ptr() as *const c_char,
        (*t).tv_sec,
        ((*t).tv_nsec as c_ulong) / NSEC_PER_MSEC,
    );
}

pub unsafe extern "C" fn timekeeping_get_mg_floor_swaps() -> c_ulong {
    let mut sum: c_ulong = 0;
    let mut cpu: c_int = 0;

    // Translation of for_each_possible_cpu(cpu) and data_race(per_cpu(...)).
    while cpu >= 0 {
        sum = sum.wrapping_add(per_cpu_timekeeping_mg_floor_swaps(cpu));
        let next = for_each_possible_cpu_next(cpu);
        if next == cpu {
            break;
        }
        cpu = next;
    }

    sum
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
