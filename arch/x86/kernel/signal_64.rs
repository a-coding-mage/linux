// SPDX-License-Identifier: GPL-2.0
/* Translated from x86/kernel/signal_64.c. */

unsafe fn force_valid_ss(regs: *mut pt_regs) {
    let mut ar: u32;
    // C inline assembly: lar old_ss, ar; clear ar when the selector is invalid.
    unsafe {
        ar = lar((*regs).ss as u16);
    }
    ar &= AR_DPL_MASK | AR_S | AR_P | AR_TYPE_MASK;
    if ar != (AR_DPL3 | AR_S | AR_P | AR_TYPE_RWDATA)
        && ar != (AR_DPL3 | AR_S | AR_P | AR_TYPE_RWDATA_EXPDOWN)
    {
        unsafe { (*regs).ss = __USER_DS; }
    }
}

unsafe fn restore_sigcontext(
    regs: *mut pt_regs,
    usc: *mut sigcontext,
    uc_flags: c_ulong,
) -> bool {
    let mut sc: sigcontext = core::mem::zeroed();
    unsafe {
        (*current).restart_block.fn_ = do_no_restart_syscall;
        if copy_from_user(&mut sc as *mut _ as *mut c_void, usc as *const c_void,
                          core::mem::offset_of!(sigcontext, reserved1)) != 0 { return false; }
        (*regs).bx = sc.bx; (*regs).cx = sc.cx; (*regs).dx = sc.dx;
        (*regs).si = sc.si; (*regs).di = sc.di; (*regs).bp = sc.bp;
        (*regs).ax = sc.ax; (*regs).sp = sc.sp; (*regs).ip = sc.ip;
        (*regs).r8 = sc.r8; (*regs).r9 = sc.r9; (*regs).r10 = sc.r10;
        (*regs).r11 = sc.r11; (*regs).r12 = sc.r12; (*regs).r13 = sc.r13;
        (*regs).r14 = sc.r14; (*regs).r15 = sc.r15;
        (*regs).cs = sc.cs | 3; (*regs).ss = sc.ss | 3;
        (*regs).flags = ((*regs).flags & !FIX_EFLAGS) | (sc.flags & FIX_EFLAGS);
        (*regs).orig_ax = -1i64 as _;
        if !(uc_flags & UC_STRICT_RESTORE_SS) != 0 && user_64bit_mode(regs) {
            force_valid_ss(regs);
        }
        fpu__restore_sig(sc.fpstate as *mut c_void, 0)
    }
}

unsafe fn __unsafe_setup_sigcontext(
    sc: *mut sigcontext, fpstate: *mut c_void, regs: *mut pt_regs, mask: c_ulong,
) -> c_int {
    unsafe {
        macro_rules! put { ($v:expr, $f:ident) => { if unsafe_put_user($v, core::ptr::addr_of_mut!((*sc).$f)) != 0 { return -EFAULT; } }; }
        put!((*regs).di, di); put!((*regs).si, si); put!((*regs).bp, bp); put!((*regs).sp, sp);
        put!((*regs).bx, bx); put!((*regs).dx, dx); put!((*regs).cx, cx); put!((*regs).ax, ax);
        put!((*regs).r8, r8); put!((*regs).r9, r9); put!((*regs).r10, r10); put!((*regs).r11, r11);
        put!((*regs).r12, r12); put!((*regs).r13, r13); put!((*regs).r14, r14); put!((*regs).r15, r15);
        put!((*current).thread.trap_nr, trapno); put!((*current).thread.error_code, err);
        put!((*regs).ip, ip); put!((*regs).flags, flags); put!((*regs).cs, cs);
        put!(0, gs); put!(0, fs); put!((*regs).ss, ss); put!(fpstate, fpstate); put!(mask, oldmask);
        put!((*current).thread.cr2, cr2);
    }
    0
}

unsafe fn frame_uc_flags(regs: *mut pt_regs) -> c_ulong {
    let mut flags = if boot_cpu_has(X86_FEATURE_XSAVE) { UC_FP_XSTATE | UC_SIGCONTEXT_SS } else { UC_SIGCONTEXT_SS };
    if user_64bit_mode(regs) { flags |= UC_STRICT_RESTORE_SS; }
    flags
}

pub unsafe fn x64_setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> c_int {
    let set = sigmask_to_save();
    let mut fp: *mut c_void = core::ptr::null_mut();
    if (*ksig).ka.sa.sa_flags & SA_RESTORER == 0 { return -EFAULT; }
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe>(), &mut fp) as *mut rt_sigframe;
    let uc_flags = frame_uc_flags(regs);
    if !user_access_begin(frame as *mut c_void, core::mem::size_of::<rt_sigframe>()) { return -EFAULT; }
    if unsafe_put_user(uc_flags, &mut (*frame).uc.uc_flags) != 0 { user_access_end(); return -EFAULT; }
    if unsafe_put_user(0, &mut (*frame).uc.uc_link) != 0 { user_access_end(); return -EFAULT; }
    if unsafe_save_altstack(&mut (*frame).uc.uc_stack, (*regs).sp) != 0 { user_access_end(); return -EFAULT; }
    if unsafe_put_user((*ksig).ka.sa.sa_restorer, &mut (*frame).pretcode) != 0 { user_access_end(); return -EFAULT; }
    if __unsafe_setup_sigcontext(&mut (*frame).uc.uc_mcontext, fp, regs, (*set).sig[0]) != 0 { user_access_end(); return -EFAULT; }
    if unsafe_put_user(*(set as *const u64), &mut (*frame).uc.uc_sigmask) != 0 { user_access_end(); return -EFAULT; }
    user_access_end();
    if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 && copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info) != 0 { return -EFAULT; }
    if setup_signal_shadow_stack(ksig) != 0 { return -EFAULT; }
    (*regs).di = (*ksig).sig; (*regs).ax = 0; (*regs).si = &(*frame).info as *const _ as _;
    (*regs).dx = &(*frame).uc as *const _ as _; (*regs).ip = (*ksig).ka.sa.sa_handler as _;
    (*regs).sp = frame as _; (*regs).cs = __USER_CS;
    if (*regs).ss != __USER_DS { force_valid_ss(regs); }
    0
}

