// SPDX-License-Identifier: GPL-2.0
/*
 * FPU register's regset abstraction, for ptrace, core dumps, etc.
 */

// Kernel dependencies are supplied by the surrounding translation.

/*
 * The xstateregs_active() routine is the same as the regset_fpregs_active() routine,
 * as the "regset->n" for the xstate regset will be updated based on the feature
 * capabilities supported by the xsave.
 */
pub unsafe fn regset_fpregs_active(target: *mut task_struct, regset: *const user_regset) -> c_int {
    (*regset).n
}

pub unsafe fn regset_xregset_fpregs_active(target: *mut task_struct, regset: *const user_regset) -> c_int {
    if boot_cpu_has(X86_FEATURE_FXSR) { (*regset).n } else { 0 }
}

/*
 * The regset get() functions are invoked from:
 *
 *   - coredump to dump the current task's fpstate. If the current task
 *     owns the FPU then the memory state has to be synchronized and the
 *     FPU register state preserved. Otherwise fpstate is already in sync.
 *
 *   - ptrace to dump fpstate of a stopped task, in which case the registers
 *     have already been saved to fpstate on context switch.
 */
unsafe fn sync_fpstate(fpu: *mut fpu) {
    if fpu == x86_task_fpu(current) { fpu_sync_fpstate(fpu); }
}

/* Invalidate cached FPU registers before modifying the stopped target task's fpstate. */
unsafe fn fpu_force_restore(fpu: *mut fpu) {
    /* Only stopped child tasks can be used to modify the FPU state in the fpstate buffer. */
    WARN_ON_FPU(fpu == x86_task_fpu(current));
    __fpu_invalidate_fpregs_state(fpu);
}

pub unsafe fn xfpregs_get(target: *mut task_struct, regset: *const user_regset, mut to: membuf) -> c_int {
    let fpu = x86_task_fpu(target);
    if !cpu_feature_enabled(X86_FEATURE_FXSR) { return -ENODEV; }
    sync_fpstate(fpu);
    if !use_xsave() {
        return membuf_write(&mut to, &(*(*fpu).fpstate).regs.fxsave as *const _ as *const c_void,
                            core::mem::size_of_val(&(*(*fpu).fpstate).regs.fxsave));
    }
    copy_xstate_to_uabi_buf(to, target, XSTATE_COPY_FX);
    0
}

pub unsafe fn xfpregs_set(target: *mut task_struct, regset: *const user_regset,
                          mut pos: c_uint, mut count: c_uint, kbuf: *const c_void,
                          ubuf: *const c_void) -> c_int {
    let fpu = x86_task_fpu(target);
    let mut newstate: fxregs_state = core::mem::zeroed();
    if !cpu_feature_enabled(X86_FEATURE_FXSR) { return -ENODEV; }
    if pos != 0 || count as usize != core::mem::size_of::<fxregs_state>() { return -EINVAL; }
    let ret = user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *mut _), &mut (ubuf as *mut _),
                                 &mut newstate as *mut _ as *mut c_void, 0, -1);
    if ret != 0 { return ret; }
    if newstate.mxcsr & !mxcsr_feature_mask != 0 { return -EINVAL; }
    fpu_force_restore(fpu);
    memcpy(&mut (*(*fpu).fpstate).regs.fxsave as *mut _ as *mut c_void,
           &newstate as *const _ as *const c_void, core::mem::size_of_val(&newstate));
    if in_ia32_syscall() { memset((*(*fpu).fpstate).regs.fxsave.xmm_space.as_mut_ptr().add(8 * 4), 0, 8 * 16); }
    if use_xsave() { (*(*fpu).fpstate).regs.xsave.header.xfeatures |= XFEATURE_MASK_FPSSE; }
    0
}

pub unsafe fn xstateregs_get(target: *mut task_struct, regset: *const user_regset, to: membuf) -> c_int {
    if !cpu_feature_enabled(X86_FEATURE_XSAVE) { return -ENODEV; }
    sync_fpstate(x86_task_fpu(target));
    copy_xstate_to_uabi_buf(to, target, XSTATE_COPY_XSAVE);
    0
}

