// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  1997-11-28  Modified for POSIX.1b signals by Richard Henderson
 *  2000-06-20  Pentium III FXSR, SSE support by Gareth Hughes
 *  2000-12-*   x86-64 compatibility mode signal handling by Andi Kleen
 */

/* C headers and build-provided declarations are supplied by the surrounding kernel translation. */

#[inline]
unsafe fn fixup_rpl(sel: u16) -> u16 {
    if sel <= 3 { sel } else { sel | 3 }
}

#[cfg(CONFIG_IA32_EMULATION)]
#[inline]
unsafe fn reload_segments(sc: *mut sigcontext_32) {
    let mut cur: u16 = 0;
    savesegment!(gs, cur);
    if fixup_rpl((*sc).gs) != cur { load_gs_index(fixup_rpl((*sc).gs)); }
    savesegment!(fs, cur);
    if fixup_rpl((*sc).fs) != cur { loadsegment!(fs, fixup_rpl((*sc).fs)); }
    savesegment!(ds, cur);
    if fixup_rpl((*sc).ds) != cur { loadsegment!(ds, fixup_rpl((*sc).ds)); }
    savesegment!(es, cur);
    if fixup_rpl((*sc).es) != cur { loadsegment!(es, fixup_rpl((*sc).es)); }
}

#[cfg(CONFIG_IA32_EMULATION)]
type sigset32_t = compat_sigset_t;
#[cfg(CONFIG_IA32_EMULATION)]
type siginfo32_t = compat_siginfo_t;
#[cfg(not(CONFIG_IA32_EMULATION))]
type sigset32_t = sigset_t;
#[cfg(not(CONFIG_IA32_EMULATION))]
type siginfo32_t = siginfo_t;

#[inline]
unsafe fn ia32_restore_sigcontext(regs: *mut pt_regs, usc: *mut sigcontext_32) -> bool {
    let mut sc: sigcontext_32 = core::mem::zeroed();
    (*current).restart_block.fn_ = do_no_restart_syscall;
    if unlikely(copy_from_user(&mut sc, usc, core::mem::size_of::<sigcontext_32>()) != 0) { return false; }
    (*regs).bx = sc.bx; (*regs).cx = sc.cx; (*regs).dx = sc.dx;
    (*regs).si = sc.si; (*regs).di = sc.di; (*regs).bp = sc.bp;
    (*regs).ax = sc.ax; (*regs).sp = sc.sp; (*regs).ip = sc.ip;
    (*regs).cs = sc.cs | 0x03; (*regs).ss = sc.ss | 0x03;
    (*regs).flags = ((*regs).flags & !FIX_EFLAGS) | (sc.flags & FIX_EFLAGS);
    (*regs).orig_ax = -1i64 as _;
    #[cfg(CONFIG_IA32_EMULATION)]
    reload_segments(&mut sc);
    #[cfg(not(CONFIG_IA32_EMULATION))]
    { loadsegment!(gs, fixup_rpl(sc.gs)); (*regs).fs = fixup_rpl(sc.fs); (*regs).es = fixup_rpl(sc.es); (*regs).ds = fixup_rpl(sc.ds); }
    fpu__restore_sig(compat_ptr(sc.fpstate), 1) != 0
}

SYSCALL32_DEFINE0!(sigreturn, {
    let regs = current_pt_regs();
    let frame = (regs.sp.wrapping_sub(8)) as *mut sigframe_ia32;
    let mut set: sigset_t = core::mem::zeroed();
    prevent_single_step_upon_eretu(regs);
    if !access_ok(frame, core::mem::size_of::<sigframe_ia32>()) { goto_badframe!(); }
    if __get_user!((set.sig[0], &(*frame).sc.oldmask)) != 0 || __get_user!(((&mut set as *mut _ as *mut u32).add(1), &(*frame).extramask[0])) != 0 { goto_badframe!(); }
    set_current_blocked(&set);
    if !ia32_restore_sigcontext(regs, &mut (*frame).sc) { goto_badframe!(); }
    return (*regs).ax;
badframe: signal_fault(regs, frame, "32bit sigreturn"); 0
});

