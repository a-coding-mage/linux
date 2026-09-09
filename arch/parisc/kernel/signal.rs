// SPDX-License-Identifier: GPL-2.0
/*
 * PA-RISC architecture-specific signal handling support.
 *
 * This is a source-level Rust translation of signal.c.  Kernel-provided
 * types, constants, globals, and functions are intentionally referenced as
 * external dependencies.
 */

const DEBUG_SIG: bool = false;
const DEBUG_SIG_LEVEL: i32 = 2;

macro_rules! DBG {
    ($level:expr, $($arg:tt)*) => {
        if DEBUG_SIG && DEBUG_SIG_LEVEL >= $level { printk(format_args!($($arg)*)); }
    };
}

#[inline]
unsafe fn A<T>(x: *const T) -> ::core::ffi::c_ulong { x as usize as ::core::ffi::c_ulong }

unsafe fn restore_sigcontext(sc: *mut sigcontext, regs: *mut pt_regs) -> ::core::ffi::c_long {
    let mut err: ::core::ffi::c_long = 0;
    err |= __copy_from_user((*regs).gr.as_mut_ptr() as *mut _, (*sc).sc_gr.as_ptr() as *const _, core::mem::size_of_val(&(*regs).gr));
    err |= __copy_from_user((*regs).fr.as_mut_ptr() as *mut _, (*sc).sc_fr.as_ptr() as *const _, core::mem::size_of_val(&(*regs).fr));
    err |= __copy_from_user((*regs).iaoq.as_mut_ptr() as *mut _, (*sc).sc_iaoq.as_ptr() as *const _, core::mem::size_of_val(&(*regs).iaoq));
    err |= __copy_from_user((*regs).iasq.as_mut_ptr() as *mut _, (*sc).sc_iasq.as_ptr() as *const _, core::mem::size_of_val(&(*regs).iasq));
    err |= __get_user(&mut (*regs).sar, &(*sc).sc_sar);
    DBG!(2, "{}: iaoq is {:#lx} / {:#lx}\n", "restore_sigcontext", (*regs).iaoq[0], (*regs).iaoq[1]);
    DBG!(2, "{}: r28 is {}\n", "restore_sigcontext", (*regs).gr[28]);
    err
}

pub unsafe extern "C" fn sys_rt_sigreturn(regs: *mut pt_regs, in_syscall: i32) {
    let mut frame: *mut rt_sigframe;
    let mut set: sigset_t = core::mem::zeroed();
    let usp = (*regs).gr[30] & !1;
    let mut sigframe_size = PARISC_RT_SIGFRAME_SIZE;
    #[cfg(CONFIG_COMPAT)] let compat_frame: *mut compat_rt_sigframe;
    #[cfg(CONFIG_COMPAT)] if is_compat_task() { sigframe_size = PARISC_RT_SIGFRAME_SIZE32; }
    (*current).restart_block.fn_ = do_no_restart_syscall;
    frame = (usp - sigframe_size) as *mut rt_sigframe;
    (*regs).orig_r28 = 1;
    #[cfg(CONFIG_COMPAT)] {
        compat_frame = frame as *mut compat_rt_sigframe;
        if is_compat_task() {
            if get_compat_sigset(&mut set, &(*compat_frame).uc.uc_sigmask) != 0 { force_sig(SIGSEGV); return; }
            if restore_sigcontext32(&mut (*compat_frame).uc.uc_mcontext, &mut (*compat_frame).regs, regs) != 0 { force_sig(SIGSEGV); return; }
            if compat_restore_altstack(&mut (*compat_frame).uc.uc_stack) != 0 { force_sig(SIGSEGV); return; }
        } else
    }
    {
        if __copy_from_user(&mut set, &(*frame).uc.uc_sigmask, core::mem::size_of::<sigset_t>()) != 0 { force_sig(SIGSEGV); return; }
        if restore_sigcontext(&mut (*frame).uc.uc_mcontext, regs) != 0 { force_sig(SIGSEGV); return; }
        if restore_altstack(&mut (*frame).uc.uc_stack) != 0 { force_sig(SIGSEGV); return; }
    }
    set_current_blocked(&set);
    if in_syscall != 0 { (*regs).gr[31] = (*regs).iaoq[0]; }
}

