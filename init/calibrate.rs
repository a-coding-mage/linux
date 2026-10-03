// SPDX-License-Identifier: GPL-2.0
/* calibrate.c: default delay calibration
 *
 * Excised from init/main.c
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

//! Generic delay calibration, including direct timer sampling and convergence.

#[allow(
    clippy::all,
    dead_code,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]
mod bindings {
    use kernel::ffi;

    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/init_calibrate_generated.rs"
    ));
}

mod main_printk;
mod main_setup;

use core::{mem, ptr};
#[cfg(CONFIG_ARCH_HAS_DELAY_TIMER)]
use kernel::ffi::c_long;
use kernel::ffi::{c_char, c_int, c_ulong, c_ulonglong};
use main_printk::main_printk;

// These architectures use the unadorned asm-generic per-CPU section and
// attributes, checked against the native declarations in calibrate_percpu.c.
// Do not silently reuse this declaration for an architecture requiring extra
// compiler attributes or a different per-CPU base section.
#[cfg(not(any(CONFIG_X86_64, CONFIG_ARM64)))]
compile_error!("Rust delay calibration requires the audited x86-64 or ARM64 per-CPU ABI");

/// Fine-grained boot-CPU calibration supplied by the architecture timer.
#[no_mangle]
#[link_section = ".bss"]
pub static mut lpj_fine: c_ulong = 0;

/// Optional loops-per-jiffy override supplied by the lpj= boot argument.
#[no_mangle]
#[link_section = ".bss"]
pub static mut preset_lpj: c_ulong = 0;

// The C companion only declares this symbol and obtains its canonical per_cpu
// address. Its storage and every read/write remain owned by this Rust object.
#[export_name = "__rust_calibrate_cpu_loops_per_jiffy"]
#[cfg_attr(CONFIG_SMP, link_section = ".data..percpu")]
#[cfg_attr(not(CONFIG_SMP), link_section = ".data")]
static mut CPU_LOOPS_PER_JIFFY: c_ulong = 0;

static mut PRINTED: bool = false;

#[link_section = ".init.text"]
#[cold]
unsafe extern "C" fn lpj_setup(argument: *mut c_char) -> c_int {
    // SAFETY: the init parser supplies a live C string and serializes access to
    // preset_lpj. Match the native kstrtoul inline's width/alignment dispatch;
    // kstrtoul itself is not an externally callable symbol.
    let result = unsafe {
        if mem::size_of::<c_ulong>() == mem::size_of::<c_ulonglong>()
            && mem::align_of::<c_ulong>() == mem::align_of::<c_ulonglong>()
        {
            bindings::kstrtoull(argument, 0, ptr::addr_of_mut!(preset_lpj).cast())
        } else {
            bindings::_kstrtoul(argument, 0, ptr::addr_of_mut!(preset_lpj))
        }
    };
    (result == 0) as c_int
}

main_setup::setup_param!("lpj=", LPJ_SETUP, Some(lpj_setup), 0);

#[inline]
fn jiffies() -> c_ulong {
    // SAFETY: jiffies is a permanently live native unsigned long. Its original
    // declaration is volatile; every loop condition must actually reload it.
    unsafe { ptr::read_volatile(ptr::addr_of!(bindings::jiffies)) }
}

#[cfg(CONFIG_ARCH_HAS_DELAY_TIMER)]
#[inline]
fn time_before_eq(a: c_ulong, b: c_ulong) -> bool {
    // linux/jiffies.h: time_after_eq(b, a), including unsigned subtraction
    // before interpreting the sign. A numeric <= comparison fails at wrap.
    (b.wrapping_sub(a) as c_long) >= 0
}

const HZ: c_ulong = bindings::HZ as c_ulong;
#[cfg(CONFIG_ARCH_HAS_DELAY_TIMER)]
const DELAY_CALIBRATION_TICKS: c_ulong = if HZ < 100 { 1 } else { HZ / 100 };
#[cfg(CONFIG_ARCH_HAS_DELAY_TIMER)]
const MAX_DIRECT_CALIBRATION_RETRIES: usize = 5;

#[cfg(CONFIG_ARCH_HAS_DELAY_TIMER)]
unsafe fn calibrate_delay_direct() -> c_ulong {
    let (mut pre_start, mut start, mut post_start) = (0, 0, 0);
    let (mut pre_end, mut end): (c_ulong, c_ulong);
    let mut post_end = 0;
    let mut good_timer_sum: c_ulong = 0;
    let mut good_timer_count: c_ulong = 0;
    let mut measured_times = [0 as c_ulong; MAX_DIRECT_CALIBRATION_RETRIES];
    let (mut max, mut min): (c_int, c_int) = (-1, -1);

    // SAFETY: each pointer refers to a live unsigned long. As in the native
    // algorithm, timer availability is probed before taking any samples.
    unsafe {
        if !bindings::delay_read_timer(&mut pre_start) {
            return 0;
        }

        for i in 0..MAX_DIRECT_CALIBRATION_RETRIES {
            pre_start = 0;
            bindings::delay_read_timer(&mut start);
            let start_jiffies = jiffies();
            // Bracket both jiffy boundaries so asynchronous interruptions can
            // be detected without assuming timer-read/tick ordering.
            while time_before_eq(jiffies(), start_jiffies.wrapping_add(1)) {
                pre_start = start;
                bindings::delay_read_timer(&mut start);
            }
            bindings::delay_read_timer(&mut post_start);

            pre_end = 0;
            end = post_start;
            while time_before_eq(
                jiffies(),
                start_jiffies
                    .wrapping_add(1)
                    .wrapping_add(DELAY_CALIBRATION_TICKS),
            ) {
                pre_end = end;
                bindings::delay_read_timer(&mut end);
            }
            bindings::delay_read_timer(&mut post_end);

            let timer_rate_max = post_end.wrapping_sub(pre_start) / DELAY_CALIBRATION_TICKS;
            let timer_rate_min = pre_end.wrapping_sub(post_start) / DELAY_CALIBRATION_TICKS;

            if start >= post_end {
                main_printk!(
                    "calibrate_delay_direct",
                    b"\x015calibrate_delay_direct() ignoring timer_rate as we had a TSC wrap around start=%lu >=post_end=%lu\n\0",
                    start,
                    post_end,
                );
            }
            if start < post_end
                && pre_start != 0
                && pre_end != 0
                && timer_rate_max.wrapping_sub(timer_rate_min) < (timer_rate_max >> 3)
            {
                good_timer_count = good_timer_count.wrapping_add(1);
                good_timer_sum = good_timer_sum.wrapping_add(timer_rate_max);
                measured_times[i] = timer_rate_max;
                if max < 0 || timer_rate_max > measured_times[max as usize] {
                    max = i as c_int;
                }
                if min < 0 || timer_rate_max < measured_times[min as usize] {
                    min = i as c_int;
                }
            } else {
                measured_times[i] = 0;
            }
        }

        // Accepted samples initialize both extrema. Reject the sample furthest
        // from the mean until the range lies within the original 12.5% bound.
        while good_timer_count > 1 {
            let estimate = good_timer_sum / good_timer_count;
            let maxdiff = estimate >> 3;
            if measured_times[max as usize].wrapping_sub(measured_times[min as usize]) < maxdiff {
                return estimate;
            }

            good_timer_sum = 0;
            good_timer_count = 0;
            if measured_times[max as usize].wrapping_sub(estimate)
                < estimate.wrapping_sub(measured_times[min as usize])
            {
                main_printk!(
                    "calibrate_delay_direct",
                    b"\x015calibrate_delay_direct() dropping min bogoMips estimate %d = %lu\n\0",
                    min,
                    measured_times[min as usize],
                );
                measured_times[min as usize] = 0;
                min = max;
            } else {
                main_printk!(
                    "calibrate_delay_direct",
                    b"\x015calibrate_delay_direct() dropping max bogoMips estimate %d = %lu\n\0",
                    max,
                    measured_times[max as usize],
                );
                measured_times[max as usize] = 0;
                max = min;
            }
            for i in 0..MAX_DIRECT_CALIBRATION_RETRIES {
                if measured_times[i] == 0 {
                    continue;
                }
                good_timer_count = good_timer_count.wrapping_add(1);
                good_timer_sum = good_timer_sum.wrapping_add(measured_times[i]);
                if measured_times[i] < measured_times[min as usize] {
                    min = i as c_int;
                }
                if measured_times[i] > measured_times[max as usize] {
                    max = i as c_int;
                }
            }
        }

        main_printk!(
            "calibrate_delay_direct",
            b"\x015calibrate_delay_direct() failed to get a good estimate for loops_per_jiffy.\nProbably due to long platform interrupts. Consider using \"lpj=\" boot option.\n\0",
        );
    }
    0
}

#[cfg(not(CONFIG_ARCH_HAS_DELAY_TIMER))]
unsafe fn calibrate_delay_direct() -> c_ulong {
    // This is the original configuration branch: convergence follows when the
    // architecture has no delay timer, not a replacement for missing code.
    0
}

const LPS_PREC: u32 = 8;

unsafe fn calibrate_delay_converge() -> c_ulong {
    let mut lpj: c_ulong = 1 << 12;
    // These are C int counters, not unsigned longs. Cast only at the original
    // multiplication boundary and preserve the kernel's wrapping arithmetic.
    let (mut trials, mut band, mut trial_in_band): (c_int, c_int, c_int) = (0, 0, 0);
    let mut ticks = jiffies();
    while ticks == jiffies() {}
    ticks = jiffies();

    // SAFETY: __delay is the architecture's original calibrated busy wait.
    unsafe {
        loop {
            trial_in_band = trial_in_band.wrapping_add(1);
            if trial_in_band == (1 as c_int).wrapping_shl(band as u32) {
                band = band.wrapping_add(1);
                trial_in_band = 0;
            }
            bindings::__delay(lpj.wrapping_mul(band as c_ulong));
            trials = trials.wrapping_add(band);
            if ticks != jiffies() {
                break;
            }
        }
        trials = trials.wrapping_sub(band);
        let mut loopadd_base = lpj.wrapping_mul(band as c_ulong);
        let mut lpj_base = lpj.wrapping_mul(trials as c_ulong);

        loop {
            lpj = lpj_base;
            let mut loopadd = loopadd_base;
            let chop_limit = lpj >> LPS_PREC;
            while loopadd > chop_limit {
                lpj = lpj.wrapping_add(loopadd);
                ticks = jiffies();
                while ticks == jiffies() {}
                ticks = jiffies();
                bindings::__delay(lpj);
                if jiffies() != ticks {
                    lpj = lpj.wrapping_sub(loopadd);
                }
                loopadd >>= 1;
            }

            // If every refinement increment fitted, restart with a wider
            // interval, just like the original recalibrate label (SMI case).
            if lpj.wrapping_add(loopadd.wrapping_mul(2))
                == lpj_base.wrapping_add(loopadd_base.wrapping_mul(2))
            {
                lpj_base = lpj;
                loopadd_base = loopadd_base.wrapping_shl(2);
                continue;
            }
            return lpj;
        }
    }
}

/// Original architecture-overridable known-calibration default.
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn calibrate_delay_is_known() -> c_ulong {
    0
}

/// Original architecture-overridable calibration-complete notification.
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn calibration_delay_done() {}

/// Calibrate the current CPU and publish the resulting loops-per-jiffy value.
///
/// # Safety
/// The caller must provide the original boot/CPU-bringup calibration context:
/// a stable current CPU, working jiffies, and serialized calibration state.
#[no_mangle]
pub unsafe extern "C" fn calibrate_delay() {
    // SAFETY: the caller supplies the same CPU/timer context as calibrate.c.
    // The companion resolves smp_processor_id and per_cpu using native macros;
    // the returned slot belongs to this Rust object's permanent per-CPU data.
    unsafe {
        let cpu_lpj = bindings::rust_init_calibrate_cpu_lpj();
        let mut lpj = cpu_lpj.read();
        if lpj != 0 {
            if !PRINTED {
                main_printk!(
                    "calibrate_delay",
                    b"\x016Calibrating delay loop (skipped) already calibrated this CPU\0",
                );
            }
        } else if preset_lpj != 0 {
            lpj = preset_lpj;
            if !PRINTED {
                main_printk!(
                    "calibrate_delay",
                    b"\x016Calibrating delay loop (skipped) preset value.. \0",
                );
            }
        } else if !PRINTED && lpj_fine != 0 {
            lpj = lpj_fine;
            main_printk!(
                "calibrate_delay",
                b"\x016Calibrating delay loop (skipped), value calculated using timer frequency.. \0",
            );
        } else {
            // Use the external interfaces so architecture strong definitions
            // override the weak Rust defaults, including under LTO.
            lpj = bindings::calibrate_delay_is_known();
            if lpj == 0 {
                lpj = calibrate_delay_direct();
                if lpj != 0 {
                    if !PRINTED {
                        main_printk!(
                            "calibrate_delay",
                            b"\x016Calibrating delay using timer specific routine.. \0",
                        );
                    }
                } else {
                    if !PRINTED {
                        main_printk!("calibrate_delay", b"\x016Calibrating delay loop... \0");
                    }
                    lpj = calibrate_delay_converge();
                }
            }
        }
        cpu_lpj.write(lpj);
        if !PRINTED {
            main_printk!(
                "calibrate_delay",
                b"\x01c%lu.%02lu BogoMIPS (lpj=%lu)\n\0",
                lpj / (500000 / HZ),
                (lpj / (5000 / HZ)) % 100,
                lpj,
            );
        }
        bindings::loops_per_jiffy = lpj;
        PRINTED = true;
        bindings::calibration_delay_done();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
