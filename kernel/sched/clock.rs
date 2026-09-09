// SPDX-License-Identifier: GPL-2.0-only
/*
 * sched_clock() for unstable CPU clocks
 *
 *  Copyright (C) 2008 Red Hat, Inc., Peter Zijlstra
 *
 *  Updates and enhancements:
 *    Copyright (C) 2008 Red Hat, Inc. Steven Rostedt <srostedt@redhat.com>
 *
 * Based on code by:
 *   Ingo Molnar <mingo@redhat.com>
 *   Guillaume Chazarain <guichaz@gmail.com>
 */

// Kernel dependencies supplied by other translation units are intentionally
// referenced by their Rust-side names below.

#[no_mangle]
pub unsafe extern "C" fn sched_clock() -> u64 {
    (jiffies.wrapping_sub(INITIAL_JIFFIES) as u64)
        .wrapping_mul(NSEC_PER_SEC / HZ)
}

extern "C" {
    static mut jiffies: usize;
    static INITIAL_JIFFIES: usize;
    static NSEC_PER_SEC: u64;
    static HZ: u64;
    static TICK_NSEC: u64;
    static mut timekeeping_suspended: bool;
    fn sched_clock_noinstr() -> u64;
    fn ktime_get_ns() -> u64;
    fn generic_sched_clock_init();
    fn smp_processor_id() -> i32;
    fn local_irq_disable();
    fn local_irq_enable();
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn preempt_disable();
    fn preempt_enable();
    fn preempt_disable_notrace();
    fn preempt_enable_notrace();
    fn disable_sched_clock_irqtime();
    fn tick_dep_set(x: i32);
    fn tick_dep_clear(x: i32);
    fn schedule_work(work: *mut work_struct);
    fn printk(fmt: *const u8, ...);
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[repr(C)]
pub struct sched_clock_data {
    pub tick_raw: u64,
    pub tick_gtod: u64,
    pub clock: u64,
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
static mut sched_clock_running: bool = false;

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
static mut __sched_clock_stable: bool = false;
#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
static mut __sched_clock_stable_early: i32 = 1;
#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub static mut __sched_clock_offset: u64 = 0;
#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
static mut __gtod_offset: u64 = 0;

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
static mut sched_clock_data: sched_clock_data = sched_clock_data {
    tick_raw: 0,
    tick_gtod: 0,
    clock: 0,
};

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn this_scd() -> *mut sched_clock_data {
    &raw mut sched_clock_data
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn cpu_sdc(_cpu: i32) -> *mut sched_clock_data {
    &raw mut sched_clock_data
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_stable() -> i32 {
    __sched_clock_stable as i32
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn __scd_stamp(scd: *mut sched_clock_data) {
    (*scd).tick_gtod = ktime_get_ns();
    (*scd).tick_raw = sched_clock();
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn __set_sched_clock_stable() {
    let scd = this_scd();
    local_irq_disable();
    __sched_clock_offset = ((*scd).tick_gtod + __gtod_offset).wrapping_sub((*scd).tick_raw);
    local_irq_enable();
    __sched_clock_stable = true;
    tick_dep_clear(0);
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn __sched_clock_work(_work: *mut work_struct) {
    let scd: *mut sched_clock_data;
    preempt_disable();
    scd = this_scd();
    __scd_stamp(scd);
    (*scd).clock = (*scd).tick_gtod + __gtod_offset;
    preempt_enable();
    disable_sched_clock_irqtime();
    __sched_clock_stable = false;
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn __clear_sched_clock_stable() {
    if !sched_clock_stable() != 0 { return; }
    tick_dep_set(0);
    __sched_clock_work(core::ptr::null_mut());
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub unsafe extern "C" fn clear_sched_clock_stable() {
    __sched_clock_stable_early = 0;
    __clear_sched_clock_stable();
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn __sched_clock_gtod_offset() {
    let scd = this_scd();
    __scd_stamp(scd);
    __gtod_offset = ((*scd).tick_raw + __sched_clock_offset).wrapping_sub((*scd).tick_gtod);
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_init() {
    local_irq_disable();
    __sched_clock_gtod_offset();
    local_irq_enable();
    sched_clock_running = true;
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn wrap_min(x: u64, y: u64) -> u64 {
    if (x.wrapping_sub(y) as i64) < 0 { x } else { y }
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn wrap_max(x: u64, y: u64) -> u64 {
    if (x.wrapping_sub(y) as i64) > 0 { x } else { y }
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn sched_clock_local(scd: *mut sched_clock_data) -> u64 {
    loop {
        let now = sched_clock_noinstr();
        let mut delta = now.wrapping_sub((*scd).tick_raw) as i64;
        if delta < 0 { delta = 0; }
        let old_clock = (*scd).clock;
        let gtod = (*scd).tick_gtod + __gtod_offset;
        let mut clock = gtod + delta as u64;
        let min_clock = wrap_max(gtod, old_clock);
        let max_clock = wrap_max(old_clock, gtod + TICK_NSEC);
        clock = wrap_max(clock, min_clock);
        clock = wrap_min(clock, max_clock);
        if (*scd).clock == old_clock {
            (*scd).clock = clock;
            return clock;
        }
    }
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub unsafe extern "C" fn local_clock_noinstr() -> u64 {
    if __sched_clock_stable { return sched_clock_noinstr() + __sched_clock_offset; }
    if !sched_clock_running { return sched_clock_noinstr(); }
    sched_clock_local(this_scd())
}

#[no_mangle]
pub unsafe extern "C" fn local_clock() -> u64 {
    preempt_disable_notrace();
    let now = local_clock_noinstr();
    preempt_enable_notrace();
    now
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
unsafe fn sched_clock_remote(scd: *mut sched_clock_data) -> u64 {
    let my_scd = this_scd();
    sched_clock_local(my_scd);
    let this_clock = (*my_scd).clock;
    let remote_clock = (*scd).clock;
    let val = if (remote_clock.wrapping_sub(this_clock) as i64) < 0 {
        this_clock
    } else { remote_clock };
    if (remote_clock.wrapping_sub(this_clock) as i64) < 0 { (*scd).clock = val; }
    else { (*my_scd).clock = val; }
    val
}

#[no_mangle]
pub unsafe extern "C" fn sched_clock_cpu(cpu: i32) -> u64 {
    #[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
    {
        if sched_clock_stable() != 0 { return sched_clock() + __sched_clock_offset; }
        if !sched_clock_running { return sched_clock(); }
        preempt_disable_notrace();
        let scd = cpu_sdc(cpu);
        let clock = if cpu != smp_processor_id() { sched_clock_remote(scd) } else { sched_clock_local(scd) };
        preempt_enable_notrace();
        return clock;
    }
    #[cfg(not(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK"))]
    { let _ = cpu; sched_clock() }
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_tick() {
    if __sched_clock_stable || !sched_clock_running { return; }
    let scd = this_scd();
    __scd_stamp(scd);
    sched_clock_local(scd);
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_tick_stable() {
    if !__sched_clock_stable { return; }
    local_irq_disable();
    __sched_clock_gtod_offset();
    local_irq_enable();
}

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_idle_sleep_event() { sched_clock_cpu(smp_processor_id()); }

#[cfg(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK")]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_idle_wakeup_event() {
    if __sched_clock_stable || timekeeping_suspended { return; }
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    sched_clock_tick();
    local_irq_restore(flags);
}

#[no_mangle]
pub unsafe extern "C" fn running_clock() -> u64 { local_clock() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
