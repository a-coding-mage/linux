// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/sh/kernel/signal.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  1997-11-28  Modified for POSIX.1b signals by Richard Henderson
 *
 *  SuperH version:  Copyright (C) 1999, 2000  Niibe Yutaka & Kaz Kojima
 *
 */
// C kernel includes supplied by other translation units.

#[repr(C)]
struct fdpic_func_descriptor { text: c_ulong, GOT: c_ulong }

const UNWINDGUARD: usize = 64;
const fn MOVW(n: u16) -> u16 { 0x9300 | (n - 2) }
#[cfg(CONFIG_CPU_SH2)] const TRAP_NOARG: u16 = 0xc320;
#[cfg(not(CONFIG_CPU_SH2))] const TRAP_NOARG: u16 = 0xc310;
const OR_R0_R0: u16 = 0x200b;

#[repr(C)]
struct sigframe { sc: sigcontext, extramask: [c_ulong; _NSIG_WORDS - 1], retcode: [u16; 8] }
#[repr(C)]
struct rt_sigframe { info: siginfo, uc: ucontext, retcode: [u16; 8] }

#[cfg(CONFIG_SH_FPU)]
unsafe fn restore_sigcontext_fpu(sc: *mut sigcontext) -> c_int {
    let tsk = current;
    if (boot_cpu_data.flags & CPU_HAS_FPU) == 0 { return 0; }
    set_used_math();
    __copy_from_user(&mut (*(*tsk).thread.xstate).hardfpu as *mut _, &(*sc).sc_fpregs[0] as *const _, core::mem::size_of::<c_ulong>() * (16 * 2 + 2))
}

#[cfg(CONFIG_SH_FPU)]
unsafe fn save_sigcontext_fpu(sc: *mut sigcontext, regs: *mut pt_regs) -> c_int {
    let tsk = current;
    if (boot_cpu_data.flags & CPU_HAS_FPU) == 0 { return 0; }
    if !used_math() { return __put_user(0, &mut (*sc).sc_ownedfp); }
    if __put_user(1, &mut (*sc).sc_ownedfp) != 0 { return -EFAULT; }
    clear_used_math();
    unlazy_fpu(tsk, regs);
    __copy_to_user(&mut (*sc).sc_fpregs[0] as *mut _, &(*(*tsk).thread.xstate).hardfpu as *const _, core::mem::size_of::<c_ulong>() * (16 * 2 + 2))
}

unsafe fn restore_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext, r0_p: *mut c_int) -> c_uint {
    let mut err: c_uint = 0;
    let sr = (*regs).sr & !SR_USER_MASK;
    macro_rules! copy { ($x:ident) => { err |= __get_user(&mut (*regs).$x, &(*sc).sc_$x) }; }
    copy!(regs[1]); copy!(regs[2]); copy!(regs[3]); copy!(regs[4]); copy!(regs[5]); copy!(regs[6]); copy!(regs[7]);
    copy!(regs[8]); copy!(regs[9]); copy!(regs[10]); copy!(regs[11]); copy!(regs[12]); copy!(regs[13]); copy!(regs[14]); copy!(regs[15]);
    copy!(gbr); copy!(mach); copy!(macl); copy!(pr); copy!(sr); copy!(pc);
    (*regs).sr = ((*regs).sr & SR_USER_MASK) | sr;
    #[cfg(CONFIG_SH_FPU)] if (boot_cpu_data.flags & CPU_HAS_FPU) != 0 {
        (*regs).sr |= SR_FD; clear_fpu(current, regs); clear_used_math();
        let mut owned_fp = 0; err |= __get_user(&mut owned_fp, &(*sc).sc_ownedfp);
        if owned_fp != 0 { err |= restore_sigcontext_fpu(sc) as c_uint; }
    }
    (*regs).tra = -1; err |= __get_user(&mut *r0_p, &(*sc).sc_regs[0]); err
}

unsafe fn sys_sigreturn() -> c_int {
    let regs = current_pt_regs(); let frame = (*regs).regs[15] as *mut sigframe; let mut set: sigset_t = core::mem::zeroed(); let mut r0 = 0;
    (*current).restart_block.fn_ = do_no_restart_syscall;
    if !access_ok(frame as *const _, core::mem::size_of::<sigframe>()) { force_sig(SIGSEGV); return 0; }
    if __get_user(&mut set.sig[0], &(*frame).sc.oldmask) != 0 || (_NSIG_WORDS > 1 && __copy_from_user(&mut set.sig[1], (*frame).extramask.as_ptr(), core::mem::size_of_val(&(*frame).extramask)) != 0) { force_sig(SIGSEGV); return 0; }
    set_current_blocked(&set); if restore_sigcontext(regs, &mut (*frame).sc, &mut r0) != 0 { force_sig(SIGSEGV); return 0; } r0
}

