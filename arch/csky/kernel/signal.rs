// SPDX-License-Identifier: GPL-2.0

// Translated from the corresponding C implementation. Kernel dependencies are
// supplied by the surrounding tree.

#[cfg(CONFIG_CPU_HAS_FPU)]
unsafe fn restore_fpu_state(sc: *mut sigcontext) -> i32 {
    let mut err: i32 = 0;
    let mut user_fp: user_fp = core::mem::zeroed();

    err |= __copy_from_user(
        &mut user_fp as *mut user_fp as *mut core::ffi::c_void,
        &(*sc).sc_user_fp as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<user_fp>(),
    ) as i32;

    restore_from_user_fp(&mut user_fp);

    err
}

#[cfg(CONFIG_CPU_HAS_FPU)]
unsafe fn save_fpu_state(sc: *mut sigcontext) -> i32 {
    let mut user_fp: user_fp = core::mem::zeroed();

    save_to_user_fp(&mut user_fp);

    __copy_to_user(
        &mut (*sc).sc_user_fp as *mut _ as *mut core::ffi::c_void,
        &user_fp as *const user_fp as *const core::ffi::c_void,
        core::mem::size_of::<user_fp>(),
    ) as i32
}

#[cfg(not(CONFIG_CPU_HAS_FPU))]
unsafe fn restore_fpu_state(_sigcontext: *mut sigcontext) -> i32 { 0 }

#[cfg(not(CONFIG_CPU_HAS_FPU))]
unsafe fn save_fpu_state(_sigcontext: *mut sigcontext) -> i32 { 0 }

#[repr(C)]
struct rt_sigframe {
    // pad[3] is compatible with the same struct defined in
    // gcc/libgcc/config/csky/linux-unwind.h
    pad: [i32; 3],
    info: siginfo,
    uc: ucontext,
}

unsafe fn restore_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext) -> i32 {
    let mut err: i32 = 0;
    let sr = (*regs).sr;

    // sc_pt_regs is structured the same as the start of pt_regs
    err |= __copy_from_user(
        regs as *mut core::ffi::c_void,
        &(*sc).sc_pt_regs as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<pt_regs>(),
    ) as i32;

    // BIT(0) of regs->sr is Condition Code/Carry bit
    (*regs).sr = (sr & !1) | ((*regs).sr & 1);

    // Restore the floating-point state.
    err |= restore_fpu_state(sc);

    err
}

pub unsafe extern "C" fn rt_sigreturn() -> long {
    let regs: *mut pt_regs = current_pt_regs();
    let mut frame: *mut rt_sigframe;
    let mut set: sigset_t = core::mem::zeroed();

    // Always make any pending restarted system calls return -EINTR
    (*current()).restart_block.fn_ = do_no_restart_syscall;

    frame = (*regs).usp as *mut rt_sigframe;

    if !access_ok(frame as *const core::ffi::c_void, core::mem::size_of::<rt_sigframe>()) {
        return badframe_rt_sigreturn();
    }

    if __copy_from_user(
        &mut set as *mut _ as *mut core::ffi::c_void,
        &(*frame).uc.uc_sigmask as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<sigset_t>(),
    ) != 0 {
        return badframe_rt_sigreturn();
    }

    set_current_blocked(&mut set);

    if restore_sigcontext(regs, &mut (*frame).uc.uc_mcontext) != 0 {
        return badframe_rt_sigreturn();
    }

    if restore_altstack(&mut (*frame).uc.uc_stack) != 0 {
        return badframe_rt_sigreturn();
    }

    (*regs).a0
}

unsafe fn badframe_rt_sigreturn() -> long {
    force_sig(SIGSEGV);
    0
}

