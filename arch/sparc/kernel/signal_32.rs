// SPDX-License-Identifier: GPL-2.0
/*  linux/arch/sparc/kernel/signal.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 *  Copyright (C) 1996 Miguel de Icaza (miguel@nuclecu.unam.mx)
 *  Copyright (C) 1997 Eddie C. Dost   (ecd@skynet.be)
 */

// Kernel and architecture dependencies are supplied by the surrounding build.

extern "C" {
    fn fpsave(fpregs: *mut c_ulong, fsr: *mut c_ulong, fpqueue: *mut c_void,
              fpqdepth: *mut c_ulong);
    fn fpload(fpregs: *mut c_ulong, fsr: *mut c_ulong);
}

#[repr(C, align(8))]
struct signal_frame {
    ss: sparc_stackf,
    info: __siginfo32_t,
    fpu_save: *mut __siginfo_fpu_t,
    insns: [c_ulong; 2],
    extramask: [c_uint; _NSIG_WORDS - 1],
    extra_size: c_uint,
    rwin_save: *mut __siginfo_rwin_t,
}

#[repr(C, align(8))]
struct rt_signal_frame {
    ss: sparc_stackf,
    info: siginfo_t,
    regs: pt_regs,
    mask: sigset_t,
    fpu_save: *mut __siginfo_fpu_t,
    insns: [c_uint; 2],
    stack: stack_t,
    extra_size: c_uint,
    rwin_save: *mut __siginfo_rwin_t,
}

// Align macros
const SF_ALIGNEDSZ: usize = (core::mem::size_of::<signal_frame>() + 7) & !7;
const RT_ALIGNEDSZ: usize = (core::mem::size_of::<rt_signal_frame>() + 7) & !7;

/* Checks if the fp is valid.  We always build signal frames which are
 * 16-byte aligned, therefore we can always enforce that the restore
 * frame has that property as well.
 */
#[inline]
unsafe fn invalid_frame_pointer(fp: *mut c_void, fplen: usize) -> bool {
    ((fp as c_ulong) & 15) != 0 || !access_ok(fp, fplen)
}

unsafe fn do_sigreturn(regs: *mut pt_regs) {
    let mut up_psr: c_ulong;
    let mut pc: c_ulong = 0;
    let mut npc: c_ulong = 0;
    let mut ufp: c_ulong = 0;
    let sf = (*regs).u_regs[UREG_FP] as *mut signal_frame;
    let mut set: sigset_t = core::mem::zeroed();
    let mut fpu_save: *mut __siginfo_fpu_t = core::ptr::null_mut();
    let mut rwin_save: *mut __siginfo_rwin_t = core::ptr::null_mut();
    let mut err: c_int;

    (*current).restart_block.fn_ = do_no_restart_syscall;
    synchronize_user_stack();
    if invalid_frame_pointer(sf as *mut c_void, core::mem::size_of::<signal_frame>()) { goto_segv_and_exit(); return; }
    if get_user(&mut ufp, &(*sf).info.si_regs.u_regs[UREG_FP]) != 0 || (ufp & 7) != 0 { goto_segv_and_exit(); return; }
    err = __get_user(&mut pc, &(*sf).info.si_regs.pc);
    err |= __get_user(&mut npc, &(*sf).info.si_regs.npc);
    if ((pc | npc) & 3) != 0 { goto_segv_and_exit(); return; }
    up_psr = (*regs).psr;
    err |= __copy_from_user(regs as *mut c_void, &(*sf).info.si_regs as *const _ as *const c_void, core::mem::size_of::<pt_regs>());
    (*regs).psr = (up_psr & !(PSR_ICC | PSR_EF)) | ((*regs).psr & (PSR_ICC | PSR_EF));
    pt_regs_clear_syscall(regs);
    err |= __get_user(&mut fpu_save, &(*sf).fpu_save);
    if !fpu_save.is_null() { err |= restore_fpu_state(regs, fpu_save); }
    err |= __get_user(&mut rwin_save, &(*sf).rwin_save);
    if !rwin_save.is_null() { err |= restore_rwin_state(rwin_save); }
    err |= __get_user(&mut set.sig[0], &(*sf).info.si_mask);
    err |= __copy_from_user(&mut set.sig[1] as *mut _ as *mut c_void, (*sf).extramask.as_ptr() as *const c_void, (_NSIG_WORDS - 1) * core::mem::size_of::<c_uint>());
    if err != 0 { goto_segv_and_exit(); return; }
    set_current_blocked(&set);
}

