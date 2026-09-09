// SPDX-License-Identifier: GPL-2.0-only
/*
 * Signal Handling for ARC
 *
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * vineetg: Jan 2010 (Restarting of timer related syscalls)
 *
 * vineetg: Nov 2009 (Everything needed for TIF_RESTORE_SIGMASK)
 *  -do_signal() supports TIF_RESTORE_SIGMASK
 *  -do_signal() no longer needs oldset, required by OLD sys_sigsuspend
 *  -sys_rt_sigsuspend() now comes from generic code, so discard arch
 *   implementation
 *  -sys_sigsuspend() no longer needs to fudge ptregs, hence that arg removed
 *  -sys_sigsuspend() no longer loops for do_signal(), sets TIF_xxx and leaves
 *   the job to do_signal()
 *
 * vineetg: July 2009
 *  -Modified Code to support the uClibc provided userland sigreturn stub
 *   to avoid kernel synthesizing it on user stack at runtime, costing TLB
 *   probes and Cache line flushes.
 *
 * vineetg: July 2009
 *  -In stash_usr_regs( ) and restore_usr_regs( ), save/restore of user regs
 *   in done in block copy rather than one word at a time.
 *   This saves around 2K of code and improves LMBench lat_sig <catch>
 *
 * rajeshwarr: Feb 2009
 *  - Support for Realtime Signals
 *
 * vineetg: Aug 11th 2008: Bug #94183
 *  -ViXS were still seeing crashes when using insmod to load drivers.
 *   It turned out that the code to change Execute permssions for TLB entries
 *   of user was not guarded for interrupts (mod_tlb_permission)
 *   This was causing TLB entries to be overwritten on unrelated indexes
 *
 * Vineetg: July 15th 2008: Bug #94183
 *  -Exception happens in Delay slot of a JMP, and before user space resumes,
 *   Signal is delivered (Ctrl + C) = >SIGINT.
 *   setup_frame( ) sets up PC,SP,BLINK to enable user space signal handler
 *   to run, but doesn't clear the Delay slot bit from status32. As a result,
 *   on resuming user mode, signal handler branches off to BTA of orig JMP
 *  -FIX: clear the DE bit from status32 in setup_frame( )
 *
 * Rahul Trivedi, Kanika Nema: Codito Technologies 2004
 */

#[repr(C)]
pub struct RtSigframe {
    pub info: siginfo,
    pub uc: ucontext,
    pub sigret_magic: u32,
}

pub const MAGIC_SIGALTSTK: u32 = 0x07302004;

unsafe fn save_arcv2_regs(mctx: *mut sigcontext, regs: *mut pt_regs) -> i32 {
    let mut err: i32 = 0;
    // CONFIG_ISA_ARCOMPACT excludes this block when enabled.
    #[cfg(not(CONFIG_ISA_ARCOMPACT))]
    {
        let mut v2abi: user_regs_arcv2 = core::mem::zeroed();
        (*(&mut v2abi)).r30 = (*regs).r30;
        #[cfg(CONFIG_ARC_HAS_ACCL_REGS)]
        {
            v2abi.r58 = (*regs).r58;
            v2abi.r59 = (*regs).r59;
        }
        #[cfg(not(CONFIG_ARC_HAS_ACCL_REGS))]
        {
            v2abi.r58 = 0;
            v2abi.r59 = 0;
        }
        err = __copy_to_user(&mut (*mctx).v2abi as *mut _, &v2abi as *const _, core::mem::size_of::<user_regs_arcv2>());
    }
    err
}

unsafe fn restore_arcv2_regs(mctx: *mut sigcontext, regs: *mut pt_regs) -> i32 {
    let mut err: i32 = 0;
    // CONFIG_ISA_ARCOMPACT excludes this block when enabled.
    #[cfg(not(CONFIG_ISA_ARCOMPACT))]
    {
        let mut v2abi: user_regs_arcv2 = core::mem::zeroed();
        err = __copy_from_user(&mut v2abi, &(*mctx).v2abi as *const _, core::mem::size_of::<user_regs_arcv2>());
        (*regs).r30 = v2abi.r30;
        #[cfg(CONFIG_ARC_HAS_ACCL_REGS)]
        {
            (*regs).r58 = v2abi.r58;
            (*regs).r59 = v2abi.r59;
        }
    }
    err
}

