// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerPC signal handling, translated literally from signal_64.c. */

// Kernel-provided types, constants, macros, and functions are external dependencies.

const GP_REGS_SIZE: usize = core::mem::size_of::<elf_gregset_t>();
const FP_REGS_SIZE: usize = core::mem::size_of::<elf_fpregset_t>();
const TRAMP_TRACEBACK: usize = 4;
const TRAMP_SIZE: usize = 7;

#[repr(C, align(16))]
pub struct rt_sigframe {
    pub uc: ucontext,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    pub uc_transact: ucontext,
    pub _unused: [c_ulong; 2],
    pub tramp: [c_uint; TRAMP_SIZE],
    pub pinfo: *mut siginfo,
    pub puc: *mut core::ffi::c_void,
    pub info: siginfo,
    pub abigap: [u8; USER_REDZONE_SIZE],
}

pub unsafe fn get_min_sigframe_size_64() -> c_ulong {
    (core::mem::size_of::<rt_sigframe>() + __SIGNAL_FRAMESIZE) as c_ulong
}

#[cfg(CONFIG_ALTIVEC)]
unsafe fn sigcontext_vmx_regs(sc: *mut sigcontext) -> *mut elf_vrreg_t {
    ((((*sc).vmx_reserve as usize) + 15) & !0xf) as *mut elf_vrreg_t
}

unsafe fn prepare_setup_sigcontext(tsk: *mut task_struct) {
    #[cfg(CONFIG_ALTIVEC)] {
        if (*tsk).thread.used_vr { flush_altivec_to_thread(tsk); }
        if cpu_has_feature(CPU_FTR_ALTIVEC) { (*tsk).thread.vrsave = mfspr(SPRN_VRSAVE); }
    }
    flush_fp_to_thread(tsk);
    #[cfg(CONFIG_VSX)] if (*tsk).thread.used_vsr { flush_vsx_to_thread(tsk); }
}

unsafe fn __unsafe_setup_sigcontext(sc: *mut sigcontext, tsk: *mut task_struct,
    signr: c_int, set: *mut sigset_t, handler: c_ulong, ctx_has_vsx_region: c_int) -> c_long {
    #[cfg(CONFIG_ALTIVEC)] let mut v_regs = sigcontext_vmx_regs(sc);
    let regs = (*tsk).thread.regs;
    let mut msr = (*regs).msr;
    let softe: c_ulong = 1;
    BUG_ON(tsk != current);
    #[cfg(CONFIG_ALTIVEC)] {
        unsafe_put_user(v_regs, &mut (*sc).v_regs, efault_out);
        if (*tsk).thread.used_vr {
            unsafe_copy_to_user(v_regs, &(*tsk).thread.vr_state, 33 * core::mem::size_of::<vector128>(), efault_out);
            msr |= MSR_VEC;
        }
        unsafe_put_user((*tsk).thread.vrsave, v_regs.add(33) as *mut u32, efault_out);
    }
    #[cfg(not(CONFIG_ALTIVEC))] unsafe_put_user(0, &mut (*sc).v_regs, efault_out);
    unsafe_copy_fpr_to_user(&mut (*sc).fp_regs, tsk, efault_out);
    msr &= !MSR_VSX;
    #[cfg(CONFIG_VSX)] if (*tsk).thread.used_vsr && ctx_has_vsx_region != 0 {
        v_regs = v_regs.add(ELF_NVRREG);
        unsafe_copy_vsx_to_user(v_regs, tsk, efault_out);
        msr |= MSR_VSX;
    }
    unsafe_put_user(&mut (*sc).gp_regs, &mut (*sc).regs, efault_out);
    unsafe_copy_to_user(&mut (*sc).gp_regs, regs, GP_REGS_SIZE, efault_out);
    unsafe_put_user(msr, (*sc).gp_regs.as_mut_ptr().add(PT_MSR), efault_out);
    unsafe_put_user(softe, (*sc).gp_regs.as_mut_ptr().add(PT_SOFTE), efault_out);
    unsafe_put_user(signr, &mut (*sc).signal, efault_out);
    unsafe_put_user(handler, &mut (*sc).handler, efault_out);
    if !set.is_null() { unsafe_put_user((*set).sig[0], &mut (*sc).oldmask, efault_out); }
    return 0;