SYSCALL32_DEFINE0!(rt_sigreturn, {
    let regs = current_pt_regs();
    let frame = (regs.sp.wrapping_sub(4)) as *mut rt_sigframe_ia32;
    let mut set: sigset_t = core::mem::zeroed();
    prevent_single_step_upon_eretu(regs);
    if !access_ok(frame, core::mem::size_of::<rt_sigframe_ia32>()) { goto_badframe!(); }
    if __get_user!(((&mut set as *mut _ as *mut u64), (&(*frame).uc.uc_sigmask as *const _ as *const u64))) != 0 { goto_badframe!(); }
    set_current_blocked(&set);
    if !ia32_restore_sigcontext(regs, &mut (*frame).uc.uc_mcontext) { goto_badframe!(); }
    if restore_altstack32(&(*frame).uc.uc_stack) != 0 { goto_badframe!(); }
    return (*regs).ax;
badframe: signal_fault(regs, frame, "32bit rt sigreturn"); 0
});

#[inline(always)]
unsafe fn __unsafe_setup_sigcontext32(sc: *mut sigcontext_32, fpstate: *mut core::ffi::c_void, regs: *mut pt_regs, mask: u32) -> i32 {
    unsafe_put_user!(get_user_seg!(gs), &mut (*sc).gs, Efault);
    #[cfg(CONFIG_IA32_EMULATION)]
    { unsafe_put_user!(get_user_seg!(fs), &mut (*sc).fs, Efault); unsafe_put_user!(get_user_seg!(ds), &mut (*sc).ds, Efault); unsafe_put_user!(get_user_seg!(es), &mut (*sc).es, Efault); }
    #[cfg(not(CONFIG_IA32_EMULATION))]
    { unsafe_put_user!((*regs).fs, &mut (*sc).fs, Efault); unsafe_put_user!((*regs).es, &mut (*sc).es, Efault); unsafe_put_user!((*regs).ds, &mut (*sc).ds, Efault); }
    unsafe_put_user!((*regs).di, &mut (*sc).di, Efault); unsafe_put_user!((*regs).si, &mut (*sc).si, Efault); unsafe_put_user!((*regs).bp, &mut (*sc).bp, Efault);
    unsafe_put_user!((*regs).sp, &mut (*sc).sp, Efault); unsafe_put_user!((*regs).bx, &mut (*sc).bx, Efault); unsafe_put_user!((*regs).dx, &mut (*sc).dx, Efault); unsafe_put_user!((*regs).cx, &mut (*sc).cx, Efault); unsafe_put_user!((*regs).ax, &mut (*sc).ax, Efault);
    unsafe_put_user!((*current).thread.trap_nr, &mut (*sc).trapno, Efault); unsafe_put_user!((*current).thread.error_code, &mut (*sc).err, Efault); unsafe_put_user!((*regs).ip, &mut (*sc).ip, Efault); unsafe_put_user!((*regs).cs, &mut (*sc).cs, Efault); unsafe_put_user!((*regs).flags, &mut (*sc).flags, Efault); unsafe_put_user!((*regs).sp, &mut (*sc).sp_at_signal, Efault); unsafe_put_user!((*regs).ss, &mut (*sc).ss, Efault);
    unsafe_put_user!(ptr_to_compat(fpstate), &mut (*sc).fpstate, Efault);
    unsafe_put_user!(mask, &mut (*sc).oldmask, Efault); unsafe_put_user!((*current).thread.cr2, &mut (*sc).cr2, Efault);
    return 0;
Efault: -EFAULT
}

