// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/process.c
 *
 * This file handles the architecture-dependent parts of process handling..
 *
 *  Copyright (C) 1995  Linus Torvalds
 *
 *  SuperH version:  Copyright (C) 1999, 2000  Niibe Yutaka & Kaz Kojima
 *		     Copyright (C) 2006 Lineo Solutions Inc. support SH4A UBC
 *		     Copyright (C) 2002 - 2008  Paul Mundt
 */
// C headers omitted; their supplied symbols remain external dependencies.

pub unsafe fn show_regs(regs: *mut pt_regs) {
    pr_info!("\n");
    show_regs_print_info!(KERN_DEFAULT);

    pr_info!("PC is at %pS\n", instruction_pointer(regs) as *mut core::ffi::c_void);
    pr_info!("PR is at %pS\n", (*regs).pr);

    pr_info!("PC  : %08lx SP  : %08lx SR  : %08lx ", (*regs).pc,
        (*regs).regs[15], (*regs).sr);
    // CONFIG_MMU is a build-time condition from the surrounding kernel.
    #[cfg(CONFIG_MMU)]
    pr_cont!("TEA : %08x\n", __raw_readl(MMU_TEA));
    #[cfg(not(CONFIG_MMU))]
    pr_cont!("\n");

    pr_info!("R0  : %08lx R1  : %08lx R2  : %08lx R3  : %08lx\n",
        (*regs).regs[0], (*regs).regs[1], (*regs).regs[2], (*regs).regs[3]);
    pr_info!("R4  : %08lx R5  : %08lx R6  : %08lx R7  : %08lx\n",
        (*regs).regs[4], (*regs).regs[5], (*regs).regs[6], (*regs).regs[7]);
    pr_info!("R8  : %08lx R9  : %08lx R10 : %08lx R11 : %08lx\n",
        (*regs).regs[8], (*regs).regs[9], (*regs).regs[10], (*regs).regs[11]);
    pr_info!("R12 : %08lx R13 : %08lx R14 : %08lx\n",
        (*regs).regs[12], (*regs).regs[13], (*regs).regs[14]);
    pr_info!("MACH: %08lx MACL: %08lx GBR : %08lx PR  : %08lx\n",
        (*regs).mach, (*regs).macl, (*regs).gbr, (*regs).pr);

    show_trace(core::ptr::null_mut(), (*regs).regs[15] as *mut usize, regs, KERN_DEFAULT);
    show_code(regs);
}

pub unsafe fn start_thread(regs: *mut pt_regs, new_pc: usize, new_sp: usize) {
    (*regs).pr = 0;
    (*regs).sr = SR_FD;
    (*regs).pc = new_pc;
    (*regs).regs[15] = new_sp;

    free_thread_xstate(current);
}

// EXPORT_SYMBOL(start_thread);

pub unsafe fn flush_thread() {
    let tsk: *mut task_struct = current;

    flush_ptrace_hw_breakpoint(tsk);

    // CONFIG_SH_FPU is a build-time condition from the surrounding kernel.
    #[cfg(CONFIG_SH_FPU)]
    {
        /* Forget lazy FPU state */
        clear_fpu(tsk, task_pt_regs(tsk));
        clear_used_math();
    }
}

extern "C" {
    fn ret_from_fork();
    fn ret_from_kernel_thread();
}

pub unsafe fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags: u64 = (*args).flags;
    let usp: usize = (*args).stack;
    let tls: usize = (*args).tls;
    let ti: *mut thread_info = task_thread_info(p);
    let mut childregs: *mut pt_regs;

    // CONFIG_SH_DSP is a build-time condition from the surrounding kernel.
    #[cfg(CONFIG_SH_DSP)]
    {
        let tsk: *mut task_struct = current;
        if is_dsp_enabled(tsk) {
            /* We can use the __save_dsp or just copy the struct:
             * __save_dsp(p);
             * p->thread.dsp_status.status |= SR_DSP
             */
            (*p).thread.dsp_status = (*tsk).thread.dsp_status;
        }
    }

    core::ptr::write_bytes((*p).thread.ptrace_bps.as_mut_ptr(), 0,
        (*p).thread.ptrace_bps.len());

    childregs = task_pt_regs(p);
    (*p).thread.sp = childregs as usize;
    if unlikely((*args).fn_.is_some()) {
        core::ptr::write_bytes(childregs, 0, core::mem::size_of::<pt_regs>());
        (*p).thread.pc = ret_from_kernel_thread as usize;
        (*childregs).regs[4] = (*args).fn_arg as usize;
        (*childregs).regs[5] = (*args).fn_ as usize;
        (*childregs).sr = SR_MD;
        #[cfg(CONFIG_SH_FPU)]
        { (*childregs).sr |= SR_FD; }
        (*ti).status &= !TS_USEDFPU;
        (*p).thread.fpu_counter = 0;
        return 0;
    }
    *childregs = *current_pt_regs();

    if usp != 0 {
        (*childregs).regs[15] = usp;
    }

    if clone_flags & CLONE_SETTLS != 0 {
        (*childregs).gbr = tls;
    }

    (*childregs).regs[0] = 0; /* Set return value for child */
    (*p).thread.pc = ret_from_fork as usize;
    0
}

/*
 *	switch_to(x,y) should switch tasks from x to y.
 *
 */
pub unsafe fn __switch_to(prev: *mut task_struct, next: *mut task_struct) -> *mut task_struct {
    let next_t: *mut thread_struct = &mut (*next).thread;

    // CONFIG_STACKPROTECTOR && !CONFIG_SMP is a build-time condition.
    #[cfg(all(CONFIG_STACKPROTECTOR, not(CONFIG_SMP)))]
    { __stack_chk_guard = (*next).stack_canary; }

    unlazy_fpu(prev, task_pt_regs(prev));

    if (*next).thread.fpu_counter > 5 {
        prefetch((*next_t).xstate);
    }

    // CONFIG_MMU is a build-time condition; this preserves the original asm intent.
    #[cfg(CONFIG_MMU)]
    core::arch::asm!("ldc {0}, r7_bank", in(reg) task_thread_info(next));

    if (*next).thread.fpu_counter > 5 {
        __fpu_state_restore();
    }

    prev
}

pub unsafe fn __get_wchan(p: *mut task_struct) -> usize {
    let pc: usize;

    /*
     * The same comment as on the Alpha applies here, too ...
     */
    pc = thread_saved_pc(p);

    // CONFIG_FRAME_POINTER is a build-time condition from the surrounding kernel.
    #[cfg(CONFIG_FRAME_POINTER)]
    if in_sched_functions(pc) {
        let schedule_frame: usize = (*p).thread.sp;
        return *((schedule_frame as *mut usize).add(21));
    }

    pc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