pub unsafe fn xstateregs_set(target: *mut task_struct, regset: *const user_regset,
                             mut pos: c_uint, mut count: c_uint, kbuf: *const c_void,
                             ubuf: *const c_void) -> c_int {
    let fpu = x86_task_fpu(target);
    let mut tmpbuf: *mut xregs_state = core::ptr::null_mut();
    let ret: c_int;
    if !cpu_feature_enabled(X86_FEATURE_XSAVE) { return -ENODEV; }
    if pos != 0 || count != (*fpu_user_cfg).max_size { return -EFAULT; }
    if kbuf.is_null() {
        tmpbuf = vmalloc(count as usize) as *mut xregs_state;
        if tmpbuf.is_null() { return -ENOMEM; }
        if copy_from_user(tmpbuf as *mut c_void, ubuf, count as usize) != 0 { ret = -EFAULT; vfree(tmpbuf as *mut c_void); return ret; }
    }
    fpu_force_restore(fpu);
    ret = copy_uabi_from_kernel_to_xstate((*fpu).fpstate, if !kbuf.is_null() { kbuf } else { tmpbuf as *const c_void }, &mut (*target).thread.pkru);
    vfree(tmpbuf as *mut c_void);
    ret
}

// CONFIG_X86_USER_SHADOW_STACK conditionally supplies the following routines.
#[cfg(feature = "CONFIG_X86_USER_SHADOW_STACK")]
pub unsafe fn ssp_active(target: *mut task_struct, regset: *const user_regset) -> c_int {
    if (*target).thread.features & ARCH_SHSTK_SHSTK != 0 { (*regset).n } else { 0 }
}

#[cfg(feature = "CONFIG_X86_USER_SHADOW_STACK")]
pub unsafe fn ssp_get(target: *mut task_struct, regset: *const user_regset, mut to: membuf) -> c_int {
    let fpu = x86_task_fpu(target);
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) || ssp_active(target, regset) == 0 { return -ENODEV; }
    sync_fpstate(fpu);
    let cetregs = get_xsave_addr(&mut (*fpu).fpstate.regs.xsave, XFEATURE_CET_USER);
    if WARN_ON(cetregs.is_null()) { return -ENODEV; }
    membuf_write(&mut to, &(*cetregs).user_ssp as *const _ as *const c_void, core::mem::size_of::<c_ulong>())
}

#[cfg(feature = "CONFIG_X86_USER_SHADOW_STACK")]
pub unsafe fn ssp_set(target: *mut task_struct, regset: *const user_regset, mut pos: c_uint, mut count: c_uint,
                      kbuf: *const c_void, ubuf: *const c_void) -> c_int {
    let fpu = x86_task_fpu(target);
    let xsave = &mut (*fpu).fpstate.regs.xsave;
    let mut user_ssp: c_ulong = 0;
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) || ssp_active(target, regset) == 0 { return -ENODEV; }
    if pos != 0 || count as usize != core::mem::size_of::<c_ulong>() { return -EINVAL; }
    let r = user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *mut _), &mut (ubuf as *mut _), &mut user_ssp as *mut _ as *mut c_void, 0, -1);
    if r != 0 { return r; }
    if user_ssp >= TASK_SIZE_MAX || !IS_ALIGNED(user_ssp, 8) { return -EINVAL; }
    fpu_force_restore(fpu);
    let cetregs = get_xsave_addr(xsave, XFEATURE_CET_USER);
    if WARN_ON(cetregs.is_null()) { return -ENODEV; }
    (*cetregs).user_ssp = user_ssp;
    0
}

// CONFIG_X86_32 || CONFIG_IA32_EMULATION conditionally supplies the legacy conversions below.
#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
unsafe fn twd_i387_to_fxsr(twd: u16) -> u16 {
    let mut tmp = (!twd as u32);
    tmp = (tmp | (tmp >> 1)) & 0x5555;
    tmp = (tmp | (tmp >> 1)) & 0x3333;
    tmp = (tmp | (tmp >> 2)) & 0x0f0f;
    tmp = (tmp | (tmp >> 4)) & 0x00ff;
    tmp as u16
}

#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
unsafe fn twd_fxsr_to_i387(fxsave: *mut fxregs_state) -> u32 {
    let mut twd = (*fxsave).twd as c_ulong;
    let tos = ((*fxsave).swd >> 11) & 7;
    let mut ret = 0xffff0000u32;
    for i in 0..8 {
        let tag = if twd & 1 != 0 {
            let st = ((*fxsave).st_space.as_mut_ptr() as *mut _fpreg).add(((i as i32 - tos as i32) & 7) as usize);
            match (*st).exponent & 0x7fff { 0x7fff => 2, 0 => if (*st).significand.iter().all(|&x| x == 0) { 1 } else { 2 }, _ => if (*st).significand[3] & 0x8000 != 0 { 0 } else { 2 } }
        } else { 3 };
        ret |= tag << (2 * i);
        twd >>= 1;
    }
    ret
}

