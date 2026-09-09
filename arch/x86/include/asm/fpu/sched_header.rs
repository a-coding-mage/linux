/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the original header:
// linux/sched.h, asm/cpufeature.h, asm/fpu/types.h, and asm/trace/fpu.h

extern "C" {
    pub fn save_fpregs_to_fpstate(fpu: *mut fpu);
    pub fn fpu__drop(tsk: *mut task_struct);
    pub fn fpu_clone(
        dst: *mut task_struct,
        clone_flags: u64,
        minimal: bool,
        shstk_addr: usize,
    ) -> i32;
    pub fn fpu_flush_thread();
}

/*
 * FPU state switching for scheduling.
 *
 * switch_fpu() saves the old state and sets TIF_NEED_FPU_LOAD if
 * TIF_NEED_FPU_LOAD is not set.  This is done within the context
 * of the old process.
 *
 * Once TIF_NEED_FPU_LOAD is set, it is required to load the
 * registers before returning to userland or using the content
 * otherwise.
 *
 * The FPU context is only stored/restored for a user task and
 * PF_KTHREAD is used to distinguish between kernel and user threads.
 */
pub unsafe fn switch_fpu(old: *mut task_struct, cpu: i32) {
    if !test_tsk_thread_flag(old, TIF_NEED_FPU_LOAD)
        && cpu_feature_enabled(X86_FEATURE_FPU)
        && ((*old).flags & (PF_KTHREAD | PF_USER_WORKER)) == 0
    {
        let old_fpu: *mut fpu = x86_task_fpu(old);

        set_tsk_thread_flag(old, TIF_NEED_FPU_LOAD);
        save_fpregs_to_fpstate(old_fpu);
        /*
         * The save operation preserved register state, so the
         * fpu_fpregs_owner_ctx is still @old_fpu. Store the
         * current CPU number in @old_fpu, so the next return
         * to user space can avoid the FPU register restore
         * when is returns on the same CPU and still owns the
         * context. See fpregs_restore_userregs().
         */
        (*old_fpu).last_cpu = cpu;

        trace_x86_fpu_regs_deactivated(old_fpu);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
