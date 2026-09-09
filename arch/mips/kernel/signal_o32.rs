/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1991, 1992  Linus Torvalds
 * Copyright (C) 1994 - 2000, 2006  Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2016, Imagination Technologies Ltd.
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub const __NR_O32_RESTART_SYSCALL: i32 = 4253;

#[repr(C)]
pub struct sigframe32 {
    pub sf_ass: [u32; 4],
    pub sf_pad: [u32; 2],
    pub sf_sc: sigcontext32,
    pub sf_mask: compat_sigset_t,
}

#[repr(C)]
pub struct ucontext32 {
    pub uc_flags: u32,
    pub uc_link: i32,
    pub uc_stack: compat_stack_t,
    pub uc_mcontext: sigcontext32,
    pub uc_sigmask: compat_sigset_t,
}

#[repr(C)]
pub struct rt_sigframe32 {
    pub rs_ass: [u32; 4],
    pub rs_pad: [u32; 2],
    pub rs_info: compat_siginfo_t,
    pub rs_uc: ucontext32,
}

unsafe fn setup_sigcontext32(regs: *mut pt_regs, sc: *mut sigcontext32) -> i32 {
    let mut err: i32 = 0;
    let mut i: usize;

    err |= __put_user((*regs).cp0_epc, &mut (*sc).sc_pc);
    err |= __put_user(0, &mut (*sc).sc_regs[0]);
    i = 1;
    while i < 32 {
        err |= __put_user((*regs).regs[i], &mut (*sc).sc_regs[i]);
        i += 1;
    }

    err |= __put_user((*regs).hi, &mut (*sc).sc_mdhi);
    err |= __put_user((*regs).lo, &mut (*sc).sc_mdlo);
    if cpu_has_dsp {
        err |= __put_user(rddsp(DSP_MASK), &mut (*sc).sc_dsp);
        err |= __put_user(mfhi1(), &mut (*sc).sc_hi1);
        err |= __put_user(mflo1(), &mut (*sc).sc_lo1);
        err |= __put_user(mfhi2(), &mut (*sc).sc_hi2);
        err |= __put_user(mflo2(), &mut (*sc).sc_lo2);
        err |= __put_user(mfhi3(), &mut (*sc).sc_hi3);
        err |= __put_user(mflo3(), &mut (*sc).sc_lo3);
    }

    // Save FPU state to signal context; the signal handler inherits it.
    err |= protected_save_fp_context(sc);
    err
}

unsafe fn restore_sigcontext32(regs: *mut pt_regs, sc: *mut sigcontext32) -> i32 {
    let mut err: i32 = 0;
    let mut treg: i32 = 0;
    let mut i: usize;

    (*current).restart_block.fn_ = do_no_restart_syscall;
    err |= __get_user(&mut (*regs).cp0_epc, &(*sc).sc_pc);
    err |= __get_user(&mut (*regs).hi, &(*sc).sc_mdhi);
    err |= __get_user(&mut (*regs).lo, &(*sc).sc_mdlo);
    if cpu_has_dsp {
        err |= __get_user(&mut treg, &(*sc).sc_hi1); mthi1(treg);
        err |= __get_user(&mut treg, &(*sc).sc_lo1); mtlo1(treg);
        err |= __get_user(&mut treg, &(*sc).sc_hi2); mthi2(treg);
        err |= __get_user(&mut treg, &(*sc).sc_lo2); mtlo2(treg);
        err |= __get_user(&mut treg, &(*sc).sc_hi3); mthi3(treg);
        err |= __get_user(&mut treg, &(*sc).sc_lo3); mtlo3(treg);
        err |= __get_user(&mut treg, &(*sc).sc_dsp); wrdsp(treg, DSP_MASK);
    }
    i = 1;
    while i < 32 {
        err |= __get_user(&mut (*regs).regs[i], &(*sc).sc_regs[i]);
        i += 1;
    }
    if err != 0 { err } else { protected_restore_fp_context(sc) }
}

unsafe fn setup_frame_32(sig_return: *mut core::ffi::c_void, ksig: *mut ksignal,
                         regs: *mut pt_regs, set: *mut sigset_t) -> i32 {
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<sigframe32>()) as *mut sigframe32;
    if !access_ok(frame, core::mem::size_of::<sigframe32>()) { return -EFAULT; }
    let mut err = setup_sigcontext32(regs, &mut (*frame).sf_sc);
    err |= __copy_conv_sigset_to_user(&mut (*frame).sf_mask, set);
    if err != 0 { return -EFAULT; }
    (*regs).regs[4] = (*ksig).sig;
    (*regs).regs[5] = 0;
    (*regs).regs[6] = &mut (*frame).sf_sc as *mut _ as unsigned_long;
    (*regs).regs[29] = frame as unsigned_long;
    (*regs).regs[31] = sig_return as unsigned_long;
    (*regs).cp0_epc = (*regs).regs[25] = (*ksig).ka.sa.sa_handler as unsigned_long;
    DEBUGP!("SIG deliver (%s:%d): sp=0x%p pc=0x%lx ra=0x%lx\\n", (*current).comm, (*current).pid, frame, (*regs).cp0_epc, (*regs).regs[31]);
    0
}

