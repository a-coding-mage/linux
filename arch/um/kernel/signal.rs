// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies: linux/module.h, linux/ptrace.h, linux/sched.h,
// linux/ftrace.h, asm/siginfo.h, asm/signal.h, asm/unistd.h, frame_kern.h,
// kern_util.h, and os.h.

// EXPORT_SYMBOL(block_signals);
// EXPORT_SYMBOL(unblock_signals);

pub unsafe fn block_signals_trace() {
    block_signals();
    if !current_thread_info().is_null() {
        trace_hardirqs_off();
    }
}

pub unsafe fn unblock_signals_trace() {
    if !current_thread_info().is_null() {
        trace_hardirqs_on();
    }
    unblock_signals();
}

pub unsafe fn um_trace_signals_on() {
    if !current_thread_info().is_null() {
        trace_hardirqs_on();
    }
}

pub unsafe fn um_trace_signals_off() {
    if !current_thread_info().is_null() {
        trace_hardirqs_off();
    }
}

/*
 * OK, we're invoking a handler
 */
unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    let oldset: *mut sigset_t = sigmask_to_save();
    let mut singlestep: i32 = 0;
    let mut sp: usize;
    let err: i32;

    if test_thread_flag(TIF_SINGLESTEP) && ((*current).ptrace & PT_PTRACED) != 0 {
        singlestep = 1;
    }

    /* Did we come from a system call? */
    if PT_REGS_SYSCALL_NR(regs) >= 0 {
        /* If so, check system call restarting.. */
        match PT_REGS_SYSCALL_RET(regs) {
            -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => {
                PT_REGS_SYSCALL_RET(regs) = -EINTR;
            }
            -ERESTARTSYS => {
                if ((*ksig).ka.sa.sa_flags & SA_RESTART) == 0 {
                    PT_REGS_SYSCALL_RET(regs) = -EINTR;
                } else {
                    PT_REGS_RESTART_SYSCALL(regs);
                    PT_REGS_ORIG_SYSCALL(regs) = PT_REGS_SYSCALL_NR(regs);
                }
            }
            -ERESTARTNOINTR => {
                PT_REGS_RESTART_SYSCALL(regs);
                PT_REGS_ORIG_SYSCALL(regs) = PT_REGS_SYSCALL_NR(regs);
            }
            _ => {}
        }
    }

    sp = PT_REGS_SP(regs);
    if ((*ksig).ka.sa.sa_flags & SA_ONSTACK) != 0 && sas_ss_flags(sp) == 0 {
        sp = (*current).sas_ss_sp + (*current).sas_ss_size;
    }

    // CONFIG_ARCH_HAS_SC_SIGNALS conditionally supplies the sigcontext setup path.
    #[cfg(CONFIG_ARCH_HAS_SC_SIGNALS)]
    {
        if ((*ksig).ka.sa.sa_flags & SA_SIGINFO) == 0 {
            err = setup_signal_stack_sc(sp, ksig, regs, oldset);
        } else {
            err = setup_signal_stack_si(sp, ksig, regs, oldset);
        }
    }
    #[cfg(not(CONFIG_ARCH_HAS_SC_SIGNALS))]
    {
        err = setup_signal_stack_si(sp, ksig, regs, oldset);
    }

    signal_setup_done(err, ksig, singlestep);
}

pub unsafe fn do_signal(regs: *mut pt_regs) {
    let mut ksig: ksignal;
    let mut handled_sig: i32 = 0;

    while get_signal(&mut ksig) {
        handled_sig = 1;
        /* Whee!  Actually deliver the signal.  */
        handle_signal(&mut ksig, regs);
    }

    /* Did we come from a system call? */
    if handled_sig == 0 && PT_REGS_SYSCALL_NR(regs) >= 0 {
        /* Restart the system call - no handlers present */
        match PT_REGS_SYSCALL_RET(regs) {
            -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => {
                PT_REGS_ORIG_SYSCALL(regs) = PT_REGS_SYSCALL_NR(regs);
                PT_REGS_RESTART_SYSCALL(regs);
            }
            -ERESTART_RESTARTBLOCK => {
                PT_REGS_ORIG_SYSCALL(regs) = __NR_restart_syscall;
                PT_REGS_RESTART_SYSCALL(regs);
            }
            _ => {}
        }
    }

    /*
     * if there's no signal to deliver, we just put the saved sigmask
     * back
     */
    if handled_sig == 0 {
        restore_saved_sigmask();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
