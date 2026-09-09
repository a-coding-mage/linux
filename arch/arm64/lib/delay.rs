// SPDX-License-Identifier: GPL-2.0-only
/*
 * Delay loops based on the OpenRISC implementation.
 *
 * Copyright (C) 2012 ARM Limited
 *
 * Author: Will Deacon <will.deacon@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, as in the original C includes.

#[inline]
unsafe fn xloops_to_cycles(xloops: u64) -> u64 {
    ((xloops.wrapping_mul(loops_per_jiffy)).wrapping_mul(HZ)) >> 32
}

#[inline]
unsafe fn usecs_to_cycles(time_usecs: u64) -> u64 {
    xloops_to_cycles(time_usecs.wrapping_mul(0x10C7u64))
}

/*
 * Force the use of CNTVCT_EL0 in order to have the same base as WFxT.
 * This avoids some annoying issues when CNTVOFF_EL2 is not reset 0 on a
 * KVM host running at EL1 until we do a vcpu_put() on the vcpu. When
 * running at EL2, the effective offset is always 0.
 *
 * Note that userspace cannot change the offset behind our back either,
 * as the vcpu mutex is held as long as KVM_RUN is in progress.
 */
unsafe fn __delay_cycles() -> u64 {
    // C's guard(preempt_notrace)() scope guard is supplied by the surrounding kernel.
    let _preempt_guard = guard_preempt_notrace();
    __arch_counter_get_cntvct_stable()
}

pub unsafe fn __delay(cycles: u64) {
    let start = __delay_cycles();

    if alternative_has_cap_unlikely(ARM64_HAS_WFXT) {
        let end = start.wrapping_add(cycles);

        /*
         * Start with WFIT. If an interrupt makes us resume
         * early, use a WFET loop to complete the delay.
         */
        wfit(end);
        while (__delay_cycles().wrapping_sub(start)) < cycles {
            wfet(end);
        }
    } else if arch_timer_evtstrm_available() {
        let timer_evt_period = usecs_to_cycles(ARCH_TIMER_EVT_STREAM_PERIOD_US);

        while (__delay_cycles()
            .wrapping_sub(start)
            .wrapping_add(timer_evt_period)
            < cycles)
        {
            wfe();
        }
    }

    while (__delay_cycles().wrapping_sub(start)) < cycles {
        cpu_relax();
    }
}

#[inline]
pub unsafe fn __const_udelay(xloops: u64) {
    __delay(xloops_to_cycles(xloops));
}

pub unsafe fn __udelay(usecs: u64) {
    __const_udelay(usecs.wrapping_mul(0x10C7u64)); // 2**32 / 1000000 (rounded up)
}

pub unsafe fn __ndelay(nsecs: u64) {
    __const_udelay(nsecs.wrapping_mul(0x5u64)); // 2**32 / 1000000000 (rounded up)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
