// SPDX-License-Identifier: GPL-2.0
/*
 *    Copyright IBM Corp. 1999, 2006
 *    Author(s): Denis Joseph Barrow (djbarrow@de.ibm.com,barrow_dj@yahoo.com)
 *
 *    Based on Intel version
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  1997-11-28  Modified for POSIX.1b signals by Richard Henderson
 */

// Linux and architecture headers are supplied by the surrounding translation unit.

#[repr(C)]
struct Sigframe {
    callee_used_stack: [u8; __SIGNAL_FRAMESIZE],
    sc: sigcontext,
    sregs: _sigregs,
    signo: i32,
    sregs_ext: _sigregs_ext,
    svc_insn: u16, // Offset of svc_insn is NOT fixed!
}

#[repr(C)]
struct RtSigframe {
    callee_used_stack: [u8; __SIGNAL_FRAMESIZE],
    svc_insn: u16,
    info: siginfo,
    uc: ucontext_extended,
}

/* Store registers needed to create the signal frame */
unsafe fn store_sigregs() {
    save_access_regs((*current).thread.acrs);
    save_user_fpu_regs();
}

/* Load registers after signal return */
unsafe fn load_sigregs() {
    restore_access_regs((*current).thread.acrs);
}

/* Returns non-zero on fault. */
unsafe fn save_sigregs(regs: *mut pt_regs, sregs: *mut _sigregs) -> i32 {
    let mut user_sregs: _sigregs = core::mem::zeroed();

    /* Copy a 'clean' PSW mask to the user to avoid leaking
       information about whether PER is currently on. */
    (*user_sregs.regs).psw.mask = PSW_USER_BITS |
        ((*regs).psw.mask & (PSW_MASK_USER | PSW_MASK_RI));
    (*user_sregs.regs).psw.addr = (*regs).psw.addr;
    core::ptr::copy_nonoverlapping(
        (*regs).gprs.as_ptr(), (*user_sregs.regs).gprs.as_mut_ptr(),
        (*sregs).regs.gprs.len());
    core::ptr::copy_nonoverlapping(
        (*current).thread.acrs.as_ptr(), (*user_sregs.regs).acrs.as_mut_ptr(),
        (*user_sregs.regs).acrs.len());
    fpregs_store(&mut user_sregs.fpregs, &(*current).thread.ufpu);
    if __copy_to_user(sregs as *mut _, &user_sregs as *const _, core::mem::size_of::<_sigregs>()) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn restore_sigregs(regs: *mut pt_regs, sregs: *const _sigregs) -> i32 {
    let mut user_sregs: _sigregs = core::mem::zeroed();

    (*current).restart_block.fn_ = do_no_restart_syscall;
    if __copy_from_user(&mut user_sregs as *mut _, sregs, core::mem::size_of::<_sigregs>()) != 0 {
        return -EFAULT;
    }
    if !is_ri_task(current) && (user_sregs.regs.psw.mask & PSW_MASK_RI) != 0 {
        return -EINVAL;
    }
    (*regs).psw.mask = ((*regs).psw.mask & !(PSW_MASK_USER | PSW_MASK_RI)) |
        (user_sregs.regs.psw.mask & (PSW_MASK_USER | PSW_MASK_RI));
    if ((*regs).psw.mask & PSW_MASK_ASC) == PSW_ASC_HOME {
        (*regs).psw.mask = PSW_ASC_PRIMARY | ((*regs).psw.mask & !PSW_MASK_ASC);
    }
    if (*regs).psw.mask & PSW_MASK_EA != 0 {
        (*regs).psw.mask |= PSW_MASK_BA;
    }
    (*regs).psw.addr = user_sregs.regs.psw.addr;
    core::ptr::copy_nonoverlapping(user_sregs.regs.gprs.as_ptr(), (*regs).gprs.as_mut_ptr(), (*sregs).regs.gprs.len());
    core::ptr::copy_nonoverlapping(user_sregs.regs.acrs.as_ptr(), (*current).thread.acrs.as_mut_ptr(), (*current).thread.acrs.len());
    fpregs_load(&user_sregs.fpregs, &mut (*current).thread.ufpu);
    clear_pt_regs_flag(regs, PIF_SYSCALL);
    0
}

/* Returns non-zero on fault. */
unsafe fn save_sigregs_ext(_regs: *mut pt_regs, sregs_ext: *mut _sigregs_ext) -> i32 {
    let mut vxrs: [u64; __NUM_VXRS_LOW] = [0; __NUM_VXRS_LOW];
    if cpu_has_vx() {
        for i in 0..__NUM_VXRS_LOW {
            vxrs[i] = (*current).thread.ufpu.vxrs[i].low;
        }
        if __copy_to_user(&mut (*sregs_ext).vxrs_low as *mut _, vxrs.as_ptr(), core::mem::size_of_val(&(*sregs_ext).vxrs_low)) != 0 ||
           __copy_to_user((*sregs_ext).vxrs_high.as_mut_ptr(), (*current).thread.ufpu.vxrs.as_ptr().add(__NUM_VXRS_LOW), core::mem::size_of_val(&(*sregs_ext).vxrs_high)) != 0 {
            return -EFAULT;
        }
    }
    0
}

unsafe fn restore_sigregs_ext(_regs: *mut pt_regs, sregs_ext: *const _sigregs_ext) -> i32 {
    let mut vxrs: [u64; __NUM_VXRS_LOW] = [0; __NUM_VXRS_LOW];
    if cpu_has_vx() {
        if __copy_from_user(vxrs.as_mut_ptr(), &(*sregs_ext).vxrs_low as *const _, core::mem::size_of_val(&(*sregs_ext).vxrs_low)) != 0 ||
           __copy_from_user((*current).thread.ufpu.vxrs.as_mut_ptr().add(__NUM_VXRS_LOW), &(*sregs_ext).vxrs_high as *const _, core::mem::size_of_val(&(*sregs_ext).vxrs_high)) != 0 {
            return -EFAULT;
        }
        for i in 0..__NUM_VXRS_LOW {
            (*current).thread.ufpu.vxrs[i].low = vxrs[i];
        }
    }
    0
}

pub unsafe fn sigreturn() -> u64 {
    let regs = task_pt_regs(current);
    let frame = (*regs).gprs[15] as *mut Sigframe;
    let mut set: sigset_t = core::mem::zeroed();
    if __copy_from_user(&mut set.sig as *mut _, &(*frame).sc.oldmask as *const _, _SIGMASK_COPY_SIZE) != 0 { goto_badframe(); return 0; }
    set_current_blocked(&set);
    save_user_fpu_regs();
    if restore_sigregs(regs, &(*frame).sregs) != 0 || restore_sigregs_ext(regs, &(*frame).sregs_ext) != 0 { goto_badframe(); return 0; }
    load_sigregs();
    (*regs).gprs[2]
}

pub unsafe fn rt_sigreturn() -> u64 {
    let regs = task_pt_regs(current);
    let frame = (*regs).gprs[15] as *mut RtSigframe;
    let mut set: sigset_t = core::mem::zeroed();
    if __copy_from_user(&mut set.sig as *mut _, &(*frame).uc.uc_sigmask as *const _, core::mem::size_of::<sigset_t>()) != 0 { goto_badframe(); return 0; }
    set_current_blocked(&set);
    if restore_altstack(&(*frame).uc.uc_stack) != 0 { goto_badframe(); return 0; }
    save_user_fpu_regs();
    if restore_sigregs(regs, &(*frame).uc.uc_mcontext) != 0 || restore_sigregs_ext(regs, &(*frame).uc.uc_mcontext_ext) != 0 { goto_badframe(); return 0; }
    load_sigregs();
    (*regs).gprs[2]
}

unsafe fn goto_badframe() { force_sig(SIGSEGV); }

/* Determine which stack to use. */
unsafe fn get_sigframe(ka: *mut k_sigaction, regs: *mut pt_regs, frame_size: usize) -> *mut core::ffi::c_void {
    let mut sp = (*regs).gprs[15];
    if on_sig_stack(sp) && !on_sig_stack((sp.wrapping_sub(frame_size)) & !7) { return usize::MAX as *mut _; }
    if (*ka).sa.sa_flags & SA_ONSTACK != 0 && sas_ss_flags(sp) == 0 { sp = (*current).sas_ss_sp + (*current).sas_ss_size; }
    ((sp.wrapping_sub(frame_size)) & !7) as *mut _
}

unsafe fn setup_frame(sig: i32, ka: *mut k_sigaction, set: *mut sigset_t, regs: *mut pt_regs) -> i32 {
    let mut frame_size = core::mem::size_of::<Sigframe>() - core::mem::size_of::<_sigregs_ext>();
    if cpu_has_vx() { frame_size += core::mem::size_of::<_sigregs_ext>(); }
    let frame = get_sigframe(ka, regs, frame_size) as *mut Sigframe;
    if frame == usize::MAX as *mut _ { return -EFAULT; }
    if __put_user((*regs).gprs[15], frame as *mut addr_t) != 0 { return -EFAULT; }
    let mut sc: sigcontext = core::mem::zeroed();
    core::ptr::copy_nonoverlapping((&(*set).sig as *const _).cast::<u8>(), (&mut sc.oldmask as *mut _).cast::<u8>(), _SIGMASK_COPY_SIZE);
    sc.sregs = (&mut (*frame).sregs) as *mut _;
    if __copy_to_user(&mut (*frame).sc as *mut _, &sc as *const _, core::mem::size_of::<sigcontext>()) != 0 { return -EFAULT; }
    store_sigregs();
    if save_sigregs(regs, &mut (*frame).sregs) != 0 { return -EFAULT; }
    if __put_user((*regs).gprs[2] as i32, &mut (*frame).signo) != 0 { return -EFAULT; }
    if save_sigregs_ext(regs, &mut (*frame).sregs_ext) != 0 { return -EFAULT; }
    let restorer = if (*ka).sa.sa_flags & SA_RESTORER != 0 { (*ka).sa.sa_restorer as usize } else { VDSO_SYMBOL(current, sigreturn) as usize };
    (*regs).gprs[14] = restorer;
    (*regs).gprs[15] = frame as usize;
    (*regs).psw.mask = PSW_MASK_EA | PSW_MASK_BA | (PSW_USER_BITS & PSW_MASK_ASC) | ((*regs).psw.mask & !PSW_MASK_ASC);
    (*regs).psw.addr = (*ka).sa.sa_handler as usize;
    (*regs).gprs[2] = sig as usize;
    (*regs).gprs[3] = (&(*frame).sc) as *const _ as usize;
    if sig == SIGSEGV || sig == SIGBUS || sig == SIGILL || sig == SIGTRAP || sig == SIGFPE {
        (*regs).gprs[4] = (*regs).int_code & 127;
        (*regs).gprs[5] = (*regs).int_parm_long;
        (*regs).gprs[6] = (*current).thread.last_break;
    }
    0
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> i32 {
    let mut frame_size = core::mem::size_of::<RtSigframe>() - core::mem::size_of::<_sigregs_ext>();
    let mut uc_flags = 0;
    if cpu_has_vx() { frame_size += core::mem::size_of::<_sigregs_ext>(); uc_flags |= UC_VXRS; }
    let frame = get_sigframe(&mut (*ksig).ka, regs, frame_size) as *mut RtSigframe;
    if frame == usize::MAX as *mut _ { return -EFAULT; }
    if __put_user((*regs).gprs[15], frame as *mut addr_t) != 0 { return -EFAULT; }
    let restorer = if (*ksig).ka.sa.sa_flags & SA_RESTORER != 0 { (*ksig).ka.sa.sa_restorer as usize } else { VDSO_SYMBOL(current, rt_sigreturn) as usize };
    if copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info) != 0 { return -EFAULT; }
    store_sigregs();
    if __put_user(uc_flags, &mut (*frame).uc.uc_flags) != 0 || __put_user(core::ptr::null_mut(), &mut (*frame).uc.uc_link) != 0 || __save_altstack(&mut (*frame).uc.uc_stack, (*regs).gprs[15]) != 0 || save_sigregs(regs, &mut (*frame).uc.uc_mcontext) != 0 || __copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>()) != 0 || save_sigregs_ext(regs, &mut (*frame).uc.uc_mcontext_ext) != 0 { return -EFAULT; }
    (*regs).gprs[14] = restorer; (*regs).gprs[15] = frame as usize;
    (*regs).psw.mask = PSW_MASK_EA | PSW_MASK_BA | (PSW_USER_BITS & PSW_MASK_ASC) | ((*regs).psw.mask & !PSW_MASK_ASC);
    (*regs).psw.addr = (*ksig).ka.sa.sa_handler as usize;
    (*regs).gprs[2] = (*ksig).sig as usize; (*regs).gprs[3] = (&(*frame).info) as *const _ as usize; (*regs).gprs[4] = (&(*frame).uc) as *const _ as usize; (*regs).gprs[5] = (*current).thread.last_break;
    0
}

