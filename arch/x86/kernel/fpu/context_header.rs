/* SPDX-License-Identifier: GPL-2.0 */

// Declarations and definitions supplied by asm/fpu/xstate.h and
// asm/trace/fpu.h are referenced here as external dependencies.

/* Functions related to FPU context tracking */

/*
 * The in-register FPU state for an FPU context on a CPU is assumed to be
 * valid if the fpu->last_cpu matches the CPU, and the fpu_fpregs_owner_ctx
 * matches the FPU.
 *
 * If the FPU register state is valid, the kernel can skip restoring the
 * FPU state from memory.
 *
 * Any code that clobbers the FPU registers or updates the in-memory
 * FPU state for a task MUST let the rest of the kernel know that the
 * FPU registers are no longer valid for this task.
 *
 * Invalidate a resource you control: CPU if using the CPU for something else
 * (with preemption disabled), FPU for the current task, or a task that
 * is prevented from running by the current task.
 */
#[inline]
pub unsafe fn __cpu_invalidate_fpregs_state() {
    __this_cpu_write(fpu_fpregs_owner_ctx, core::ptr::null_mut());
}

#[inline]
pub unsafe fn __fpu_invalidate_fpregs_state(fpu: *mut fpu) {
    (*fpu).last_cpu = -1;
}

#[inline]
pub unsafe fn fpregs_state_valid(fpu: *mut fpu, cpu: u32) -> bool {
    fpu == this_cpu_read(fpu_fpregs_owner_ctx) && cpu == (*fpu).last_cpu as u32
}

#[inline]
pub unsafe fn fpregs_deactivate(fpu: *mut fpu) {
    __this_cpu_write(fpu_fpregs_owner_ctx, core::ptr::null_mut());
    trace_x86_fpu_regs_deactivated(fpu);
}

#[inline]
pub unsafe fn fpregs_activate(fpu: *mut fpu) {
    __this_cpu_write(fpu_fpregs_owner_ctx, fpu);
    trace_x86_fpu_regs_activated(fpu);
}

/* Internal helper for switch_fpu_return() and signal frame setup */
#[inline]
pub unsafe fn fpregs_restore_userregs() {
    let fpu: *mut fpu = x86_task_fpu(current);
    let cpu: i32 = smp_processor_id();

    if WARN_ON_ONCE((*current).flags & (PF_KTHREAD | PF_USER_WORKER) != 0) {
        return;
    }

    if !fpregs_state_valid(fpu, cpu as u32) {
        /*
         * This restores _all_ xstate which has not been
         * established yet.
         *
         * If PKRU is enabled, then the PKRU value is already
         * correct because it was either set in switch_to() or in
         * flush_thread(). So it is excluded because it might be
         * not up to date in current->thread.fpu->xsave state.
         *
         * XFD state is handled in restore_fpregs_from_fpstate().
         */
        restore_fpregs_from_fpstate((*fpu).fpstate, XFEATURE_MASK_FPSTATE);

        fpregs_activate(fpu);
        (*fpu).last_cpu = cpu;
    }
    clear_thread_flag(TIF_NEED_FPU_LOAD);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
