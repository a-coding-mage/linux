// SPDX-License-Identifier: GPL-2.0
/* FPU signal frame handling routines. */

/* Kernel and architecture dependencies are supplied by the surrounding translation unit. */

#[inline]
unsafe fn check_xstate_in_sigframe(
    fxbuf: *mut fxregs_state,
    fx_sw: *mut _fpx_sw_bytes,
) -> bool {
    let min_xstate_size = core::mem::size_of::<fxregs_state>()
        + core::mem::size_of::<xstate_header>();
    let fpstate = fxbuf as *mut u8;
    let mut magic2: u32 = 0;

    if __copy_from_user(fx_sw, &(*fxbuf).sw_reserved[0] as *const _ as *const _, core::mem::size_of::<_fpx_sw_bytes>()) != 0 {
        return false;
    }
    if (*fx_sw).magic1 != FP_XSTATE_MAGIC1
        || (*fx_sw).xstate_size < min_xstate_size
        || (*fx_sw).xstate_size > (*x86_task_fpu(current())).fpstate.user_size
        || (*fx_sw).xstate_size > (*fx_sw).extended_size
    {
        (*fx_sw).magic1 = 0;
        (*fx_sw).xstate_size = core::mem::size_of::<fxregs_state>();
        (*fx_sw).xfeatures = XFEATURE_MASK_FPSSE;
        trace_x86_fpu_xstate_check_failed(x86_task_fpu(current()));
        return true;
    }
    if __get_user(&mut magic2, fpstate.add((*fx_sw).xstate_size as usize) as *const u32) != 0 {
        return false;
    }
    if likely(magic2 == FP_XSTATE_MAGIC2) { return true; }
    trace_x86_fpu_xstate_check_failed(x86_task_fpu(current()));
    (*fx_sw).magic1 = 0;
    (*fx_sw).xstate_size = core::mem::size_of::<fxregs_state>();
    (*fx_sw).xfeatures = XFEATURE_MASK_FPSSE;
    true
}

#[inline]
unsafe fn save_fsave_header(tsk: *mut task_struct, buf: *mut core::ffi::c_void) -> bool {
    if use_fxsr() {
        let xsave = &mut (*(*x86_task_fpu(tsk)).fpstate).regs.xsave;
        let mut env: user_i387_ia32_struct = core::mem::zeroed();
        let fp = buf as *mut _fpstate_32;
        fpregs_lock();
        if !test_thread_flag(TIF_NEED_FPU_LOAD) { fxsave(&mut (*(*x86_task_fpu(tsk)).fpstate).regs.fxsave); }
        fpregs_unlock();
        convert_from_fxsr(&mut env, tsk);
        if __copy_to_user(buf, &env, core::mem::size_of::<user_i387_ia32_struct>()) != 0
            || __put_user(xsave.i387.swd, &mut (*fp).status) != 0
            || __put_user(X86_FXSR_MAGIC, &mut (*fp).magic) != 0 { return false; }
    } else {
        let fp = buf as *mut fregs_state;
        let mut swd = 0u32;
        if __get_user(&mut swd, &(*fp).swd) != 0 || __put_user(swd, &mut (*fp).status) != 0 { return false; }
    }
    true
}

#[inline]
unsafe fn save_sw_bytes(sw_bytes: *mut _fpx_sw_bytes, ia32_frame: bool, fpstate: *mut fpstate) {
    (*sw_bytes).magic1 = FP_XSTATE_MAGIC1;
    (*sw_bytes).extended_size = (*fpstate).user_size + FP_XSTATE_MAGIC2_SIZE;
    (*sw_bytes).xfeatures = (*fpstate).user_xfeatures;
    (*sw_bytes).xstate_size = (*fpstate).user_size;
    if ia32_frame { (*sw_bytes).extended_size += core::mem::size_of::<fregs_state>() as u32; }
}

#[inline]
unsafe fn save_xstate_epilog(buf: *mut core::ffi::c_void, ia32_frame: bool, fpstate: *mut fpstate) -> bool {
    let x = buf as *mut xregs_state;
    let mut sw_bytes: _fpx_sw_bytes = core::mem::zeroed();
    save_sw_bytes(&mut sw_bytes, ia32_frame, fpstate);
    let mut err = __copy_to_user(&mut (*x).i387.sw_reserved, &sw_bytes, core::mem::size_of::<_fpx_sw_bytes>());
    if !use_xsave() { return err == 0; }
    err |= __put_user(FP_XSTATE_MAGIC2, (buf as *mut u8).add((*fpstate).user_size as usize) as *mut u32);
    err |= set_xfeature_in_sigframe(x, XFEATURE_MASK_FPSSE);
    err == 0
}

