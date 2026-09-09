// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation.

pub unsafe fn save_fpu_state(regs: *mut pt_regs, fpu: *mut __siginfo_fpu_t) -> i32 {
    let mut err: i32 = 0;

    // CONFIG_SMP selects the corresponding C preprocessor branch.
    #[cfg(feature = "CONFIG_SMP")]
    {
        if test_tsk_thread_flag(current, TIF_USEDFPU) {
            put_psr(get_psr() | PSR_EF);
            fpsave(
                &mut (*current).thread.float_regs[0],
                &mut (*current).thread.fsr,
                &mut (*current).thread.fpqueue[0],
                &mut (*current).thread.fpqdepth,
            );
            (*regs).psr &= !PSR_EF;
            clear_tsk_thread_flag(current, TIF_USEDFPU);
        }
    }
    #[cfg(not(feature = "CONFIG_SMP"))]
    {
        if current == last_task_used_math {
            put_psr(get_psr() | PSR_EF);
            fpsave(
                &mut (*current).thread.float_regs[0],
                &mut (*current).thread.fsr,
                &mut (*current).thread.fpqueue[0],
                &mut (*current).thread.fpqdepth,
            );
            last_task_used_math = core::ptr::null_mut();
            (*regs).psr &= !PSR_EF;
        }
    }

    err |= __copy_to_user(
        &mut (*fpu).si_float_regs[0],
        &(*current).thread.float_regs[0],
        core::mem::size_of::<c_ulong>() * 32,
    );
    err |= __put_user((*current).thread.fsr, &mut (*fpu).si_fsr);
    err |= __put_user((*current).thread.fpqdepth, &mut (*fpu).si_fpqdepth);
    if (*current).thread.fpqdepth != 0 {
        err |= __copy_to_user(
            &mut (*fpu).si_fpqueue[0],
            &(*current).thread.fpqueue[0],
            (core::mem::size_of::<c_ulong>() + core::mem::size_of::<*mut c_ulong>()) * 16,
        );
    }
    clear_used_math();
    err
}

pub unsafe fn restore_fpu_state(regs: *mut pt_regs, fpu: *mut __siginfo_fpu_t) -> i32 {
    let mut err: i32;

    if (fpu as c_ulong) & 3 != 0 {
        return -EFAULT;
    }

    #[cfg(feature = "CONFIG_SMP")]
    {
        if test_tsk_thread_flag(current, TIF_USEDFPU) {
            (*regs).psr &= !PSR_EF;
        }
    }
    #[cfg(not(feature = "CONFIG_SMP"))]
    {
        if current == last_task_used_math {
            last_task_used_math = core::ptr::null_mut();
            (*regs).psr &= !PSR_EF;
        }
    }
    set_used_math();
    clear_tsk_thread_flag(current, TIF_USEDFPU);

    if !access_ok(fpu, core::mem::size_of::<__siginfo_fpu_t>()) {
        return -EFAULT;
    }

    err = __copy_from_user(
        &mut (*current).thread.float_regs[0],
        &(*fpu).si_float_regs[0],
        core::mem::size_of::<c_ulong>() * 32,
    );
    err |= __get_user((*current).thread.fsr, &(*fpu).si_fsr);
    err |= __get_user((*current).thread.fpqdepth, &(*fpu).si_fpqdepth);
    if (*current).thread.fpqdepth != 0 {
        err |= __copy_from_user(
            &mut (*current).thread.fpqueue[0],
            &(*fpu).si_fpqueue[0],
            (core::mem::size_of::<c_ulong>() + core::mem::size_of::<*mut c_ulong>()) * 16,
        );
    }
    err
}

pub unsafe fn save_rwin_state(wsaved: i32, rwin: *mut __siginfo_rwin_t) -> i32 {
    let mut err = __put_user(wsaved, &mut (*rwin).wsaved);
    let mut i = 0;
    while i < wsaved {
        let rp = &mut (*current_thread_info()).reg_window[i as usize];
        let fp = (*current_thread_info()).rwbuf_stkptrs[i as usize];
        err |= copy_to_user(
            &mut (*rwin).reg_window[i as usize],
            rp,
            core::mem::size_of::<reg_window32>(),
        );
        err |= __put_user(fp, &mut (*rwin).rwbuf_stkptrs[i as usize]);
        i += 1;
    }
    err
}

pub unsafe fn restore_rwin_state(rp: *mut __siginfo_rwin_t) -> i32 {
    let t = current_thread_info();
    let mut wsaved: i32;

    if (rp as c_ulong) & 3 != 0 {
        return -EFAULT;
    }

    get_user(wsaved, &(*rp).wsaved);
    if wsaved > NSWINS {
        return -EFAULT;
    }

    let mut err = 0;
    let mut i = 0;
    while i < wsaved {
        err |= copy_from_user(
            &mut (*t).reg_window[i as usize],
            &(*rp).reg_window[i as usize],
            core::mem::size_of::<reg_window32>(),
        );
        err |= __get_user(
            (*t).rwbuf_stkptrs[i as usize],
            &(*rp).rwbuf_stkptrs[i as usize],
        );
        i += 1;
    }
    if err != 0 {
        return err;
    }

    (*t).w_saved = wsaved;
    synchronize_user_stack();
    if (*t).w_saved != 0 {
        return -EFAULT;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
