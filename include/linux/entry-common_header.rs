/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/entry-common.h. C header dependencies are supplied externally.

// #ifndef _TIF_UPROBE
// # define _TIF_UPROBE (0)
// #endif

/* SYSCALL_WORK flags handled in syscall_enter_from_user_mode_work(). */
pub const SYSCALL_WORK_ENTER: c_ulong = SYSCALL_WORK_SECCOMP
    | SYSCALL_WORK_SYSCALL_TRACEPOINT
    | SYSCALL_WORK_SYSCALL_TRACE
    | SYSCALL_WORK_SYSCALL_EMU
    | SYSCALL_WORK_SYSCALL_AUDIT
    | SYSCALL_WORK_SYSCALL_USER_DISPATCH
    | SYSCALL_WORK_SYSCALL_RSEQ_SLICE;

/* SYSCALL_WORK flags handled in syscall_exit_to_user_mode(). */
pub const SYSCALL_WORK_EXIT: c_ulong = SYSCALL_WORK_SYSCALL_TRACEPOINT
    | SYSCALL_WORK_SYSCALL_TRACE
    | SYSCALL_WORK_SYSCALL_AUDIT
    | SYSCALL_WORK_SYSCALL_USER_DISPATCH
    | SYSCALL_WORK_SYSCALL_EXIT_TRAP;

/// Architecture specific wrapper for ptrace_report_syscall_permit_entry().
pub unsafe fn arch_ptrace_report_syscall_permit_entry(regs: *mut pt_regs) -> bool {
    ptrace_report_syscall_permit_entry(regs)
}

extern "C" {
    pub fn trace_syscall_enter(regs: *mut pt_regs);
    pub fn trace_syscall_exit(regs: *mut pt_regs, ret: c_long);
    pub fn syscall_enter_audit(regs: *mut pt_regs);
}

pub unsafe fn syscall_trace_enter(
    regs: *mut pt_regs,
    mut work: c_ulong,
    syscall: c_long,
) -> bool {
    /* Handle Syscall User Dispatch first; its ABI may be incompatible with other features. */
    if work & SYSCALL_WORK_SYSCALL_USER_DISPATCH != 0 {
        if syscall_user_dispatch(regs) {
            return false;
        }
    }

    /* User space relinquishes a granted time-slice extension. */
    if work & SYSCALL_WORK_SYSCALL_RSEQ_SLICE != 0 {
        rseq_syscall_enter_work(syscall);
    }

    /* Handle ptrace. */
    if work & (SYSCALL_WORK_SYSCALL_TRACE | SYSCALL_WORK_SYSCALL_EMU) != 0 {
        if !arch_ptrace_report_syscall_permit_entry(regs)
            || work & SYSCALL_WORK_SYSCALL_EMU != 0
        {
            return false;
        }

        /* ptrace might have changed work flags. */
        work = READ_ONCE(current_thread_info()->syscall_work);
    }

    /* Do seccomp after ptrace, to catch any tracer changes. */
    if work & SYSCALL_WORK_SECCOMP != 0 {
        if !__seccomp_permit_syscall() {
            return false;
        }
    }

    if unlikely(work & SYSCALL_WORK_SYSCALL_TRACEPOINT != 0) {
        trace_syscall_enter(regs);
    }

    if unlikely(audit_context()) {
        syscall_enter_audit(regs);
    }

    true
}

/// Check and handle work before invoking a syscall.
pub unsafe fn syscall_enter_from_user_mode_work(
    regs: *mut pt_regs,
    syscall: *mut c_long,
) -> bool {
    let work: c_ulong = READ_ONCE(current_thread_info()->syscall_work);

    if work & SYSCALL_WORK_ENTER == 0 {
        return true;
    }

    if unlikely(!syscall_trace_enter(regs, work, *syscall)) {
        return false;
    }

    /* Reread the syscall number as it might have been modified. */
    *syscall = syscall_get_nr(current, regs);
    true
}

/* Implemented as a macro so stack randomization remains effective in the caller's scope. */
#[macro_export]
macro_rules! enter_from_user_mode_randomize_stack {
    ($regs:expr) => {{
        enter_from_user_mode($regs);
        instrumentation_begin();
        add_random_kstack_offset_irqsoff();
        instrumentation_end();
    }};
}

#[macro_export]
macro_rules! syscall_enter_from_user_mode_randomize_stack {
    ($regs:expr, $syscall:expr) => {{
        enter_from_user_mode_randomize_stack!($regs);
        instrumentation_begin();
        local_irq_enable();
        let _ret = syscall_enter_from_user_mode_work($regs, $syscall);
        instrumentation_end();
        _ret
    }};
}

pub unsafe fn report_single_step(work: c_ulong) -> bool {
    if work & SYSCALL_WORK_SYSCALL_EMU != 0 {
        return false;
    }
    work & SYSCALL_WORK_SYSCALL_EXIT_TRAP != 0
}

/// Architecture specific wrapper for ptrace_report_syscall_exit().
pub unsafe fn arch_ptrace_report_syscall_exit(regs: *mut pt_regs, step: c_int) {
    ptrace_report_syscall_exit(regs, step);
}

pub unsafe fn syscall_exit_work(regs: *mut pt_regs, work: c_ulong) {
    let step: bool;

    /* A dispatched syscall has an unknown ABI and is not traced below. */
    if work & SYSCALL_WORK_SYSCALL_USER_DISPATCH != 0 {
        if syscall_user_dispatch_clear_on_dispatch() {
            return;
        }
    }

    audit_syscall_exit(regs);

    if work & SYSCALL_WORK_SYSCALL_TRACEPOINT != 0 {
        trace_syscall_exit(regs, syscall_get_return_value(current, regs));
    }

    step = report_single_step(work);
    if step || work & SYSCALL_WORK_SYSCALL_TRACE != 0 {
        arch_ptrace_report_syscall_exit(regs, step as c_int);
    }
}

pub unsafe fn syscall_exit_to_user_mode_work(regs: *mut pt_regs) {
    let work: c_ulong = READ_ONCE(current_thread_info()->syscall_work);
    let nr: c_ulong = syscall_get_nr(current, regs);

    CT_WARN_ON(ct_state() != CT_STATE_KERNEL);

    // Preserves the CONFIG_PROVE_LOCKING build-time condition.
    if IS_ENABLED(CONFIG_PROVE_LOCKING) {
        if WARN(irqs_disabled(), "syscall %lu left IRQs disabled", nr) {
            local_irq_enable();
        }
    }

    rseq_debug_syscall_return(regs);

    if unlikely(work & SYSCALL_WORK_EXIT != 0) {
        syscall_exit_work(regs, work);
    }
}

pub unsafe fn syscall_exit_to_user_mode(regs: *mut pt_regs) {
    instrumentation_begin();
    syscall_exit_to_user_mode_work(regs);
    local_irq_disable();
    syscall_exit_to_user_mode_prepare(regs);
    instrumentation_end();
    exit_to_user_mode();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