efault_out: return -EFAULT;
}

unsafe fn __unsafe_restore_sigcontext(tsk: *mut task_struct, set: *mut sigset_t,
    sig: c_int, sc: *mut sigcontext) -> c_long {
    #[cfg(CONFIG_ALTIVEC)] let mut v_regs: *mut elf_vrreg_t = core::ptr::null_mut();
    let mut save_r13 = 0;
    let regs = (*tsk).thread.regs;
    let msr: c_ulong;
    #[cfg(CONFIG_VSX)] let mut i: c_int;
    BUG_ON(tsk != current);
    if sig == 0 { save_r13 = (*regs).gpr[13]; }
    unsafe_copy_from_user((*regs).gpr.as_mut_ptr(), (*sc).gp_regs.as_ptr(), core::mem::size_of_val(&(*regs).gpr), efault_out);
    unsafe_get_user((*regs).nip, (*sc).gp_regs.as_ptr().add(PT_NIP), efault_out);
    unsafe_get_user(msr, (*sc).gp_regs.as_ptr().add(PT_MSR), efault_out);
    if sig != 0 { regs_set_return_msr(regs, ((*regs).msr & !MSR_LE) | (msr & MSR_LE)); }
    unsafe_get_user((*regs).orig_gpr3, (*sc).gp_regs.as_ptr().add(PT_ORIG_R3), efault_out);
    unsafe_get_user((*regs).ctr, (*sc).gp_regs.as_ptr().add(PT_CTR), efault_out);
    unsafe_get_user((*regs).link, (*sc).gp_regs.as_ptr().add(PT_LNK), efault_out);
    unsafe_get_user((*regs).xer, (*sc).gp_regs.as_ptr().add(PT_XER), efault_out);
    unsafe_get_user((*regs).ccr, (*sc).gp_regs.as_ptr().add(PT_CCR), efault_out);
    set_trap_norestart(regs);
    unsafe_get_user((*regs).dar, (*sc).gp_regs.as_ptr().add(PT_DAR), efault_out);
    unsafe_get_user((*regs).dsisr, (*sc).gp_regs.as_ptr().add(PT_DSISR), efault_out);
    unsafe_get_user((*regs).result, (*sc).gp_regs.as_ptr().add(PT_RESULT), efault_out);
    if sig == 0 { (*regs).gpr[13] = save_r13; }
    if !set.is_null() { unsafe_get_user((*set).sig[0], &(*sc).oldmask, efault_out); }
    regs_set_return_msr(regs, (*regs).msr & !(MSR_FP|MSR_FE0|MSR_FE1|MSR_VEC|MSR_VSX));
    #[cfg(CONFIG_ALTIVEC)] {
        unsafe_get_user(v_regs, &(*sc).v_regs, efault_out);
        if !v_regs.is_null() && !access_ok(v_regs, 34 * core::mem::size_of::<vector128>()) { return -EFAULT; }
        if !v_regs.is_null() && (msr & MSR_VEC) != 0 { unsafe_copy_from_user(&mut (*tsk).thread.vr_state, v_regs, 33*core::mem::size_of::<vector128>(), efault_out); (*tsk).thread.used_vr=true; }
        else if (*tsk).thread.used_vr { memset(&mut (*tsk).thread.vr_state, 0, 33*core::mem::size_of::<vector128>()); }
        if !v_regs.is_null() { unsafe_get_user((*tsk).thread.vrsave, v_regs.add(33) as *mut u32, efault_out); } else { (*tsk).thread.vrsave=0; }
        if cpu_has_feature(CPU_FTR_ALTIVEC) { mtspr(SPRN_VRSAVE, (*tsk).thread.vrsave); }
    }
    unsafe_copy_fpr_from_user(tsk, &mut (*sc).fp_regs, efault_out);
    #[cfg(CONFIG_VSX)] { v_regs = v_regs.add(ELF_NVRREG); if (msr & MSR_VSX)!=0 { unsafe_copy_vsx_from_user(tsk,v_regs,efault_out); (*tsk).thread.used_vsr=true; } else { i=0; while i<32 { (*tsk).thread.fp_state.fpr[i as usize][TS_VSRLOWOFFSET]=0; i+=1; } } }
    return 0;
