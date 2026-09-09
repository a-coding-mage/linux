/* SPDX-License-Identifier: GPL-2.0 */
/*
 * x86 TSC related functions
 */

use core::ffi::c_char;

/**
 * rdtsc() - returns the current TSC without ordering constraints
 *
 * rdtsc() returns the result of RDTSC as a 64-bit integer.  The
 * only ordering constraint it supplies is the ordering implied by
 * "asm volatile": it will put the RDTSC in the place you expect.  The
 * CPU can and will speculatively execute that RDTSC, though, so the
 * results can be non-monotonic if compared on different CPUs.
 */
#[inline(always)]
pub unsafe fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;
    core::arch::asm!("rdtsc", out("eax") low, out("edx") high);
    ((high as u64) << 32) | low as u64
}

/**
 * rdtsc_ordered() - read the current TSC in program order
 *
 * rdtsc_ordered() returns the result of RDTSC as a 64-bit integer.
 * It is ordered like a load to a global in-memory counter.  It should
 * be impossible to observe non-monotonic rdtsc_unordered() behavior
 * across multiple CPUs as long as the TSC is synced.
 */
#[inline(always)]
pub unsafe fn rdtsc_ordered() -> u64 {
    let low: u32;
    let high: u32;

    /*
     * The C implementation selects among "rdtsc", "lfence; rdtsc",
     * and "rdtscp" using the architecture's alternative instruction
     * mechanism (X86_FEATURE_LFENCE_RDTSC and X86_FEATURE_RDTSCP).
     * Preserve the preferred ordered RDTSCP operation here; the
     * surrounding build may provide an equivalent alternatives pass.
     */
    core::arch::asm!(
        "rdtscp",
        out("eax") low,
        out("edx") high,
        lateout("ecx") _,
    );

    ((high as u64) << 32) | low as u64
}

/* Standard way to access the cycle counter. */
pub type cycles_t = u64;

unsafe extern "C" {
    pub static mut cpu_khz: u32;
    pub static mut tsc_khz: u32;

    pub fn disable_TSC();

    pub fn tsc_early_init();
    pub fn tsc_init();
    pub fn mark_tsc_unstable(reason: *mut c_char);
    pub fn unsynchronized_tsc() -> i32;
    pub fn check_tsc_unstable() -> i32;
    pub fn mark_tsc_async_resets(reason: *mut c_char);
    pub fn native_calibrate_cpu_early() -> c_ulong;
    pub fn native_calibrate_tsc() -> c_ulong;
    pub fn native_sched_clock_from_tsc(tsc: u64) -> u64;

    pub static mut tsc_clocksource_reliable: i32;
    pub static mut tsc_async_resets: bool;

    /*
     * Boot-time check whether the TSCs are synchronized across
     * all CPUs/cores:
     */
    pub fn tsc_store_and_check_tsc_adjust(bootcpu: bool) -> bool;
    pub fn tsc_verify_tsc_adjust(resume: bool);
    pub fn check_tsc_sync_target();

    pub fn notsc_setup(arg: *mut c_char) -> i32;
    pub fn tsc_save_sched_clock_state();
    pub fn tsc_restore_sched_clock_state();

    pub fn cpu_khz_from_msr() -> c_ulong;
}

use core::ffi::c_ulong;

#[inline]
pub unsafe fn get_cycles() -> cycles_t {
    if !cpu_feature_enabled(X86_FEATURE_TSC) {
        return 0;
    }
    rdtsc()
}

/* C macro: #define get_cycles get_cycles */

/* Supplied by the CPU feature implementation included by the original header. */
unsafe extern "C" {
    fn cpu_feature_enabled(feature: i32) -> bool;
    static X86_FEATURE_TSC: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
