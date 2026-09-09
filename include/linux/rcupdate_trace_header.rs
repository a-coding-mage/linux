/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Read-Copy Update mechanism for mutual exclusion, adapted for tracing.
 *
 * Copyright (C) 2020 Paul E. McKenney.
 */

// #include <linux/sched.h>
// #include <linux/rcupdate.h>
// #include <linux/cleanup.h>

#[cfg(CONFIG_TASKS_TRACE_RCU)]
extern "C" {
    pub static mut rcu_tasks_trace_srcu_struct: srcu_struct;
}

#[cfg(all(CONFIG_DEBUG_LOCK_ALLOC, CONFIG_TASKS_TRACE_RCU))]
#[inline]
pub unsafe fn rcu_read_lock_trace_held() -> i32 {
    srcu_read_lock_held(&rcu_tasks_trace_srcu_struct)
}

#[cfg(not(all(CONFIG_DEBUG_LOCK_ALLOC, CONFIG_TASKS_TRACE_RCU)))]
#[inline]
pub unsafe fn rcu_read_lock_trace_held() -> i32 {
    1
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
#[inline]
pub unsafe fn rcu_read_lock_tasks_trace() -> *mut srcu_ctr {
    let ret = __srcu_read_lock_fast(&mut rcu_tasks_trace_srcu_struct);

    rcu_try_lock_acquire(&mut (*(&mut rcu_tasks_trace_srcu_struct as *mut srcu_struct)).dep_map);
    if !cfg!(CONFIG_TASKS_TRACE_RCU_NO_MB) {
        smp_mb(); // Provide ordering on noinstr-incomplete architectures.
    }
    ret
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
#[inline]
pub unsafe fn rcu_read_unlock_tasks_trace(scp: *mut srcu_ctr) {
    if !cfg!(CONFIG_TASKS_TRACE_RCU_NO_MB) {
        smp_mb(); // Provide ordering on noinstr-incomplete architectures.
    }
    __srcu_read_unlock_fast(&mut rcu_tasks_trace_srcu_struct, scp);
    srcu_lock_release(&mut rcu_tasks_trace_srcu_struct.dep_map);
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
#[inline]
pub unsafe fn rcu_read_lock_trace() {
    let mut n: i32;
    let t: *mut task_struct = current;

    rcu_try_lock_acquire(&mut rcu_tasks_trace_srcu_struct.dep_map);
    n = READ_ONCE((*t).trc_reader_nesting);
    WRITE_ONCE(&mut (*t).trc_reader_nesting, n + 1);
    if n != 0 {
        // In case we interrupted a Tasks Trace RCU reader.
        return;
    }
    barrier(); // nesting before scp to protect against interrupt handler.
    (*t).trc_reader_scp = __srcu_read_lock_fast(&mut rcu_tasks_trace_srcu_struct);
    if !cfg!(CONFIG_TASKS_TRACE_RCU_NO_MB) {
        smp_mb(); // Placeholder for more selective ordering
    }
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
#[inline]
pub unsafe fn rcu_read_unlock_trace() {
    let n: i32;
    let scp: *mut srcu_ctr;
    let t: *mut task_struct = current;

    n = READ_ONCE((*t).trc_reader_nesting) - 1;
    if n != 0 {
        WRITE_ONCE(&mut (*t).trc_reader_nesting, n);
    } else {
        scp = (*t).trc_reader_scp; // Compiler cannot hoist load due to data raciness.
        barrier(); // scp before nesting to protect against interrupt handler.
        WRITE_ONCE(&mut (*t).trc_reader_nesting, n);
        if !cfg!(CONFIG_TASKS_TRACE_RCU_NO_MB) {
            smp_mb(); // Placeholder for more selective ordering
        }
        __srcu_read_unlock_fast(&mut rcu_tasks_trace_srcu_struct, scp);
    }
    srcu_lock_release(&mut rcu_tasks_trace_srcu_struct.dep_map);
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
#[inline]
pub unsafe fn call_rcu_tasks_trace(rhp: *mut rcu_head, func: rcu_callback_t) {
    call_srcu(&mut rcu_tasks_trace_srcu_struct, rhp, func);
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
#[inline]
pub unsafe fn synchronize_rcu_tasks_trace() {
    synchronize_srcu(&mut rcu_tasks_trace_srcu_struct);
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
#[inline]
pub unsafe fn rcu_barrier_tasks_trace() {
    srcu_barrier(&mut rcu_tasks_trace_srcu_struct);
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
#[inline]
pub unsafe fn rcu_tasks_trace_expedite_current() {
    srcu_expedite_current(&mut rcu_tasks_trace_srcu_struct);
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
extern "C" {
    pub fn rcu_tasks_trace_batches_completed() -> c_ulong;
    // Placeholders to enable stepwise transition.
    pub fn rcu_tasks_trace_suppress_unused();
}

#[cfg(not(CONFIG_TASKS_TRACE_RCU))]
#[inline]
pub unsafe fn rcu_tasks_trace_batches_completed() -> c_ulong { 0 }

/*
 * The BPF JIT forms these addresses even when it doesn't call these
 * functions, so provide definitions that result in runtime errors.
 */
#[cfg(not(CONFIG_TASKS_TRACE_RCU))]
#[inline]
pub unsafe fn call_rcu_tasks_trace(_rhp: *mut rcu_head, _func: rcu_callback_t) { BUG(); }
#[cfg(not(CONFIG_TASKS_TRACE_RCU))]
#[inline]
pub unsafe fn rcu_read_lock_trace() { BUG(); }
#[cfg(not(CONFIG_TASKS_TRACE_RCU))]
#[inline]
pub unsafe fn rcu_read_unlock_trace() { BUG(); }

// DEFINE_LOCK_GUARD_0(rcu_tasks_trace,
//     rcu_read_lock_trace(),
//     rcu_read_unlock_trace())

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