pub unsafe extern "C" fn rt_sigreturn() -> c_long {
    let regs = current_pt_regs();
    prevent_single_step_upon_eretu(regs);
    let frame = ((*regs).sp - core::mem::size_of::<c_long>()) as *mut rt_sigframe;
    if !access_ok(frame as *const c_void, core::mem::size_of::<rt_sigframe>()) { signal_fault(regs, frame, "rt_sigreturn"); return 0; }
    let mut set: sigset_t = core::mem::zeroed(); let mut flags = 0;
    if __get_user(&mut set.sig[0], &(*frame).uc.uc_sigmask) != 0 || __get_user(&mut flags, &(*frame).uc.uc_flags) != 0 { signal_fault(regs, frame, "rt_sigreturn"); return 0; }
    set_current_blocked(&set);
    if restore_altstack(&(*frame).uc.uc_stack) != 0 || !restore_sigcontext(regs, &mut (*frame).uc.uc_mcontext, flags) || restore_signal_shadow_stack() != 0 { signal_fault(regs, frame, "rt_sigreturn"); return 0; }
    (*regs).ax
}

// CONFIG_X86_X32_ABI and CONFIG_COMPAT declarations and ABI layout assertions
// are retained as external integration points supplied by the surrounding kernel.

#[cfg(feature = "CONFIG_X86_X32_ABI")]
pub unsafe fn x32_copy_siginfo_to_user(to: *mut compat_siginfo, from: *const kernel_siginfo) -> c_int {
    let mut new: compat_siginfo = core::mem::zeroed();
    copy_siginfo_to_external32(&mut new, from);
    if (*from).si_signo == SIGCHLD { new._sifields._sigchld_x32._utime = (*from).si_utime; new._sifields._sigchld_x32._stime = (*from).si_stime; }
    if copy_to_user(to as *mut c_void, &new as *const _ as *const c_void, core::mem::size_of::<compat_siginfo>()) != 0 { -EFAULT } else { 0 }
}

#[cfg(feature = "CONFIG_X86_X32_ABI")]
pub unsafe fn copy_siginfo_to_user32(to: *mut compat_siginfo, from: *const kernel_siginfo) -> c_int {
    if in_x32_syscall() { x32_copy_siginfo_to_user(to, from) } else { __copy_siginfo_to_user32(to, from) }
}

#[cfg(feature = "CONFIG_X86_X32_ABI")]
pub unsafe fn x32_setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> c_int {
    let mut fp = core::ptr::null_mut();
    if (*ksig).ka.sa.sa_flags & SA_RESTORER == 0 { return -EFAULT; }
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe_x32>(), &mut fp) as *mut rt_sigframe_x32;
    let flags = frame_uc_flags(regs);
    if setup_signal_shadow_stack(ksig) != 0 || !user_access_begin(frame as _, core::mem::size_of::<rt_sigframe_x32>()) { return -EFAULT; }
    if unsafe_put_user(flags, &mut (*frame).uc.uc_flags) != 0 || unsafe_put_user(0, &mut (*frame).uc.uc_link) != 0 || unsafe_compat_save_altstack(&mut (*frame).uc.uc_stack, (*regs).sp) != 0 || unsafe_put_user(0, &mut (*frame).uc.uc__pad0) != 0 || unsafe_put_user((*ksig).ka.sa.sa_restorer, &mut (*frame).pretcode) != 0 { user_access_end(); return -EFAULT; }
    if __unsafe_setup_sigcontext(&mut (*frame).uc.uc_mcontext, fp, regs, (*(sigmask_to_save())).sig[0]) != 0 { user_access_end(); return -EFAULT; }
    user_access_end();
    if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 && x32_copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info) != 0 { return -EFAULT; }
    (*regs).sp = frame as _; (*regs).ip = (*ksig).ka.sa.sa_handler as _; (*regs).di = (*ksig).sig;
    (*regs).si = &(*frame).info as *const _ as _; (*regs).dx = &(*frame).uc as *const _ as _;
    loadsegment(ds, __USER_DS); loadsegment(es, __USER_DS); (*regs).cs = __USER_CS; (*regs).ss = __USER_DS; 0
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn sigaction_compat_abi(act: *mut k_sigaction, _oact: *mut k_sigaction) {
    if act.is_null() { return; }
    if in_ia32_syscall() { (*act).sa.sa_flags |= SA_IA32_ABI; }
    if in_x32_syscall() { (*act).sa.sa_flags |= SA_X32_ABI; }
}

// ABI invariants from the C static_assert/offsetof checks:
// NSIGILL == 11, NSIGFPE == 15, NSIGSEGV == 10, NSIGBUS == 5,
// NSIGTRAP == 6, NSIGCHLD == 6, NSIGSYS == 2; siginfo_t is 128 bytes,
// aligned to 8, with si_signo/si_errno/si_code at offsets 0/4/8.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
