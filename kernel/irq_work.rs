// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 Red Hat, Inc., Peter Zijlstra
 *
 * Provides a framework for enqueueing and running callbacks from hardirq
 * context. The enqueueing is NMI-safe.
 */
// Continued from Rust blob ccdfc46cfcf0e8d6d8e9cf0db984108b9ec14291.
// Native oracle: irq_work.c at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
#![no_std]

compile_error!("SOURCE ONLY HOLD: IRQ work is not admitted");

// Proposed configured native declarations; no projected kernel layouts.
use kernel::bindings::irq_work_native::*;
use kernel::ffi::{c_int, c_uint, c_void};

unsafe fn wake_irq_workd() {
    // SAFETY: Original caller's CPU-local context protects the per-CPU lookup.
    // The task slot is the one owned and populated by the hotplug descriptor.
    unsafe {
        let tsk = lupos_irq_work_this_task();

        if !lupos_irq_work_list_empty(lupos_irq_work_this_lazy()) && !tsk.is_null() {
            lupos_irq_work_wake_process(tsk);
        }
    }
}

#[cfg(CONFIG_SMP)]
#[export_name = "lupos_irq_work_wake"]
unsafe extern "C" fn irq_work_wake(_entry: *mut irq_work) {
    // SAFETY: Invoked by the native per-CPU hard IRQ-work callback descriptor.
    unsafe { wake_irq_workd() };
}

#[export_name = "lupos_irq_workd_should_run"]
unsafe extern "C" fn irq_workd_should_run(_cpu: c_uint) -> c_int {
    // SAFETY: smpboot calls this predicate with preemption disabled.
    unsafe { (!lupos_irq_work_list_empty(lupos_irq_work_this_lazy())) as c_int }
}

/* Claim the entry so that no one else will poke at it. */
unsafe fn irq_work_claim(work: *mut irq_work) -> bool {
    // SAFETY: The caller retains the live native work object. Use the exact
    // unsuffixed kernel atomic, without borrowing shared atomic storage.
    unsafe {
        let oflags = lupos_irq_work_atomic_fetch_or(
            (IRQ_WORK_CLAIMED | CSD_TYPE_IRQ_WORK) as c_int,
            work,
        );
        /*
         * If the work is already pending, no need to raise the IPI.
         * The pairing smp_mb() in irq_work_single() makes sure
         * everything we did before is visible.
         */
        if oflags & IRQ_WORK_PENDING as c_int != 0 {
            return false;
        }
        true
    }
}

#[inline(always)]
unsafe fn irq_work_raise(work: *mut irq_work) {
    // SAFETY: The work is claimed and the caller has disabled preemption.
    // arch_irq_work_raise is the native external symbol, with a weak fallback
    // that a strong architecture implementation can replace at link time.
    unsafe {
        if lupos_irq_work_trace_enabled() && lupos_irq_work_has_interrupt() {
            lupos_irq_work_trace_send(work);
        }
        arch_irq_work_raise();
    }
}

/* Enqueue on current CPU, work must already be claimed and preempt disabled */
unsafe fn __irq_work_queue_local(work: *mut irq_work) {
    // SAFETY: Claiming owns the pending entry; preemption keeps these native
    // per-CPU addresses stable. List insertion uses the native llist primitive.
    unsafe {
        let list: *mut llist_head;
        let mut rt_lazy_work = false;
        let mut lazy_work = false;

        let work_flags = lupos_irq_work_atomic_read(work);
        if work_flags & IRQ_WORK_LAZY as c_int != 0 {
            lazy_work = true;
        } else if cfg!(CONFIG_PREEMPT_RT) && work_flags & IRQ_WORK_HARD_IRQ as c_int == 0 {
            rt_lazy_work = true;
        }

        if lazy_work || rt_lazy_work {
            list = lupos_irq_work_this_lazy();
        } else {
            list = lupos_irq_work_this_raised();
        }

        if !lupos_irq_work_list_add(work, list) {
            return;
        }

        /* If the work is "lazy", handle it from next tick if any */
        if !lazy_work || lupos_irq_work_tick_stopped() {
            irq_work_raise(work);
        }
    }
}

/// Enqueue work on the current CPU if it is not already pending.
///
/// # Safety
/// Work and its callable callback obey the original irq_work_queue lifetime,
/// initialization, context and reuse contract, including NMI-safe primitives.
#[no_mangle]
pub unsafe extern "C" fn irq_work_queue(work: *mut irq_work) -> bool {
    // SAFETY: Retain original claim-before-preemption ordering and balance.
    unsafe {
        if !irq_work_claim(work) {
            return false;
        }
        lupos_irq_work_preempt_disable();
        __irq_work_queue_local(work);
        lupos_irq_work_preempt_enable();
        true
    }
}