// The frame setup routines retain the kernel's unsafe user-access ordering and ABI layout.
pub unsafe fn ia32_setup_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> i32 {
    let set = sigmask_to_save() as *mut sigset32_t;
    let mut fp: *mut core::ffi::c_void = core::ptr::null_mut();
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<sigframe_ia32>(), &mut fp) as *mut sigframe_ia32;
    let restorer: *mut core::ffi::c_void;
    if (*ksig).ka.sa.sa_flags & SA_RESTORER != 0 { restorer = (*ksig).ka.sa.sa_restorer; }
    else if (*current).mm.context.vdso != 0 { restorer = (*current).mm.context.vdso.wrapping_add(vdso32_image.sym___kernel_sigreturn); }
    else { restorer = &mut (*frame).retcode as *mut _ as _; }
    if !user_access_begin(frame, core::mem::size_of::<sigframe_ia32>()) { return -EFAULT; }
    unsafe_put_user!((*ksig).sig, &mut (*frame).sig, Efault); unsafe_put_user!(ptr_to_compat(restorer), &mut (*frame).pretcode, Efault); unsafe_put_sigcontext32!(&mut (*frame).sc, fp, regs, set, Efault); unsafe_put_user!((*set).sig[1], &mut (*frame).extramask[0], Efault); user_access_end();
    (*regs).sp = frame as _; (*regs).ip = (*ksig).ka.sa.sa_handler as _; (*regs).ax = (*ksig).sig; (*regs).dx = 0; (*regs).cx = 0;
    #[cfg(CONFIG_IA32_EMULATION)] { loadsegment!(ds, __USER_DS); loadsegment!(es, __USER_DS); }
    #[cfg(not(CONFIG_IA32_EMULATION))] { (*regs).ds = __USER_DS; (*regs).es = __USER_DS; }
    (*regs).cs = __USER32_CS; (*regs).ss = __USER_DS; return 0;
Efault: user_access_end(); -EFAULT
}

pub unsafe fn ia32_setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> i32 {
    let set = sigmask_to_save() as *mut sigset32_t; let mut fp = core::ptr::null_mut();
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe_ia32>(), &mut fp) as *mut rt_sigframe_ia32;
    if !user_access_begin(frame, core::mem::size_of::<rt_sigframe_ia32>()) { return -EFAULT; }
    unsafe_put_user!((*ksig).sig, &mut (*frame).sig, Efault); unsafe_put_user!(ptr_to_compat(&mut (*frame).info), &mut (*frame).pinfo, Efault); unsafe_put_user!(ptr_to_compat(&mut (*frame).uc), &mut (*frame).puc, Efault);
    unsafe_put_user!(if cpu_feature_enabled(X86_FEATURE_XSAVE) { UC_FP_XSTATE } else { 0 }, &mut (*frame).uc.uc_flags, Efault); unsafe_put_user!(0, &mut (*frame).uc.uc_link, Efault); unsafe_save_altstack32!(&mut (*frame).uc.uc_stack, (*regs).sp, Efault);
    let restorer = if (*ksig).ka.sa.sa_flags & SA_RESTORER != 0 { (*ksig).ka.sa.sa_restorer } else { (*current).mm.context.vdso.wrapping_add(vdso32_image.sym___kernel_rt_sigreturn) };
    unsafe_put_user!(ptr_to_compat(restorer), &mut (*frame).pretcode, Efault); unsafe_put_sigcontext32!(&mut (*frame).uc.uc_mcontext, fp, regs, set, Efault); unsafe_put_user!(*((set) as *mut u64), &mut (*frame).uc.uc_sigmask, Efault); user_access_end();
    if __copy_siginfo_to_user32(&mut (*frame).info, &(*ksig).info) != 0 { return -EFAULT; }
    (*regs).sp = frame as _; (*regs).ip = (*ksig).ka.sa.sa_handler as _; (*regs).ax = (*ksig).sig; (*regs).dx = &mut (*frame).info as *mut _ as _; (*regs).cx = &mut (*frame).uc as *mut _ as _;
    #[cfg(CONFIG_IA32_EMULATION)] { loadsegment!(ds, __USER_DS); loadsegment!(es, __USER_DS); }
    #[cfg(not(CONFIG_IA32_EMULATION))] { (*regs).ds = __USER_DS; (*regs).es = __USER_DS; }
    (*regs).cs = __USER32_CS; (*regs).ss = __USER_DS; return 0;