// The remaining legacy conversion and get/set routines retain the C ABI and field-level behavior.
#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
unsafe fn __convert_from_fxsr(env: *mut user_i387_ia32_struct, tsk: *mut task_struct, fx: *mut fxregs_state) {
    (*env).cwd = (*fx).cwd | 0xffff0000;
    (*env).swd = (*fx).swd | 0xffff0000;
    (*env).twd = twd_fxsr_to_i387(fx);
    #[cfg(target_arch = "x86_64")] {
        (*env).fip = (*fx).rip;
        (*env).foo = (*fx).rdp;
        (*env).fcs = (*task_pt_regs(tsk)).cs;
        (*env).fos = if tsk == current { savesegment_ds() } else { (*tsk).thread.ds };
        (*env).fos |= 0xffff0000;
    }
    for i in 0..8 { memcpy((*env).st_space.as_mut_ptr().add(i) as *mut c_void, (*fx).st_space.as_ptr().add(i) as *const c_void, core::mem::size_of::<_fpreg>()); }
}

#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
pub unsafe fn convert_from_fxsr(env: *mut user_i387_ia32_struct, tsk: *mut task_struct) {
    let fx = &mut (*x86_task_fpu(tsk)).fpstate.regs.fxsave;
    __convert_from_fxsr(env, tsk, fx);
}

#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
pub unsafe fn convert_to_fxsr(fxsave: *mut fxregs_state, env: *const user_i387_ia32_struct) {
    (*fxsave).cwd = (*env).cwd; (*fxsave).swd = (*env).swd; (*fxsave).twd = twd_i387_to_fxsr((*env).twd); (*fxsave).fop = ((*env).fcs >> 16) as u16;
    #[cfg(target_arch = "x86_64")] { (*fxsave).rip = (*env).fip; (*fxsave).rdp = (*env).foo; }
    for i in 0..8 { memcpy((*fxsave).st_space.as_mut_ptr().add(i) as *mut c_void, (*env).st_space.as_ptr().add(i) as *const c_void, core::mem::size_of::<_fpreg>()); }
}

#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
pub unsafe fn fpregs_get(target: *mut task_struct, regset: *const user_regset, mut to: membuf) -> c_int {
    let fpu = x86_task_fpu(target);
    let mut env: user_i387_ia32_struct = core::mem::zeroed();
    let mut fxsave: fxregs_state = core::mem::zeroed();
    sync_fpstate(fpu);
    if !cpu_feature_enabled(X86_FEATURE_FPU) { return fpregs_soft_get(target, regset, to); }
    if !cpu_feature_enabled(X86_FEATURE_FXSR) {
        return membuf_write(&mut to, &(*(*fpu).fpstate).regs.fsave as *const _ as *const c_void,
                            core::mem::size_of::<fregs_state>());
    }
    let fx: *mut fxregs_state = if use_xsave() {
        let mb = membuf { p: &mut fxsave as *mut _ as *mut c_void, left: core::mem::size_of::<fxregs_state>() };
        copy_xstate_to_uabi_buf(mb, target, XSTATE_COPY_FP);
        &mut fxsave
    } else { &mut (*(*fpu).fpstate).regs.fxsave };
    __convert_from_fxsr(&mut env, target, fx);
    membuf_write(&mut to, &env as *const _ as *const c_void, core::mem::size_of::<user_i387_ia32_struct>())
}

#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
pub unsafe fn fpregs_set(target: *mut task_struct, regset: *const user_regset,
                         mut pos: c_uint, mut count: c_uint, kbuf: *const c_void,
                         ubuf: *const c_void) -> c_int {
    let fpu = x86_task_fpu(target);
    let mut env: user_i387_ia32_struct = core::mem::zeroed();
    if pos != 0 || count as usize != core::mem::size_of::<user_i387_ia32_struct>() { return -EINVAL; }
    if !cpu_feature_enabled(X86_FEATURE_FPU) { return fpregs_soft_set(target, regset, pos, count, kbuf, ubuf); }
    let ret = user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *mut _), &mut (ubuf as *mut _), &mut env as *mut _ as *mut c_void, 0, -1);
    if ret != 0 { return ret; }
    fpu_force_restore(fpu);
    if cpu_feature_enabled(X86_FEATURE_FXSR) { convert_to_fxsr(&mut (*(*fpu).fpstate).regs.fxsave, &env); }
    else { memcpy(&mut (*(*fpu).fpstate).regs.fsave as *mut _ as *mut c_void, &env as *const _ as *const c_void, core::mem::size_of_val(&env)); }
    if cpu_feature_enabled(X86_FEATURE_XSAVE) { (*(*fpu).fpstate).regs.xsave.header.xfeatures |= XFEATURE_MASK_FP; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
