// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/kernel/signal.c
 *
 *  Copyright (C) 1995  Linus Torvalds
 *
 *  1997-11-02  Modified for POSIX.1b signals by Richard Henderson
 */

// C headers supply the kernel and architecture dependencies used below.

const DEBUG_SIG: usize = 0;
const _BLOCKABLE: c_ulong = !(sigmask(SIGKILL) | sigmask(SIGSTOP));

extern "C" {
    fn ret_from_sys_call();
}

/* The OSF/1 sigprocmask calling sequence is different from the C sequence. */
pub unsafe fn alpha_schedule_user_work() {
    local_irq_enable();
    schedule();
    local_irq_disable();
}

pub unsafe fn osf_sigprocmask(how: c_int, newmask: c_ulong) -> c_ulong {
    let mut oldmask: sigset_t = core::mem::zeroed();
    let mut mask: sigset_t = core::mem::zeroed();
    siginitset(&mut mask, newmask & _BLOCKABLE);
    let mut res = sigprocmask(how, &mut mask, &mut oldmask);
    if res == 0 {
        force_successful_syscall_return();
        res = oldmask.sig[0];
    }
    res
}

pub unsafe fn osf_sigaction(sig: c_int, act: *const osf_sigaction, oact: *mut osf_sigaction) -> c_int {
    let mut new_ka: k_sigaction = core::mem::zeroed();
    let mut old_ka: k_sigaction = core::mem::zeroed();
    let mut ret: c_int;
    if !act.is_null() {
        let mut mask: old_sigset_t = 0;
        if !access_ok(act, core::mem::size_of::<osf_sigaction>())
            || __get_user(&mut new_ka.sa.sa_handler, &(*act).sa_handler)
            || __get_user(&mut new_ka.sa.sa_flags, &(*act).sa_flags)
            || __get_user(&mut mask, &(*act).sa_mask) { return -EFAULT; }
        siginitset(&mut new_ka.sa.sa_mask, mask);
        new_ka.ka_restorer = core::ptr::null_mut();
    }
    ret = do_sigaction(sig, if act.is_null() { core::ptr::null_mut() } else { &mut new_ka },
                       if oact.is_null() { core::ptr::null_mut() } else { &mut old_ka });
    if ret == 0 && !oact.is_null() {
        if !access_ok(oact, core::mem::size_of::<osf_sigaction>())
            || __put_user(old_ka.sa.sa_handler, &mut (*oact).sa_handler)
            || __put_user(old_ka.sa.sa_flags, &mut (*oact).sa_flags)
            || __put_user(old_ka.sa.sa_mask.sig[0], &mut (*oact).sa_mask) { return -EFAULT; }
    }
    ret
}

pub unsafe fn rt_sigaction(sig: c_int, act: *const sigaction, oact: *mut sigaction,
                           sigsetsize: usize, restorer: *mut c_void) -> c_int {
    let mut new_ka: k_sigaction = core::mem::zeroed();
    let mut old_ka: k_sigaction = core::mem::zeroed();
    if sigsetsize != core::mem::size_of::<sigset_t>() { return -EINVAL; }
    if !act.is_null() {
        new_ka.ka_restorer = restorer;
        if copy_from_user(&mut new_ka.sa, act, core::mem::size_of::<sigaction>()) != 0 { return -EFAULT; }
    }
    let ret = do_sigaction(sig, if act.is_null() { core::ptr::null_mut() } else { &mut new_ka },
                           if oact.is_null() { core::ptr::null_mut() } else { &mut old_ka });
    if ret == 0 && !oact.is_null() && copy_to_user(oact, &old_ka.sa, core::mem::size_of::<sigaction>()) != 0 { return -EFAULT; }
    ret
}

#[repr(C)]
pub struct sigframe { pub sc: sigcontext, pub retcode: [c_uint; 3] }
#[repr(C)]
pub struct rt_sigframe { pub info: siginfo, pub uc: ucontext, pub retcode: [c_uint; 3] }

const INSN_MOV_R30_R16: c_uint = 0x47fe0410;
const INSN_LDI_R0: c_uint = 0x201f0000;
const INSN_CALLSYS: c_uint = 0x00000083;