#[inline]
unsafe fn copy_fpregs_to_sigframe(buf: *mut xregs_state, pkru: u32) -> i32 {
    if use_xsave() { xsave_to_user_sigframe(buf, pkru) }
    else if use_fxsr() { fxsave_to_user_sigframe(buf as *mut fxregs_state) }
    else { fnsave_to_user_sigframe(buf as *mut fregs_state) }
}

pub unsafe fn copy_fpstate_to_sigframe(buf: *mut core::ffi::c_void, buf_fx: *mut core::ffi::c_void, size: i32, pkru: u32) -> bool {
    let tsk = current();
    let fpstate = (*x86_task_fpu(tsk)).fpstate;
    let mut ia32_fxstate = buf != buf_fx;
    ia32_fxstate &= IS_ENABLED(CONFIG_X86_32) || IS_ENABLED(CONFIG_IA32_EMULATION);
    if !cpu_feature_enabled(X86_FEATURE_FPU) {
        let mut fp: user_i387_ia32_struct = core::mem::zeroed();
        fpregs_soft_get(tsk, core::ptr::null_mut(), membuf { p: &mut fp as *mut _ as *mut u8, left: core::mem::size_of_val(&fp) });
        return copy_to_user(buf, &fp, core::mem::size_of_val(&fp)) == 0;
    }
    if !access_ok(buf, size as usize) { return false; }
    if use_xsave() && __clear_user(&mut (*(buf_fx as *mut xregs_state)).header, core::mem::size_of::<xstate_header>()) != 0 { return false; }
    loop {
        fpregs_lock();
        if test_thread_flag(TIF_NEED_FPU_LOAD) { fpregs_restore_userregs(); }
        pagefault_disable(); let ret = copy_fpregs_to_sigframe(buf_fx as *mut xregs_state, pkru); pagefault_enable(); fpregs_unlock();
        if ret != 0 { if __clear_user(buf_fx, (*fpstate).user_size as usize) == 0 { continue; } return false; }
        if (ia32_fxstate || !use_fxsr()) && !save_fsave_header(tsk, buf) { return false; }
        if use_fxsr() && !save_xstate_epilog(buf_fx, ia32_fxstate, fpstate) { return false; }
        return true;
    }
}

unsafe fn __restore_fpregs_from_user(buf: *mut core::ffi::c_void, ufeatures: u64, xrestore: u64, fx_only: bool) -> i32 {
    if use_xsave() { let init_bv = ufeatures & !xrestore; let ret = if !fx_only { xrstor_from_user_sigframe(buf, xrestore) } else { fxrstor_from_user_sigframe(buf) }; if ret == 0 && unlikely(init_bv != 0) { os_xrstor(&init_fpstate, init_bv); } ret }
    else if use_fxsr() { fxrstor_from_user_sigframe(buf) } else { frstor_from_user_sigframe(buf) }
}

unsafe fn restore_fpregs_from_user(buf: *mut core::ffi::c_void, mut xrestore: u64, fx_only: bool) -> bool {
    let fpu = x86_task_fpu(current()); xrestore &= (*(*fpu).fpstate).user_xfeatures;
    loop {
        fpregs_lock(); xfd_update_state((*fpu).fpstate); pagefault_disable(); let ret = __restore_fpregs_from_user(buf, (*(*fpu).fpstate).user_xfeatures, xrestore, fx_only); pagefault_enable();
        if ret != 0 { if test_thread_flag(TIF_NEED_FPU_LOAD) { __cpu_invalidate_fpregs_state(); } fpregs_unlock(); if ret != X86_TRAP_PF { return false; } if !fault_in_readable(buf, (*(*fpu).fpstate).user_size as usize) { continue; } return false; }
        if test_thread_flag(TIF_NEED_FPU_LOAD) && xfeatures_mask_supervisor() { os_xrstor_supervisor((*fpu).fpstate); }
        fpregs_mark_activate(); fpregs_unlock(); return true;
    }
}

