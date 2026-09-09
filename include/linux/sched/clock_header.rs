/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/smp.h

/*
 * Do not use outside of architecture code which knows its limitations.
 *
 * sched_clock() has no promise of monotonicity or bounded drift between
 * CPUs, use (which you should not) requires disabling IRQs.
 *
 * Please use one of the three interfaces below.
 */
unsafe extern "C" {
    pub fn sched_clock() -> u64;
}

#[cfg(any(CONFIG_ARCH_WANTS_NO_INSTR, CONFIG_GENERIC_SCHED_CLOCK))]
unsafe extern "C" {
    pub fn sched_clock_noinstr() -> u64;
}

#[cfg(not(any(CONFIG_ARCH_WANTS_NO_INSTR, CONFIG_GENERIC_SCHED_CLOCK)))]
#[inline(always)]
pub unsafe fn sched_clock_noinstr() -> u64 {
    unsafe { sched_clock() }
}

/*
 * See the comment in kernel/sched/clock.c
 */
unsafe extern "C" {
    pub fn running_clock() -> u64;
    pub fn sched_clock_cpu(cpu: i32) -> u64;
    pub fn sched_clock_init();
}

#[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
#[inline]
pub fn sched_clock_stable() -> i32 {
    1
}

#[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
#[inline]
pub fn sched_clock_tick() {}

#[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
#[inline]
pub fn clear_sched_clock_stable() {}

#[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
#[inline]
pub fn sched_clock_idle_sleep_event() {}

#[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
#[inline]
pub fn sched_clock_idle_wakeup_event() {}

#[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
#[inline]
pub unsafe fn cpu_clock(_cpu: i32) -> u64 {
    unsafe { sched_clock() }
}

#[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
#[inline(always)]
pub unsafe fn local_clock_noinstr() -> u64 {
    unsafe { sched_clock_noinstr() }
}

#[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
#[inline(always)]
pub unsafe fn local_clock() -> u64 {
    unsafe { sched_clock() }
}

#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
unsafe extern "C" {
    pub fn sched_clock_stable() -> i32;
    pub fn clear_sched_clock_stable();
}

#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
/*
 * When sched_clock_stable(), __sched_clock_offset provides the offset
 * between local_clock() and sched_clock().
 */
unsafe extern "C" {
    pub static mut __sched_clock_offset: u64;
    pub fn sched_clock_tick();
    pub fn sched_clock_tick_stable();
    pub fn sched_clock_idle_sleep_event();
    pub fn sched_clock_idle_wakeup_event();
}

/*
 * As outlined in clock.c, provides a fast, high resolution, nanosecond
 * time source that is monotonic per cpu argument and has bounded drift
 * between cpus.
 *
 * ######################### BIG FAT WARNING ##########################
 * # when comparing cpu_clock(i) to cpu_clock(j) for i != j, time can #
 * # go backwards !!                                                  #
 * ####################################################################
 */
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[inline]
pub unsafe fn cpu_clock(cpu: i32) -> u64 {
    unsafe { sched_clock_cpu(cpu) }
}

#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
unsafe extern "C" {
    pub fn local_clock_noinstr() -> u64;
    pub fn local_clock() -> u64;
}

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
/*
 * An i/f to runtime opt-in for irq time accounting based off of sched_clock.
 * The reason for this explicit opt-in is not to have perf penalty with
 * slow sched_clocks.
 */
unsafe extern "C" {
    pub fn enable_sched_clock_irqtime();
    pub fn disable_sched_clock_irqtime();
}

#[cfg(not(CONFIG_IRQ_TIME_ACCOUNTING))]
#[inline]
pub fn enable_sched_clock_irqtime() {}

#[cfg(not(CONFIG_IRQ_TIME_ACCOUNTING))]
#[inline]
pub fn disable_sched_clock_irqtime() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