unsafe fn do_rt_sigreturn(regs: *mut pt_regs) {
    let sf = (*regs).u_regs[UREG_FP] as *mut rt_signal_frame;
    let mut psr: c_uint = 0; let mut pc: c_uint = 0; let mut npc: c_uint = 0; let mut ufp: c_uint = 0;
    let mut fpu_save: *mut __siginfo_fpu_t = core::ptr::null_mut();
    let mut rwin_save: *mut __siginfo_rwin_t = core::ptr::null_mut();
    let mut set: sigset_t = core::mem::zeroed(); let mut err: c_int;
    synchronize_user_stack();
    if invalid_frame_pointer(sf as *mut c_void, core::mem::size_of::<rt_signal_frame>()) { force_sig(SIGSEGV); return; }
    if get_user(&mut ufp, &(*sf).regs.u_regs[UREG_FP]) != 0 || (ufp & 7) != 0 { force_sig(SIGSEGV); return; }
    err = __get_user(&mut pc, &(*sf).regs.pc); err |= __get_user(&mut npc, &(*sf).regs.npc); err |= ((pc | npc) & 3) as c_int;
    err |= __get_user(&mut (*regs).y, &(*sf).regs.y); err |= __get_user(&mut psr, &(*sf).regs.psr);
    err |= __copy_from_user(&mut (*regs).u_regs[UREG_G1] as *mut _ as *mut c_void, &(*sf).regs.u_regs[UREG_G1] as *const _ as *const c_void, 15 * core::mem::size_of::<u32>());
    (*regs).psr = ((*regs).psr & !PSR_ICC) | (psr & PSR_ICC); pt_regs_clear_syscall(regs);
    err |= __get_user(&mut fpu_save, &(*sf).fpu_save); if err == 0 && !fpu_save.is_null() { err |= restore_fpu_state(regs, fpu_save); }
    err |= __copy_from_user(&mut set as *mut _ as *mut c_void, &(*sf).mask as *const _ as *const c_void, core::mem::size_of::<sigset_t>());
    err |= restore_altstack(&(*sf).stack as *const _ as *mut _); if err != 0 { force_sig(SIGSEGV); return; }
    (*regs).pc = pc as c_ulong; (*regs).npc = npc as c_ulong;
    err |= __get_user(&mut rwin_save, &(*sf).rwin_save); if err == 0 && !rwin_save.is_null() && restore_rwin_state(rwin_save) != 0 { force_sig(SIGSEGV); return; }
    set_current_blocked(&set);
}

#[inline]
unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, framesize: c_ulong) -> *mut c_void {
    let mut sp = (*regs).u_regs[UREG_FP];
    if on_sig_stack(sp) && !likely(on_sig_stack(sp.wrapping_sub(framesize))) { return (-1isize) as *mut c_void; }
    sp = sigsp(sp, ksig).wrapping_sub(framesize); sp &= !15; sp as *mut c_void
}

