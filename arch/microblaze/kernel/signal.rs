/*
 * Signal handling
 *
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2003,2004 John Williams <jwilliams@itee.uq.edu.au>
 * Copyright (C) 2001 NEC Corporation
 * Copyright (C) 2001 Miles Bader <miles@gnu.org>
 * Copyright (C) 1999,2000 Niibe Yutaka & Kaz Kojima
 * Copyright (C) 1991,1992 Linus Torvalds
 *
 * 1997-11-28 Modified for POSIX.1b signals by Richard Henderson
 *
 * This file was derived from the sh version, arch/sh/kernel/signal.c
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License. See the file COPYING in the main directory of this
 * archive for more details.
 */

// Kernel and architecture dependencies are supplied by the surrounding crate.

#[repr(C)]
pub struct sigframe {
    pub sc: sigcontext,
    pub extramask: [c_ulong; _NSIG_WORDS - 1],
    pub tramp: [c_ulong; 2],
}

#[repr(C)]
pub struct rt_sigframe {
    pub info: siginfo,
    pub uc: ucontext,
    pub tramp: [c_ulong; 2],
}

unsafe fn restore_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext, rval_p: *mut c_int) -> c_uint {
    let mut err: c_uint = 0;
    macro_rules! copy { ($x:ident) => {{ err |= __get_user(unsafe { (*regs).$x }, unsafe { &(*sc).regs.$x }); }}; }
    copy!(r0); copy!(r1); copy!(r2); copy!(r3); copy!(r4); copy!(r5);
    copy!(r6); copy!(r7); copy!(r8); copy!(r9); copy!(r10); copy!(r11);
    copy!(r12); copy!(r13); copy!(r14); copy!(r15); copy!(r16); copy!(r17);
    copy!(r18); copy!(r19); copy!(r20); copy!(r21); copy!(r22); copy!(r23);
    copy!(r24); copy!(r25); copy!(r26); copy!(r27); copy!(r28); copy!(r29);
    copy!(r30); copy!(r31); copy!(pc); copy!(ear); copy!(esr); copy!(fsr);
    unsafe { *rval_p = (*regs).r3; }
    err
}

pub unsafe extern "C" fn sys_rt_sigreturn(regs: *mut pt_regs) -> c_long {
    let frame = (*regs).r1 as *mut rt_sigframe;
    let mut set: sigset_t = core::mem::zeroed();
    let mut rval: c_int = 0;
    (*current).restart_block.fn_ = do_no_restart_syscall;
    if !access_ok(frame, core::mem::size_of::<rt_sigframe>()) { return badframe(regs); }
    if __copy_from_user(&mut set, &(*frame).uc.uc_sigmask, core::mem::size_of::<sigset_t>()) != 0 { return badframe(regs); }
    set_current_blocked(&set);
    if restore_sigcontext(regs, &mut (*frame).uc.uc_mcontext, &mut rval) != 0 { return badframe(regs); }
    if restore_altstack(&(*frame).uc.uc_stack) != 0 { return badframe(regs); }
    rval as c_long
}

unsafe fn badframe(_regs: *mut pt_regs) -> c_long { force_sig(SIGSEGV); 0 }

unsafe fn setup_sigcontext(sc: *mut sigcontext, regs: *mut pt_regs, mask: c_ulong) -> c_int {
    let mut err: c_int = 0;
    macro_rules! copy { ($x:ident) => {{ err |= __put_user((*regs).$x, &mut (*sc).regs.$x); }}; }
    copy!(r0); copy!(r1); copy!(r2); copy!(r3); copy!(r4); copy!(r5);
    copy!(r6); copy!(r7); copy!(r8); copy!(r9); copy!(r10); copy!(r11);
    copy!(r12); copy!(r13); copy!(r14); copy!(r15); copy!(r16); copy!(r17);
    copy!(r18); copy!(r19); copy!(r20); copy!(r21); copy!(r22); copy!(r23);
    copy!(r24); copy!(r25); copy!(r26); copy!(r27); copy!(r28); copy!(r29);
    copy!(r30); copy!(r31); copy!(pc); copy!(ear); copy!(esr); copy!(fsr);
    err |= __put_user(mask, &mut (*sc).oldmask); err
}

unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, frame_size: usize) -> *mut core::ffi::c_void {
    let sp = sigsp((*regs).r1, ksig);
    ((sp.wrapping_sub(frame_size as c_ulong)) & !7) as *mut core::ffi::c_void
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe>()) as *mut rt_sigframe;
    let mut err: c_int = 0;
    let sig = (*ksig).sig;
    if !access_ok(frame, core::mem::size_of::<rt_sigframe>()) { return -EFAULT; }
    if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { err |= copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info); }
    err |= __put_user(0, &mut (*frame).uc.uc_flags);
    err |= __put_user(core::ptr::null_mut(), &mut (*frame).uc.uc_link);
    err |= __save_altstack(&mut (*frame).uc.uc_stack, (*regs).r1);
    err |= setup_sigcontext(&mut (*frame).uc.uc_mcontext, regs, (*set).sig[0]);
    err |= __copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>());
    err |= __put_user(0x31800000 | __NR_rt_sigreturn, (*frame).tramp.as_mut_ptr());
    err |= __put_user(0xb9cc0008, (*frame).tramp.as_mut_ptr().add(1));
    (*regs).r15 = (*frame).tramp.as_ptr() as c_ulong - 8;
    let mut address = (*frame).tramp.as_ptr() as c_ulong;
    let pmdp = pmd_off((*current).mm, address);
    preempt_disable();
    let ptep = pte_offset_map(pmdp, address);
    if !ptep.is_null() && pte_present(*ptep) {
        address = page_address(pte_page(*ptep));
        address += (*frame).tramp.as_ptr() as c_ulong & !PAGE_MASK;
        address = __virt_to_phys(address);
        invalidate_icache_range(address, address + 8); flush_dcache_range(address, address + 8);
    }
    if !ptep.is_null() { pte_unmap(ptep); } preempt_enable();
    if err != 0 { return -EFAULT; }
    (*regs).r1 = frame as c_ulong; (*regs).r5 = sig; (*regs).r6 = &(*frame).info as *const _ as c_ulong;
    (*regs).r7 = &(*frame).uc as *const _ as c_ulong; (*regs).pc = (*ksig).ka.sa.sa_handler as c_ulong; 0
}

unsafe fn handle_restart(regs: *mut pt_regs, ka: *mut k_sigaction, has_handler: c_int) {
    match (*regs).r3 {
        -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => { if has_handler == 0 { (*regs).pc -= 4; } else { (*regs).r3 = -EINTR; } }
        -ERESTARTSYS => { if has_handler != 0 && (*ka).sa.sa_flags & SA_RESTART == 0 { (*regs).r3 = -EINTR; } else { (*regs).pc -= 4; } }
        -ERESTARTNOINTR => (*regs).pc -= 4,
        _ => {}
    }
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    let oldset = sigmask_to_save();
    let ret = setup_rt_frame(ksig, oldset, regs);
    signal_setup_done(ret, ksig, test_thread_flag(TIF_SINGLESTEP));
}

unsafe fn do_signal(regs: *mut pt_regs, in_syscall: c_int) {
    let mut ksig: ksignal = core::mem::zeroed();
    if get_signal(&mut ksig) != 0 { if in_syscall != 0 { handle_restart(regs, &mut ksig.ka, 1); } handle_signal(&mut ksig, regs); return; }
    if in_syscall != 0 { handle_restart(regs, core::ptr::null_mut(), 0); } restore_saved_sigmask();
}

pub unsafe extern "C" fn do_notify_resume(regs: *mut pt_regs, in_syscall: c_int) {
    if test_thread_flag(TIF_SIGPENDING) != 0 || test_thread_flag(TIF_NOTIFY_SIGNAL) != 0 { do_signal(regs, in_syscall); }
    if test_thread_flag(TIF_NOTIFY_RESUME) != 0 { resume_user_mode_work(regs); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
