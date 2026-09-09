// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/kernel/signal.c
 *
 * Copyright (C) 1995-2009 Russell King
 * Copyright (C) 2012 ARM Ltd.
 * Modified by Will Deacon <will.deacon@arm.com>
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[repr(C, align(8))]
pub struct CompatVfpSigframe {
    pub magic: compat_ulong_t,
    pub size: compat_ulong_t,
    pub ufp: CompatUserVfp,
    pub ufp_exc: CompatUserVfpExc,
}
#[repr(C)]
pub struct CompatUserVfp { pub fpregs: [compat_u64; 32], pub fpscr: compat_ulong_t }
#[repr(C)]
pub struct CompatUserVfpExc { pub fpexc: compat_ulong_t, pub fpinst: compat_ulong_t, pub fpinst2: compat_ulong_t }

pub const VFP_MAGIC: compat_ulong_t = 0x56465001;
pub const VFP_STORAGE_SIZE: usize = core::mem::size_of::<CompatVfpSigframe>();
pub const FSR_WRITE_SHIFT: u32 = 11;

#[repr(C, align(8))]
pub struct CompatAuxSigframe {
    pub vfp: CompatVfpSigframe,
    /* Something that isn't a valid magic number for any coprocessor. */
    pub end_magic: u64,
}

#[inline]
unsafe fn put_sigset_t(uset: *mut compat_sigset_t, set: *const sigset_t) -> i32 {
    let mut cset: compat_sigset_t = core::mem::zeroed();
    (*cset.sig.as_mut_ptr()) = (*set).sig[0] & 0xffffffffu64;
    (*cset.sig.as_mut_ptr().add(1)) = (*set).sig[0] >> 32;
    copy_to_user(uset as *mut _, &cset as *const _ as *const _, core::mem::size_of::<compat_sigset_t>())
}

#[inline]
unsafe fn get_sigset_t(set: *mut sigset_t, uset: *const compat_sigset_t) -> i32 {
    let mut s32: compat_sigset_t = core::mem::zeroed();
    if copy_from_user(&mut s32 as *mut _ as *mut _, uset as *const _, core::mem::size_of::<compat_sigset_t>()) != 0 { return -EFAULT; }
    (*set).sig[0] = s32.sig[0] | ((s32.sig[1] as i64 as u64) << 32);
    0
}

#[repr(C)]
pub union FpsimdVreg { pub raw: u128, pub parts: FpsimdVregParts }
#[repr(C)]
pub struct FpsimdVregParts { pub lo: u64, pub hi: u64 }

unsafe fn compat_preserve_vfp_context(frame: *mut CompatVfpSigframe) -> i32 {
    let fpsimd = &(*current).thread.uw.fpsimd_state as *const _;
    let magic = VFP_MAGIC; let size = VFP_STORAGE_SIZE as compat_ulong_t;
    let mut err = 0i32;
    fpsimd_save_and_flush_current_state();
    if __put_user_error(magic, &mut (*frame).magic, &mut err) != 0 {};
    if __put_user_error(size, &mut (*frame).size, &mut err) != 0 {};
    let mut i = 0usize;
    while i < 32 {
        let v = FpsimdVreg { raw: (*fpsimd).vregs[i >> 1] };
        __put_user_error(v.parts.lo, &mut (*frame).ufp.fpregs[i], &mut err);
        __put_user_error(v.parts.hi, &mut (*frame).ufp.fpregs[i + 1], &mut err);
        i += 2;
    }
    let fpscr = ((*fpsimd).fpsr & VFP_FPSCR_STAT_MASK) | ((*fpsimd).fpcr & VFP_FPSCR_CTRL_MASK);
    __put_user_error(fpscr, &mut (*frame).ufp.fpscr, &mut err);
    __put_user_error(1u64 << 30, &mut (*frame).ufp_exc.fpexc, &mut err);
    __put_user_error(0, &mut (*frame).ufp_exc.fpinst, &mut err);
    __put_user_error(0, &mut (*frame).ufp_exc.fpinst2, &mut err);
    if err != 0 { -EFAULT } else { 0 }
}