unsafe fn __fpu_restore_sig(buf: *mut core::ffi::c_void, buf_fx: *mut core::ffi::c_void, ia32_fxstate: bool) -> bool {
    let tsk = current();
    let fpu = x86_task_fpu(tsk);
    let mut env: user_i387_ia32_struct = core::mem::zeroed();
    let mut success: bool;
    let mut fx_only = false;
    let mut user_xfeatures: u64 = 0;
    if use_xsave() {
        let mut fx_sw_user: _fpx_sw_bytes = core::mem::zeroed();
        if !check_xstate_in_sigframe(buf_fx as *mut fxregs_state, &mut fx_sw_user) { return false; }
        fx_only = fx_sw_user.magic1 == 0;
        user_xfeatures = fx_sw_user.xfeatures;
    } else { user_xfeatures = XFEATURE_MASK_FPSSE; }
    if !ia32_fxstate { return restore_fpregs_from_user(buf_fx, user_xfeatures, fx_only); }
    if __copy_from_user(&mut env, buf, core::mem::size_of::<user_i387_ia32_struct>()) != 0 { return false; }
    fpregs_lock();
    if !test_thread_flag(TIF_NEED_FPU_LOAD) {
        if xfeatures_mask_supervisor() { os_xsave((*fpu).fpstate); }
        set_thread_flag(TIF_NEED_FPU_LOAD);
    }
    __fpu_invalidate_fpregs_state(fpu); __cpu_invalidate_fpregs_state(); fpregs_unlock();
    let fpregs = &mut (*(*fpu).fpstate).regs;
    if use_xsave() && !fx_only {
        if copy_sigframe_from_user_to_xstate(tsk, buf_fx) { return false; }
    } else {
        if __copy_from_user(&mut fpregs.fxsave, buf_fx, core::mem::size_of::<fxregs_state>()) != 0 { return false; }
        if IS_ENABLED(CONFIG_X86_64) {
            if fpregs.fxsave.mxcsr & !mxcsr_feature_mask != 0 { return false; }
        } else { fpregs.fxsave.mxcsr &= mxcsr_feature_mask; }
        if use_xsave() { fpregs.xsave.header.xfeatures |= XFEATURE_MASK_FPSSE; }
    }
    convert_to_fxsr(&mut fpregs.fxsave, &env);
    fpregs_lock();
    if use_xsave() {
        let mask = user_xfeatures | xfeatures_mask_supervisor();
        fpregs.xsave.header.xfeatures &= mask;
        success = !os_xrstor_safe((*fpu).fpstate, fpu_kernel_cfg.max_features);
    } else { success = !fxrstor_safe(&mut fpregs.fxsave); }
    if success { fpregs_mark_activate(); }
    fpregs_unlock(); success
}

#[inline]
unsafe fn xstate_sigframe_size(fpstate: *mut fpstate) -> u32 { (*fpstate).user_size + if use_xsave() { FP_XSTATE_MAGIC2_SIZE } else { 0 } }

pub unsafe fn fpu__restore_sig(buf: *mut core::ffi::c_void, mut ia32_frame: i32) -> bool {
    let fpu = x86_task_fpu(current()); if buf.is_null() { fpu__clear_user_states(fpu); return true; }
    let mut size = xstate_sigframe_size((*fpu).fpstate); ia32_frame &= (IS_ENABLED(CONFIG_X86_32) || IS_ENABLED(CONFIG_IA32_EMULATION)) as i32;
    let mut buf_fx = buf; let mut ia32_fxstate = false;
    if ia32_frame != 0 && use_fxsr() { buf_fx = (buf as *mut u8).add(core::mem::size_of::<fregs_state>()) as *mut _; size += core::mem::size_of::<fregs_state>() as u32; ia32_fxstate = true; }
    if !access_ok(buf, size as usize) { fpu__clear_user_states(fpu); return false; }
    let success = if !IS_ENABLED(CONFIG_X86_64) && !cpu_feature_enabled(X86_FEATURE_FPU) { !fpregs_soft_set(current(), core::ptr::null_mut(), 0, core::mem::size_of::<user_i387_ia32_struct>(), core::ptr::null_mut(), buf) } else { __fpu_restore_sig(buf, buf_fx, ia32_fxstate) };
    if !success { fpu__clear_user_states(fpu); } success
}

pub unsafe fn fpu__alloc_mathframe(mut sp: usize, ia32_frame: i32, buf_fx: *mut usize, size: *mut usize) -> usize {
    let mut frame_size = xstate_sigframe_size((*x86_task_fpu(current())).fpstate) as usize; sp = (sp - frame_size) & !63; *buf_fx = sp;
    if ia32_frame != 0 && use_fxsr() { frame_size += core::mem::size_of::<fregs_state>(); sp -= core::mem::size_of::<fregs_state>(); }
    *size = frame_size; sp
}

pub unsafe fn fpu__get_fpstate_size() -> usize {
    let mut ret = fpu_user_cfg.max_size as usize; if use_xsave() { ret += FP_XSTATE_MAGIC2_SIZE as usize; }
    if (IS_ENABLED(CONFIG_IA32_EMULATION) || IS_ENABLED(CONFIG_X86_32)) && use_fxsr() { ret += core::mem::size_of::<fregs_state>(); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
