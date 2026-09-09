// SPDX-License-Identifier: GPL-2.0

// Linux kernel dependencies supplied by the surrounding build.

#[repr(C)]
pub struct cpumask_t {
    _private: [u8; 0],
}

extern "C" {
    static mut watchdog_hardlockup_miss_thresh: ::core::ffi::c_uint;
    static nr_cpu_ids: ::core::ffi::c_uint;

    fn cpumask_next_wrap(cpu: ::core::ffi::c_uint, mask: *const cpumask_t) -> ::core::ffi::c_uint;
    fn cpumask_set_cpu(cpu: ::core::ffi::c_uint, mask: *mut cpumask_t);
    fn cpumask_clear_cpu(cpu: ::core::ffi::c_uint, mask: *mut cpumask_t);
    fn watchdog_hardlockup_touch_cpu(cpu: ::core::ffi::c_uint);
    fn smp_processor_id() -> ::core::ffi::c_uint;
    fn watchdog_hardlockup_check(cpu: ::core::ffi::c_uint, arg: *mut ::core::ffi::c_void);
}

static mut watchdog_cpus: cpumask_t = cpumask_t { _private: [] };

unsafe fn watchdog_next_cpu(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let next_cpu: ::core::ffi::c_uint;

    next_cpu = cpumask_next_wrap(cpu, &raw const watchdog_cpus);
    if next_cpu == cpu {
        return nr_cpu_ids;
    }

    next_cpu
}

pub unsafe fn watchdog_hardlockup_probe() -> ::core::ffi::c_int {
    watchdog_hardlockup_miss_thresh = 3;
    0
}

pub unsafe fn watchdog_hardlockup_enable(cpu: ::core::ffi::c_uint) {
    let next_cpu: ::core::ffi::c_uint;

    /*
     * The new CPU will be marked online before the hrtimer interrupt
     * gets a chance to run on it. If another CPU tests for a
     * hardlockup on the new CPU before it has run its the hrtimer
     * interrupt, it will get a false positive. Touch the watchdog on
     * the new CPU to delay the check for at least 3 sampling periods
     * to guarantee one hrtimer has run on the new CPU.
     */
    watchdog_hardlockup_touch_cpu(cpu);

    /*
     * We are going to check the next CPU. Our watchdog_hrtimer
     * need not be zero if the CPU has already been online earlier.
     * Touch the watchdog on the next CPU to avoid false positive
     * if we try to check it in less then 3 interrupts.
     */
    next_cpu = watchdog_next_cpu(cpu);
    if next_cpu < nr_cpu_ids {
        watchdog_hardlockup_touch_cpu(next_cpu);
    }

    /*
     * Makes sure that watchdog is touched on this CPU before
     * other CPUs could see it in watchdog_cpus. The counter
     * part is in watchdog_buddy_check_hardlockup().
     */
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);

    cpumask_set_cpu(cpu, &raw mut watchdog_cpus);
}

pub unsafe fn watchdog_hardlockup_disable(cpu: ::core::ffi::c_uint) {
    let next_cpu: ::core::ffi::c_uint = watchdog_next_cpu(cpu);

    /*
     * Offlining this CPU will cause the CPU before this one to start
     * checking the one after this one. If this CPU just finished checking
     * the next CPU and updating hrtimer_interrupts_saved, and then the
     * previous CPU checks it within one sample period, it will trigger
     * a false positive. Touch the watchdog on the next CPU to prevent it.
     */
    if next_cpu < nr_cpu_ids {
        watchdog_hardlockup_touch_cpu(next_cpu);
    }

    /*
     * Makes sure that watchdog is touched on the next CPU before
     * this CPU disappear in watchdog_cpus. The counter part is in
     * watchdog_buddy_check_hardlockup().
     */
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);

    cpumask_clear_cpu(cpu, &raw mut watchdog_cpus);
}

pub unsafe fn watchdog_buddy_check_hardlockup(hrtimer_interrupts: ::core::ffi::c_int) {
    let next_cpu: ::core::ffi::c_uint;

    /* check for a hardlockup on the next CPU */
    next_cpu = watchdog_next_cpu(smp_processor_id());
    if next_cpu >= nr_cpu_ids {
        return;
    }

    /*
     * Make sure that the watchdog was touched on next CPU when
     * watchdog_next_cpu() returned another one because of
     * a change in watchdog_hardlockup_enable()/disable().
     */
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);

    watchdog_hardlockup_check(next_cpu, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