unsafe fn compat_restore_vfp_context(frame: *mut CompatVfpSigframe) -> i32 {
    let mut fpsimd: user_fpsimd_state = core::mem::zeroed();
    let mut magic = VFP_MAGIC; let mut size = VFP_STORAGE_SIZE as compat_ulong_t; let mut err = 0i32;
    __get_user_error(&mut magic, &(*frame).magic, &mut err); __get_user_error(&mut size, &(*frame).size, &mut err);
    if err != 0 { return -EFAULT; } if magic != VFP_MAGIC || size as usize != VFP_STORAGE_SIZE { return -EINVAL; }
    let mut i = 0usize;
    while i < 32 { let mut v = FpsimdVreg { raw: 0 }; __get_user_error(&mut v.parts.lo, &(*frame).ufp.fpregs[i], &mut err); __get_user_error(&mut v.parts.hi, &(*frame).ufp.fpregs[i+1], &mut err); fpsimd.vregs[i>>1] = v.raw; i += 2; }
    let mut fpscr = 0; __get_user_error(&mut fpscr, &(*frame).ufp.fpscr, &mut err);
    fpsimd.fpsr = fpscr & VFP_FPSCR_STAT_MASK; fpsimd.fpcr = fpscr & VFP_FPSCR_CTRL_MASK;
    if err != 0 { return -EFAULT; }
    fpsimd_save_and_flush_current_state(); (*current).thread.uw.fpsimd_state = fpsimd; 0
}

unsafe fn compat_restore_sigframe(regs: *mut pt_regs, sf: *mut compat_sigframe) -> i32 {
    let mut err = 0i32; let mut set: sigset_t = core::mem::zeroed();
    if get_sigset_t(&mut set, &(*sf).uc.uc_sigmask) == 0 { set_current_blocked(&set); }
    __get_user_error(&mut (*regs).regs[0], &(*sf).uc.uc_mcontext.arm_r0, &mut err);
    __get_user_error(&mut (*regs).regs[1], &(*sf).uc.uc_mcontext.arm_r1, &mut err);
    __get_user_error(&mut (*regs).regs[2], &(*sf).uc.uc_mcontext.arm_r2, &mut err);
    __get_user_error(&mut (*regs).regs[3], &(*sf).uc.uc_mcontext.arm_r3, &mut err);
    __get_user_error(&mut (*regs).regs[4], &(*sf).uc.uc_mcontext.arm_r4, &mut err);
    __get_user_error(&mut (*regs).regs[5], &(*sf).uc.uc_mcontext.arm_r5, &mut err);
    __get_user_error(&mut (*regs).regs[6], &(*sf).uc.uc_mcontext.arm_r6, &mut err);
    __get_user_error(&mut (*regs).regs[7], &(*sf).uc.uc_mcontext.arm_r7, &mut err);
    __get_user_error(&mut (*regs).regs[8], &(*sf).uc.uc_mcontext.arm_r8, &mut err);
    __get_user_error(&mut (*regs).regs[9], &(*sf).uc.uc_mcontext.arm_r9, &mut err);
    __get_user_error(&mut (*regs).regs[10], &(*sf).uc.uc_mcontext.arm_r10, &mut err);
    __get_user_error(&mut (*regs).regs[11], &(*sf).uc.uc_mcontext.arm_fp, &mut err);
    __get_user_error(&mut (*regs).regs[12], &(*sf).uc.uc_mcontext.arm_ip, &mut err);
    __get_user_error(&mut (*regs).compat_sp, &(*sf).uc.uc_mcontext.arm_sp, &mut err);
    __get_user_error(&mut (*regs).compat_lr, &(*sf).uc.uc_mcontext.arm_lr, &mut err);
    __get_user_error(&mut (*regs).pc, &(*sf).uc.uc_mcontext.arm_pc, &mut err);
    let mut psr = 0; __get_user_error(&mut psr, &(*sf).uc.uc_mcontext.arm_cpsr, &mut err);
    (*regs).pstate = compat_psr_to_pstate(psr); forget_syscall(regs);
    err |= (!valid_user_regs(&(*regs).user_regs, current)) as i32;
    let aux = (*sf).uc.uc_regspace.as_mut_ptr() as *mut CompatAuxSigframe;
    if err == 0 && system_supports_fpsimd() { err |= compat_restore_vfp_context(&mut (*aux).vfp); }
    err
}