unsafe fn restore_sigcontext(sc: *mut sigcontext, regs: *mut pt_regs) -> c_long {
    let mut usp: c_ulong = 0;
    let sw = (regs as *mut switch_stack).offset(-1);
    let mut err = __get_user(&mut (*regs).pc, &(*sc).sc_pc);
    (*current).restart_block.fn_ = do_no_restart_syscall;
    (*current_thread_info()).status |= TS_SAVED_FP | TS_RESTORE_FP;
    (*sw).r26 = ret_from_sys_call as usize as c_ulong;
    err |= __get_user(&mut (*regs).r0, (*sc).sc_regs.add(0)); err |= __get_user(&mut (*regs).r1, (*sc).sc_regs.add(1));
    err |= __get_user(&mut (*regs).r2, (*sc).sc_regs.add(2)); err |= __get_user(&mut (*regs).r3, (*sc).sc_regs.add(3));
    err |= __get_user(&mut (*regs).r4, (*sc).sc_regs.add(4)); err |= __get_user(&mut (*regs).r5, (*sc).sc_regs.add(5));
    err |= __get_user(&mut (*regs).r6, (*sc).sc_regs.add(6)); err |= __get_user(&mut (*regs).r7, (*sc).sc_regs.add(7));
    err |= __get_user(&mut (*regs).r8, (*sc).sc_regs.add(8));
    err |= __get_user(&mut (*sw).r9, (*sc).sc_regs.add(9)); err |= __get_user(&mut (*sw).r10, (*sc).sc_regs.add(10));
    err |= __get_user(&mut (*sw).r11, (*sc).sc_regs.add(11)); err |= __get_user(&mut (*sw).r12, (*sc).sc_regs.add(12));
    err |= __get_user(&mut (*sw).r13, (*sc).sc_regs.add(13)); err |= __get_user(&mut (*sw).r14, (*sc).sc_regs.add(14));
    err |= __get_user(&mut (*sw).r15, (*sc).sc_regs.add(15));
    err |= __get_user(&mut (*regs).r16, (*sc).sc_regs.add(16)); err |= __get_user(&mut (*regs).r17, (*sc).sc_regs.add(17));
    err |= __get_user(&mut (*regs).r18, (*sc).sc_regs.add(18)); err |= __get_user(&mut (*regs).r19, (*sc).sc_regs.add(19));
    err |= __get_user(&mut (*regs).r20, (*sc).sc_regs.add(20)); err |= __get_user(&mut (*regs).r21, (*sc).sc_regs.add(21));
    err |= __get_user(&mut (*regs).r22, (*sc).sc_regs.add(22)); err |= __get_user(&mut (*regs).r23, (*sc).sc_regs.add(23));
    err |= __get_user(&mut (*regs).r24, (*sc).sc_regs.add(24)); err |= __get_user(&mut (*regs).r25, (*sc).sc_regs.add(25));
    err |= __get_user(&mut (*regs).r26, (*sc).sc_regs.add(26)); err |= __get_user(&mut (*regs).r27, (*sc).sc_regs.add(27));
    err |= __get_user(&mut (*regs).r28, (*sc).sc_regs.add(28)); err |= __get_user(&mut (*regs).gp, (*sc).sc_regs.add(29));
    err |= __get_user(&mut usp, (*sc).sc_regs.add(30)); wrusp(usp);
    err |= __copy_from_user((*current_thread_info()).fp.as_mut_ptr(), (*sc).sc_fpregs, 31 * 8);
    err |= __get_user(&mut (*current_thread_info()).fp[31], &(*sc).sc_fpcr); err
}

pub unsafe fn do_sigreturn(sc: *mut sigcontext) {
    let regs = current_pt_regs(); let mut set: sigset_t = core::mem::zeroed();
    if !access_ok(sc, core::mem::size_of::<sigcontext>()) || __get_user(&mut set.sig[0], &(*sc).sc_mask) != 0 { force_sig(SIGSEGV); return; }
    set_current_blocked(&set); if restore_sigcontext(sc, regs) != 0 { force_sig(SIGSEGV); return; }
    if ptrace_cancel_bpt(current) { send_sig_fault(SIGTRAP, TRAP_BRKPT, (*regs).pc as *mut c_void, current); }
}

pub unsafe fn do_rt_sigreturn(frame: *mut rt_sigframe) {
    let regs = current_pt_regs(); let mut set: sigset_t = core::mem::zeroed();
    if !access_ok(&(*frame).uc, core::mem::size_of::<ucontext>()) || __copy_from_user(&mut set, &(*frame).uc.uc_sigmask, core::mem::size_of::<sigset_t>()) != 0 { force_sig(SIGSEGV); return; }
    set_current_blocked(&set); if restore_sigcontext(&mut (*frame).uc.uc_mcontext, regs) != 0 { force_sig(SIGSEGV); return; }
    if ptrace_cancel_bpt(current) { send_sig_fault(SIGTRAP, TRAP_BRKPT, (*regs).pc as *mut c_void, current); }
}

#[inline] unsafe fn get_sigframe(ksig: *mut ksignal, sp: c_ulong, frame_size: usize) -> *mut c_void {
    ((sigsp(sp, ksig) - frame_size as c_ulong) & !31) as *mut c_void
}