unsafe fn sys_rt_sigreturn() -> c_int {
    let regs = current_pt_regs(); let frame = (*regs).regs[15] as *mut rt_sigframe; let mut set: sigset_t = core::mem::zeroed(); let mut r0 = 0;
    (*current).restart_block.fn_ = do_no_restart_syscall;
    if !access_ok(frame as *const _, core::mem::size_of::<rt_sigframe>()) || __copy_from_user(&mut set, &(*frame).uc.uc_sigmask, core::mem::size_of::<sigset_t>()) != 0 { force_sig(SIGSEGV); return 0; }
    set_current_blocked(&set); if restore_sigcontext(regs, &mut (*frame).uc.uc_mcontext, &mut r0) != 0 || restore_altstack(&(*frame).uc.uc_stack) != 0 { force_sig(SIGSEGV); return 0; } r0
}

unsafe fn setup_sigcontext(sc: *mut sigcontext, regs: *mut pt_regs, mask: c_ulong) -> c_int {
    let mut err = 0; macro_rules! copy { ($x:ident) => { err |= __put_user((*regs).$x, &mut (*sc).sc_$x) }; }
    copy!(regs[0]); copy!(regs[1]); copy!(regs[2]); copy!(regs[3]); copy!(regs[4]); copy!(regs[5]); copy!(regs[6]); copy!(regs[7]); copy!(regs[8]); copy!(regs[9]); copy!(regs[10]); copy!(regs[11]); copy!(regs[12]); copy!(regs[13]); copy!(regs[14]); copy!(regs[15]); copy!(gbr); copy!(mach); copy!(macl); copy!(pr); copy!(sr); copy!(pc);
    #[cfg(CONFIG_SH_FPU)] { err |= save_sigcontext_fpu(sc, regs); }
    err |= __put_user(mask, &mut (*sc).oldmask); err
}

unsafe fn get_sigframe(ka: *mut k_sigaction, mut sp: c_ulong, frame_size: usize) -> *mut core::ffi::c_void {
    if (*ka).sa.sa_flags & SA_ONSTACK != 0 && sas_ss_flags(sp) == 0 { sp = (*current).sas_ss_sp + (*current).sas_ss_size; }
    ((sp as usize - (frame_size + UNWINDGUARD)) & !7) as *mut _
}

extern "C" { fn __kernel_sigreturn(); fn __kernel_rt_sigreturn(); }