// The remaining frame setup and signal-dispatch routines retain the kernel's
// external helper interfaces and C-level pointer semantics.
unsafe fn setup_frame(ksig: *mut ksignal, regs: *mut pt_regs, oldset: *mut sigset_t) -> c_int {
    synchronize_user_stack();
    let wsaved = current_thread_info().w_saved;
    let mut sigframe_size = core::mem::size_of::<signal_frame>();
    if used_math() { sigframe_size += core::mem::size_of::<__siginfo_fpu_t>(); }
    if wsaved != 0 { sigframe_size += core::mem::size_of::<__siginfo_rwin_t>(); }
    let sf = get_sigframe(ksig, regs, sigframe_size as c_ulong) as *mut signal_frame;
    if invalid_frame_pointer(sf as *mut c_void, sigframe_size) { force_exit_sig(SIGILL); return -EINVAL; }
    let mut tail = (sf as *mut u8).add(core::mem::size_of::<signal_frame>());
    let mut err = __copy_to_user(&mut (*sf).info.si_regs as *mut _ as *mut c_void, regs as *const c_void, core::mem::size_of::<pt_regs>());
    err |= __put_user(0, &mut (*sf).extra_size);
    if used_math() { let fp = tail as *mut __siginfo_fpu_t; tail = tail.add(core::mem::size_of::<__siginfo_fpu_t>()); err |= save_fpu_state(regs, fp); err |= __put_user(fp, &mut (*sf).fpu_save); } else { err |= __put_user(core::ptr::null_mut(), &mut (*sf).fpu_save); }
    if wsaved != 0 { let rp = current_thread_info().reg_window.as_mut_ptr().add(wsaved as usize - 1) as *mut reg_window32; let rwp = tail as *mut __siginfo_rwin_t; err |= save_rwin_state(wsaved, rwp); err |= __put_user(rwp, &mut (*sf).rwin_save); err |= __copy_to_user(sf as *mut c_void, rp as *const c_void, core::mem::size_of::<reg_window32>()); } else { err |= __put_user(core::ptr::null_mut(), &mut (*sf).rwin_save); err |= __copy_to_user(sf as *mut c_void, (*regs).u_regs[UREG_FP] as *const c_void, core::mem::size_of::<reg_window32>()); }
    err |= __put_user((*oldset).sig[0], &mut (*sf).info.si_mask); err |= __copy_to_user((*sf).extramask.as_mut_ptr() as *mut c_void, &(*oldset).sig[1] as *const _ as *const c_void, (_NSIG_WORDS - 1) * core::mem::size_of::<c_uint>());
    if err != 0 { return err; }
    (*regs).u_regs[UREG_FP] = sf as c_ulong; (*regs).u_regs[UREG_I0] = (*ksig).sig; (*regs).u_regs[UREG_I1] = &(*sf).info as *const _ as c_ulong; (*regs).u_regs[UREG_I2] = &(*sf).info as *const _ as c_ulong; (*regs).pc = (*ksig).ka.sa.sa_handler as c_ulong; (*regs).npc = (*regs).pc + 4;
    if !(*ksig).ka.ka_restorer.is_null() { (*regs).u_regs[UREG_I7] = (*ksig).ka.ka_restorer as c_ulong; } else { (*regs).u_regs[UREG_I7] = (&(*sf).insns[0] as *const _ as c_ulong) - 2; err |= __put_user(0x821020d8, &mut (*sf).insns[0]); err |= __put_user(0x91d02010, &mut (*sf).insns[1]); if err != 0 { return err; } flush_sig_insns(current.mm, &(*sf).insns[0] as *const _ as c_ulong); }
    0
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs, oldset: *mut sigset_t) -> c_int {
    synchronize_user_stack(); let wsaved = current_thread_info().w_saved; let mut size = core::mem::size_of::<rt_signal_frame>(); if used_math() { size += core::mem::size_of::<__siginfo_fpu_t>(); } if wsaved != 0 { size += core::mem::size_of::<__siginfo_rwin_t>(); }
    let sf = get_sigframe(ksig, regs, size as c_ulong) as *mut rt_signal_frame; if invalid_frame_pointer(sf as *mut c_void, size) { force_exit_sig(SIGILL); return -EINVAL; }
    let mut tail = (sf as *mut u8).add(core::mem::size_of::<rt_signal_frame>()); let mut psr = (*regs).psr; if used_math() { psr |= PSR_EF; }
    let mut err = __put_user((*regs).pc, &mut (*sf).regs.pc); err |= __put_user((*regs).npc, &mut (*sf).regs.npc); err |= __put_user((*regs).y, &mut (*sf).regs.y); err |= __put_user(psr, &mut (*sf).regs.psr); err |= __copy_to_user((*sf).regs.u_regs.as_mut_ptr() as *mut c_void, (*regs).u_regs.as_ptr() as *const c_void, core::mem::size_of_val(&(*regs).u_regs)); err |= __put_user(0, &mut (*sf).extra_size);
    if psr & PSR_EF != 0 { let fp = tail as *mut __siginfo_fpu_t; tail = tail.add(core::mem::size_of::<__siginfo_fpu_t>()); err |= save_fpu_state(regs, fp); err |= __put_user(fp, &mut (*sf).fpu_save); } else { err |= __put_user(core::ptr::null_mut(), &mut (*sf).fpu_save); }
    if wsaved != 0 { let rwp = tail as *mut __siginfo_rwin_t; err |= save_rwin_state(wsaved, rwp); err |= __put_user(rwp, &mut (*sf).rwin_save); } else { err |= __put_user(core::ptr::null_mut(), &mut (*sf).rwin_save); }
    err |= __copy_to_user(&mut (*sf).mask as *mut _ as *mut c_void, (*oldset).sig.as_ptr() as *const c_void, core::mem::size_of::<sigset_t>()); err |= __save_altstack(&mut (*sf).stack, (*regs).u_regs[UREG_FP]); err |= copy_siginfo_to_user(&mut (*sf).info, &(*ksig).info); if err != 0 { return err; }
    (*regs).u_regs[UREG_FP] = sf as c_ulong; (*regs).u_regs[UREG_I0] = (*ksig).sig; (*regs).u_regs[UREG_I1] = &(*sf).info as *const _ as c_ulong; (*regs).u_regs[UREG_I2] = &(*sf).regs as *const _ as c_ulong; (*regs).pc = (*ksig).ka.sa.sa_handler as c_ulong; (*regs).npc = (*regs).pc + 4;
    if !(*ksig).ka.ka_restorer.is_null() { (*regs).u_regs[UREG_I7] = (*ksig).ka.ka_restorer as c_ulong; } else { (*regs).u_regs[UREG_I7] = (&(*sf).insns[0] as *const _ as c_ulong) - 2; err |= __put_user(0x82102065, &mut (*sf).insns[0]); err |= __put_user(0x91d02010, &mut (*sf).insns[1]); if err != 0 { return err; } flush_sig_insns(current.mm, &(*sf).insns[0] as *const _ as c_ulong); }
    0
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    let oldset = sigmask_to_save();
    let err = if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { setup_rt_frame(ksig, regs, oldset) } else { setup_frame(ksig, regs, oldset) };
    signal_setup_done(err, ksig, 0);
}