unsafe fn setup_sigcontext(sc: *mut sigcontext, regs: *mut pt_regs, mask: c_ulong, sp: c_ulong) -> c_long {
    let sw = (regs as *mut switch_stack).offset(-1); let mut err = 0;
    err |= __put_user(on_sig_stack(sc as c_ulong), &mut (*sc).sc_onstack); err |= __put_user(mask, &mut (*sc).sc_mask);
    err |= __put_user((*regs).pc, &mut (*sc).sc_pc); err |= __put_user(8, &mut (*sc).sc_ps);
    err |= __put_user((*regs).r0, (*sc).sc_regs.add(0)); err |= __put_user((*regs).r1, (*sc).sc_regs.add(1)); err |= __put_user((*regs).r2, (*sc).sc_regs.add(2)); err |= __put_user((*regs).r3, (*sc).sc_regs.add(3)); err |= __put_user((*regs).r4, (*sc).sc_regs.add(4)); err |= __put_user((*regs).r5, (*sc).sc_regs.add(5)); err |= __put_user((*regs).r6, (*sc).sc_regs.add(6)); err |= __put_user((*regs).r7, (*sc).sc_regs.add(7)); err |= __put_user((*regs).r8, (*sc).sc_regs.add(8));
    err |= __put_user((*sw).r9, (*sc).sc_regs.add(9)); err |= __put_user((*sw).r10, (*sc).sc_regs.add(10)); err |= __put_user((*sw).r11, (*sc).sc_regs.add(11)); err |= __put_user((*sw).r12, (*sc).sc_regs.add(12)); err |= __put_user((*sw).r13, (*sc).sc_regs.add(13)); err |= __put_user((*sw).r14, (*sc).sc_regs.add(14)); err |= __put_user((*sw).r15, (*sc).sc_regs.add(15));
    err |= __put_user((*regs).r16, (*sc).sc_regs.add(16)); err |= __put_user((*regs).r17, (*sc).sc_regs.add(17)); err |= __put_user((*regs).r18, (*sc).sc_regs.add(18)); err |= __put_user((*regs).r19, (*sc).sc_regs.add(19)); err |= __put_user((*regs).r20, (*sc).sc_regs.add(20)); err |= __put_user((*regs).r21, (*sc).sc_regs.add(21)); err |= __put_user((*regs).r22, (*sc).sc_regs.add(22)); err |= __put_user((*regs).r23, (*sc).sc_regs.add(23)); err |= __put_user((*regs).r24, (*sc).sc_regs.add(24)); err |= __put_user((*regs).r25, (*sc).sc_regs.add(25)); err |= __put_user((*regs).r26, (*sc).sc_regs.add(26)); err |= __put_user((*regs).r27, (*sc).sc_regs.add(27)); err |= __put_user((*regs).r28, (*sc).sc_regs.add(28)); err |= __put_user((*regs).gp, (*sc).sc_regs.add(29)); err |= __put_user(sp, (*sc).sc_regs.add(30)); err |= __put_user(0, (*sc).sc_regs.add(31));
    err |= __copy_to_user((*sc).sc_fpregs, (*current_thread_info()).fp.as_ptr(), 31 * 8); err |= __put_user(0, (*sc).sc_fpregs.add(31)); err |= __put_user((*current_thread_info()).fp[31], &mut (*sc).sc_fpcr);
    err |= __put_user((*regs).trap_a0, &mut (*sc).sc_traparg_a0); err |= __put_user((*regs).trap_a1, &mut (*sc).sc_traparg_a1); err |= __put_user((*regs).trap_a2, &mut (*sc).sc_traparg_a2); err
}

