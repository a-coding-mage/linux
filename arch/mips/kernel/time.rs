// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2001 MontaVista Software Inc.
 * Author: Jun Sun, jsun@mvista.com or jsun@junsun.net
 * Copyright (c) 2003, 2004  Maciej W. Rozycki
 *
 * Common time service routines for MIPS machines.
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut PCP_LPJ_REF: *mut ::core::ffi::c_ulong = core::ptr::null_mut();
#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut PCP_LPJ_REF_FREQ: *mut ::core::ffi::c_ulong = core::ptr::null_mut();
#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut GLB_LPJ_REF: ::core::ffi::c_ulong = 0;
#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut GLB_LPJ_REF_FREQ: ::core::ffi::c_ulong = 0;

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn cpufreq_callback(
    _nb: *mut notifier_block,
    val: ::core::ffi::c_ulong,
    data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let freq = data as *mut cpufreq_freqs;
    let cpus = (*(*freq).policy).cpus;
    let mut lpj: ::core::ffi::c_ulong;
    let mut cpu: ::core::ffi::c_int;

    /*
     * Skip lpj numbers adjustment if the CPU-freq transition is safe for
     * the loops delay. (Is this possible?)
     */
    if (*freq).flags & CPUFREQ_CONST_LOOPS != 0 {
        return NOTIFY_OK;
    }

    /* Save the initial values of the lpjes for future scaling. */
    if GLB_LPJ_REF == 0 {
        GLB_LPJ_REF = boot_cpu_data.udelay_val as ::core::ffi::c_ulong;
        GLB_LPJ_REF_FREQ = (*freq).old;

        for_each_online_cpu!(cpu) {
            *PCP_LPJ_REF.add(cpu as usize) = cpu_data[cpu as usize].udelay_val as _;
            *PCP_LPJ_REF_FREQ.add(cpu as usize) = (*freq).old;
        }
    }

    /*
     * Adjust global lpj variable and per-CPU udelay_val number in
     * accordance with the new CPU frequency.
     */
    if (val == CPUFREQ_PRECHANGE && (*freq).old < (*freq).new)
        || (val == CPUFREQ_POSTCHANGE && (*freq).old > (*freq).new)
    {
        loops_per_jiffy = cpufreq_scale(GLB_LPJ_REF, GLB_LPJ_REF_FREQ, (*freq).new);

        for_each_cpu!(cpu, cpus) {
            lpj = cpufreq_scale(
                *PCP_LPJ_REF.add(cpu as usize),
                *PCP_LPJ_REF_FREQ.add(cpu as usize),
                (*freq).new,
            );
            cpu_data[cpu as usize].udelay_val = lpj as ::core::ffi::c_uint;
        }
    }

    NOTIFY_OK
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut CPUFREQ_NOTIFIER: notifier_block = notifier_block {
    notifier_call: Some(cpufreq_callback),
};

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn register_cpufreq_notifier() -> ::core::ffi::c_int {
    cpufreq_register_notifier(&mut CPUFREQ_NOTIFIER, CPUFREQ_TRANSITION_NOTIFIER)
}

// core_initcall(register_cpufreq_notifier)

/* forward reference */
pub static mut rtc_lock: spinlock_t = spinlock_t::new();

unsafe fn null_perf_irq() -> ::core::ffi::c_int {
    0
}

pub static mut perf_irq: unsafe fn() -> ::core::ffi::c_int = null_perf_irq;

/*
 * time_init() - it does the following things.
 *
 * 1) plat_time_init() -
 *      a) (optional) set up RTC routines,
 *      b) (optional) calibrate and set the mips_hpt_frequency
 *         (only needed if you intended to use cpu counter as timer interrupt
 *          source)
 * 2) calculate a couple of cached variables for later usage
 */

pub static mut mips_hpt_frequency: ::core::ffi::c_uint = 0;

unsafe fn cpu_has_mfc0_count_bug() -> bool {
    match current_cpu_type() {
        CPU_R4000PC | CPU_R4000SC | CPU_R4000MC => {
            /*
             * V3.0 is documented as suffering from the mfc0 from count bug.
             * Afaik this is the last version of the R4000. Later versions
             * were marketed as R4400.
             */
            true
        }
        CPU_R4400PC | CPU_R4400SC | CPU_R4400MC => {
            /*
             * The published errata for the R4400 up to 3.0 say the CPU
             * has the mfc0 from count bug. This seems the last version
             * produced.
             */
            true
        }
        _ => false,
    }
}

pub unsafe fn time_init() {
    plat_time_init();

    /*
     * The use of the R4k timer as a clock event takes precedence;
     * if reading the Count register might interfere with the timer
     * interrupt, then we don't use the timer as a clock source.
     * We may still use the timer as a clock source though if the
     * timer interrupt isn't reliable; the interference doesn't
     * matter then, because we don't use the interrupt.
     */
    if mips_clockevent_init() != 0 || !cpu_has_mfc0_count_bug() {
        init_mips_clocksource();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
