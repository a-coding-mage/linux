/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations and build-time conditions are supplied by the
// corresponding Linux/Rust translation units.

pub const ARCH_EXIT_TO_USER_MODE_WORK: ::core::ffi::c_ulong =
    _TIF_MTE_ASYNC_FAULT | _TIF_FOREIGN_FPSTATE;

#[inline(always)]
pub unsafe fn arch_exit_to_user_mode_work(
    regs: *mut pt_regs,
    ti_work: ::core::ffi::c_ulong,
) {
    let _ = regs;

    if ti_work & _TIF_MTE_ASYNC_FAULT != 0 {
        clear_thread_flag(TIF_MTE_ASYNC_FAULT);
        send_sig_fault(SIGSEGV, SEGV_MTEAERR, ::core::ptr::null_mut(), current);
    }

    if ti_work & _TIF_FOREIGN_FPSTATE != 0 {
        fpsimd_restore_current_state();
    }
}

// C macro alias: arch_exit_to_user_mode_work expands to the function above.

#[inline]
pub unsafe fn arch_irqentry_exit_need_resched() -> bool {
    /*
     * DAIF.DA are cleared at the start of IRQ/FIQ handling, and when GIC
     * priority masking is used the GIC irqchip driver will clear DAIF.IF
     * in gic_unmask_pnmis() for normal IRQs. If anything is set in
     * DAIF we must have handled an NMI, so skip preemption.
     */
    if system_uses_irq_prio_masking() && read_sysreg(daif) != 0 {
        return false;
    }

    /*
     * Preempting a task from an IRQ means we leave copies of PSTATE
     * on the stack. cpufeature's enable calls may modify PSTATE, but
     * resuming one of these preempted tasks would undo those changes.
     *
     * Only allow a task to be preempted once cpufeatures have been
     * enabled.
     */
    if !system_capabilities_finalized() {
        return false;
    }

    true
}

// C macro alias: arch_irqentry_exit_need_resched expands to the function above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
