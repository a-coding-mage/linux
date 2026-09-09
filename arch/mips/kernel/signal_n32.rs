// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2003 Broadcom Corporation
 */

// Kernel and architecture dependencies supplied by other translation units.

pub const __NR_N32_RESTART_SYSCALL: i32 = 6214;

#[repr(C)]
pub struct ucontextn32 {
    pub uc_flags: u32,
    pub uc_link: i32,
    pub uc_stack: compat_stack_t,
    pub uc_mcontext: sigcontext,
    pub uc_sigmask: compat_sigset_t, // mask last for extensibility
}

#[repr(C)]
pub struct rt_sigframe_n32 {
    pub rs_ass: [u32; 4], // argument save space for o32
    pub rs_pad: [u32; 2], // Was: signal trampoline
    pub rs_info: compat_siginfo,
    pub rs_uc: ucontextn32,
}

extern "C" {
    static mut vdso_image_n32: vdso_image;
    static mut current: *mut task_struct;

    fn current_pt_regs() -> *mut pt_regs;
    fn access_ok(addr: *const core::ffi::c_void, size: usize) -> bool;
    fn __copy_conv_sigset_from_user(dst: *mut sigset_t, src: *const compat_sigset_t) -> i32;
    fn set_current_blocked(set: *const sigset_t);
    fn restore_sigcontext(regs: *mut pt_regs, context: *const sigcontext) -> i32;
    fn force_sig(sig: i32);
    fn compat_restore_altstack(stack: *const compat_stack_t) -> i32;
    fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, size: usize) -> *mut rt_sigframe_n32;
    fn copy_siginfo_to_user32(dst: *mut compat_siginfo, src: *const siginfo) -> i32;
    fn __put_user<T>(value: T, dst: *mut T) -> i32;
    fn __compat_save_altstack(stack: *mut compat_stack_t, sp: u64) -> i32;
    fn setup_sigcontext(regs: *mut pt_regs, context: *mut sigcontext) -> i32;
    fn __copy_conv_sigset_to_user(dst: *mut compat_sigset_t, src: *const sigset_t) -> i32;
    fn debugp(format: *const u8, ...);
}

#[repr(C)]
pub struct mips_abi {
    pub setup_rt_frame: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut ksignal, *mut pt_regs, *mut sigset_t) -> i32>,
    pub restart: i32,
    pub off_sc_fpregs: usize,
    pub off_sc_fpc_csr: usize,
    pub off_sc_used_math: usize,
    pub vdso: *mut vdso_image,
}

pub unsafe extern "C" fn sysn32_rt_sigreturn() {
    let regs = current_pt_regs();
    let frame = (*regs).regs[29] as *mut rt_sigframe_n32;
    let mut set: sigset_t = core::mem::zeroed();

    if !access_ok(frame as *const core::ffi::c_void, core::mem::size_of::<rt_sigframe_n32>())
        || __copy_conv_sigset_from_user(&mut set, &(*frame).rs_uc.uc_sigmask) != 0
    {
        force_sig(SIGSEGV);
        return;
    }

    set_current_blocked(&set);

    let sig = restore_sigcontext(regs, &(*frame).rs_uc.uc_mcontext);
    if sig < 0 {
        force_sig(SIGSEGV);
        return;
    } else if sig != 0 {
        force_sig(sig);
    }

    if compat_restore_altstack(&(*frame).rs_uc.uc_stack) != 0 {
        force_sig(SIGSEGV);
        return;
    }

    // Don't let your children do this ...
    core::arch::asm!(
        "move $29, {0}\n\t",
        "j syscall_exit",
        in(reg) regs,
        options(noreturn)
    );
}

unsafe extern "C" fn setup_rt_frame_n32(
    sig_return: *mut core::ffi::c_void,
    ksig: *mut ksignal,
    regs: *mut pt_regs,
    set: *mut sigset_t,
) -> i32 {
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe_n32>());
    if !access_ok(frame as *const core::ffi::c_void, core::mem::size_of::<rt_sigframe_n32>()) {
        return -EFAULT;
    }

    // Create siginfo.
    let mut err = copy_siginfo_to_user32(&mut (*frame).rs_info, &(*ksig).info);

    // Create the ucontext.
    err |= __put_user(0, &mut (*frame).rs_uc.uc_flags);
    err |= __put_user(0, &mut (*frame).rs_uc.uc_link);
    err |= __compat_save_altstack(&mut (*frame).rs_uc.uc_stack, (*regs).regs[29]);
    err |= setup_sigcontext(regs, &mut (*frame).rs_uc.uc_mcontext);
    err |= __copy_conv_sigset_to_user(&mut (*frame).rs_uc.uc_sigmask, set);

    if err != 0 {
        return -EFAULT;
    }

    // Arguments to signal handler:
    //
    //   a0 = signal number
    //   a1 = 0 (should be cause)
    //   a2 = pointer to ucontext
    //
    // $25 and c0_epc point to the signal handler, $29 points to the struct
    // rt_sigframe.
    (*regs).regs[4] = (*ksig).sig as u64;
    (*regs).regs[5] = (&mut (*frame).rs_info as *mut compat_siginfo) as u64;
    (*regs).regs[6] = (&mut (*frame).rs_uc as *mut ucontextn32) as u64;
    (*regs).regs[29] = frame as u64;
    (*regs).regs[31] = sig_return as u64;
    (*regs).cp0_epc = (*regs).regs[25] = (*ksig).ka.sa.sa_handler as u64;

    // DEBUGP("SIG deliver (%s:%d): sp=0x%p pc=0x%lx ra=0x%lx\n",
    //         current->comm, current->pid, frame, regs->cp0_epc, regs->regs[31]);

    0
}

pub static mut mips_abi_n32: mips_abi = mips_abi {
    setup_rt_frame: Some(setup_rt_frame_n32),
    restart: __NR_N32_RESTART_SYSCALL,
    off_sc_fpregs: core::mem::offset_of!(sigcontext, sc_fpregs),
    off_sc_fpc_csr: core::mem::offset_of!(sigcontext, sc_fpc_csr),
    off_sc_used_math: core::mem::offset_of!(sigcontext, sc_used_math),
    vdso: unsafe { &mut vdso_image_n32 },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
