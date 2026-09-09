// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 Red Hat, Inc., Peter Zijlstra
 *
 * Provides a framework for enqueueing and running callbacks from hardirq
 * context. The enqueueing is NMI-safe.
 */

// Linux kernel dependencies from the original implementation are supplied externally.

static mut RAISED_LIST: PerCpu<LlistHead> = DEFINE_PER_CPU!();
static mut LAZY_LIST: PerCpu<LlistHead> = DEFINE_PER_CPU!();
static mut IRQ_WORKD: PerCpu<*mut TaskStruct> = DEFINE_PER_CPU!();

unsafe fn wake_irq_workd() {
    let tsk = __this_cpu_read(IRQ_WORKD);

    if !llist_empty(this_cpu_ptr(&LAZY_LIST)) && !tsk.is_null() {
        wake_up_process(tsk);
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn irq_work_wake(_entry: *mut IrqWork) {
    wake_irq_workd();
}

#[cfg(CONFIG_SMP)]
static mut IRQ_WORK_WAKEUP: PerCpu<IrqWork> = DEFINE_PER_CPU_INIT!(IRQ_WORK_INIT_HARD!(irq_work_wake));

unsafe fn irq_workd_should_run(_cpu: u32) -> i32 {
    (!llist_empty(this_cpu_ptr(&LAZY_LIST))) as i32
}

/*
 * Claim the entry so that no one else will poke at it.
 */
unsafe fn irq_work_claim(work: *mut IrqWork) -> bool {
    let oflags = atomic_fetch_or(
        IRQ_WORK_CLAIMED | CSD_TYPE_IRQ_WORK,
        &mut (*work).node.a_flags,
    );
    /*
     * If the work is already pending, no need to raise the IPI.
     * The pairing smp_mb() in irq_work_single() makes sure
     * everything we did before is visible.
     */
    if oflags & IRQ_WORK_PENDING != 0 {
        return false;
    }
    true
}

pub unsafe extern "C" fn arch_irq_work_raise() {
    /*
     * Lame architectures will get the timer tick callback
     */
}

#[inline(always)]
unsafe fn irq_work_raise(work: *mut IrqWork) {
    if trace_ipi_send_cpu_enabled() && arch_irq_work_has_interrupt() {
        trace_call__ipi_send_cpu(smp_processor_id(), _RET_IP_, (*work).func);
    }

    arch_irq_work_raise();
}

/* Enqueue on current CPU, work must already be claimed and preempt disabled */
unsafe fn __irq_work_queue_local(work: *mut IrqWork) {
    let list: *mut LlistHead;
    let mut rt_lazy_work = false;
    let mut lazy_work = false;

    let work_flags = atomic_read(&(*work).node.a_flags);
    if work_flags & IRQ_WORK_LAZY != 0 {
        lazy_work = true;
    } else if IS_ENABLED!(CONFIG_PREEMPT_RT) && work_flags & IRQ_WORK_HARD_IRQ == 0 {
        rt_lazy_work = true;
    }

    if lazy_work || rt_lazy_work {
        list = this_cpu_ptr(&LAZY_LIST);
    } else {
        list = this_cpu_ptr(&RAISED_LIST);
    }

    if !llist_add(&mut (*work).node.llist, list) {
        return;
    }

    /* If the work is "lazy", handle it from next tick if any */
    if !lazy_work || tick_nohz_tick_stopped() {
        irq_work_raise(work);
    }
}

/* Enqueue the irq work @work on the current CPU */
pub unsafe extern "C" fn irq_work_queue(work: *mut IrqWork) -> bool {
    /* Only queue if not already pending */
    if !irq_work_claim(work) {
        return false;
    }

    /* Queue the entry and raise the IPI if needed. */
    preempt_disable();
    __irq_work_queue_local(work);
    preempt_enable();

    true
}

pub unsafe extern "C" fn irq_work_queue_on(work: *mut IrqWork, cpu: i32) -> bool {
    #[cfg(not(CONFIG_SMP))]
    {
        return irq_work_queue(work);
    }

    #[cfg(CONFIG_SMP)]
    {
        /* All work should have been flushed before going offline */
        WARN_ON_ONCE!(cpu_is_offline(cpu));

        /* Only queue if not already pending */
        if !irq_work_claim(work) {
            return false;
        }

        kasan_record_aux_stack(work);
        preempt_disable();
        if cpu != smp_processor_id() {
            /* Arch remote IPI send/receive backend aren't NMI safe */
            WARN_ON_ONCE!(in_nmi());

            /*
             * On PREEMPT_RT the items which are not marked as
             * IRQ_WORK_HARD_IRQ are added to the lazy list and a HARD work
             * item is used on the remote CPU to wake the thread.
             */
            if IS_ENABLED!(CONFIG_PREEMPT_RT)
                && atomic_read(&(*work).node.a_flags) & IRQ_WORK_HARD_IRQ == 0
            {
                if !llist_add(&mut (*work).node.llist, &per_cpu!(LAZY_LIST, cpu)) {
                    preempt_enable();
                    return true;
                }

                work = &mut per_cpu!(IRQ_WORK_WAKEUP, cpu);
                if !irq_work_claim(work) {
                    preempt_enable();
                    return true;
                }
            }

            __smp_call_single_queue(cpu, &mut (*work).node.llist);
        } else {
            __irq_work_queue_local(work);
        }
        preempt_enable();
        true
    }
}

pub unsafe extern "C" fn irq_work_needs_cpu() -> bool {
    let raised = this_cpu_ptr(&RAISED_LIST);
    let lazy = this_cpu_ptr(&LAZY_LIST);

    if llist_empty(raised) || arch_irq_work_has_interrupt() {
        if llist_empty(lazy) {
            return false;
        }
    }

    /* All work should have been flushed before going offline */
    WARN_ON_ONCE!(cpu_is_offline(smp_processor_id()));
    true
}

pub unsafe extern "C" fn irq_work_single(arg: *mut core::ffi::c_void) {
    let work = arg as *mut IrqWork;
    let mut flags;

    /*
     * Clear the PENDING bit, after this point the @work can be re-used.
     * The PENDING bit acts as a lock, and we own it, so we can clear it
     * without atomic ops.
     */
    flags = atomic_read(&(*work).node.a_flags);
    flags &= !IRQ_WORK_PENDING;
    atomic_set(&mut (*work).node.a_flags, flags);

    /*
     * See irq_work_claim().
     */
    smp_mb();

    lockdep_irq_work_enter(flags);
    ((*work).func)(work);
    lockdep_irq_work_exit(flags);

    /*
     * Clear the BUSY bit, if set, and return to the free state if no-one
     * else claimed it meanwhile.
     */
    let _ = atomic_cmpxchg(&mut (*work).node.a_flags, flags, flags & !IRQ_WORK_BUSY);

    if (IS_ENABLED!(CONFIG_PREEMPT_RT) && !irq_work_is_hard(work))
        || !arch_irq_work_has_interrupt()
    {
        rcuwait_wake_up(&mut (*work).irqwait);
    }
}

unsafe fn irq_work_run_list(list: *mut LlistHead) {
    /* On PREEMPT_RT non-HARD IRQ-work runs in preemptible thread context. */
    BUG_ON!(!irqs_disabled() && !IS_ENABLED!(CONFIG_PREEMPT_RT));

    if llist_empty(list) {
        return;
    }

    let mut llnode = llist_del_all(list);
    while !llnode.is_null() {
        let work = llist_entry(llnode, IrqWork, node.llist);
        llnode = (*llnode).next;
        irq_work_single(work as *mut core::ffi::c_void);
    }
}

/*
 * hotplug calls this through:
 *  hotplug_cfd() -> flush_smp_call_function_queue()
 */
pub unsafe extern "C" fn irq_work_run() {
    irq_work_run_list(this_cpu_ptr(&RAISED_LIST));
    if !IS_ENABLED!(CONFIG_PREEMPT_RT) {
        irq_work_run_list(this_cpu_ptr(&LAZY_LIST));
    } else {
        wake_irq_workd();
    }
}

pub unsafe extern "C" fn irq_work_tick() {
    let raised = this_cpu_ptr(&RAISED_LIST);

    if !llist_empty(raised) && !arch_irq_work_has_interrupt() {
        irq_work_run_list(raised);
    }

    if !IS_ENABLED!(CONFIG_PREEMPT_RT) {
        irq_work_run_list(this_cpu_ptr(&LAZY_LIST));
    } else {
        wake_irq_workd();
    }
}

/*
 * Synchronize against the irq_work @entry, ensures the entry is not
 * currently in use.
 */
pub unsafe extern "C" fn irq_work_sync(work: *mut IrqWork) {
    lockdep_assert_irqs_enabled();
    might_sleep();

    if (IS_ENABLED!(CONFIG_PREEMPT_RT) && !irq_work_is_hard(work))
        || !arch_irq_work_has_interrupt()
    {
        rcuwait_wait_event(
            &mut (*work).irqwait,
            !irq_work_is_busy(work),
            TASK_UNINTERRUPTIBLE,
        );
        /* Ensure irq_work_single() does not access @work after clearing BUSY. */
        synchronize_rcu();
        return;
    }

    while irq_work_is_busy(work) {
        cpu_relax();
    }
}

unsafe fn run_irq_workd(_cpu: u32) {
    guard_rcu!();
    irq_work_run_list(this_cpu_ptr(&LAZY_LIST));
}

unsafe fn irq_workd_setup(_cpu: u32) {
    sched_set_fifo_low(current());
}

static mut IRQWORK_THREADS: SmpHotplugThread = SmpHotplugThread {
    store: &mut IRQ_WORKD,
    setup: irq_workd_setup,
    thread_should_run: irq_workd_should_run,
    thread_fn: run_irq_workd,
    thread_comm: "irq_work/%u\0",
};

#[init]
unsafe fn irq_work_init_threads() -> i32 {
    if IS_ENABLED!(CONFIG_PREEMPT_RT) {
        BUG_ON!(smpboot_register_percpu_thread(&mut IRQWORK_THREADS));
    }
    0
}

early_initcall!(irq_work_init_threads);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