efault_out: return -EFAULT;
}

unsafe fn setup_trampoline(syscall: c_uint, tramp: *mut c_uint) -> c_long {
    let mut err: c_long=0;
    err |= __put_user(PPC_RAW_BCTRL(), tramp); err |= __put_user(PPC_RAW_ADDI(_R1,_R1,__SIGNAL_FRAMESIZE), tramp.add(1));
    err |= __put_user(PPC_RAW_LI(_R0,syscall), tramp.add(2)); err |= __put_user(PPC_RAW_SC(), tramp.add(3));
    let mut i=TRAMP_TRACEBACK; while i<TRAMP_SIZE { err |= __put_user(0,tramp.add(i)); i+=1; }
    if err==0 { flush_icache_range(tramp as c_ulong, tramp.add(TRAMP_SIZE) as c_ulong); } err
}

// The remaining syscall and signal-frame entry points retain the kernel ABI and
// are declared with their original operations; external kernel helpers are unresolved here.
pub unsafe fn handle_rt_signal64(ksig: *mut ksignal, set: *mut sigset_t, tsk: *mut task_struct) -> c_int {
    let frame = get_sigframe(ksig, tsk, core::mem::size_of::<rt_sigframe>(), 0);
    let regs=(*tsk).thread.regs;
    if !MSR_TM_ACTIVE((*regs).msr) { prepare_setup_sigcontext(tsk); }
    if !user_write_access_begin(frame, core::mem::size_of::<rt_sigframe>()) { signal_fault(current,regs,"handle_rt_signal64",frame); return 1; }
    unsafe_put_user(&mut (*frame).info,&mut (*frame).pinfo,badframe_block);
    unsafe_put_user(&mut (*frame).uc,&mut (*frame).puc,badframe_block);
    unsafe_put_user(0,&mut (*frame).uc.uc_flags,badframe_block);
    unsafe_save_altstack(&mut (*frame).uc.uc_stack,(*regs).gpr[1],badframe_block);
    unsafe_put_user(0,&mut (*frame).uc.uc_link,badframe_block);
    unsafe_setup_sigcontext(&mut (*frame).uc.uc_mcontext,tsk,(*ksig).sig,core::ptr::null_mut(),(*ksig).ka.sa.sa_handler as c_ulong,1,badframe_block);
    unsafe_copy_to_user(&mut (*frame).uc.uc_sigmask,set,core::mem::size_of::<sigset_t>(),badframe_block);
    user_write_access_end();
    if copy_siginfo_to_user(&mut (*frame).info,&(*ksig).info)!=0 { goto_badframe!(); }
    (*tsk).thread.fp_state.fpscr=0;
    if !(*tsk).mm.context.vdso.is_null() { regs_set_return_ip(regs,VDSO64_SYMBOL((*tsk).mm.context.vdso,sigtramp_rt64)); } else { if setup_trampoline(__NR_rt_sigreturn,(*frame).tramp.as_mut_ptr())!=0 { goto_badframe!(); } regs_set_return_ip(regs,(*frame).tramp.as_mut_ptr() as c_ulong); }
    let newsp=frame as c_ulong-__SIGNAL_FRAMESIZE; put_user((*regs).gpr[1],newsp as *mut c_ulong);
    regs_set_return_msr(regs,((*regs).msr&!MSR_LE)|(MSR_KERNEL&MSR_LE)); (*regs).gpr[1]=newsp; (*regs).gpr[3]=(*ksig).sig as c_ulong; (*regs).result=0;
    if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { (*regs).gpr[4]=&mut (*frame).info as *mut _ as c_ulong; (*regs).gpr[5]=&mut (*frame).uc as *mut _ as c_ulong; (*regs).gpr[6]=frame as c_ulong; } else { (*regs).gpr[4]=&mut (*frame).uc.uc_mcontext as *mut _ as c_ulong; }
    return 0;
badframe_block: user_write_access_end();
badframe: signal_fault(current,regs,"handle_rt_signal64",frame); 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