unsafe fn handle_signal(ksig: *mut ksignal, oldset: *mut sigset_t, regs: *mut pt_regs) {
    let ret = if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { setup_rt_frame(ksig, oldset, regs) } else { setup_frame((*ksig).sig, &mut (*ksig).ka, oldset, regs) };
    signal_setup_done(ret, ksig, test_thread_flag(TIF_SINGLE_STEP));
}

pub unsafe fn arch_do_signal_or_restart(regs: *mut pt_regs) {
    let mut ksig: ksignal = core::mem::zeroed();
    let oldset = sigmask_to_save();
    (*current).thread.system_call = if test_pt_regs_flag(regs, PIF_SYSCALL) { (*regs).int_code } else { 0 };
    if get_signal(&mut ksig) {
        if (*current).thread.system_call != 0 {
            (*regs).int_code = (*current).thread.system_call;
            match (*regs).gprs[2] as isize {
                -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => (*regs).gprs[2] = -EINTR as usize,
                -ERESTARTSYS => if (*ksig).ka.sa.sa_flags & SA_RESTART == 0 { (*regs).gprs[2] = -EINTR as usize; } else { (*regs).gprs[2] = (*regs).orig_gpr2; (*regs).psw.addr = __rewind_psw((*regs).psw, (*regs).int_code >> 16); },
                -ERESTARTNOINTR => { (*regs).gprs[2] = (*regs).orig_gpr2; (*regs).psw.addr = __rewind_psw((*regs).psw, (*regs).int_code >> 16); }, _ => {}
            }
        }
        clear_pt_regs_flag(regs, PIF_SYSCALL); rseq_signal_deliver(&mut ksig, regs); handle_signal(&mut ksig, oldset, regs); return;
    }
    clear_pt_regs_flag(regs, PIF_SYSCALL);
    if (*current).thread.system_call != 0 { (*regs).int_code = (*current).thread.system_call; match (*regs).gprs[2] as isize { -ERESTART_RESTARTBLOCK => { (*regs).gprs[2] = (*regs).orig_gpr2; (*current).restart_block.arch_data = (*regs).psw.addr; (*regs).psw.addr = VDSO_SYMBOL(current, restart_syscall); }, -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => { (*regs).gprs[2] = (*regs).orig_gpr2; (*regs).psw.addr = __rewind_psw((*regs).psw, (*regs).int_code >> 16); }, _ => {} } }
    restore_saved_sigmask();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