#[inline]
unsafe fn get_sigframe(ka: *mut k_sigaction, mut sp: ::core::ffi::c_ulong, _frame_size: usize) -> *mut core::ffi::c_void {
    if ((*ka).sa.sa_flags & SA_ONSTACK) != 0 && sas_ss_flags(sp) == 0 { sp = ((*current).sas_ss_sp + 0x7f) & !0x3f; }
    sp as *mut core::ffi::c_void
}

unsafe fn setup_sigcontext(sc: *mut sigcontext, regs: *mut pt_regs, in_syscall: ::core::ffi::c_long) -> ::core::ffi::c_long {
    let mut flags: ::core::ffi::c_ulong = 0; let mut err: ::core::ffi::c_long = 0;
    if on_sig_stack(sc as usize as _) { flags |= PARISC_SC_FLAG_ONSTACK; }
    if in_syscall != 0 {
        flags |= PARISC_SC_FLAG_IN_SYSCALL;
        err |= __put_user((*regs).gr[31], &mut (*sc).sc_iaoq[0]);
        err |= __put_user((*regs).gr[31].wrapping_add(4), &mut (*sc).sc_iaoq[1]);
        err |= __put_user((*regs).sr[3], &mut (*sc).sc_iasq[0]); err |= __put_user((*regs).sr[3], &mut (*sc).sc_iasq[1]);
    } else { err |= __copy_to_user((*sc).sc_iaoq.as_mut_ptr(), (*regs).iaoq.as_ptr(), core::mem::size_of_val(&(*regs).iaoq)); err |= __copy_to_user((*sc).sc_iasq.as_mut_ptr(), (*regs).iasq.as_ptr(), core::mem::size_of_val(&(*regs).iasq)); }
    err |= __put_user(flags, &mut (*sc).sc_flags);
    err |= __copy_to_user((*sc).sc_gr.as_mut_ptr(), (*regs).gr.as_ptr(), core::mem::size_of_val(&(*regs).gr));
    err |= __copy_to_user((*sc).sc_fr.as_mut_ptr(), (*regs).fr.as_ptr(), core::mem::size_of_val(&(*regs).fr));
    err |= __put_user((*regs).sar, &mut (*sc).sc_sar); err
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs, in_syscall: ::core::ffi::c_long) -> i32 {
    let mut usp = (*regs).gr[30] & !1; let mut size = PARISC_RT_SIGFRAME_SIZE;
    #[cfg(CONFIG_COMPAT)] if is_compat_task() { usp = usp as u32 as _; size = PARISC_RT_SIGFRAME_SIZE32; }
    let frame = get_sigframe(&mut (*ksig).ka, usp, size) as *mut rt_sigframe;
    if frame as usize >= TASK_SIZE_MAX - size { return -EFAULT; }
    let mut err = 0;
    #[cfg(CONFIG_COMPAT)] if is_compat_task() { let cf = frame as *mut compat_rt_sigframe; err |= copy_siginfo_to_user32(&mut (*cf).info, &mut (*ksig).info); err |= __compat_save_altstack(&mut (*cf).uc.uc_stack, (*regs).gr[30]); err |= setup_sigcontext32(&mut (*cf).uc.uc_mcontext, &mut (*cf).regs, regs, in_syscall); err |= put_compat_sigset(&mut (*cf).uc.uc_sigmask, set, core::mem::size_of::<compat_sigset_t>()); } else
    { err |= copy_siginfo_to_user(&mut (*frame).info, &mut (*ksig).info); err |= __save_altstack(&mut (*frame).uc.uc_stack, (*regs).gr[30]); err |= setup_sigcontext(&mut (*frame).uc.uc_mcontext, regs, in_syscall); err |= __copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>()); }
    if err != 0 { return -EFAULT; }
    let mut rp = VDSO32_SYMBOL(current, sigtramp_rt); #[cfg(CONFIG_64BIT)] if !is_compat_task() { rp = VDSO64_SYMBOL(current, sigtramp_rt); }
    if in_syscall != 0 { rp += 4 * 4; }
    let haddr = A((*ksig).ka.sa.sa_handler as *const _);
    (*regs).gr[31] = if in_syscall != 0 { haddr } else { haddr };
    if in_syscall == 0 { (*regs).gr[0] = USER_PSW; (*regs).iaoq[0] = haddr | PRIV_USER; (*regs).iaoq[1] = (*regs).iaoq[0] + 4; }
    (*regs).gr[2] = rp; (*regs).gr[26] = (*ksig).sig;
    (*regs).gr[25] = A(&(*frame).info); (*regs).gr[24] = A(&(*frame).uc); (*regs).gr[30] = A(frame) + size; 0
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs, in_syscall: ::core::ffi::c_long) { let oldset = sigmask_to_save(); let ret = setup_rt_frame(ksig, oldset, regs, in_syscall); signal_setup_done(ret, ksig, test_thread_flag(TIF_SINGLESTEP) || test_thread_flag(TIF_BLOCKSTEP)); }

