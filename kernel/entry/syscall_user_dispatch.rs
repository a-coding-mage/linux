// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Collabora Ltd.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

static mut syscall_user_dispatch_allowed: bool = true;

unsafe fn trigger_sigsys(regs: *mut pt_regs) {
    let mut info: kernel_siginfo = core::mem::zeroed();

    clear_siginfo(&mut info);
    info.si_signo = SIGSYS;
    info.si_code = SYS_USER_DISPATCH;
    info.si_call_addr = KSTK_EIP(current) as *mut core::ffi::c_void;
    info.si_errno = 0;
    info.si_arch = syscall_get_arch(current);
    info.si_syscall = syscall_get_nr(current, regs);

    force_sig_info(&info);
}

pub unsafe fn syscall_user_dispatch(regs: *mut pt_regs) -> bool {
    let sd: *mut syscall_user_dispatch = &mut (*current).syscall_dispatch;
    let mut state: core::ffi::c_char;

    if instruction_pointer(regs).wrapping_sub((*sd).offset) < (*sd).len {
        return false;
    }

    if arch_syscall_is_vdso_sigreturn(regs) {
        return false;
    }

    if !(*sd).selector.is_null() {
        /*
         * access_ok() is performed once, at prctl time, when
         * the selector is loaded by userspace.
         */
        if __get_user(&mut state, (*sd).selector) != 0 {
            force_exit_sig(SIGSEGV);
            return true;
        }

        if state == SYSCALL_DISPATCH_FILTER_ALLOW {
            return false;
        }

        if state != SYSCALL_DISPATCH_FILTER_BLOCK {
            force_exit_sig(SIGSYS);
            return true;
        }
    }

    (*sd).on_dispatch = true;
    syscall_rollback(current, regs);
    trigger_sigsys(regs);

    true
}

unsafe fn task_set_syscall_user_dispatch(
    task: *mut task_struct,
    mode: c_ulong,
    mut offset: c_ulong,
    mut len: c_ulong,
    selector: *mut core::ffi::c_char,
) -> c_int {
    match mode {
        PR_SYS_DISPATCH_OFF => {
            if offset != 0 || len != 0 || !selector.is_null() {
                return -EINVAL;
            }
        }
        PR_SYS_DISPATCH_EXCLUSIVE_ON => {
            /*
             * Validate the direct dispatcher region just for basic
             * sanity against overflow and a 0-sized dispatcher
             * region.  If the user is able to submit a syscall from
             * an address, that address is obviously valid.
             */
            if offset != 0 && offset.wrapping_add(len) <= offset {
                return -EINVAL;
            }
        }
        PR_SYS_DISPATCH_INCLUSIVE_ON => {
            if len == 0 || offset.wrapping_add(len) <= offset {
                return -EINVAL;
            }
            /*
             * Invert the range, the check in syscall_user_dispatch()
             * supports wrap-around.
             */
            offset = offset.wrapping_add(len);
            len = (0 as c_ulong).wrapping_sub(len);
        }
        _ => return -EINVAL,
    }

    /* Arming can be denied at runtime via sysctl, disarming is allowed */
    if mode != PR_SYS_DISPATCH_OFF && !syscall_user_dispatch_allowed {
        return -EPERM;
    }

    /*
     * access_ok() will clear memory tags for tagged addresses
     * if current has memory tagging enabled.
     *
     * To enable a tracer to set a tracees selector the
     * selector address must be untagged for access_ok(),
     * otherwise an untagged tracer will always fail to set a
     * tagged tracees selector.
     */
    if mode != PR_SYS_DISPATCH_OFF
        && !selector.is_null()
        && !access_ok(untagged_addr(selector), core::mem::size_of::<core::ffi::c_char>())
    {
        return -EFAULT;
    }

    (*task).syscall_dispatch.selector = selector;
    (*task).syscall_dispatch.offset = offset;
    (*task).syscall_dispatch.len = len;
    (*task).syscall_dispatch.on_dispatch = false;

    if mode != PR_SYS_DISPATCH_OFF {
        set_task_syscall_work(task, SYSCALL_USER_DISPATCH);
    } else {
        clear_task_syscall_work(task, SYSCALL_USER_DISPATCH);
    }

    0
}

pub unsafe fn set_syscall_user_dispatch(
    mode: c_ulong,
    offset: c_ulong,
    len: c_ulong,
    selector: *mut core::ffi::c_char,
) -> c_int {
    task_set_syscall_user_dispatch(current, mode, offset, len, selector)
}

pub unsafe fn syscall_user_dispatch_get_config(
    task: *mut task_struct,
    size: c_ulong,
    data: *mut core::ffi::c_void,
) -> c_int {
    let sd: *mut syscall_user_dispatch = &mut (*task).syscall_dispatch;
    let mut cfg: ptrace_sud_config = core::mem::zeroed();

    if size != core::mem::size_of::<ptrace_sud_config>() as c_ulong {
        return -EINVAL;
    }

    if test_task_syscall_work(task, SYSCALL_USER_DISPATCH) {
        cfg.mode = PR_SYS_DISPATCH_ON;
    } else {
        cfg.mode = PR_SYS_DISPATCH_OFF;
    }

    cfg.offset = (*sd).offset;
    cfg.len = (*sd).len;
    cfg.selector = (*sd).selector as usize as u64;

    if copy_to_user(data, &cfg, core::mem::size_of::<ptrace_sud_config>()) != 0 {
        return -EFAULT;
    }

    0
}

pub unsafe fn syscall_user_dispatch_set_config(
    task: *mut task_struct,
    size: c_ulong,
    data: *mut core::ffi::c_void,
) -> c_int {
    let mut cfg: ptrace_sud_config = core::mem::zeroed();

    if size != core::mem::size_of::<ptrace_sud_config>() as c_ulong {
        return -EINVAL;
    }

    if copy_from_user(&mut cfg, data, core::mem::size_of::<ptrace_sud_config>()) != 0 {
        return -EFAULT;
    }

    task_set_syscall_user_dispatch(
        task,
        cfg.mode,
        cfg.offset,
        cfg.len,
        cfg.selector as usize as *mut core::ffi::c_char,
    )
}

// CONFIG_PROC_SYSCTL condition: preserve the source's conditional intent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
