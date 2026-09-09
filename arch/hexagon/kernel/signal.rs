// SPDX-License-Identifier: GPL-2.0-only
/*
 * Signal support for Hexagon processor
 *
 * Copyright (c) 2010-2012, The Linux Foundation. All rights reserved.
 */

// Kernel and architecture dependencies supplied by other translation units.

#[repr(C)]
pub struct RtSigframe {
    pub tramp: [::core::ffi::c_ulong; 2],
    pub info: Siginfo,
    pub uc: Ucontext,
}

unsafe fn get_sigframe(
    ksig: *mut Ksignal,
    regs: *mut PtRegs,
    frame_size: usize,
) -> *mut ::core::ffi::c_void {
    let sp = sigsp((*regs).r29, ksig);
    ((sp.wrapping_sub(frame_size as ::core::ffi::c_ulong))
        & !(::core::mem::size_of::<u64>() as ::core::ffi::c_ulong - 1)) as *mut ::core::ffi::c_void
}

unsafe fn setup_sigcontext(regs: *mut PtRegs, sc: *mut Sigcontext) -> i32 {
    let mut tmp: ::core::ffi::c_ulong;
    let mut err: i32 = 0;

    err |= copy_to_user(
        &mut (*sc).sc_regs.r0 as *mut _ as *mut ::core::ffi::c_void,
        &(*regs).r00 as *const _ as *const ::core::ffi::c_void,
        32 * ::core::mem::size_of::<::core::ffi::c_ulong>(),
    );

    err |= __put_user((*regs).sa0, &mut (*sc).sc_regs.sa0);
    err |= __put_user((*regs).lc0, &mut (*sc).sc_regs.lc0);
    err |= __put_user((*regs).sa1, &mut (*sc).sc_regs.sa1);
    err |= __put_user((*regs).lc1, &mut (*sc).sc_regs.lc1);
    err |= __put_user((*regs).m0, &mut (*sc).sc_regs.m0);
    err |= __put_user((*regs).m1, &mut (*sc).sc_regs.m1);
    err |= __put_user((*regs).usr, &mut (*sc).sc_regs.usr);
    err |= __put_user((*regs).preds, &mut (*sc).sc_regs.p3_0);
    err |= __put_user((*regs).gp, &mut (*sc).sc_regs.gp);
    err |= __put_user((*regs).ugp, &mut (*sc).sc_regs.ugp);
    // CONFIG_HEXAGON_ARCH_VERSION >= 4
    err |= __put_user((*regs).cs0, &mut (*sc).sc_regs.cs0);
    err |= __put_user((*regs).cs1, &mut (*sc).sc_regs.cs1);
    tmp = pt_elr(regs); err |= __put_user(tmp, &mut (*sc).sc_regs.pc);
    tmp = pt_cause(regs); err |= __put_user(tmp, &mut (*sc).sc_regs.cause);
    tmp = pt_badva(regs); err |= __put_user(tmp, &mut (*sc).sc_regs.badva);
    err
}

unsafe fn restore_sigcontext(regs: *mut PtRegs, sc: *mut Sigcontext) -> i32 {
    let mut tmp: ::core::ffi::c_ulong = 0;
    let mut err: i32 = 0;

    err |= copy_from_user(
        &mut (*regs).r00 as *mut _ as *mut ::core::ffi::c_void,
        &(*sc).sc_regs.r0 as *const _ as *const ::core::ffi::c_void,
        32 * ::core::mem::size_of::<::core::ffi::c_ulong>(),
    );
    err |= __get_user(&mut (*regs).sa0, &(*sc).sc_regs.sa0);
    err |= __get_user(&mut (*regs).lc0, &(*sc).sc_regs.lc0);
    err |= __get_user(&mut (*regs).sa1, &(*sc).sc_regs.sa1);
    err |= __get_user(&mut (*regs).lc1, &(*sc).sc_regs.lc1);
    err |= __get_user(&mut (*regs).m0, &(*sc).sc_regs.m0);
    err |= __get_user(&mut (*regs).m1, &(*sc).sc_regs.m1);
    err |= __get_user(&mut (*regs).usr, &(*sc).sc_regs.usr);
    err |= __get_user(&mut (*regs).preds, &(*sc).sc_regs.p3_0);
    err |= __get_user(&mut (*regs).gp, &(*sc).sc_regs.gp);
    err |= __get_user(&mut (*regs).ugp, &(*sc).sc_regs.ugp);
    // CONFIG_HEXAGON_ARCH_VERSION >= 4
    err |= __get_user(&mut (*regs).cs0, &(*sc).sc_regs.cs0);
    err |= __get_user(&mut (*regs).cs1, &(*sc).sc_regs.cs1);
    err |= __get_user(&mut tmp, &(*sc).sc_regs.pc); pt_set_elr(regs, tmp);
    err
}