unsafe fn setup_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let frame = get_sigframe(&mut (*ksig).ka, (*regs).regs[15], core::mem::size_of::<sigframe>()) as *mut sigframe;
    if !access_ok(frame as *const _, core::mem::size_of::<sigframe>()) { return -EFAULT; }
    let mut err = setup_sigcontext(&mut (*frame).sc, regs, (*set).sig[0]);
    if _NSIG_WORDS > 1 { err |= __copy_to_user((*frame).extramask.as_mut_ptr(), (*set).sig.as_ptr().add(1), core::mem::size_of_val(&(*frame).extramask)); }
    if (*ksig).ka.sa.sa_flags & SA_RESTORER != 0 { (*regs).pr = (*ksig).ka.sa.sa_restorer as c_ulong; }
    else { err |= __put_user(MOVW(7), &mut (*frame).retcode[0]); err |= __put_user(TRAP_NOARG, &mut (*frame).retcode[1]); for i in 2..7 { err |= __put_user(OR_R0_R0, &mut (*frame).retcode[i]); } err |= __put_user(__NR_sigreturn, &mut (*frame).retcode[7]); (*regs).pr = (*frame).retcode.as_mut_ptr() as c_ulong; flush_icache_range((*regs).pr, (*regs).pr + core::mem::size_of_val(&(*frame).retcode) as c_ulong); }
    if err != 0 { return -EFAULT; }
    (*regs).regs[15] = frame as c_ulong; (*regs).regs[4] = (*ksig).sig as c_ulong; (*regs).regs[5] = 0; (*regs).regs[6] = &mut (*frame).sc as *mut _ as c_ulong;
    if (*current).personality & FDPIC_FUNCPTRS != 0 { let f = (*ksig).ka.sa.sa_handler as *mut fdpic_func_descriptor; err |= __get_user(&mut (*regs).pc, &(*f).text); err |= __get_user(&mut (*regs).regs[12], &(*f).GOT); } else { (*regs).pc = (*ksig).ka.sa.sa_handler as c_ulong; } if err != 0 { -EFAULT } else { 0 }
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let frame = get_sigframe(&mut (*ksig).ka, (*regs).regs[15], core::mem::size_of::<rt_sigframe>()) as *mut rt_sigframe;
    if !access_ok(frame as *const _, core::mem::size_of::<rt_sigframe>()) { return -EFAULT; }
    let mut err = copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info); err |= __put_user(0, &mut (*frame).uc.uc_flags); err |= __put_user(core::ptr::null_mut(), &mut (*frame).uc.uc_link); err |= __save_altstack(&mut (*frame).uc.uc_stack, (*regs).regs[15]); err |= setup_sigcontext(&mut (*frame).uc.uc_mcontext, regs, (*set).sig[0]); err |= __copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>());
    if (*ksig).ka.sa.sa_flags & SA_RESTORER != 0 { (*regs).pr = (*ksig).ka.sa.sa_restorer as c_ulong; } else { err |= __put_user(MOVW(7), &mut (*frame).retcode[0]); err |= __put_user(TRAP_NOARG, &mut (*frame).retcode[1]); for i in 2..7 { err |= __put_user(OR_R0_R0, &mut (*frame).retcode[i]); } err |= __put_user(__NR_rt_sigreturn, &mut (*frame).retcode[7]); (*regs).pr = (*frame).retcode.as_mut_ptr() as c_ulong; }
    if err != 0 { return -EFAULT; } (*regs).regs[15] = frame as c_ulong; (*regs).regs[4] = (*ksig).sig as c_ulong; (*regs).regs[5] = &mut (*frame).info as *mut _ as c_ulong; (*regs).regs[6] = &mut (*frame).uc as *mut _ as c_ulong; (*regs).pc = (*ksig).ka.sa.sa_handler as c_ulong; 0
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs, save_r0: c_uint) { let oldset = sigmask_to_save(); let ret = if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { setup_rt_frame(ksig, oldset, regs) } else { setup_frame(ksig, oldset, regs) }; signal_setup_done(ret, ksig, test_thread_flag(TIF_SINGLESTEP)); }

// The remaining signal-frame setup and resume logic is preserved below in direct unsafe form.
unsafe fn handle_syscall_restart(save_r0: c_ulong, regs: *mut pt_regs, sa: *mut sigaction) {
    if (*regs).tra < 0 { return; }
    match (*regs).regs[0] as c_long {
        -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => { (*regs).regs[0] = -EINTR as c_ulong; }
        -ERESTARTSYS => { if (*sa).sa_flags & SA_RESTART == 0 { (*regs).regs[0] = -EINTR as c_ulong; } else { (*regs).regs[0] = save_r0; (*regs).pc -= instruction_size(__raw_readw((*regs).pc - 4)); } }
        -ERESTARTNOINTR => { (*regs).regs[0] = save_r0; (*regs).pc -= instruction_size(__raw_readw((*regs).pc - 4)); }
        _ => {}
    }
}

unsafe fn do_signal(regs: *mut pt_regs, save_r0: c_uint) { if !user_mode(regs) { return; } let mut ksig: ksignal = core::mem::zeroed(); if get_signal(&mut ksig) { handle_syscall_restart(save_r0 as _, regs, &mut ksig.ka.sa); handle_signal(&mut ksig, regs, save_r0); return; } if (*regs).tra >= 0 { if (*regs).regs[0] as c_long == -ERESTART_RESTARTBLOCK { (*regs).pc -= instruction_size(__raw_readw((*regs).pc - 4)); (*regs).regs[3] = __NR_restart_syscall; } else if (*regs).regs[0] as c_long == -ERESTARTNOHAND || (*regs).regs[0] as c_long == -ERESTARTSYS || (*regs).regs[0] as c_long == -ERESTARTNOINTR { (*regs).regs[0] = save_r0 as _; (*regs).pc -= instruction_size(__raw_readw((*regs).pc - 4)); } } restore_saved_sigmask(); }

unsafe fn do_notify_resume(regs: *mut pt_regs, save_r0: c_uint, flags: c_ulong) { if flags & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 { do_signal(regs, save_r0); } if flags & _TIF_NOTIFY_RESUME != 0 { resume_user_mode_work(regs); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