unsafe fn stash_usr_regs(sf: *mut RtSigframe, regs: *mut pt_regs, set: *mut sigset_t) -> i32 {
    let mut err: i32;
    let mut uregs: user_regs_struct = core::mem::zeroed();
    uregs.scratch.bta = (*regs).bta; uregs.scratch.lp_start = (*regs).lp_start;
    uregs.scratch.lp_end = (*regs).lp_end; uregs.scratch.lp_count = (*regs).lp_count;
    uregs.scratch.status32 = (*regs).status32; uregs.scratch.ret = (*regs).ret;
    uregs.scratch.blink = (*regs).blink; uregs.scratch.fp = (*regs).fp;
    uregs.scratch.gp = (*regs).r26; uregs.scratch.r12 = (*regs).r12;
    uregs.scratch.r11 = (*regs).r11; uregs.scratch.r10 = (*regs).r10;
    uregs.scratch.r9 = (*regs).r9; uregs.scratch.r8 = (*regs).r8;
    uregs.scratch.r7 = (*regs).r7; uregs.scratch.r6 = (*regs).r6;
    uregs.scratch.r5 = (*regs).r5; uregs.scratch.r4 = (*regs).r4;
    uregs.scratch.r3 = (*regs).r3; uregs.scratch.r2 = (*regs).r2;
    uregs.scratch.r1 = (*regs).r1; uregs.scratch.r0 = (*regs).r0;
    uregs.scratch.sp = (*regs).sp;
    err = __copy_to_user(&mut (*sf).uc.uc_mcontext.regs.scratch as *mut _, &uregs.scratch as *const _, core::mem::size_of_val(&uregs.scratch));
    if is_isa_arcv2() { err |= save_arcv2_regs(&mut (*sf).uc.uc_mcontext, regs); }
    err |= __copy_to_user(&mut (*sf).uc.uc_sigmask as *mut _, set as *const _, core::mem::size_of::<sigset_t>());
    if err != 0 { -EFAULT } else { 0 }
}

unsafe fn restore_usr_regs(regs: *mut pt_regs, sf: *mut RtSigframe) -> i32 {
    let mut set: sigset_t = core::mem::zeroed();
    let mut err: i32;
    let mut uregs: user_regs_struct = core::mem::zeroed();
    err = __copy_from_user(&mut set, &(*sf).uc.uc_sigmask as *const _, core::mem::size_of_val(&set));
    err |= __copy_from_user(&mut uregs.scratch, &(*sf).uc.uc_mcontext.regs.scratch as *const _, core::mem::size_of_val(&uregs.scratch));
    if is_isa_arcv2() { err |= restore_arcv2_regs(&mut (*sf).uc.uc_mcontext, regs); }
    if err != 0 { return -EFAULT; }
    set_current_blocked(&set);
    (*regs).bta=uregs.scratch.bta; (*regs).lp_start=uregs.scratch.lp_start; (*regs).lp_end=uregs.scratch.lp_end;
    (*regs).lp_count=uregs.scratch.lp_count; (*regs).status32=uregs.scratch.status32; (*regs).ret=uregs.scratch.ret;
    (*regs).blink=uregs.scratch.blink; (*regs).fp=uregs.scratch.fp; (*regs).r26=uregs.scratch.gp;
    (*regs).r12=uregs.scratch.r12; (*regs).r11=uregs.scratch.r11; (*regs).r10=uregs.scratch.r10;
    (*regs).r9=uregs.scratch.r9; (*regs).r8=uregs.scratch.r8; (*regs).r7=uregs.scratch.r7;
    (*regs).r6=uregs.scratch.r6; (*regs).r5=uregs.scratch.r5; (*regs).r4=uregs.scratch.r4;
    (*regs).r3=uregs.scratch.r3; (*regs).r2=uregs.scratch.r2; (*regs).r1=uregs.scratch.r1;
    (*regs).r0=uregs.scratch.r0; (*regs).sp=uregs.scratch.sp;
    0
}

#[inline] unsafe fn is_do_ss_needed(magic: u32) -> i32 { if MAGIC_SIGALTSTK == magic { 1 } else { 0 } }

