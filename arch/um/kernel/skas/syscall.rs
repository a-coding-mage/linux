// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the surrounding UML kernel translation unit:
// linux/kernel.h, linux/ptrace.h, linux/seccomp.h, kern_util.h,
// sysdep/ptrace.h, sysdep/ptrace_user.h, linux/time-internal.h,
// asm/syscall.h, asm/unistd.h, and asm/delay.h.

pub unsafe fn handle_syscall(r: *mut uml_pt_regs) {
    let regs: *mut pt_regs = container_of(r, regs);
    let mut syscall: i32;

    /* Initialize the syscall number and default return value. */
    UPT_SYSCALL_NR(r) = PT_SYSCALL_NR((*r).gp);
    PT_REGS_SET_SYSCALL_RETURN(regs, -ENOSYS);

    if syscall_trace_enter(regs) != 0 {
        syscall_trace_leave(regs);
        return;
    }

    /* Do the seccomp check after ptrace; failures should be fast. */
    if !seccomp_permit_syscall() {
        syscall_trace_leave(regs);
        return;
    }

    syscall = UPT_SYSCALL_NR(r);

    /*
     * If no time passes, then sched_yield may not actually yield, causing
     * broken spinlock implementations in userspace (ASAN) to hang for long
     * periods of time.
     */
    if (time_travel_mode == TT_MODE_INFCPU || time_travel_mode == TT_MODE_EXTERNAL)
        && syscall == __NR_sched_yield
    {
        tt_extra_sched_jiffies += 1;
    }

    if syscall >= 0 && syscall < __NR_syscalls {
        let ret: usize;

        ret = (*sys_call_table[syscall as usize])(
            UPT_SYSCALL_ARG1((&mut (*regs).regs) as *mut _),
            UPT_SYSCALL_ARG2((&mut (*regs).regs) as *mut _),
            UPT_SYSCALL_ARG3((&mut (*regs).regs) as *mut _),
            UPT_SYSCALL_ARG4((&mut (*regs).regs) as *mut _),
            UPT_SYSCALL_ARG5((&mut (*regs).regs) as *mut _),
            UPT_SYSCALL_ARG6((&mut (*regs).regs) as *mut _),
        );

        PT_REGS_SET_SYSCALL_RETURN(regs, ret);

        /*
         * An error value here can be some form of -ERESTARTSYS
         * and then we'd just loop. Make any error syscalls take
         * some time, so that it won't just loop if something is
         * not ready, and hopefully other things will make some
         * progress.
         */
        if IS_ERR_VALUE(ret)
            && (time_travel_mode == TT_MODE_INFCPU || time_travel_mode == TT_MODE_EXTERNAL)
        {
            um_udelay(1);
            schedule();
        }
    }

    syscall_trace_leave(regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