unsafe fn check_syscallno_in_delay_branch(regs: *mut pt_regs) {
    let mut opcode = 0u32; (*regs).gr[31] = (*regs).gr[31].wrapping_sub(8);
    if get_user(&mut opcode, ((((*regs).gr[31] & !3) + 4) as *const u32)) != 0 { return; }
    if (opcode & 0xffff0000) == 0x34140000 || opcode == INSN_NOP { return; }
    if (opcode & 0xffe0ffff) == 0x08000254 { let source = ((opcode >> 16) & 31) as usize; (*regs).gr[source] = (*regs).gr[20]; }
}

unsafe fn syscall_restart(regs: *mut pt_regs, ka: *mut k_sigaction) { if (*regs).orig_r28 != 0 { return; } (*regs).orig_r28 = 1; match (*regs).gr[28] { -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => (*regs).gr[28] = -EINTR, -ERESTARTSYS if ((*ka).sa.sa_flags & SA_RESTART) == 0 => (*regs).gr[28] = -EINTR, -ERESTARTSYS | -ERESTARTNOINTR => check_syscallno_in_delay_branch(regs), _ => {} } }

unsafe fn insert_restart_trampoline(regs: *mut pt_regs) { if (*regs).orig_r28 != 0 { return; } (*regs).orig_r28 = 1; match (*regs).gr[28] { -ERESTART_RESTARTBLOCK => { (*regs).gr[31] = VDSO32_SYMBOL(current, restart_syscall); }, -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => check_syscallno_in_delay_branch(regs), _ => {} } }

unsafe fn do_signal(regs: *mut pt_regs, in_syscall: ::core::ffi::c_long) { let mut ksig: ksignal = core::mem::zeroed(); let restart = in_syscall != 0; if get_signal(&mut ksig) { if restart { syscall_restart(regs, &mut ksig.ka); } handle_signal(&mut ksig, regs, in_syscall); } else { if restart { insert_restart_trampoline(regs); } restore_saved_sigmask(); } }

pub unsafe extern "C" fn do_notify_resume(regs: *mut pt_regs, in_syscall: ::core::ffi::c_long) { if test_thread_flag(TIF_SIGPENDING) || test_thread_flag(TIF_NOTIFY_SIGNAL) { do_signal(regs, in_syscall); } if test_thread_flag(TIF_NOTIFY_RESUME) { resume_user_mode_work(regs); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