/// Enqueue work on the requested CPU unless it is already pending.
///
/// # Safety
/// The original queue_on CPU, work lifetime and remote non-NMI requirements
/// apply. Work may be reclaimed while its prior callback is still running.
#[no_mangle]
pub unsafe extern "C" fn irq_work_queue_on(work: *mut irq_work, cpu: c_int) -> bool {
    #[cfg(not(CONFIG_SMP))]
    {
        let _ = cpu;
        // SAFETY: UP ignores the target CPU exactly as the original branch.
        return unsafe { irq_work_queue(work) };
    }

    #[cfg(CONFIG_SMP)]
    // SAFETY: Native primitives preserve real per-CPU storage and diagnostics.
    // All claimed remote/local paths balance the original preemption pair.
    unsafe {
        let mut work = work;
        lupos_irq_work_warn_queue_cpu_offline(cpu);
        if !irq_work_claim(work) {
            return false;
        }

        lupos_irq_work_record_aux_stack(work);
        lupos_irq_work_preempt_disable();
        if cpu as c_uint != lupos_irq_work_processor_id() {
            lupos_irq_work_warn_remote_nmi();

            /*
             * PREEMPT_RT non-HARD work goes on the remote lazy list; a
             * separate native HARD work entry wakes that CPU's thread.
             */
            if cfg!(CONFIG_PREEMPT_RT)
                && lupos_irq_work_atomic_read(work) & IRQ_WORK_HARD_IRQ as c_int == 0
            {
                if !lupos_irq_work_list_add(work, lupos_irq_work_cpu_lazy(cpu)) {
                    lupos_irq_work_preempt_enable();
                    return true;
                }

                work = lupos_irq_work_cpu_wakeup(cpu);
                if !irq_work_claim(work) {
                    lupos_irq_work_preempt_enable();
                    return true;
                }
            }

            lupos_irq_work_queue_remote(cpu, work);
        } else {
            __irq_work_queue_local(work);
        }
        lupos_irq_work_preempt_enable();
        true
    }
}

/// Indicate whether this CPU still needs a tick for queued IRQ work.
///
/// # Safety
/// Called in the original CPU-local irq_work_needs_cpu context.
#[no_mangle]
pub unsafe extern "C" fn irq_work_needs_cpu() -> bool {
    // SAFETY: Caller context pins the original CPU-local list observations.
    unsafe {
        let raised = lupos_irq_work_this_raised();
        let lazy = lupos_irq_work_this_lazy();

        if lupos_irq_work_list_empty(raised) || lupos_irq_work_has_interrupt() {
            if lupos_irq_work_list_empty(lazy) {
                return false;
            }
        }

        lupos_irq_work_warn_needs_cpu_offline();
        true
    }
}

/// Run one claimed IRQ-work entry.
///
/// # Safety
/// Arg designates a live, claimed native irq_work with a callable callback.
/// The original execution/RCU context keeps it live through the post-BUSY wake.
#[no_mangle]
pub unsafe extern "C" fn irq_work_single(arg: *mut c_void) {
    // SAFETY: Native field/atomic/callback leaves retain original ownership.
    // Callback completion alone does not end the entry's required lifetime.
    unsafe {
        let work = arg as *mut irq_work;
        let mut flags;

        /*
         * Clear PENDING so the entry can be reclaimed. We own PENDING;
         * retain the original atomic_read/atomic_set, then full barrier.
         */
        flags = lupos_irq_work_atomic_read(work);
        flags &= !(IRQ_WORK_PENDING as c_int);
        lupos_irq_work_atomic_set(work, flags);
        lupos_irq_work_mb();

        lupos_irq_work_lockdep_enter(flags);
        lupos_irq_work_invoke(work);
        lupos_irq_work_lockdep_exit(flags);

        /* Do not clear BUSY if someone else claimed the work meanwhile. */
        let _ = lupos_irq_work_atomic_cmpxchg(work, flags, flags & !(IRQ_WORK_BUSY as c_int));

        if (cfg!(CONFIG_PREEMPT_RT) && !lupos_irq_work_is_hard(work))
            || !lupos_irq_work_has_interrupt()
        {
            lupos_irq_work_wait_wake(work);
        }
    }
}

