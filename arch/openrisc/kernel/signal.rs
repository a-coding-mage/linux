// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC signal.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct rt_sigframe {
    pub info: siginfo,
    pub uc: ucontext,
    pub retcode: [u8; 16], // trampoline code
}

extern "C" {
    fn _sys_rt_sigreturn(regs: *mut pt_regs) -> c_long;
    fn do_work_pending(regs: *mut pt_regs, thread_flags: c_uint, syscall: c_int) -> c_int;
}

#[cfg(CONFIG_FPU)]
unsafe fn restore_fp_state(sc: *mut sigcontext) -> c_long {
    let err = __copy_from_user(
        &mut (*current).thread.fpcsr as *mut _ as *mut c_void,
        &(*sc).fpcsr as *const _ as *const c_void,
        core::mem::size_of::<c_ulong>(),
    );
    if unlikely(err != 0) {
        return err;
    }
    // Restore the FPU state
    restore_fpu(current);
    0
}

#[cfg(CONFIG_FPU)]
unsafe fn save_fp_state(sc: *mut sigcontext) -> c_long {
    // Sync the user FPU state so we can copy to sigcontext
    save_fpu(current);
    __copy_to_user(
        &mut (*sc).fpcsr as *mut _ as *mut c_void,
        &(*current).thread.fpcsr as *const _ as *const c_void,
        core::mem::size_of::<c_ulong>(),
    )
}

#[cfg(not(CONFIG_FPU))]
unsafe fn save_fp_state(_sc: *mut sigcontext) -> c_long { 0 }
#[cfg(not(CONFIG_FPU))]
unsafe fn restore_fp_state(_sc: *mut sigcontext) -> c_long { 0 }

unsafe fn restore_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext) -> c_int {
    let old_sr = (*regs).sr;
    let mut err: c_int = 0;

    // Always make any pending restarted system calls return -EINTR
    (*current).restart_block.fn_ = do_no_restart_syscall;

    // Restore the regs from &sc->regs.
    err |= __copy_from_user((*regs).gpr.as_mut_ptr() as *mut c_void,
        (*sc).regs.gpr.as_ptr() as *const c_void, 32 * core::mem::size_of::<c_ulong>()) as c_int;
    err |= __copy_from_user(&mut (*regs).pc as *mut _ as *mut c_void,
        &(*sc).regs.pc as *const _ as *const c_void, core::mem::size_of::<c_ulong>()) as c_int;
    err |= __copy_from_user(&mut (*regs).sr as *mut _ as *mut c_void,
        &(*sc).regs.sr as *const _ as *const c_void, core::mem::size_of::<c_ulong>()) as c_int;
    err |= restore_fp_state(sc) as c_int;

    // keep the privileged SR bits kernel owned, restore only user flags
    (*regs).sr = (old_sr & !SPR_SR_USER_MASK) | ((*regs).sr & SPR_SR_USER_MASK);
    (*regs).orig_gpr11 = -1;
    err
}

unsafe fn _sys_rt_sigreturn_impl(regs: *mut pt_regs) -> c_long {
    let frame = (*regs).sp as *mut rt_sigframe;
    let mut set: sigset_t = core::mem::zeroed();
    if ((frame as usize) & 3) != 0 { goto_badframe(regs); return 0; }
    if !access_ok(frame as *const c_void, core::mem::size_of::<rt_sigframe>()) { goto_badframe(regs); return 0; }
    if __copy_from_user(&mut set as *mut _ as *mut c_void,
        &(*frame).uc.uc_sigmask as *const _ as *const c_void, core::mem::size_of::<sigset_t>()) != 0 { goto_badframe(regs); return 0; }
    set_current_blocked(&set);
    if restore_sigcontext(regs, &mut (*frame).uc.uc_mcontext) != 0 { goto_badframe(regs); return 0; }
    if restore_altstack(&(*frame).uc.uc_stack) != 0 { goto_badframe(regs); return 0; }
    (*regs).gpr[11] as c_long
}

unsafe fn goto_badframe(_regs: *mut pt_regs) { force_sig(SIGSEGV); }

unsafe fn setup_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext) -> c_int {
    let mut err: c_int = 0;
    err |= __copy_to_user((*sc).regs.gpr.as_mut_ptr() as *mut c_void, regs as *const c_void,
        32 * core::mem::size_of::<c_ulong>()) as c_int;
    err |= __copy_to_user(&mut (*sc).regs.pc as *mut _ as *mut c_void,
        &(*regs).pc as *const _ as *const c_void, core::mem::size_of::<c_ulong>()) as c_int;
    err |= __copy_to_user(&mut (*sc).regs.sr as *mut _ as *mut c_void,
        &(*regs).sr as *const _ as *const c_void, core::mem::size_of::<c_ulong>()) as c_int;
    err |= save_fp_state(sc) as c_int;
    err
}

