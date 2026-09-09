/*
 * Copyright (C) 2003 PathScale, Inc.
 * Copyright (C) 2003 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[cfg(CONFIG_X86_32)]
#[repr(C)]
struct _xstate_64 {
    fpstate: _fpstate_64,
    xstate_hdr: _header,
    ymmh: _ymmh_state,
}

#[cfg(not(CONFIG_X86_32))]
type _xstate_64 = _xstate;

unsafe fn copy_sc_from_user(regs: *mut pt_regs, from: *mut sigcontext) -> c_int {
    let mut sc: sigcontext = core::mem::zeroed();

    (*current).restart_block.fn_ = do_no_restart_syscall;
    let err = copy_from_user(&mut sc as *mut _ as *mut c_void, from as *const c_void,
                             core::mem::size_of::<sigcontext>());
    if err != 0 { return err; }

    macro_rules! getreg { ($regno:ident, $regname:ident) => {
        (*regs).regs.gp[HOST_$regno] = sc.$regname;
    }}
    #[cfg(CONFIG_X86_32)] { getreg!(GS, gs); getreg!(FS, fs); getreg!(ES, es); getreg!(DS, ds); }
    getreg!(DI, di); getreg!(SI, si); getreg!(BP, bp); getreg!(SP, sp);
    getreg!(BX, bx); getreg!(DX, dx); getreg!(CX, cx); getreg!(AX, ax); getreg!(IP, ip);
    #[cfg(CONFIG_X86_64)] {
        getreg!(R8, r8); getreg!(R9, r9); getreg!(R10, r10); getreg!(R11, r11);
        getreg!(R12, r12); getreg!(R13, r13); getreg!(R14, r14); getreg!(R15, r15);
    }
    getreg!(CS, cs); getreg!(EFLAGS, flags);
    #[cfg(CONFIG_X86_32)] { getreg!(SS, ss); }

    #[cfg(CONFIG_X86_32)]
    let from_fp64 = (sc.fpstate as usize + core::mem::offset_of!(_fpstate_32, _fxsr_env)) as *mut _xstate_64;
    #[cfg(not(CONFIG_X86_32))]
    let from_fp64 = sc.fpstate as *mut _xstate_64;
    if copy_from_user((*regs).regs.fp as *mut c_void, from_fp64 as *const c_void, host_fp_size) != 0 { return 1; }
    #[cfg(CONFIG_X86_32)] {
        let err = copy_regset_from_user(current, task_user_regset_view(current), REGSET_FP_LEGACY, 0,
            core::mem::size_of::<user_i387_struct>(), sc.fpstate as *mut c_void);
        if err < 0 { return err; }
    }
    0
}

unsafe fn copy_sc_to_user(to: *mut sigcontext, to_fp: *mut _xstate, regs: *mut pt_regs, mask: c_ulong) -> c_int {
    let mut sc: sigcontext = core::mem::zeroed();
    let fi = &(*current).thread.arch.faultinfo;
    macro_rules! putreg { ($regno:ident, $regname:ident) => { sc.$regname = (*regs).regs.gp[HOST_$regno]; }}
    #[cfg(CONFIG_X86_32)] { putreg!(GS, gs); putreg!(FS, fs); putreg!(ES, es); putreg!(DS, ds); }
    putreg!(DI, di); putreg!(SI, si); putreg!(BP, bp); putreg!(SP, sp); putreg!(BX, bx);
    putreg!(DX, dx); putreg!(CX, cx); putreg!(AX, ax);
    #[cfg(CONFIG_X86_64)] { putreg!(R8,r8); putreg!(R9,r9); putreg!(R10,r10); putreg!(R11,r11); putreg!(R12,r12); putreg!(R13,r13); putreg!(R14,r14); putreg!(R15,r15); }
    sc.cr2 = fi.cr2; sc.err = fi.error_code; sc.trapno = fi.trap_no;
    putreg!(IP, ip); putreg!(CS, cs); putreg!(EFLAGS, flags);
    #[cfg(CONFIG_X86_32)] { putreg!(SP, sp_at_signal); putreg!(SS, ss); }
    sc.oldmask = mask; sc.fpstate = to_fp as usize as c_ulong;
    if copy_to_user(to as *mut c_void, &sc as *const _ as *const c_void, core::mem::size_of::<sigcontext>()) != 0 { return 1; }
    #[cfg(CONFIG_X86_32)] {
        let err = copy_regset_to_user(current, task_user_regset_view(current), REGSET_FP_LEGACY, 0,
            core::mem::size_of::<_fpstate_32>(), to_fp as *mut c_void);
        if err < 0 { return err; }
        __put_user(X86_FXSR_MAGIC, &mut (*to_fp).fpstate.magic);
    }
    let to_fp64 = to_fp as *mut _xstate_64;
    if copy_to_user(to_fp64 as *mut c_void, (*regs).regs.fp as *const c_void, host_fp_size) != 0 { return 1; }
    if host_fp_size <= core::mem::size_of::<(*to_fp64).fpstate>() { return 0; }
    #[cfg(CONFIG_X86_32)] let extended_size = core::mem::offset_of!(_fpstate_32, _fxsr_env) + host_fp_size + FP_XSTATE_MAGIC2_SIZE;
    #[cfg(not(CONFIG_X86_32))] let extended_size = host_fp_size + FP_XSTATE_MAGIC2_SIZE;
    __put_user(extended_size, &mut (*to_fp64).fpstate.sw_reserved.extended_size);
    __put_user(host_fp_size, &mut (*to_fp64).fpstate.sw_reserved.xstate_size);
    __put_user(FP_XSTATE_MAGIC1, &mut (*to_fp64).fpstate.sw_reserved.magic1);
    __put_user(FP_XSTATE_MAGIC2, (to_fp64 as *mut u8).add(host_fp_size) as *mut c_int);
    0
}

#[cfg(CONFIG_X86_32)]
unsafe fn copy_ucontext_to_user(uc: *mut ucontext, fp: *mut _xstate, set: *mut sigset_t, sp: c_ulong) -> c_int {
    let mut err = 0;
    err |= __save_altstack(&mut (*uc).uc_stack, sp);
    err |= copy_sc_to_user(&mut (*uc).uc_mcontext, fp, &mut (*current).thread.regs, 0);
    err |= copy_to_user(&mut (*uc).uc_sigmask as *mut _ as *mut c_void, set as *const c_void, core::mem::size_of::<sigset_t>());
    err
}

#[cfg(CONFIG_X86_32)]
unsafe fn setup_signal_stack_sc(mut stack_top: c_ulong, ksig: *mut ksignal, regs: *mut pt_regs, mask: *mut sigset_t) -> c_int {
    let math_size = core::mem::offset_of!(_fpstate_32, _fxsr_env) + host_fp_size + FP_XSTATE_MAGIC2_SIZE;
    stack_top = (stack_top.wrapping_add(4) & !15).wrapping_sub(4);
    let mut frame = (stack_top as *mut sigframe).offset(-1);
    if !access_ok(frame as *mut c_void, core::mem::size_of::<sigframe>()) { return 1; }
    frame = ((frame as usize).wrapping_sub(math_size)) as *mut sigframe;
    let restorer = if (*ksig).ka.sa.sa_flags & SA_RESTORER != 0 { (*ksig).ka.sa.sa_restorer } else { (*frame).retcode };
    let mut err = 0;
    err |= __put_user(restorer, &mut (*frame).pretcode);
    err |= __put_user((*ksig).sig, &mut (*frame).sig);
    let fp_to = frame as usize + core::mem::size_of::<sigframe>();
    err |= copy_sc_to_user(&mut (*frame).sc, fp_to as *mut _xstate, regs, (*mask).sig[0]);
    if _NSIG_WORDS > 1 { err |= __copy_to_user(&mut (*frame).extramask as *mut _ as *mut c_void, &(*mask).sig[1] as *const _ as *const c_void, core::mem::size_of_val(&(*frame).extramask)); }
    err |= __put_user(0xb858u16, (*frame).retcode as *mut u16);
    err |= __put_user(__NR_sigreturn, (*frame).retcode.add(2) as *mut c_int);
    err |= __put_user(0x80cdu16, (*frame).retcode.add(6) as *mut u16);
    if err != 0 { return err; }
    PT_REGS_SP(regs) = frame as c_ulong; PT_REGS_IP(regs) = (*ksig).ka.sa.sa_handler as c_ulong;
    PT_REGS_AX(regs) = (*ksig).sig as c_ulong; PT_REGS_DX(regs) = 0; PT_REGS_CX(regs) = 0; 0
}

#[cfg(CONFIG_X86_32)]
unsafe fn setup_signal_stack_si(mut stack_top: c_ulong, ksig: *mut ksignal, regs: *mut pt_regs, mask: *mut sigset_t) -> c_int {
    let math_size = core::mem::offset_of!(_fpstate_32, _fxsr_env) + host_fp_size + FP_XSTATE_MAGIC2_SIZE;
    stack_top &= !7; let mut frame = (stack_top as *mut rt_sigframe).offset(-1);
    if !access_ok(frame as *mut c_void, core::mem::size_of::<rt_sigframe>()) { return 1; }
    frame = ((frame as usize).wrapping_sub(math_size)) as *mut rt_sigframe;
    let restorer = if (*ksig).ka.sa.sa_flags & SA_RESTORER != 0 { (*ksig).ka.sa.sa_restorer } else { (*frame).retcode };
    let mut err = 0; err |= __put_user(restorer, &mut (*frame).pretcode); err |= __put_user((*ksig).sig, &mut (*frame).sig);
    err |= __put_user(&mut (*frame).info, &mut (*frame).pinfo); err |= __put_user(&mut (*frame).uc, &mut (*frame).puc);
    err |= copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info);
    let fp_to = frame as usize + core::mem::size_of::<rt_sigframe>();
    err |= copy_ucontext_to_user(&mut (*frame).uc, fp_to as *mut _xstate, mask, PT_REGS_SP(regs));
    err |= __put_user(0xb8u8, (*frame).retcode as *mut u8); err |= __put_user(__NR_rt_sigreturn, (*frame).retcode.add(1) as *mut c_int); err |= __put_user(0x80cdu16, (*frame).retcode.add(5) as *mut u16);
    if err != 0 { return err; } PT_REGS_SP(regs)=frame as c_ulong; PT_REGS_IP(regs)=(*ksig).ka.sa.sa_handler as c_ulong; PT_REGS_AX(regs)=(*ksig).sig as c_ulong; PT_REGS_DX(regs)=&mut (*frame).info as *mut _ as c_ulong; PT_REGS_CX(regs)=&mut (*frame).uc as *mut _ as c_ulong; 0
}

#[cfg(not(CONFIG_X86_32))]
unsafe fn setup_signal_stack_si(mut stack_top: c_ulong, ksig: *mut ksignal, regs: *mut pt_regs, set: *mut sigset_t) -> c_int {
    let math_size = host_fp_size + FP_XSTATE_MAGIC2_SIZE;
    let mut frame = (stack_top.wrapping_sub(core::mem::size_of::<rt_sigframe>()) as usize).wrapping_sub(math_size);
    frame = round_down(frame, 16).wrapping_sub(128 + 8);
    let frame = frame as *mut rt_sigframe; if !access_ok(frame as *mut c_void, core::mem::size_of::<rt_sigframe>() + math_size) { return 0; }
    let mut err = 0; if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { err |= copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info); }
    err |= __put_user(0, &mut (*frame).uc.uc_flags); err |= __put_user(core::ptr::null_mut(), &mut (*frame).uc.uc_link); err |= __save_altstack(&mut (*frame).uc.uc_stack, PT_REGS_SP(regs));
    let fp_to=frame as usize+core::mem::size_of::<rt_sigframe>(); err |= copy_sc_to_user(&mut (*frame).uc.uc_mcontext, fp_to as *mut _xstate, regs, (*set).sig[0]); err |= __put_user(fp_to, &mut (*frame).uc.uc_mcontext.fpstate);
    err |= __copy_to_user(&mut (*frame).uc.uc_sigmask as *mut _ as *mut c_void, set as *const c_void, core::mem::size_of::<sigset_t>());
    if (*ksig).ka.sa.sa_flags & SA_RESTORER == 0 { return err; } err |= __put_user((*ksig).ka.sa.sa_restorer as *mut c_void, &mut (*frame).pretcode); if err != 0 { return err; }
    PT_REGS_SP(regs)=frame as c_ulong; PT_REGS_DI(regs)=(*ksig).sig as c_ulong; PT_REGS_AX(regs)=0; PT_REGS_SI(regs)=&mut (*frame).info as *mut _ as c_ulong; PT_REGS_DX(regs)=&mut (*frame).uc as *mut _ as c_ulong; PT_REGS_IP(regs)=(*ksig).ka.sa.sa_handler as c_ulong; 0
}

unsafe fn sigreturn() -> c_long { let sp=PT_REGS_SP(&mut (*current).thread.regs); let frame=(sp-8) as *mut sigframe; let mut set: sigset_t=core::mem::zeroed(); if copy_from_user(&mut set.sig[0] as *mut _ as *mut c_void,&(*frame).sc.oldmask as *const _ as *const c_void,core::mem::size_of_val(&set.sig[0]))!=0 { force_sig(SIGSEGV); return 0; } set_current_blocked(&mut set); if copy_sc_from_user(&mut (*current).thread.regs,&mut (*frame).sc)!=0 { force_sig(SIGSEGV); return 0; } PT_REGS_SYSCALL_NR(&mut (*current).thread.regs)=-1; PT_REGS_SYSCALL_RET(&mut (*current).thread.regs) }
unsafe fn rt_sigreturn() -> c_long { let sp=PT_REGS_SP(&mut (*current).thread.regs); let frame=(sp-core::mem::size_of::<c_ulong>()) as *mut rt_sigframe; let mut set: sigset_t=core::mem::zeroed(); if copy_from_user(&mut set as *mut _ as *mut c_void,&(*frame).uc.uc_sigmask as *const _ as *const c_void,core::mem::size_of::<sigset_t>())!=0 { force_sig(SIGSEGV); return 0; } set_current_blocked(&mut set); if copy_sc_from_user(&mut (*current).thread.regs,&mut (*frame).uc.uc_mcontext)!=0 { force_sig(SIGSEGV); return 0; } PT_REGS_SYSCALL_NR(&mut (*current).thread.regs)=-1; PT_REGS_SYSCALL_RET(&mut (*current).thread.regs) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