/* Signal-frame construction and delivery retain the original Alpha ABI. */
unsafe fn setup_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let oldsp = rdusp(); let frame = get_sigframe(ksig, oldsp, core::mem::size_of::<sigframe>()) as *mut sigframe; if !access_ok(frame, core::mem::size_of::<sigframe>()) { return -EFAULT; }
    let mut err = setup_sigcontext(&mut (*frame).sc, regs, (*set).sig[0], oldsp); if err != 0 { return -EFAULT; }
    let mut r26 = (*ksig).ka.ka_restorer as usize as c_ulong; if r26 == 0 { err |= __put_user(INSN_MOV_R30_R16, (*frame).retcode.as_mut_ptr()); err |= __put_user(INSN_LDI_R0 + __NR_sigreturn, (*frame).retcode.as_mut_ptr().add(1)); err |= __put_user(INSN_CALLSYS, (*frame).retcode.as_mut_ptr().add(2)); imb(); r26 = (*frame).retcode.as_mut_ptr() as usize as c_ulong; }
    if err != 0 { return err as c_int; } (*regs).r26 = r26; (*regs).r27 = (*regs).pc = (*ksig).ka.sa.sa_handler as usize as c_ulong; (*regs).r16 = (*ksig).sig; (*regs).r17 = 0; (*regs).r18 = &mut (*frame).sc as *mut _ as usize as c_ulong; wrusp(frame as usize as c_ulong); 0
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let oldsp = rdusp(); let frame = get_sigframe(ksig, oldsp, core::mem::size_of::<rt_sigframe>()) as *mut rt_sigframe; if !access_ok(frame, core::mem::size_of::<rt_sigframe>()) { return -EFAULT; }
    let mut err = copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info); err |= __put_user(0, &mut (*frame).uc.uc_flags); err |= __put_user(0, &mut (*frame).uc.uc_link); err |= __put_user((*set).sig[0], &mut (*frame).uc.uc_osf_sigmask); err |= __save_altstack(&mut (*frame).uc.uc_stack, oldsp); err |= setup_sigcontext(&mut (*frame).uc.uc_mcontext, regs, (*set).sig[0], oldsp); err |= __copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>()); if err != 0 { return -EFAULT; }
    let mut r26 = (*ksig).ka.ka_restorer as usize as c_ulong; if r26 == 0 { err |= __put_user(INSN_MOV_R30_R16, (*frame).retcode.as_mut_ptr()); err |= __put_user(INSN_LDI_R0 + __NR_rt_sigreturn, (*frame).retcode.as_mut_ptr().add(1)); err |= __put_user(INSN_CALLSYS, (*frame).retcode.as_mut_ptr().add(2)); imb(); r26 = (*frame).retcode.as_mut_ptr() as usize as c_ulong; } if err != 0 { return -EFAULT; }
    (*regs).r26 = r26; (*regs).r27 = (*regs).pc = (*ksig).ka.sa.sa_handler as usize as c_ulong; (*regs).r16 = (*ksig).sig; (*regs).r17 = &mut (*frame).info as *mut _ as usize as c_ulong; (*regs).r18 = &mut (*frame).uc as *mut _ as usize as c_ulong; wrusp(frame as usize as c_ulong); 0
}

#[inline] unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) { let oldset = sigmask_to_save(); let ret = if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { setup_rt_frame(ksig, oldset, regs) } else { setup_frame(ksig, oldset, regs) }; signal_setup_done(ret, ksig, 0); }

#[inline] unsafe fn syscall_restart(r0: c_ulong, r19: c_ulong, regs: *mut pt_regs, ka: *mut k_sigaction) {
    match (*regs).r0 { ERESTARTSYS if (*ka).sa.sa_flags & SA_RESTART == 0 => { (*regs).r0 = EINTR; }, ERESTARTSYS | ERESTARTNOINTR => { (*regs).r0 = r0; (*regs).r19 = r19; (*regs).pc -= 4; }, ERESTARTNOHAND | ERESTART_RESTARTBLOCK => { (*regs).r0 = EINTR; }, _ => {} }
}

unsafe fn do_signal(regs: *mut pt_regs, r0: c_ulong, r19: c_ulong) {
    let mut single_stepping = ptrace_cancel_bpt(current); let mut ksig: ksignal = core::mem::zeroed();
    if get_signal(&mut ksig) { single_stepping |= ptrace_cancel_bpt(current); if r0 != 0 { syscall_restart(r0, r19, regs, &mut ksig.ka); } handle_signal(&mut ksig, regs); }
    else { single_stepping |= ptrace_cancel_bpt(current); if r0 != 0 { match (*regs).r0 { ERESTARTNOHAND | ERESTARTSYS | ERESTARTNOINTR => { (*regs).r0 = r0; (*regs).r19 = r19; (*regs).pc -= 4; }, ERESTART_RESTARTBLOCK => { (*regs).r0 = __NR_restart_syscall; (*regs).pc -= 4; }, _ => {} } } restore_saved_sigmask(); }
    if single_stepping { ptrace_set_bpt(current); }
}

pub unsafe fn do_work_pending(regs: *mut pt_regs, mut thread_flags: c_ulong, r0: c_ulong, r19: c_ulong) {
    let mut r0 = r0; loop { if thread_flags & _TIF_NEED_RESCHED != 0 { local_irq_enable(); schedule(); } else { local_irq_enable(); if thread_flags & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 { preempt_disable(); save_fpu(); preempt_enable(); do_signal(regs, r0, r19); r0 = 0; } else { resume_user_mode_work(regs); } } local_irq_disable(); thread_flags = read_thread_flags(); if thread_flags & _TIF_WORK_MASK == 0 { break; } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