/* Setup signal stack frame with siginfo structure */
unsafe fn setup_rt_frame(ksig: *mut Ksignal, set: *mut Sigset, regs: *mut PtRegs) -> i32 {
    let mut err = 0;
    let frame = get_sigframe(ksig, regs, ::core::mem::size_of::<RtSigframe>()) as *mut RtSigframe;
    let vdso = (*(*current()).mm).context.vdso;

    if !access_ok(frame as *const ::core::ffi::c_void, ::core::mem::size_of::<RtSigframe>()) { return -EFAULT; }
    if copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info) != 0 { return -EFAULT; }
    /* The on-stack signal trampoline is no longer executed;
     * however, the libgcc signal frame unwinding code checks for
     * the presence of these two numeric magic values.
     */
    err |= __put_user(0x7800d166, &mut (*frame).tramp[0]);
    err |= __put_user(0x5400c004, &mut (*frame).tramp[1]);
    err |= setup_sigcontext(regs, &mut (*frame).uc.uc_mcontext);
    err |= __copy_to_user(&mut (*frame).uc.uc_sigmask, set, ::core::mem::size_of::<Sigset>());
    err |= __save_altstack(&mut (*frame).uc.uc_stack, user_stack_pointer(regs));
    if err != 0 { return -EFAULT; }

    /* Load r0/r1 pair with signumber/siginfo pointer... */
    (*regs).r0100 = (((&(*frame).info as *const _) as ::core::ffi::c_ulonglong) << 32)
        | (*ksig).sig as ::core::ffi::c_ulonglong;
    (*regs).r02 = &(*frame).uc as *const _ as ::core::ffi::c_ulong;
    (*regs).r31 = (*vdso).rt_signal_trampoline as ::core::ffi::c_ulong;
    pt_psp(regs) = frame as ::core::ffi::c_ulong;
    pt_set_elr(regs, (*ksig).ka.sa.sa_handler as ::core::ffi::c_ulong);
    0
}

/* Setup invocation of signal handler */
unsafe fn handle_signal(ksig: *mut Ksignal, regs: *mut PtRegs) {
    let mut ret;
    if (*regs).syscall_nr >= 0 {
        match (*regs).r00 {
            -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => (*regs).r00 = -EINTR,
            -ERESTARTSYS => {
                if (*ksig).ka.sa.sa_flags & SA_RESTART == 0 { (*regs).r00 = -EINTR; }
                else { (*regs).r06 = (*regs).syscall_nr; pt_set_elr(regs, pt_elr(regs).wrapping_sub(4)); (*regs).r00 = (*regs).restart_r0; }
            }
            -ERESTARTNOINTR => { (*regs).r06 = (*regs).syscall_nr; pt_set_elr(regs, pt_elr(regs).wrapping_sub(4)); (*regs).r00 = (*regs).restart_r0; }
            _ => {}
        }
    }
    ret = setup_rt_frame(ksig, sigmask_to_save(), regs);
    signal_setup_done(ret, ksig, test_thread_flag(TIF_SINGLESTEP));
}

/* Called from return-from-event code. */
pub unsafe fn do_signal(regs: *mut PtRegs) {
    let mut ksig = Ksignal::default();
    if !user_mode(regs) { return; }
    if get_signal(&mut ksig) { handle_signal(&mut ksig, regs); return; }
    if (*regs).syscall_nr >= 0 {
        match (*regs).r00 {
            -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => (*regs).r06 = (*regs).syscall_nr,
            -ERESTART_RESTARTBLOCK => (*regs).r06 = __NR_restart_syscall,
            _ => { restore_saved_sigmask(); return; }
        }
        pt_set_elr(regs, pt_elr(regs).wrapping_sub(4));
        (*regs).r00 = (*regs).restart_r0;
    }
    restore_saved_sigmask();
}

/* Architecture-specific wrapper for signal-related system call */
pub unsafe fn rt_sigreturn() -> ::core::ffi::c_long {
    let regs = current_pt_regs();
    let frame = pt_psp(regs) as *mut RtSigframe;
    let mut blocked = ::core::mem::zeroed::<Sigset>();
    (*current()).restart_block.fn_ = do_no_restart_syscall;
    if !access_ok(frame as *const _, ::core::mem::size_of::<RtSigframe>()) { force_sig(SIGSEGV); return 0; }
    if __copy_from_user(&mut blocked, &(*frame).uc.uc_sigmask, ::core::mem::size_of::<Sigset>()) != 0 { force_sig(SIGSEGV); return 0; }
    set_current_blocked(&blocked);
    if restore_sigcontext(regs, &mut (*frame).uc.uc_mcontext) != 0 { force_sig(SIGSEGV); return 0; }
    pt_psp(regs) = (*regs).r29;
    (*regs).syscall_nr = -1;
    if restore_altstack(&(*frame).uc.uc_stack) != 0 { force_sig(SIGSEGV); return 0; }
    (*regs).r00 as ::core::ffi::c_long
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
