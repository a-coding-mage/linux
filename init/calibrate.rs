// SPDX-License-Identifier: GPL-2.0
/* calibrate.c: default delay calibration
 *
 * Excised from init/main.c
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

use core::ffi::c_char;

extern "C" {
    static mut jiffies: c_ulong;
    static mut loops_per_jiffy: c_ulong;
    fn kstrtoul(s: *mut c_char, base: c_uint, result: *mut c_ulong) -> c_int;
    fn delay_read_timer(value: *mut c_ulong) -> bool;
    fn smp_processor_id() -> c_int;
    fn __delay(loops: c_ulong);
    fn printk(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_cont(fmt: *const c_char, ...);
}

type c_ulong = usize;
type c_uint = u32;
type c_int = i32;

pub static mut lpj_fine: c_ulong = 0;
pub static mut preset_lpj: c_ulong = 0;

unsafe fn lpj_setup(str_: *mut c_char) -> c_int {
    (kstrtoul(str_, 0, &mut preset_lpj) == 0) as c_int
}

// __setup("lpj=", lpj_setup);

#[cfg(CONFIG_ARCH_HAS_DELAY_TIMER)]
const DELAY_CALIBRATION_TICKS: c_ulong = if HZ < 100 { 1 } else { HZ / 100 };
#[cfg(CONFIG_ARCH_HAS_DELAY_TIMER)]
const MAX_DIRECT_CALIBRATION_RETRIES: usize = 5;

#[cfg(CONFIG_ARCH_HAS_DELAY_TIMER)]
unsafe fn calibrate_delay_direct() -> c_ulong {
    let (mut pre_start, mut start, mut post_start): (c_ulong, c_ulong, c_ulong);
    let (mut pre_end, mut end, mut post_end): (c_ulong, c_ulong, c_ulong);
    let mut start_jiffies: c_ulong;
    let (mut timer_rate_min, mut timer_rate_max): (c_ulong, c_ulong);
    let mut good_timer_sum: c_ulong = 0;
    let mut good_timer_count: c_ulong = 0;
    let mut measured_times = [0 as c_ulong; MAX_DIRECT_CALIBRATION_RETRIES];
    let (mut max, mut min): (isize, isize) = (-1, -1);

    if !delay_read_timer(&mut pre_start) { return 0; }

    for i in 0..MAX_DIRECT_CALIBRATION_RETRIES {
        pre_start = 0;
        delay_read_timer(&mut start);
        start_jiffies = jiffies;
        while jiffies <= start_jiffies + 1 {
            pre_start = start;
            delay_read_timer(&mut start);
        }
        delay_read_timer(&mut post_start);

        pre_end = 0;
        end = post_start;
        while jiffies <= start_jiffies + 1 + DELAY_CALIBRATION_TICKS {
            pre_end = end;
            delay_read_timer(&mut end);
        }
        delay_read_timer(&mut post_end);

        timer_rate_max = (post_end - pre_start) / DELAY_CALIBRATION_TICKS;
        timer_rate_min = (pre_end - post_start) / DELAY_CALIBRATION_TICKS;

        if start >= post_end {
            // printk(KERN_NOTICE "calibrate_delay_direct() ignoring timer_rate ...");
        }
        if start < post_end && pre_start != 0 && pre_end != 0 &&
           (timer_rate_max - timer_rate_min) < (timer_rate_max >> 3) {
            good_timer_count += 1;
            good_timer_sum += timer_rate_max;
            measured_times[i] = timer_rate_max;
            if max < 0 || timer_rate_max > measured_times[max as usize] { max = i as isize; }
            if min < 0 || timer_rate_max < measured_times[min as usize] { min = i as isize; }
        } else { measured_times[i] = 0; }
    }

    while good_timer_count > 1 {
        let estimate = good_timer_sum / good_timer_count;
        let maxdiff = estimate >> 3;
        if measured_times[max as usize] - measured_times[min as usize] < maxdiff { return estimate; }
        good_timer_sum = 0;
        good_timer_count = 0;
        if measured_times[max as usize] - estimate < estimate - measured_times[min as usize] {
            measured_times[min as usize] = 0;
            min = max;
        } else {
            measured_times[max as usize] = 0;
            max = min;
        }
        for i in 0..MAX_DIRECT_CALIBRATION_RETRIES {
            if measured_times[i] == 0 { continue; }
            good_timer_count += 1;
            good_timer_sum += measured_times[i];
            if measured_times[i] < measured_times[min as usize] { min = i as isize; }
            if measured_times[i] > measured_times[max as usize] { max = i as isize; }
        }
    }
    0
}

#[cfg(not(CONFIG_ARCH_HAS_DELAY_TIMER))]
unsafe fn calibrate_delay_direct() -> c_ulong { 0 }

const LPS_PREC: c_ulong = 8;

unsafe fn calibrate_delay_converge() -> c_ulong {
    let (mut lpj, mut lpj_base, mut ticks, mut loopadd, mut loopadd_base, mut chop_limit): (c_ulong, c_ulong, c_ulong, c_ulong, c_ulong, c_ulong);
    let (mut trials, mut band, mut trial_in_band) = (0 as c_ulong, 0 as c_ulong, 0 as c_ulong);
    lpj = 1 << 12;
    ticks = jiffies;
    while ticks == jiffies {}
    ticks = jiffies;
    loop {
        trial_in_band += 1;
        if trial_in_band == (1 << band) { band += 1; trial_in_band = 0; }
        __delay(lpj * band);
        trials += band;
        if ticks != jiffies { break; }
    }
    trials -= band;
    loopadd_base = lpj * band;
    lpj_base = lpj * trials;
    loop {
        lpj = lpj_base; loopadd = loopadd_base;
        chop_limit = lpj >> LPS_PREC;
        while loopadd > chop_limit {
            lpj += loopadd; ticks = jiffies;
            while ticks == jiffies {}
            ticks = jiffies; __delay(lpj);
            if jiffies != ticks { lpj -= loopadd; }
            loopadd >>= 1;
        }
        if lpj + loopadd * 2 == lpj_base + loopadd_base * 2 {
            lpj_base = lpj; loopadd_base <<= 2; continue;
        }
        return lpj;
    }
}

static mut cpu_loops_per_jiffy: c_ulong = 0;

#[no_mangle]
pub unsafe extern "C" fn calibrate_delay_is_known() -> c_ulong { 0 }

#[no_mangle]
pub unsafe extern "C" fn calibration_delay_done() {}

#[no_mangle]
pub unsafe extern "C" fn calibrate_delay() {
    let mut lpj;
    static mut printed: bool = false;
    let this_cpu = smp_processor_id();
    let _ = this_cpu;
    if cpu_loops_per_jiffy != 0 { lpj = cpu_loops_per_jiffy; }
    else if preset_lpj != 0 { lpj = preset_lpj; }
    else if !printed && lpj_fine != 0 { lpj = lpj_fine; }
    else if (lpj = calibrate_delay_is_known()) != 0 {}
    else if (lpj = calibrate_delay_direct()) != 0 {}
    else { lpj = calibrate_delay_converge(); }
    cpu_loops_per_jiffy = lpj;
    loops_per_jiffy = lpj;
    printed = true;
    calibration_delay_done();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