pub unsafe extern "C" fn sys32_rt_sigreturn() {
    let regs = current_pt_regs();
    let frame = (*regs).regs[29] as *mut rt_sigframe32;
    let mut set: sigset_t = core::mem::zeroed();
    if !access_ok(frame, core::mem::size_of::<rt_sigframe32>()) { force_sig(SIGSEGV); return; }
    if __copy_conv_sigset_from_user(&mut set, &(*frame).rs_uc.uc_sigmask) != 0 { force_sig(SIGSEGV); return; }
    set_current_blocked(&set);
    let sig = restore_sigcontext32(regs, &mut (*frame).rs_uc.uc_mcontext);
    if sig < 0 { force_sig(SIGSEGV); return; } else if sig != 0 { force_sig(sig); }
    if compat_restore_altstack(&(*frame).rs_uc.uc_stack) != 0 { force_sig(SIGSEGV); return; }
    asm!("move $29, {0}", "j syscall_exit", in(reg) regs);
}

unsafe fn setup_rt_frame_32(sig_return: *mut core::ffi::c_void, ksig: *mut ksignal,
                            regs: *mut pt_regs, set: *mut sigset_t) -> i32 {
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe32>()) as *mut rt_sigframe32;
    if !access_ok(frame, core::mem::size_of::<rt_sigframe32>()) { return -EFAULT; }
    let mut err = copy_siginfo_to_user32(&mut (*frame).rs_info, &(*ksig).info);
    err |= __put_user(0, &mut (*frame).rs_uc.uc_flags);
    err |= __put_user(0, &mut (*frame).rs_uc.uc_link);
    err |= __compat_save_altstack(&mut (*frame).rs_uc.uc_stack, (*regs).regs[29]);
    err |= setup_sigcontext32(regs, &mut (*frame).rs_uc.uc_mcontext);
    err |= __copy_conv_sigset_to_user(&mut (*frame).rs_uc.uc_sigmask, set);
    if err != 0 { return -EFAULT; }
    (*regs).regs[4] = (*ksig).sig;
    (*regs).regs[5] = &mut (*frame).rs_info as *mut _ as unsigned_long;
    (*regs).regs[6] = &mut (*frame).rs_uc as *mut _ as unsigned_long;
    (*regs).regs[29] = frame as unsigned_long;
    (*regs).regs[31] = sig_return as unsigned_long;
    (*regs).cp0_epc = (*regs).regs[25] = (*ksig).ka.sa.sa_handler as unsigned_long;
    DEBUGP!("SIG deliver (%s:%d): sp=0x%p pc=0x%lx ra=0x%lx\\n", (*current).comm, (*current).pid, frame, (*regs).cp0_epc, (*regs).regs[31]);
    0
}

// o32 compatibility on 64-bit kernels, without DSP ASE.
pub static mut mips_abi_32: mips_abi = mips_abi {
    setup_frame: Some(setup_frame_32),
    setup_rt_frame: Some(setup_rt_frame_32),
    restart: __NR_O32_RESTART_SYSCALL,
    off_sc_fpregs: core::mem::offset_of!(sigcontext32, sc_fpregs),
    off_sc_fpc_csr: core::mem::offset_of!(sigcontext32, sc_fpc_csr),
    off_sc_used_math: core::mem::offset_of!(sigcontext32, sc_used_math),
    vdso: &vdso_image_o32,
};

pub unsafe extern "C" fn sys32_sigreturn() {
    let regs = current_pt_regs();
    let frame = (*regs).regs[29] as *mut sigframe32;
    let mut blocked: sigset_t = core::mem::zeroed();
    if !access_ok(frame, core::mem::size_of::<sigframe32>()) { force_sig(SIGSEGV); return; }
    if __copy_conv_sigset_from_user(&mut blocked, &(*frame).sf_mask) != 0 { force_sig(SIGSEGV); return; }
    set_current_blocked(&blocked);
    let sig = restore_sigcontext32(regs, &mut (*frame).sf_sc);
    if sig < 0 { force_sig(SIGSEGV); return; } else if sig != 0 { force_sig(sig); }
    asm!("move $29, {0}", "j syscall_exit", in(reg) regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
