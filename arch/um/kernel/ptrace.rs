// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn user_enable_single_step(child: *mut task_struct) {
    set_tsk_thread_flag(child, TIF_SINGLESTEP);

    // C build-time condition: SUBARCH_SET_SINGLESTEPPING.
    #[cfg(SUBARCH_SET_SINGLESTEPPING)]
    SUBARCH_SET_SINGLESTEPPING(child, 1);
}

pub unsafe fn user_disable_single_step(child: *mut task_struct) {
    clear_tsk_thread_flag(child, TIF_SINGLESTEP);

    // C build-time condition: SUBARCH_SET_SINGLESTEPPING.
    #[cfg(SUBARCH_SET_SINGLESTEPPING)]
    SUBARCH_SET_SINGLESTEPPING(child, 0);
}

/*
 * Called by kernel/ptrace.c when detaching..
 */
pub unsafe fn ptrace_disable(child: *mut task_struct) {
    user_disable_single_step(child);
}

pub unsafe fn arch_ptrace(
    child: *mut task_struct,
    request: c_long,
    addr: c_ulong,
    data: c_ulong,
) -> c_long {
    let mut ret: c_int;
    let mut p = data as *mut c_ulong;
    let vp = p as *mut c_void;

    match request {
        /* read the word at location addr in the USER area. */
        PTRACE_PEEKUSR => {
            ret = peek_user(child, addr, data);
        }

        /* write the word at location addr in the USER area */
        PTRACE_POKEUSR => {
            ret = poke_user(child, addr, data);
        }

        PTRACE_SYSEMU | PTRACE_SYSEMU_SINGLESTEP => {
            ret = -EIO;
        }

        // C build-time condition: PTRACE_GETREGS.
        #[cfg(PTRACE_GETREGS)]
        PTRACE_GETREGS => {
            /* Get all gp regs from the child. */
            if !access_ok(p, MAX_REG_OFFSET) {
                ret = -EIO;
            } else {
                let mut i: usize = 0;
                while i < MAX_REG_OFFSET {
                    __put_user(getreg(child, i), p);
                    p = p.add(1);
                    i += core::mem::size_of::<c_long>();
                }
                ret = 0;
            }
        }

        // C build-time condition: PTRACE_SETREGS.
        #[cfg(PTRACE_SETREGS)]
        PTRACE_SETREGS => {
            /* Set all gp regs in the child. */
            let mut tmp: c_ulong = 0;
            if !access_ok(p, MAX_REG_OFFSET) {
                ret = -EIO;
            } else {
                let mut i: usize = 0;
                while i < MAX_REG_OFFSET {
                    __get_user(&mut tmp, p);
                    putreg(child, i, tmp);
                    p = p.add(1);
                    i += core::mem::size_of::<c_long>();
                }
                ret = 0;
            }
        }

        PTRACE_GET_THREAD_AREA => {
            ret = ptrace_get_thread_area(child, addr, vp);
        }

        PTRACE_SET_THREAD_AREA => {
            ret = ptrace_set_thread_area(child, addr, vp);
        }

        _ => {
            ret = ptrace_request(child, request, addr, data);
            if ret == -EIO {
                ret = subarch_ptrace(child, request, addr, data);
            }
        }
    }

    ret as c_long
}

unsafe fn send_sigtrap(regs: *mut uml_pt_regs, _error_code: c_int) {
    /* Send us the fake SIGTRAP */
    force_sig_fault(
        SIGTRAP,
        TRAP_BRKPT,
        /* User-mode eip? */
        if UPT_IS_USER(regs) {
            UPT_IP(regs) as *mut c_void
        } else {
            core::ptr::null_mut()
        },
    );
}

/*
 * XXX Check TIF_SINGLESTEP for singlestepping check and
 * PT_PTRACED vs TIF_SYSCALL_TRACE for syscall tracing check
 */
pub unsafe fn syscall_trace_enter(regs: *mut pt_regs) -> c_int {
    audit_syscall_entry(
        UPT_SYSCALL_NR(&mut (*regs).regs),
        UPT_SYSCALL_ARG1(&mut (*regs).regs),
        UPT_SYSCALL_ARG2(&mut (*regs).regs),
        UPT_SYSCALL_ARG3(&mut (*regs).regs),
        UPT_SYSCALL_ARG4(&mut (*regs).regs),
    );

    if test_thread_flag(TIF_SYSCALL_TRACEPOINT) {
        trace_sys_enter(regs, UPT_SYSCALL_NR(&mut (*regs).regs));
    }

    if !test_thread_flag(TIF_SYSCALL_TRACE) {
        return 0;
    }

    (!ptrace_report_syscall_permit_entry(regs)) as c_int
}

pub unsafe fn syscall_trace_leave(regs: *mut pt_regs) {
    let ptraced = (*current).ptrace;

    audit_syscall_exit(regs);

    /* Fake a debug trap */
    if test_thread_flag(TIF_SINGLESTEP) {
        send_sigtrap(&mut (*regs).regs, 0);
    }

    if test_thread_flag(TIF_SYSCALL_TRACEPOINT) {
        trace_sys_exit(regs, PT_REGS_SYSCALL_RET(regs));
    }

    if !test_thread_flag(TIF_SYSCALL_TRACE) {
        return;
    }

    ptrace_report_syscall_exit(regs, 0);
    /* force do_signal() --> is_syscall() */
    if ptraced & PT_PTRACED != 0 {
        set_thread_flag(TIF_SIGPENDING);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