// Signal-return syscall entry points and frame setup routines retain their kernel ABI.
pub unsafe fn compat_setup_restart_syscall(regs: *mut pt_regs) { (*regs).regs[7] = __NR_compat32_restart_syscall; }

pub unsafe fn compat_setup_rt_frame(usig: i32, ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> i32 {
    let frame = compat_get_sigframe(ksig, regs, core::mem::size_of::<compat_rt_sigframe>()) as *mut compat_rt_sigframe;
    if frame.is_null() { return 1; }
    let mut err = copy_siginfo_to_user32(&mut (*frame).info, &(*ksig).info);
    __put_user_error(0, &mut (*frame).sig.uc.uc_flags, &mut err);
    __put_user_error(0, &mut (*frame).sig.uc.uc_link, &mut err);
    err |= __compat_save_altstack(&mut (*frame).sig.uc.uc_stack, (*regs).compat_sp);
    err |= compat_setup_sigframe(&mut (*frame).sig, regs, set);
    if err == 0 { compat_setup_return(regs, &mut (*ksig).ka, (*frame).sig.retcode, frame as *mut _, usig); (*regs).regs[1] = &mut (*frame).info as *mut _ as usize as u64; (*regs).regs[2] = &mut (*frame).sig.uc as *mut _ as usize as u64; }
    err
}

pub unsafe fn compat_setup_frame(usig: i32, ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> i32 {
    let frame = compat_get_sigframe(ksig, regs, core::mem::size_of::<compat_sigframe>()) as *mut compat_sigframe;
    if frame.is_null() { return 1; }
    let mut err = 0; __put_user_error(0x5ac3c35a, &mut (*frame).uc.uc_flags, &mut err);
    err |= compat_setup_sigframe(frame, regs, set);
    if err == 0 { compat_setup_return(regs, &mut (*ksig).ka, (*frame).retcode, frame as *mut _, usig); } err
}

unsafe fn compat_get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, framesize: usize) -> *mut core::ffi::c_void {
    let sp = sigsp((*regs).compat_sp, ksig); let frame = compat_ptr(((sp - framesize as u64) & !7) as compat_uptr_t);
    if !access_ok(frame, framesize) { core::ptr::null_mut() } else { frame }
}

unsafe fn compat_setup_return(regs: *mut pt_regs, ka: *mut k_sigaction, rc: *mut compat_ulong_t, frame: *mut core::ffi::c_void, usig: i32) {
    let handler = ptr_to_compat((*ka).sa.sa_handler); let thumb = handler & 1; let mut spsr = (*regs).pstate & !(PSR_f | PSR_AA32_E_BIT);
    if thumb != 0 { spsr |= PSR_AA32_T_BIT; } else { spsr &= !PSR_AA32_T_BIT; } spsr &= !PSR_AA32_IT_MASK; spsr |= PSR_AA32_ENDSTATE;
    let retcode = if (*ka).sa.sa_flags & SA_RESTORER != 0 { ptr_to_compat((*ka).sa.sa_restorer) } else { let mut idx = thumb << 1; if (*ka).sa.sa_flags & SA_SIGINFO != 0 { idx += 3; } (*current).mm.context.sigpage as u64 + ((idx as u64) << 2) + thumb as u64 };
    (*regs).regs[0] = usig as u64; (*regs).compat_sp = ptr_to_compat(frame); (*regs).compat_lr = retcode; (*regs).pc = handler; (*regs).pstate = spsr;
}

unsafe fn compat_setup_sigframe(sf: *mut compat_sigframe, regs: *mut pt_regs, set: *mut sigset_t) -> i32 {
    let mut err = 0; let psr = pstate_to_compat_psr((*regs).pstate);
    __put_user_error((*regs).regs[0], &mut (*sf).uc.uc_mcontext.arm_r0, &mut err); __put_user_error((*regs).regs[1], &mut (*sf).uc.uc_mcontext.arm_r1, &mut err); __put_user_error((*regs).regs[2], &mut (*sf).uc.uc_mcontext.arm_r2, &mut err); __put_user_error((*regs).regs[3], &mut (*sf).uc.uc_mcontext.arm_r3, &mut err); __put_user_error((*regs).regs[4], &mut (*sf).uc.uc_mcontext.arm_r4, &mut err); __put_user_error((*regs).regs[5], &mut (*sf).uc.uc_mcontext.arm_r5, &mut err); __put_user_error((*regs).regs[6], &mut (*sf).uc.uc_mcontext.arm_r6, &mut err); __put_user_error((*regs).regs[7], &mut (*sf).uc.uc_mcontext.arm_r7, &mut err); __put_user_error((*regs).regs[8], &mut (*sf).uc.uc_mcontext.arm_r8, &mut err); __put_user_error((*regs).regs[9], &mut (*sf).uc.uc_mcontext.arm_r9, &mut err); __put_user_error((*regs).regs[10], &mut (*sf).uc.uc_mcontext.arm_r10, &mut err); __put_user_error((*regs).regs[11], &mut (*sf).uc.uc_mcontext.arm_fp, &mut err); __put_user_error((*regs).regs[12], &mut (*sf).uc.uc_mcontext.arm_ip, &mut err); __put_user_error((*regs).compat_sp, &mut (*sf).uc.uc_mcontext.arm_sp, &mut err); __put_user_error((*regs).compat_lr, &mut (*sf).uc.uc_mcontext.arm_lr, &mut err); __put_user_error((*regs).pc, &mut (*sf).uc.uc_mcontext.arm_pc, &mut err); __put_user_error(psr, &mut (*sf).uc.uc_mcontext.arm_cpsr, &mut err);
    __put_user_error(0, &mut (*sf).uc.uc_mcontext.trap_no, &mut err); __put_user_error((( (*current).thread.fault_code & ESR_ELx_WNR != 0) as u64) << FSR_WRITE_SHIFT, &mut (*sf).uc.uc_mcontext.error_code, &mut err); __put_user_error((*current).thread.fault_address, &mut (*sf).uc.uc_mcontext.fault_address, &mut err); __put_user_error((*set).sig[0], &mut (*sf).uc.uc_mcontext.oldmask, &mut err);
    err |= put_sigset_t(&mut (*sf).uc.uc_sigmask, set); let aux = (*sf).uc.uc_regspace.as_mut_ptr() as *mut CompatAuxSigframe; if err == 0 && system_supports_fpsimd() { err |= compat_preserve_vfp_context(&mut (*aux).vfp); } __put_user_error(0, &mut (*aux).end_magic, &mut err); err
}

/* Compile-time assertions for siginfo_t offsets are retained as a dependency note. */

pub unsafe fn compat_sigreturn() -> u64 {
    let regs = current_pt_regs(); (*current).restart_block.fn = do_no_restart_syscall;
    if (*regs).compat_sp & 7 != 0 { arm64_notify_segfault((*regs).compat_sp); return 0; }
    let frame = (*regs).compat_sp as *mut compat_sigframe;
    if !access_ok(frame, core::mem::size_of::<compat_sigframe>()) || compat_restore_sigframe(regs, frame) != 0 { arm64_notify_segfault((*regs).compat_sp); return 0; }
    (*regs).regs[0]
}

pub unsafe fn compat_rt_sigreturn() -> u64 {
    let regs = current_pt_regs(); (*current).restart_block.fn = do_no_restart_syscall;
    if (*regs).compat_sp & 7 != 0 { arm64_notify_segfault((*regs).compat_sp); return 0; }
    let frame = (*regs).compat_sp as *mut compat_rt_sigframe;
    if !access_ok(frame, core::mem::size_of::<compat_rt_sigframe>()) || compat_restore_sigframe(regs, &mut (*frame).sig) != 0 || compat_restore_altstack(&(*frame).sig.uc.uc_stack) != 0 { arm64_notify_segfault((*regs).compat_sp); return 0; }
    (*regs).regs[0]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
