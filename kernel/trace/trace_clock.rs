// SPDX-License-Identifier: GPL-2.0
/*
 * tracing clocks
 *
 *  Copyright (C) 2009 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
 *
 * Implements 3 trace clock variants, with differing scalability/precision
 * tradeoffs:
 *
 *  -   local: CPU-local trace clock
 *  -  medium: scalable global clock with some jitter
 *  -  global: globally monotonic, serialized clock
 *
 * Tracer plugins will chose a default from these clocks.
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn preempt_disable_notrace();
    fn preempt_enable_notrace();
    fn sched_clock() -> u64;
    fn local_clock() -> u64;
    fn jiffies_64_to_clock_t(value: u64) -> u64;
    fn sched_clock_cpu(cpu: i32) -> u64;
    fn raw_local_irq_save(flags: *mut usize);
    fn raw_local_irq_restore(flags: usize);
    fn raw_smp_processor_id() -> i32;
    fn in_nmi() -> bool;
    fn smp_rmb();
}

extern "C" {
    static mut jiffies_64: u64;
    static INITIAL_JIFFIES: u64;
}

#[repr(C)]
pub struct ArchSpinlock {
    _private: [u8; 0],
}

extern "C" {
    fn arch_spin_trylock(lock: *mut ArchSpinlock) -> bool;
    fn arch_spin_unlock(lock: *mut ArchSpinlock);
    fn atomic64_inc_return(counter: *mut i64) -> u64;
}

/*
 * trace_clock_local(): the simplest and least coherent tracing clock.
 *
 * Useful for tracing that does not cross to other CPUs nor
 * does it go through idle events.
 */
pub unsafe fn trace_clock_local() -> u64 {
    let clock: u64;

    /*
     * sched_clock() is an architecture implemented, fast, scalable,
     * lockless clock. It is not guaranteed to be coherent across
     * CPUs, nor across CPU idle events.
     */
    preempt_disable_notrace();
    clock = sched_clock();
    preempt_enable_notrace();

    clock
}

/*
 * trace_clock(): 'between' trace clock. Not completely serialized,
 * but not completely incorrect when crossing CPUs either.
 *
 * This is based on cpu_clock(), which will allow at most ~1 jiffy of
 * jitter between CPUs. So it's a pretty scalable clock, but there
 * can be offsets in the trace data.
 */
pub unsafe fn trace_clock() -> u64 {
    local_clock()
}

/*
 * trace_jiffy_clock(): Simply use jiffies as a clock counter.
 * Note that this use of jiffies_64 is not completely safe on
 * 32-bit systems. But the window is tiny, and the effect if
 * we are affected is that we will have an obviously bogus
 * timestamp on a trace event - i.e. not life threatening.
 */
pub unsafe fn trace_clock_jiffies() -> u64 {
    jiffies_64_to_clock_t(jiffies_64.wrapping_sub(INITIAL_JIFFIES))
}

/*
 * trace_clock_global(): special globally coherent trace clock
 *
 * It has higher overhead than the other trace clocks but is still
 * an order of magnitude faster than GTOD derived hardware clocks.
 *
 * Used by plugins that need globally coherent timestamps.
 */

/* keep prev_time and lock in the same cacheline. */
#[repr(C)]
struct TraceClockStruct {
    prev_time: u64,
    lock: ArchSpinlock,
}

static mut trace_clock_struct: TraceClockStruct = TraceClockStruct {
    prev_time: 0,
    lock: ArchSpinlock { _private: [] },
};

pub unsafe fn trace_clock_global() -> u64 {
    let mut flags: usize = 0;
    let this_cpu: i32;
    let mut now: u64;
    let mut prev_time: u64;

    raw_local_irq_save(&mut flags);

    this_cpu = raw_smp_processor_id();

    /*
     * The global clock "guarantees" that the events are ordered
     * between CPUs. But if two events on two different CPUS call
     * trace_clock_global at roughly the same time, it really does
     * not matter which one gets the earlier time. Just make sure
     * that the same CPU will always show a monotonic clock.
     *
     * Use a read memory barrier to get the latest written
     * time that was recorded.
     */
    smp_rmb();
    prev_time = core::ptr::read_volatile(&trace_clock_struct.prev_time);
    now = sched_clock_cpu(this_cpu);

    /* Make sure that now is always greater than or equal to prev_time */
    if ((now.wrapping_sub(prev_time) as i64) < 0) {
        now = prev_time;
    }

    /*
     * If in an NMI context then dont risk lockups and simply return
     * the current time.
     */
    if in_nmi() {
        raw_local_irq_restore(flags);
        return now;
    }

    /* Tracing can cause strange recursion, always use a try lock */
    if arch_spin_trylock(&mut trace_clock_struct.lock) {
        /* Reread prev_time in case it was already updated */
        prev_time = core::ptr::read_volatile(&trace_clock_struct.prev_time);
        if (now.wrapping_sub(prev_time) as i64) < 0 {
            now = prev_time;
        }

        trace_clock_struct.prev_time = now;

        /* The unlock acts as the wmb for the above rmb */
        arch_spin_unlock(&mut trace_clock_struct.lock);
    }

    raw_local_irq_restore(flags);

    now
}

static mut trace_counter: i64 = 0;

/*
 * trace_clock_counter(): simply an atomic counter.
 * Use the trace_counter "counter" for cases where you do not care
 * about timings, but are interested in strict ordering.
 */
pub unsafe fn trace_clock_counter() -> u64 {
    atomic64_inc_return(&mut trace_counter)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
