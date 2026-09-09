// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel dependencies supplied by the surrounding RISC-V implementation.

const COMPAT_DEBUG_SIG: u32 = 0;

#[repr(C)]
pub struct compat_sigcontext {
    pub sc_regs: compat_user_regs_struct,
    pub sc_fpregs: __riscv_fp_state,
}

#[repr(C)]
pub struct compat_ucontext {
    pub uc_flags: compat_ulong_t,
    pub uc_link: *mut compat_ucontext,
    pub uc_stack: compat_stack_t,
    pub uc_sigmask: sigset_t,
    // Padding permits sigset_t to be expanded in the future.
    pub __unused: [__u8; 1024 / 8 - core::mem::size_of::<sigset_t>()],
    // Keep sigcontext last so future ISA state can be added indefinitely.
    pub uc_mcontext: compat_sigcontext,
}

#[repr(C)]
pub struct compat_rt_sigframe {
    pub info: compat_siginfo,
    pub uc: compat_ucontext,
}

#[cfg(feature = "CONFIG_FPU")]
unsafe fn compat_restore_fp_state(
    regs: *mut pt_regs,
    sc_fpregs: *mut __riscv_fp_state,
) -> core::ffi::c_long {
    let mut err: core::ffi::c_long;
    let state = &mut (*sc_fpregs).d as *mut __riscv_d_ext_state;
    let mut i: usize = 0;

    err = __copy_from_user(&mut (*current()).thread.fstate as *mut _, state, core::mem::size_of::<__riscv_d_ext_state>());
    if err != 0 {
        return err;
    }

    fstate_restore(current(), regs);

    // We support no other extension state at this time.
    while i < (*sc_fpregs).q.reserved.len() {
        let mut value: u32 = 0;
        err = __get_user(&mut value, &(*sc_fpregs).q.reserved[i]);
        if err != 0 {
            break;
        }
        if value != 0 {
            return -EINVAL;
        }
        i += 1;
    }
    err
}

#[cfg(feature = "CONFIG_FPU")]
unsafe fn compat_save_fp_state(
    regs: *mut pt_regs,
    sc_fpregs: *mut __riscv_fp_state,
) -> core::ffi::c_long {
    let mut err: core::ffi::c_long;
    let state = &mut (*sc_fpregs).d as *mut __riscv_d_ext_state;
    let mut i: usize = 0;

    fstate_save(current(), regs);
    err = __copy_to_user(state, &(*current()).thread.fstate as *const _, core::mem::size_of::<__riscv_d_ext_state>());
    if err != 0 {
        return err;
    }

    // We support no other extension state at this time.
    while i < (*sc_fpregs).q.reserved.len() {
        err = __put_user(0, &mut (*sc_fpregs).q.reserved[i]);
        if err != 0 {
            break;
        }
        i += 1;
    }
    err
}

#[cfg(not(feature = "CONFIG_FPU"))]
unsafe fn compat_restore_fp_state(_regs: *mut pt_regs, _sc_fpregs: *mut __riscv_fp_state) -> core::ffi::c_long { 0 }

#[cfg(not(feature = "CONFIG_FPU"))]
unsafe fn compat_save_fp_state(_regs: *mut pt_regs, _sc_fpregs: *mut __riscv_fp_state) -> core::ffi::c_long { 0 }

unsafe fn compat_restore_sigcontext(
    regs: *mut pt_regs,
    sc: *mut compat_sigcontext,
) -> core::ffi::c_long {
    let mut cregs: compat_user_regs_struct = core::mem::zeroed();
    let mut err = __copy_from_user(&mut cregs, &(*sc).sc_regs, core::mem::size_of::<compat_user_regs_struct>());
    if err != 0 {
        return err;
    }
    cregs_to_regs(&cregs, regs);
    if has_fpu() {
        err |= compat_restore_fp_state(regs, &mut (*sc).sc_fpregs);
    }
    err
}

pub unsafe fn rt_sigreturn() -> core::ffi::c_long {
    let regs = current_pt_regs();
    let frame = regs.sp as *mut compat_rt_sigframe;
    let mut set: sigset_t = core::mem::zeroed();

    (*current()).restart_block.fn_ = do_no_restart_syscall;
    if !access_ok(frame, core::mem::size_of::<compat_rt_sigframe>())
        || __copy_from_user(&mut set, &(*frame).uc.uc_sigmask, core::mem::size_of::<sigset_t>()) != 0
        || { set_current_blocked(&set); compat_restore_sigcontext(regs, &mut (*frame).uc.uc_mcontext) != 0 }
        || compat_restore_altstack(&(*frame).uc.uc_stack) != 0
    {
        let task = current();
        if show_unhandled_signals {
            pr_info_ratelimited("%s[%d]: bad frame in %s: frame=%p pc=%p sp=%p\n", (*task).comm, task_pid_nr(task), "rt_sigreturn", frame, regs.epc as *mut _, regs.sp as *mut _);
        }
        force_sig(SIGSEGV);
        return 0;
    }
    regs.a0 as core::ffi::c_long
}

unsafe fn compat_setup_sigcontext(frame: *mut compat_rt_sigframe, regs: *mut pt_regs) -> core::ffi::c_long {
    let sc = &mut (*frame).uc.uc_mcontext;
    let mut cregs: compat_user_regs_struct = core::mem::zeroed();
    regs_to_cregs(&mut cregs, regs);
    let mut err = __copy_to_user(&mut sc.sc_regs, &cregs, core::mem::size_of::<compat_user_regs_struct>());
    if has_fpu() { err |= compat_save_fp_state(regs, &mut sc.sc_fpregs); }
    err
}

unsafe fn compat_get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, framesize: usize) -> *mut core::ffi::c_void {
    let mut sp = (*regs).sp;
    if on_sig_stack(sp) && !on_sig_stack(sp.wrapping_sub(framesize)) { return (-1isize) as *mut _; }
    sp = sigsp(sp, ksig).wrapping_sub(framesize) & !0xf;
    sp as *mut _
}

pub unsafe fn compat_setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> i32 {
    let frame = compat_get_sigframe(ksig, regs, core::mem::size_of::<compat_rt_sigframe>()) as *mut compat_rt_sigframe;
    if !access_ok(frame, core::mem::size_of::<compat_rt_sigframe>()) { return -EFAULT; }
    let mut err: core::ffi::c_long = copy_siginfo_to_user32(&mut (*frame).info, &(*ksig).info);
    err |= __put_user(0, &mut (*frame).uc.uc_flags);
    err |= __put_user(core::ptr::null_mut(), &mut (*frame).uc.uc_link);
    err |= __compat_save_altstack(&mut (*frame).uc.uc_stack, (*regs).sp);
    err |= compat_setup_sigcontext(frame, regs);
    err |= __copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>());
    if err != 0 { return -EFAULT; }
    (*regs).ra = COMPAT_VDSO_SYMBOL((*current()).mm.context.vdso, rt_sigreturn) as usize;
    (*regs).epc = (*ksig).ka.sa.sa_handler as usize;
    (*regs).sp = frame as usize;
    (*regs).a0 = (*ksig).sig;
    (*regs).a1 = &mut (*frame).info as *mut _ as usize;
    (*regs).a2 = &mut (*frame).uc as *mut _ as usize;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