pub unsafe fn rt_sigreturn() -> usize {
    let regs = current_pt_regs();
    (*current()).restart_block.fn_ = do_no_restart_syscall;
    if (*regs).sp & 3 != 0 { force_sig(SIGSEGV); return 0; }
    let sf = (*regs).sp as *mut RtSigframe;
    if !access_ok(sf as *const _, core::mem::size_of::<RtSigframe>()) { force_sig(SIGSEGV); return 0; }
    let mut magic: u32 = 0;
    if __get_user(&mut magic, &(*sf).sigret_magic) != 0 { force_sig(SIGSEGV); return 0; }
    if is_do_ss_needed(magic) != 0 && restore_altstack(&(*sf).uc.uc_stack) != 0 { force_sig(SIGSEGV); return 0; }
    if restore_usr_regs(regs, sf) != 0 { force_sig(SIGSEGV); return 0; }
    syscall_wont_restart(regs);
    (*regs).status32 |= STATUS_U_MASK;
    (*regs).r0 as usize
}

unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, framesize: usize) -> *mut core::ffi::c_void {
    let sp = sigsp((*regs).sp, ksig);
    let frame = ((sp - framesize) & !7) as *mut core::ffi::c_void;
    if !access_ok(frame as *const _, framesize) { core::ptr::null_mut() } else { frame }
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> i32 {
    let sf = get_sigframe(ksig, regs, core::mem::size_of::<RtSigframe>()) as *mut RtSigframe;
    if sf.is_null() { return 1; }
    let mut err = stash_usr_regs(sf, regs, set);
    let mut magic: u32 = 0;
    if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 {
        err |= copy_siginfo_to_user(&mut (*sf).info, &(*ksig).info);
        err |= __put_user(0, &mut (*sf).uc.uc_flags);
        err |= __put_user(core::ptr::null_mut(), &mut (*sf).uc.uc_link);
        err |= __save_altstack(&mut (*sf).uc.uc_stack, (*regs).sp);
        (*regs).r1 = &mut (*sf).info as *mut _ as usize;
        (*regs).r2 = &mut (*sf).uc as *mut _ as usize;
        magic = MAGIC_SIGALTSTK;
    }
    err |= __put_user(magic, &mut (*sf).sigret_magic);
    if err != 0 { return err; }
    (*regs).r0 = (*ksig).sig as _;
    (*regs).ret = (*ksig).ka.sa.sa_handler as usize;
    if (*ksig).ka.sa.sa_flags & SA_RESTORER == 0 { return 1; }
    (*regs).blink = (*ksig).ka.sa.sa_restorer as usize;
    (*regs).sp = sf as usize;
    (*regs).status32 &= !STATUS_DE_MASK;
    (*regs).status32 |= STATUS_L_MASK;
    err
}

unsafe fn arc_restart_syscall(ka: *mut k_sigaction, regs: *mut pt_regs) {
    match (*regs).r0 as isize {
        -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => (*regs).r0 = (-EINTR) as _,
        -ERESTARTSYS => { if (*ka).sa.sa_flags & SA_RESTART == 0 { (*regs).r0 = (-EINTR) as _; } else { (*regs).r0 = (*regs).orig_r0; (*regs).ret -= if is_isa_arcv2() { 2 } else { 4 }; } },
        -ERESTARTNOINTR => { (*regs).r0 = (*regs).orig_r0; (*regs).ret -= if is_isa_arcv2() { 2 } else { 4 }; },
        _ => {}
    }
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    let oldset = sigmask_to_save();
    let failed = setup_rt_frame(ksig, oldset, regs);
    signal_setup_done(failed, ksig, 0);
}

pub unsafe fn do_signal(regs: *mut pt_regs) {
    let mut ksig: ksignal = core::mem::zeroed();
    let restart_scall = in_syscall(regs) && syscall_restartable(regs);
    if test_thread_flag(TIF_SIGPENDING) && get_signal(&mut ksig) {
        if restart_scall { arc_restart_syscall(&mut ksig.ka, regs); syscall_wont_restart(regs); }
        handle_signal(&mut ksig, regs); return;
    }
    if restart_scall {
        match (*regs).r0 as isize {
            -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => { (*regs).r0 = (*regs).orig_r0; (*regs).ret -= if is_isa_arcv2() { 2 } else { 4 }; },
            -ERESTART_RESTARTBLOCK => { (*regs).r8 = __NR_restart_syscall; (*regs).ret -= if is_isa_arcv2() { 2 } else { 4 }; },
            _ => {}
        }
        syscall_wont_restart(regs);
    }
    restore_saved_sigmask();
}

pub unsafe fn do_notify_resume(regs: *mut pt_regs) {
    if test_thread_flag(TIF_NOTIFY_RESUME) { resume_user_mode_work(regs); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
