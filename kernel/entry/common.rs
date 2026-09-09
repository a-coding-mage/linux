// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux kernel headers and other translation units.

#[no_mangle]
pub unsafe extern "C" fn arch_do_signal_or_restart(_regs: *mut pt_regs) {}

#[cfg(feature = "CONFIG_HAVE_GENERIC_TIF_BITS")]
const EXIT_TO_USER_MODE_WORK_LOOP: c_ulong = EXIT_TO_USER_MODE_WORK & !(_TIF_RSEQ);
#[cfg(not(feature = "CONFIG_HAVE_GENERIC_TIF_BITS"))]
const EXIT_TO_USER_MODE_WORK_LOOP: c_ulong = EXIT_TO_USER_MODE_WORK;

#[cfg(feature = "CONFIG_PREEMPT_RT")]
const TIF_SLICE_EXT_SCHED: c_ulong = _TIF_NEED_RESCHED_LAZY;
#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
const TIF_SLICE_EXT_SCHED: c_ulong = _TIF_NEED_RESCHED | _TIF_NEED_RESCHED_LAZY;
const TIF_SLICE_EXT_DENY: c_ulong = EXIT_TO_USER_MODE_WORK & !TIF_SLICE_EXT_SCHED;

unsafe fn __exit_to_user_mode_loop(regs: *mut pt_regs, mut ti_work: c_ulong) -> c_ulong {
    /*
     * Before returning to user space ensure that all pending work
     * items have been completed.
     */
    while ti_work & EXIT_TO_USER_MODE_WORK_LOOP != 0 {
        local_irq_enable();

        if ti_work & (_TIF_NEED_RESCHED | _TIF_NEED_RESCHED_LAZY) != 0 {
            if !rseq_grant_slice_extension(ti_work, TIF_SLICE_EXT_DENY) {
                schedule();
            }
        }

        if ti_work & _TIF_UPROBE != 0 {
            uprobe_notify_resume(regs);
        }

        if ti_work & _TIF_PATCH_PENDING != 0 {
            klp_update_patch_state(current);
        }

        if ti_work & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 {
            futex_fixup_robust_unlock(regs);
            arch_do_signal_or_restart(regs);
        }

        if ti_work & _TIF_NOTIFY_RESUME != 0 {
            resume_user_mode_work(regs);
        }

        /* Architecture specific TIF work */
        arch_exit_to_user_mode_work(regs, ti_work);

        /*
         * Disable interrupts and reevaluate the work flags as they
         * might have changed while interrupts and preemption was
         * enabled above.
         */
        local_irq_disable();

        /* Check if any of the above work has queued a deferred wakeup */
        tick_nohz_user_enter_prepare();

        ti_work = read_thread_flags();
    }

    /* Return the latest work state for arch_exit_to_user_mode() */
    ti_work
}

#[no_mangle]
pub unsafe extern "C" fn exit_to_user_mode_loop(
    regs: *mut pt_regs,
    mut ti_work: c_ulong,
) -> c_ulong {
    loop {
        ti_work = __exit_to_user_mode_loop(regs, ti_work);

        if !rseq_exit_to_user_mode_restart(regs, ti_work) {
            return ti_work;
        }
        ti_work = read_thread_flags();
    }
}

#[no_mangle]
pub unsafe extern "C" fn irqentry_enter(regs: *mut pt_regs) -> irqentry_state_t {
    if user_mode(regs) {
        let ret = irqentry_state_t { exit_rcu: false };
        irqentry_enter_from_user_mode(regs);
        return ret;
    }

    irqentry_enter_from_kernel_mode(regs)
}

#[inline]
unsafe fn arch_irqentry_exit_need_resched() -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn raw_irqentry_exit_cond_resched() {
    if preempt_count() == 0 {
        /* Sanity check RCU and thread stack */
        rcu_irq_exit_check_preempt();
        if cfg!(feature = "CONFIG_DEBUG_ENTRY") {
            WARN_ON_ONCE(!on_thread_stack());
        }
        if need_resched() && arch_irqentry_exit_need_resched() {
            preempt_schedule_irq();
        }
    }
}

#[cfg(feature = "CONFIG_PREEMPT_DYNAMIC")]
#[cfg(feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_CALL")]
DEFINE_STATIC_CALL!(irqentry_exit_cond_resched, raw_irqentry_exit_cond_resched);

#[cfg(all(feature = "CONFIG_PREEMPT_DYNAMIC", feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_KEY"))]
static mut sk_dynamic_irqentry_exit_cond_resched: StaticKey = StaticKey;

#[cfg(all(feature = "CONFIG_PREEMPT_DYNAMIC", feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_KEY"))]
unsafe extern "C" fn dynamic_irqentry_exit_cond_resched() {
    if !static_branch_unlikely(&sk_dynamic_irqentry_exit_cond_resched) {
        return;
    }
    raw_irqentry_exit_cond_resched();
}

#[no_mangle]
pub unsafe extern "C" fn irqentry_exit(regs: *mut pt_regs, state: irqentry_state_t) {
    if user_mode(regs) {
        irqentry_exit_to_user_mode(regs);
    } else {
        irqentry_exit_to_kernel_mode(regs, state);
    }
}

#[no_mangle]
pub unsafe extern "C" fn irqentry_nmi_enter(regs: *mut pt_regs) -> irqentry_state_t {
    let mut irq_state: irqentry_state_t = core::mem::zeroed();

    irq_state.lockdep = lockdep_hardirqs_enabled();

    __nmi_enter();
    lockdep_hardirqs_off(CALLER_ADDR0);
    lockdep_hardirq_enter();
    ct_nmi_enter();

    instrumentation_begin();
    kmsan_unpoison_entry_regs(regs);
    trace_hardirqs_off_finish();
    ftrace_nmi_enter();
    instrumentation_end();

    irq_state
}

#[no_mangle]
pub unsafe extern "C" fn irqentry_nmi_exit(
    _regs: *mut pt_regs,
    irq_state: irqentry_state_t,
) {
    instrumentation_begin();
    ftrace_nmi_exit();
    if irq_state.lockdep {
        trace_hardirqs_on_prepare();
        lockdep_hardirqs_on_prepare();
    }
    instrumentation_end();

    ct_nmi_exit();
    lockdep_hardirq_exit();
    if irq_state.lockdep {
        lockdep_hardirqs_on(CALLER_ADDR0);
    }
    __nmi_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