#[inline]
unsafe fn align_sigframe(sp: c_ulong) -> c_ulong { sp & !3 }

#[inline]
unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, frame_size: usize) -> *mut c_void {
    let mut sp = (*regs).sp;
    sp -= STACK_FRAME_OVERHEAD;
    sp = sigsp(sp, ksig);
    align_sigframe(sp - frame_size as c_ulong) as *mut c_void
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe>()) as *mut rt_sigframe;
    let mut err: c_int = 0;
    if !access_ok(frame as *const c_void, core::mem::size_of::<rt_sigframe>()) { return -EFAULT; }
    if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { err |= copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info); }
    err |= __put_user(0, &mut (*frame).uc.uc_flags);
    err |= __put_user(core::ptr::null_mut(), &mut (*frame).uc.uc_link);
    err |= __save_altstack(&mut (*frame).uc.uc_stack, (*regs).sp);
    err |= setup_sigcontext(regs, &mut (*frame).uc.uc_mcontext);
    err |= __copy_to_user(&mut (*frame).uc.uc_sigmask as *mut _ as *mut c_void, set as *const c_void, core::mem::size_of::<sigset_t>()) as c_int;
    if err != 0 { return -EFAULT; }
    let return_ip = &mut (*frame).retcode as *mut _ as c_ulong;
    err |= __put_user(0xa960u16, (*frame).retcode.as_mut_ptr() as *mut u16);
    err |= __put_user(__NR_rt_sigreturn, (*frame).retcode.as_mut_ptr().add(2) as *mut u16);
    err |= __put_user(0x20000001u32, (*frame).retcode.as_mut_ptr().add(4) as *mut u32);
    err |= __put_user(0x15000000u32, (*frame).retcode.as_mut_ptr().add(8) as *mut u32);
    if err != 0 { return -EFAULT; }
    (*regs).pc = (*ksig).ka.sa.sa_handler as c_ulong;
    (*regs).gpr[9] = return_ip; (*regs).gpr[3] = (*ksig).sig as c_ulong;
    (*regs).gpr[4] = &mut (*frame).info as *mut _ as c_ulong;
    (*regs).gpr[5] = &mut (*frame).uc as *mut _ as c_ulong;
    (*regs).sp = frame as c_ulong;
    0
}

#[inline]
unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    rseq_signal_deliver(ksig, regs);
    let ret = setup_rt_frame(ksig, sigmask_to_save(), regs);
    signal_setup_done(ret, ksig, test_thread_flag(TIF_SINGLESTEP));
}

unsafe fn do_signal(regs: *mut pt_regs, syscall: c_int) -> c_int {
    let mut ksig: ksignal = core::mem::zeroed();
    let mut continue_addr = 0; let mut restart_addr = 0; let mut retval = 0; let mut restart = 0;
    if syscall != 0 {
        continue_addr = (*regs).pc; restart_addr = continue_addr - 4; retval = (*regs).gpr[11];
        match retval as c_long { -ERESTART_RESTARTBLOCK => { restart = -2; }, -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => {}, _ => {} }
        if retval as c_long == -ERESTART_RESTARTBLOCK || retval as c_long == -ERESTARTNOHAND || retval as c_long == -ERESTARTSYS || retval as c_long == -ERESTARTNOINTR { restart += 1; (*regs).gpr[11] = (*regs).orig_gpr11; (*regs).pc = restart_addr; }
    }
    if get_signal(&mut ksig) {
        if unlikely(restart != 0) && (*regs).pc == restart_addr && ((retval as c_long == -ERESTARTNOHAND) || (retval as c_long == -ERESTART_RESTARTBLOCK) || (retval as c_long == -ERESTARTSYS && (*ksig).ka.sa.sa_flags & SA_RESTART == 0)) { (*regs).gpr[11] = -EINTR as c_ulong; (*regs).pc = continue_addr; }
        handle_signal(&mut ksig, regs);
    } else { restore_saved_sigmask(); if unlikely(restart != 0) && (*regs).pc == restart_addr { (*regs).pc = continue_addr; return restart; } }
    0
}

#[no_mangle]
pub unsafe extern "C" fn do_work_pending_impl(regs: *mut pt_regs, mut thread_flags: c_uint, mut syscall: c_int) -> c_int {
    loop { if likely(thread_flags & _TIF_NEED_RESCHED != 0) { schedule(); } else { if unlikely(!user_mode(regs)) { return 0; } local_irq_enable(); if thread_flags & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 { let restart = do_signal(regs, syscall); if unlikely(restart != 0) { return restart; } syscall = 0; } else { resume_user_mode_work(regs); } } local_irq_disable(); thread_flags = read_thread_flags(); if thread_flags & _TIF_WORK_MASK == 0 { break; } } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