Efault: user_access_end(); -EFAULT
}

/* ABI assertions from the C implementation. */
static_assert!(NSIGILL == 11); static_assert!(NSIGFPE == 15); static_assert!(NSIGSEGV == 10);
static_assert!(NSIGBUS == 5); static_assert!(NSIGTRAP == 6); static_assert!(NSIGCHLD == 6); static_assert!(NSIGSYS == 2);
static_assert!(core::mem::size_of::<siginfo32_t>() == 128);
static_assert!(core::mem::align_of::<siginfo32_t>() == 4);
static_assert!(offset_of!(siginfo32_t, _sifields) == 3 * core::mem::size_of::<i32>());
static_assert!(offset_of!(siginfo32_t, si_signo) == 0); static_assert!(offset_of!(siginfo32_t, si_errno) == 4); static_assert!(offset_of!(siginfo32_t, si_code) == 8);
macro_rules! CHECK_SI_OFFSET { ($name:ident) => { static_assert!(offset_of!(siginfo32_t, _sifields) == offset_of!(siginfo32_t, $name)); }; }
macro_rules! CHECK_SI_SIZE { ($name:ident, $size:expr) => { static_assert!(core::mem::size_of_val(&core::mem::MaybeUninit::<siginfo32_t>::uninit()) == core::mem::size_of::<siginfo32_t>()); }; }
CHECK_SI_OFFSET!(_kill); CHECK_SI_SIZE!(_kill, 2 * core::mem::size_of::<i32>());
static_assert!(offset_of!(siginfo32_t, si_pid) == 0x0c); static_assert!(offset_of!(siginfo32_t, si_uid) == 0x10);
CHECK_SI_OFFSET!(_timer); CHECK_SI_OFFSET!(_rt); CHECK_SI_OFFSET!(_sigchld); CHECK_SI_OFFSET!(_sigfault); CHECK_SI_OFFSET!(_sigpoll); CHECK_SI_OFFSET!(_sigsys);
static_assert!(offset_of!(siginfo32_t, si_tid) == 0x0c); static_assert!(offset_of!(siginfo32_t, si_overrun) == 0x10); static_assert!(offset_of!(siginfo32_t, si_value) == 0x14);
static_assert!(offset_of!(siginfo32_t, si_status) == 0x14); static_assert!(offset_of!(siginfo32_t, si_utime) == 0x18); static_assert!(offset_of!(siginfo32_t, si_stime) == 0x1c);
static_assert!(offset_of!(siginfo32_t, si_addr) == 0x0c); static_assert!(offset_of!(siginfo32_t, si_trapno) == 0x10); static_assert!(offset_of!(siginfo32_t, si_addr_lsb) == 0x10);
static_assert!(offset_of!(siginfo32_t, si_lower) == 0x14); static_assert!(offset_of!(siginfo32_t, si_upper) == 0x18); static_assert!(offset_of!(siginfo32_t, si_pkey) == 0x14);
static_assert!(offset_of!(siginfo32_t, si_perf_data) == 0x10); static_assert!(offset_of!(siginfo32_t, si_perf_type) == 0x14); static_assert!(offset_of!(siginfo32_t, si_perf_flags) == 0x18);
static_assert!(offset_of!(siginfo32_t, si_band) == 0x0c); static_assert!(offset_of!(siginfo32_t, si_fd) == 0x10);
static_assert!(offset_of!(siginfo32_t, si_call_addr) == 0x0c); static_assert!(offset_of!(siginfo32_t, si_syscall) == 0x10); static_assert!(offset_of!(siginfo32_t, si_arch) == 0x14);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