unsafe fn irq_work_run_list(list: *mut llist_head) {
    // SAFETY: The original CPU/context contract permits detaching this list.
    // Capture the plain next link before callback execution; never borrow a
    // node or follow its old link after the callback can reclaim/requeue it.
    unsafe {
        lupos_irq_work_bug_run_context();
        if lupos_irq_work_list_empty(list) {
            return;
        }

        let mut llnode = lupos_irq_work_list_del_all(list);
        while !llnode.is_null() {
            let work = lupos_irq_work_node_work(llnode);
            llnode = lupos_irq_work_node_next(llnode);
            irq_work_single(work.cast());
        }
    }
}

/// Drain the current CPU's IRQ work, including the original hotplug path.
///
/// # Safety
/// Called in the original hardirq/CPU-local context, including
/// hotplug_cfd -> flush_smp_call_function_queue.
#[no_mangle]
pub unsafe extern "C" fn irq_work_run() {
    // SAFETY: Preserve hard work execution and PREEMPT_RT thread delegation.
    unsafe {
        irq_work_run_list(lupos_irq_work_this_raised());
        if !cfg!(CONFIG_PREEMPT_RT) {
            irq_work_run_list(lupos_irq_work_this_lazy());
        } else {
            wake_irq_workd();
        }
    }
}

/// Process the IRQ-work queues due at this CPU's timer tick.
///
/// # Safety
/// Called in the original timer tick CPU/IRQ context.
#[no_mangle]
pub unsafe extern "C" fn irq_work_tick() {
    // SAFETY: Native CPU-local storage and original interrupt distinction.
    unsafe {
        let raised = lupos_irq_work_this_raised();
        if !lupos_irq_work_list_empty(raised) && !lupos_irq_work_has_interrupt() {
            irq_work_run_list(raised);
        }

        if !cfg!(CONFIG_PREEMPT_RT) {
            irq_work_run_list(lupos_irq_work_this_lazy());
        } else {
            wake_irq_workd();
        }
    }
}

/// Synchronize with the IRQ-work entry until it is no longer in use.
///
/// # Safety
/// Caller retains work through this call, obeys the original serialization
/// for its sole rcuwait waiter, and can sleep with IRQs enabled. No racing
/// producer may invalidate the caller's original reclamation contract.
#[no_mangle]
pub unsafe extern "C" fn irq_work_sync(work: *mut irq_work) {
    // SAFETY: The wait leaves use the actual work->irqwait and native current
    // state/barriers. Every BUSY test is after that iteration's state publish.
    unsafe {
        lupos_irq_work_assert_irqs_enabled();
        lupos_irq_work_might_sleep();

        if (cfg!(CONFIG_PREEMPT_RT) && !lupos_irq_work_is_hard(work))
            || !lupos_irq_work_has_interrupt()
        {
            lupos_irq_work_wait_prepare(work);
            loop {
                lupos_irq_work_wait_set_state();
                if !lupos_irq_work_is_busy(work) {
                    break;
                }
                // Preserve the macro's signal check position. Native
                // TASK_UNINTERRUPTIBLE makes this false without signal reads.
                if lupos_irq_work_wait_signal_pending() {
                    break;
                }
                lupos_irq_work_schedule();
            }
            lupos_irq_work_wait_finish(work);
            // irq_work_single still accesses work after its successful BUSY
            // clear. Its RCU read-side lifetime must end before this returns.
            lupos_irq_work_synchronize_rcu();
            return;
        }

        while lupos_irq_work_is_busy(work) {
            lupos_irq_work_relax();
        }
    }
}

#[export_name = "lupos_irq_work_run_workd"]
unsafe extern "C" fn run_irq_workd(_cpu: c_uint) {
    // SAFETY: smpboot invokes this for the CPU-bound worker. The native RCU
    // read section covers the complete drain, callback and post-BUSY wake.
    unsafe {
        lupos_irq_work_rcu_read_lock();
        irq_work_run_list(lupos_irq_work_this_lazy());
        lupos_irq_work_rcu_read_unlock();
    }
}

#[export_name = "lupos_irq_workd_setup"]
unsafe extern "C" fn irq_workd_setup(_cpu: c_uint) {
    // SAFETY: smpboot setup runs in the new worker's context, as in C.
    unsafe { lupos_irq_work_setup_current() };
}

#[cold]
#[link_section = ".init.text"]
#[export_name = "lupos_irq_work_init_threads"]
unsafe extern "C" fn irq_work_init_threads() -> c_int {
    // SAFETY: Native early_initcall owns the boot-only call; the native
    // descriptor owns the actual per-CPU task store and C callback fields.
    unsafe {
        if cfg!(CONFIG_PREEMPT_RT) {
            lupos_irq_work_bug_register_threads();
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
