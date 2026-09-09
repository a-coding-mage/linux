/*
 * Copyright (C) 2013-2014 Altera Corporation
 * Copyright (C) 2011-2012 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd
 * Copyright (C) 1991, 1992 Linus Torvalds
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.

const _BLOCKABLE: libc::c_ulong = !(sigmask(SIGKILL) | sigmask(SIGSTOP));

#[repr(C)]
struct rt_sigframe {
    info: siginfo,
    uc: ucontext,
}

unsafe fn rt_restore_ucontext(
    regs: *mut pt_regs,
    sw: *mut switch_stack,
    uc: *mut ucontext,
    pr2: *mut libc::c_int,
) -> libc::c_int {
    let mut temp: libc::c_int = 0;
    let gregs = unsafe { (*uc).uc_mcontext.gregs };
    let mut err: libc::c_int;

    unsafe { (*current).restart_block.fn_ = do_no_restart_syscall; }

    err = unsafe { __get_user(&mut temp, &(*uc).uc_mcontext.version) };
    if temp != MCONTEXT_VERSION { return 1; }
    err |= unsafe { __get_user(&mut (*regs).r1, gregs.add(0)) };
    err |= unsafe { __get_user(&mut (*regs).r2, gregs.add(1)) };
    err |= unsafe { __get_user(&mut (*regs).r3, gregs.add(2)) };
    err |= unsafe { __get_user(&mut (*regs).r4, gregs.add(3)) };
    err |= unsafe { __get_user(&mut (*regs).r5, gregs.add(4)) };
    err |= unsafe { __get_user(&mut (*regs).r6, gregs.add(5)) };
    err |= unsafe { __get_user(&mut (*regs).r7, gregs.add(6)) };
    err |= unsafe { __get_user(&mut (*regs).r8, gregs.add(7)) };
    err |= unsafe { __get_user(&mut (*regs).r9, gregs.add(8)) };
    err |= unsafe { __get_user(&mut (*regs).r10, gregs.add(9)) };
    err |= unsafe { __get_user(&mut (*regs).r11, gregs.add(10)) };
    err |= unsafe { __get_user(&mut (*regs).r12, gregs.add(11)) };
    err |= unsafe { __get_user(&mut (*regs).r13, gregs.add(12)) };
    err |= unsafe { __get_user(&mut (*regs).r14, gregs.add(13)) };
    err |= unsafe { __get_user(&mut (*regs).r15, gregs.add(14)) };
    err |= unsafe { __get_user(&mut (*sw).r16, gregs.add(15)) };
    err |= unsafe { __get_user(&mut (*sw).r17, gregs.add(16)) };
    err |= unsafe { __get_user(&mut (*sw).r18, gregs.add(17)) };
    err |= unsafe { __get_user(&mut (*sw).r19, gregs.add(18)) };
    err |= unsafe { __get_user(&mut (*sw).r20, gregs.add(19)) };
    err |= unsafe { __get_user(&mut (*sw).r21, gregs.add(20)) };
    err |= unsafe { __get_user(&mut (*sw).r22, gregs.add(21)) };
    err |= unsafe { __get_user(&mut (*sw).r23, gregs.add(22)) };
    err |= unsafe { __get_user(&mut (*sw).fp, gregs.add(24)) };
    err |= unsafe { __get_user(&mut (*sw).gp, gregs.add(25)) };
    err |= unsafe { __get_user(&mut temp, gregs.add(26)) };
    err |= unsafe { __get_user(&mut (*regs).ea, gregs.add(27)) };
    err |= unsafe { __get_user(&mut (*regs).ra, gregs.add(23)) };
    err |= unsafe { __get_user(&mut (*regs).sp, gregs.add(28)) };
    unsafe { (*regs).orig_r2 = -1; }
    err |= unsafe { restore_altstack(&(*uc).uc_stack) };
    if err != 0 { return 1; }
    unsafe { *pr2 = (*regs).r2; }
    err
}

unsafe fn do_rt_sigreturn(sw: *mut switch_stack) -> libc::c_int {
    let regs = unsafe { sw.add(1) as *mut pt_regs };
    let frame = unsafe { (*regs).sp as *mut rt_sigframe };
    let mut set: sigset_t = core::mem::zeroed();
    let mut rval = 0;
    if !unsafe { access_ok(frame, core::mem::size_of::<rt_sigframe>()) } { force_sig(SIGSEGV); return 0; }
    if unsafe { __copy_from_user(&mut set, &(*frame).uc.uc_sigmask, core::mem::size_of::<sigset_t>()) } != 0 { force_sig(SIGSEGV); return 0; }
    unsafe { set_current_blocked(&set); }
    if unsafe { rt_restore_ucontext(regs, sw, &mut (*frame).uc, &mut rval) } != 0 { force_sig(SIGSEGV); return 0; }
    rval
}

unsafe fn rt_setup_ucontext(uc: *mut ucontext, regs: *mut pt_regs) -> libc::c_int {
    let sw = unsafe { (regs as *mut switch_stack).sub(1) };
    let gregs = unsafe { (*uc).uc_mcontext.gregs };
    let mut err = 0;
    err |= unsafe { __put_user(MCONTEXT_VERSION, &mut (*uc).uc_mcontext.version) };
    err |= unsafe { __put_user((*regs).r1, gregs.add(0)) }; err |= unsafe { __put_user((*regs).r2, gregs.add(1)) };
    err |= unsafe { __put_user((*regs).r3, gregs.add(2)) }; err |= unsafe { __put_user((*regs).r4, gregs.add(3)) };
    err |= unsafe { __put_user((*regs).r5, gregs.add(4)) }; err |= unsafe { __put_user((*regs).r6, gregs.add(5)) };
    err |= unsafe { __put_user((*regs).r7, gregs.add(6)) }; err |= unsafe { __put_user((*regs).r8, gregs.add(7)) };
    err |= unsafe { __put_user((*regs).r9, gregs.add(8)) }; err |= unsafe { __put_user((*regs).r10, gregs.add(9)) };
    err |= unsafe { __put_user((*regs).r11, gregs.add(10)) }; err |= unsafe { __put_user((*regs).r12, gregs.add(11)) };
    err |= unsafe { __put_user((*regs).r13, gregs.add(12)) }; err |= unsafe { __put_user((*regs).r14, gregs.add(13)) };
    err |= unsafe { __put_user((*regs).r15, gregs.add(14)) }; err |= unsafe { __put_user((*sw).r16, gregs.add(15)) };
    err |= unsafe { __put_user((*sw).r17, gregs.add(16)) }; err |= unsafe { __put_user((*sw).r18, gregs.add(17)) };
    err |= unsafe { __put_user((*sw).r19, gregs.add(18)) }; err |= unsafe { __put_user((*sw).r20, gregs.add(19)) };
    err |= unsafe { __put_user((*sw).r21, gregs.add(20)) }; err |= unsafe { __put_user((*sw).r22, gregs.add(21)) };
    err |= unsafe { __put_user((*sw).r23, gregs.add(22)) }; err |= unsafe { __put_user((*regs).ra, gregs.add(23)) };
    err |= unsafe { __put_user((*sw).fp, gregs.add(24)) }; err |= unsafe { __put_user((*sw).gp, gregs.add(25)) };
    err |= unsafe { __put_user((*regs).ea, gregs.add(27)) }; err |= unsafe { __put_user((*regs).sp, gregs.add(28)) };
    err
}

unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, frame_size: usize) -> *mut libc::c_void {
    let usp = unsafe { sigsp((*regs).sp, ksig) };
    ((usp.wrapping_sub(frame_size as u64)) & !7u64) as *mut libc::c_void
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> libc::c_int {
    let frame = unsafe { get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe>()) as *mut rt_sigframe };
    let mut err = 0;
    if unsafe { (*ksig).ka.sa.sa_flags & SA_SIGINFO } != 0 { err |= unsafe { copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info) }; }
    err |= unsafe { __put_user(0, &mut (*frame).uc.uc_flags) }; err |= unsafe { __put_user(0, &mut (*frame).uc.uc_link) };
    err |= unsafe { __save_altstack(&mut (*frame).uc.uc_stack, (*regs).sp) };
    err |= unsafe { rt_setup_ucontext(&mut (*frame).uc, regs) };
    err |= unsafe { copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>()) };
    if err != 0 { unsafe { force_sigsegv((*ksig).sig); } return -EFAULT; }
    unsafe { (*regs).ra = 0x1044; (*regs).sp = frame as usize as u64; (*regs).r4 = (*ksig).sig as u64; (*regs).r5 = &mut (*frame).info as *mut _ as u64; (*regs).r6 = &mut (*frame).uc as *mut _ as u64; (*regs).ea = (*ksig).ka.sa.sa_handler as u64; }
    0
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    let oldset = unsafe { sigmask_to_save() };
    let ret = unsafe { setup_rt_frame(ksig, oldset, regs) };
    unsafe { signal_setup_done(ret, ksig, 0); }
}

unsafe fn do_signal(regs: *mut pt_regs) -> libc::c_int {
    let mut retval = 0; let mut continue_addr = 0; let mut restart_addr = 0; let mut restart = 0; let mut ksig: ksignal = core::mem::zeroed();
    unsafe { (*current).thread.kregs = regs; }
    if unsafe { (*regs).orig_r2 >= 0 && (*regs).r1 != 0 } {
        continue_addr = unsafe { (*regs).ea }; restart_addr = continue_addr - 4; retval = unsafe { (*regs).r2 };
        match retval { ERESTART_RESTARTBLOCK => { restart = -2; }, ERESTARTNOHAND | ERESTARTSYS | ERESTARTNOINTR => {}, _ => {} }
        if retval == ERESTART_RESTARTBLOCK || retval == ERESTARTNOHAND || retval == ERESTARTSYS || retval == ERESTARTNOINTR { restart += 1; unsafe { (*regs).r2 = (*regs).orig_r2; (*regs).r7 = (*regs).orig_r7; (*regs).ea = restart_addr; } }
        unsafe { (*regs).orig_r2 = -1; }
    }
    if unsafe { get_signal(&mut ksig) } != 0 { if restart != 0 && unsafe { (*regs).ea } == restart_addr && (retval == ERESTARTNOHAND || retval == ERESTART_RESTARTBLOCK || (retval == ERESTARTSYS && unsafe { (*ksig).ka.sa.sa_flags & SA_RESTART == 0 })) { unsafe { (*regs).r2 = EINTR; (*regs).r7 = 1; (*regs).ea = continue_addr; } } unsafe { handle_signal(&mut ksig, regs); } return 0; }
    if restart != 0 && unsafe { (*regs).ea } == restart_addr { unsafe { (*regs).ea = continue_addr; (*regs).r2 = __NR_restart_syscall; } }
    unsafe { restore_saved_sigmask(); }
    restart
}

unsafe fn do_notify_resume(regs: *mut pt_regs) -> libc::c_int {
    if unsafe { !user_mode(regs) } { return 0; }
    if unsafe { test_thread_flag(TIF_SIGPENDING) || test_thread_flag(TIF_NOTIFY_SIGNAL) } { let restart = unsafe { do_signal(regs) }; if restart != 0 { return restart; } }
    else if unsafe { test_thread_flag(TIF_NOTIFY_RESUME) } { unsafe { resume_user_mode_work(regs); } }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