#[inline]
unsafe fn syscall_restart(orig_i0: c_ulong, regs: *mut pt_regs, sa: *mut sigaction) {
    match (*regs).u_regs[UREG_I0] {
        ERESTART_RESTARTBLOCK | ERESTARTNOHAND => { (*regs).u_regs[UREG_I0] = EINTR; (*regs).psr |= PSR_C; }
        ERESTARTSYS => { if (*sa).sa_flags & SA_RESTART == 0 { (*regs).u_regs[UREG_I0] = EINTR; (*regs).psr |= PSR_C; } else { (*regs).u_regs[UREG_I0] = orig_i0; (*regs).pc -= 4; (*regs).npc -= 4; } }
        ERESTARTNOINTR => { (*regs).u_regs[UREG_I0] = orig_i0; (*regs).pc -= 4; (*regs).npc -= 4; }
        _ => {}
    }
}

/* Note that 'init' is a special process: it doesn't get signals it doesn't
 * want to handle. Thus you cannot kill init even with a SIGKILL even by
 * mistake.
 */
unsafe fn do_signal(regs: *mut pt_regs, orig_i0: c_ulong) {
    let mut ksig: ksignal = core::mem::zeroed();
    let mut restart_syscall = false;
    if pt_regs_is_syscall(regs) && (*regs).psr & PSR_C != 0 { (*regs).u_regs[UREG_G6] = orig_i0; }
    let has_handler = get_signal(&mut ksig);
    if pt_regs_is_syscall(regs) && (*regs).psr & PSR_C != 0 { restart_syscall = true; }
    if has_handler { if restart_syscall { syscall_restart(orig_i0, regs, &mut ksig.ka.sa); } handle_signal(&mut ksig, regs); }
    else { if restart_syscall { match (*regs).u_regs[UREG_I0] { ERESTARTNOHAND | ERESTARTSYS | ERESTARTNOINTR => { (*regs).u_regs[UREG_I0] = orig_i0; (*regs).pc -= 4; (*regs).npc -= 4; pt_regs_clear_syscall(regs); }, ERESTART_RESTARTBLOCK => { (*regs).u_regs[UREG_G1] = __NR_restart_syscall; (*regs).pc -= 4; (*regs).npc -= 4; pt_regs_clear_syscall(regs); }, _ => {} } } restore_saved_sigmask(); }
}

unsafe fn do_notify_resume(regs: *mut pt_regs, orig_i0: c_ulong, thread_info_flags: c_ulong) {
    if thread_info_flags & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 { do_signal(regs, orig_i0); }
    if thread_info_flags & _TIF_NOTIFY_RESUME != 0 { resume_user_mode_work(regs); }
}

unsafe fn do_sys_sigstack(ssptr: *mut sigstack, ossptr: *mut sigstack, sp: c_ulong) -> c_int {
    let mut ret = -EFAULT;
    if !ossptr.is_null() { if put_user(current.sas_ss_sp + current.sas_ss_size, &mut (*ossptr).the_stack) != 0 || __put_user(on_sig_stack(sp), &mut (*ossptr).cur_status) != 0 { return ret; } }
    if !ssptr.is_null() { let mut ss_sp: *mut c_char = core::ptr::null_mut(); if get_user(&mut ss_sp, &(*ssptr).the_stack) != 0 { return ret; } ret = -EPERM; if current.sas_ss_sp != 0 && on_sig_stack(sp) { return ret; } current.sas_ss_sp = ss_sp as c_ulong - SIGSTKSZ; current.sas_ss_size = SIGSTKSZ; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