unsafe fn setup_sigcontext(frame: *mut rt_sigframe, regs: *mut pt_regs) -> i32 {
    let sc: *mut sigcontext = &mut (*frame).uc.uc_mcontext;
    let mut err: i32 = 0;

    err |= __copy_to_user(
        &mut (*sc).sc_pt_regs as *mut _ as *mut core::ffi::c_void,
        regs as *const core::ffi::c_void,
        core::mem::size_of::<pt_regs>(),
    ) as i32;
    err |= save_fpu_state(sc);
    err
}

unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, framesize: usize) -> *mut core::ffi::c_void {
    let mut sp: usize = (*regs).usp;

    // Default to using normal stack
    if on_sig_stack(sp) && !likely(on_sig_stack(sp.wrapping_sub(framesize))) {
        return usize::MAX as *mut core::ffi::c_void;
    }

    // This is the X/Open sanctioned signal stack switching.
    sp = sigsp(sp, ksig).wrapping_sub(framesize);

    // Align the stack frame.
    sp &= !7usize;

    sp as *mut core::ffi::c_void
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> i32 {
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe>()) as *mut rt_sigframe;
    let mut err: i32 = 0;

    if !access_ok(frame as *const core::ffi::c_void, core::mem::size_of::<rt_sigframe>()) { return -EFAULT; }

    err |= copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info);
    err |= __put_user(0, &mut (*frame).uc.uc_flags);
    err |= __put_user(core::ptr::null_mut(), &mut (*frame).uc.uc_link);
    err |= __save_altstack(&mut (*frame).uc.uc_stack, (*regs).usp);
    err |= setup_sigcontext(frame, regs);
    err |= __copy_to_user(&mut (*frame).uc.uc_sigmask as *mut _ as *mut core::ffi::c_void, set as *const core::ffi::c_void, core::mem::size_of::<sigset_t>()) as i32;
    if err != 0 { return -EFAULT; }

    (*regs).lr = VDSO_SYMBOL((*current()).mm.context.vdso, rt_sigreturn) as usize;
    (*regs).pc = (*ksig).ka.sa.sa_handler as usize;
    (*regs).usp = frame as usize;
    (*regs).a0 = (*ksig).sig;
    (*regs).a1 = &mut (*frame).info as *mut _ as usize;
    (*regs).a2 = &mut (*frame).uc as *mut _ as usize;
    0
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    let oldset = sigmask_to_save();
    let mut ret: i32;
    if in_syscall(regs) {
        forget_syscall(regs);
        match (*regs).a0 {
            -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => (*regs).a0 = -EINTR,
            -ERESTARTSYS if ((*ksig).ka.sa.sa_flags & SA_RESTART) == 0 => (*regs).a0 = -EINTR,
            -ERESTARTSYS | -ERESTARTNOINTR => { (*regs).a0 = (*regs).orig_a0; (*regs).pc -= TRAP0_SIZE; },
            _ => {}
        }
    }
    ret = setup_rt_frame(ksig, oldset, regs);
    signal_setup_done(ret, ksig, 0);
}

unsafe fn do_signal(regs: *mut pt_regs) {
    let mut ksig: ksignal = core::mem::zeroed();
    if get_signal(&mut ksig) { handle_signal(&mut ksig, regs); return; }
    if in_syscall(regs) {
        forget_syscall(regs);
        match (*regs).a0 {
            -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => { (*regs).a0 = (*regs).orig_a0; (*regs).pc -= TRAP0_SIZE; },
            -ERESTART_RESTARTBLOCK => { (*regs).a0 = (*regs).orig_a0; *regs_syscallid(regs) = __NR_restart_syscall; (*regs).pc -= TRAP0_SIZE; },
            _ => {}
        }
    }
    restore_saved_sigmask();
}

pub unsafe extern "C" fn do_notify_resume(regs: *mut pt_regs, thread_info_flags: usize) {
    if thread_info_flags & _TIF_UPROBE != 0 { uprobe_notify_resume(regs); }
    if thread_info_flags & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 { do_signal(regs); }
    if thread_info_flags & _TIF_NOTIFY_RESUME != 0 { resume_user_mode_work(regs); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
