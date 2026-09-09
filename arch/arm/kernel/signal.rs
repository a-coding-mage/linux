// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/signal.c
 *
 *  Copyright (C) 1995-2009 Russell King
 */

// Kernel and architecture headers from the C source provide the external
// types, constants, functions, and configuration symbols used below.

extern "C" {
    static sigreturn_codes: [c_ulong; 17];
}

static mut signal_return_offset: c_ulong = 0;

#[cfg(CONFIG_IWMMXT)]
unsafe fn preserve_iwmmxt_context(frame: *mut iwmmxt_sigframe) -> c_int {
    let mut kbuf = [0u8; core::mem::size_of::<iwmmxt_sigframe>() + 8];
    let kframe = ((kbuf.as_mut_ptr().add(8) as usize) & !7) as *mut iwmmxt_sigframe;
    if test_thread_flag(TIF_USING_IWMMXT) {
        (*kframe).magic = IWMMXT_MAGIC;
        (*kframe).size = IWMMXT_STORAGE_SIZE;
        iwmmxt_task_copy(current_thread_info(), &mut (*kframe).storage);
    } else {
        *kframe = iwmmxt_sigframe { magic: DUMMY_MAGIC, size: IWMMXT_STORAGE_SIZE, ..core::mem::zeroed() };
    }
    __copy_to_user(frame as *mut _, kframe as *const _, core::mem::size_of::<iwmmxt_sigframe>()) as c_int
}

#[cfg(CONFIG_IWMMXT)]
unsafe fn restore_iwmmxt_context(auxp: *mut *mut c_char) -> c_int {
    let frame = *auxp as *mut iwmmxt_sigframe;
    let mut kbuf = [0u8; core::mem::size_of::<iwmmxt_sigframe>() + 8];
    let kframe = ((kbuf.as_mut_ptr().add(8) as usize) & !7) as *mut iwmmxt_sigframe;
    if __copy_from_user(kframe as *mut _, frame as *const _, core::mem::size_of::<iwmmxt_sigframe>()) != 0 { return -1; }
    if !test_thread_flag(TIF_USING_IWMMXT) && (*kframe).magic != DUMMY_MAGIC { return 0; }
    if (*kframe).size != IWMMXT_STORAGE_SIZE { return -1; }
    if test_thread_flag(TIF_USING_IWMMXT) {
        if (*kframe).magic != IWMMXT_MAGIC { return -1; }
        iwmmxt_task_restore(current_thread_info(), &(*kframe).storage);
    }
    *auxp = (*auxp).add(IWMMXT_STORAGE_SIZE as usize);
    0
}

#[cfg(CONFIG_VFP)]
unsafe fn preserve_vfp_context(frame: *mut vfp_sigframe) -> c_int {
    let mut kframe: vfp_sigframe = core::mem::zeroed();
    kframe.magic = VFP_MAGIC;
    kframe.size = VFP_STORAGE_SIZE;
    let err = vfp_preserve_user_clear_hwstate(&mut kframe.ufp, &mut kframe.ufp_exc);
    if err != 0 { return err; }
    __copy_to_user(frame as *mut _, &kframe as *const _ as *const _, core::mem::size_of::<vfp_sigframe>()) as c_int
}

#[cfg(CONFIG_VFP)]
unsafe fn restore_vfp_context(auxp: *mut *mut c_char) -> c_int {
    let mut frame: vfp_sigframe = core::mem::zeroed();
    let err = __copy_from_user(&mut frame as *mut _ as *mut _, *auxp as *const _, core::mem::size_of::<vfp_sigframe>()) as c_int;
    if err != 0 { return err; }
    if frame.magic != VFP_MAGIC || frame.size != VFP_STORAGE_SIZE { return -EINVAL; }
    *auxp = (*auxp).add(core::mem::size_of::<vfp_sigframe>());
    vfp_restore_user_hwstate(&mut frame.ufp, &mut frame.ufp_exc)
}

unsafe fn restore_sigframe(regs: *mut pt_regs, sf: *mut sigframe) -> c_int {
    let mut context: sigcontext = core::mem::zeroed();
    let mut set: sigset_t = core::mem::zeroed();
    let mut err = __copy_from_user(&mut set as *mut _ as *mut _, &(*sf).uc.uc_sigmask as *const _ as *const _, core::mem::size_of::<sigset_t>()) as c_int;
    if err == 0 { set_current_blocked(&set); }
    err |= __copy_from_user(&mut context as *mut _ as *mut _, &(*sf).uc.uc_mcontext as *const _ as *const _, core::mem::size_of::<sigcontext>()) as c_int;
    if err == 0 {
        (*regs).ARM_r0=context.arm_r0; (*regs).ARM_r1=context.arm_r1; (*regs).ARM_r2=context.arm_r2; (*regs).ARM_r3=context.arm_r3;
        (*regs).ARM_r4=context.arm_r4; (*regs).ARM_r5=context.arm_r5; (*regs).ARM_r6=context.arm_r6; (*regs).ARM_r7=context.arm_r7;
        (*regs).ARM_r8=context.arm_r8; (*regs).ARM_r9=context.arm_r9; (*regs).ARM_r10=context.arm_r10; (*regs).ARM_fp=context.arm_fp;
        (*regs).ARM_ip=context.arm_ip; (*regs).ARM_sp=context.arm_sp; (*regs).ARM_lr=context.arm_lr; (*regs).ARM_pc=context.arm_pc;
        (*regs).ARM_cpsr=context.arm_cpsr;
    }
    err |= (!valid_user_regs(regs)) as c_int;
    let mut aux = (*sf).uc.uc_regspace.as_mut_ptr() as *mut c_char;
    #[cfg(CONFIG_IWMMXT)] if err == 0 { err |= restore_iwmmxt_context(&mut aux); }
    #[cfg(CONFIG_VFP)] if err == 0 { err |= restore_vfp_context(&mut aux); }
    err
}

pub unsafe extern "C" fn sys_sigreturn(regs: *mut pt_regs) -> c_int {
    (*current).restart_block.fn_ = do_no_restart_syscall;
    if (*regs).ARM_sp & 7 != 0 { force_sig(SIGSEGV); return 0; }
    let frame = (*regs).ARM_sp as *mut sigframe;
    if !access_ok(frame as *const _, core::mem::size_of::<sigframe>()) || restore_sigframe(regs, frame) != 0 { force_sig(SIGSEGV); return 0; }
    (*regs).ARM_r0 as c_int
}

pub unsafe extern "C" fn sys_rt_sigreturn(regs: *mut pt_regs) -> c_int {
    (*current).restart_block.fn_ = do_no_restart_syscall;
    if (*regs).ARM_sp & 7 != 0 { force_sig(SIGSEGV); return 0; }
    let frame = (*regs).ARM_sp as *mut rt_sigframe;
    if !access_ok(frame as *const _, core::mem::size_of::<rt_sigframe>()) || restore_sigframe(regs, &mut (*frame).sig) != 0 || restore_altstack(&(*frame).sig.uc.uc_stack) != 0 { force_sig(SIGSEGV); return 0; }
    (*regs).ARM_r0 as c_int
}

unsafe fn setup_sigframe(sf: *mut sigframe, regs: *mut pt_regs, set: *mut sigset_t) -> c_int {
    let context = sigcontext { arm_r0:(*regs).ARM_r0, arm_r1:(*regs).ARM_r1, arm_r2:(*regs).ARM_r2, arm_r3:(*regs).ARM_r3, arm_r4:(*regs).ARM_r4, arm_r5:(*regs).ARM_r5, arm_r6:(*regs).ARM_r6, arm_r7:(*regs).ARM_r7, arm_r8:(*regs).ARM_r8, arm_r9:(*regs).ARM_r9, arm_r10:(*regs).ARM_r10, arm_fp:(*regs).ARM_fp, arm_ip:(*regs).ARM_ip, arm_sp:(*regs).ARM_sp, arm_lr:(*regs).ARM_lr, arm_pc:(*regs).ARM_pc, arm_cpsr:(*regs).ARM_cpsr, trap_no:(*current).thread.trap_no, error_code:(*current).thread.error_code, fault_address:(*current).thread.address, oldmask:(*set).sig[0] };
    let mut err = __copy_to_user(&mut (*sf).uc.uc_mcontext as *mut _ as *mut _, &context as *const _ as *const _, core::mem::size_of::<sigcontext>()) as c_int;
    err |= __copy_to_user(&mut (*sf).uc.uc_sigmask as *mut _ as *mut _, set as *const _, core::mem::size_of::<sigset_t>()) as c_int;
    let aux = (*sf).uc.uc_regspace.as_mut_ptr() as *mut aux_sigframe;
    #[cfg(CONFIG_IWMMXT)] if err == 0 { err |= preserve_iwmmxt_context(&mut (*aux).iwmmxt); }
    #[cfg(CONFIG_VFP)] if err == 0 { err |= preserve_vfp_context(&mut (*aux).vfp); }
    err |= __put_user(0, &mut (*aux).end_magic) as c_int;
    err
}

unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, framesize: usize) -> *mut c_void {
    let sp = sigsp((*regs).ARM_sp, ksig);
    let frame = ((sp as usize - framesize) & !7) as *mut c_void;
    if !access_ok(frame as *const _, framesize) { core::ptr::null_mut() } else { frame }
}

unsafe fn setup_return(regs: *mut pt_regs, ksig: *mut ksignal, rc: *mut c_ulong, frame: *mut c_void) -> c_int {
    let mut handler = (*ksig).ka.sa.sa_handler as c_ulong;
    let mut got = 0;
    let mut idx: usize;
    let mut thumb = 0;
    let mut cpsr = (*regs).ARM_cpsr & !(PSR_f | PSR_E_BIT);
    let fdpic = IS_ENABLED(CONFIG_BINFMT_ELF_FDPIC) && ((*current).personality & FDPIC_FUNCPTRS) != 0;
    if fdpic {
        let desc = handler as *mut c_ulong;
        if __get_user(&mut handler, desc) != 0 || __get_user(&mut got, desc.add(1)) != 0 { return 1; }
    }
    if IS_ENABLED(CONFIG_CPU_ENDIAN_BE8) { cpsr |= PSR_E_BIT; }
    if (*ksig).ka.sa.sa_flags & SA_THIRTYTWO != 0 { cpsr = (cpsr & !MODE_MASK) | USR_MODE; }
    #[cfg(CONFIG_ARM_THUMB)] if elf_hwcap & HWCAP_THUMB != 0 {
        thumb = handler & 1; cpsr &= !PSR_IT_MASK;
        if thumb != 0 { cpsr |= PSR_T_BIT; } else { cpsr &= !PSR_T_BIT; }
    }
    if (*ksig).ka.sa.sa_flags & SA_RESTORER != 0 {
        let retcode = (*ksig).ka.sa.sa_restorer as c_ulong;
        if fdpic {
            idx = 6 + thumb as usize * 3; if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { idx += 5; }
            if __put_user(sigreturn_codes[idx], rc) != 0 || __put_user(sigreturn_codes[idx+1], rc.add(1)) != 0 || __put_user(sigreturn_codes[idx+2], rc.add(2)) != 0 || __put_user(retcode, rc.add(3)) != 0 { return 1; }
        } else { idx = 0; }
    } else {
        idx = (thumb << 1) as usize; if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { idx += 3; }
        if __put_user(sigreturn_codes[idx], rc) != 0 || __put_user(sigreturn_codes[idx+1], rc.add(1)) != 0 { return 1; }
    }
    let retcode = if IS_ENABLED(CONFIG_MMU) && cpsr & MODE32_BIT != 0 { (*(*current).mm).context.sigpage + signal_return_offset + (idx as c_ulong << 2) + thumb } else { flush_icache_range(rc as c_ulong, rc.add(3) as c_ulong); rc as c_ulong + thumb };
    (*regs).ARM_r0 = (*ksig).sig; (*regs).ARM_sp = frame as c_ulong; (*regs).ARM_lr = retcode; (*regs).ARM_pc = handler;
    if fdpic { (*regs).ARM_r9 = got; } (*regs).ARM_cpsr = cpsr; 0
}

unsafe fn setup_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<sigframe>()) as *mut sigframe; if frame.is_null() { return 1; }
    let mut err = __put_user(0x5ac3c35a, &mut (*frame).uc.uc_flags) as c_int; err |= setup_sigframe(frame, regs, set);
    if err == 0 { err = setup_return(regs, ksig, (*frame).retcode.as_mut_ptr(), frame as *mut _); } err
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let frame = get_sigframe(ksig, regs, core::mem::size_of::<rt_sigframe>()) as *mut rt_sigframe; if frame.is_null() { return 1; }
    let mut err = copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info); err |= __put_user(0, &mut (*frame).sig.uc.uc_flags) as c_int; err |= __put_user(core::ptr::null_mut(), &mut (*frame).sig.uc.uc_link) as c_int; err |= __save_altstack(&mut (*frame).sig.uc.uc_stack, (*regs).ARM_sp); err |= setup_sigframe(&mut (*frame).sig, regs, set);
    if err == 0 { err = setup_return(regs, ksig, (*frame).sig.retcode.as_mut_ptr(), frame as *mut _); }
    if err == 0 { (*regs).ARM_r1 = &mut (*frame).info as *mut _ as c_ulong; (*regs).ARM_r2 = &mut (*frame).sig.uc as *mut _ as c_ulong; } err
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) { let oldset = sigmask_to_save(); rseq_signal_deliver(ksig, regs); let ret = if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { setup_rt_frame(ksig, oldset, regs) } else { setup_frame(ksig, oldset, regs) }; signal_setup_done(ret | (!valid_user_regs(regs) as c_int), ksig, 0); }

unsafe fn do_signal(regs: *mut pt_regs, syscall: c_int) -> c_int {
    let (mut retval, mut cont, mut restart_addr, mut restart) = (0u32, 0u32, 0u32, 0); let mut ksig: ksignal = core::mem::zeroed();
    if syscall != 0 { cont=(*regs).ARM_pc; restart_addr=cont-(if thumb_mode(regs){2}else{4}); retval=(*regs).ARM_r0; match retval as c_int { -ERESTART_RESTARTBLOCK => {restart-=2; restart+=1; (*regs).ARM_r0=(*regs).ARM_ORIG_r0; (*regs).ARM_pc=restart_addr;}, -ERESTARTNOHAND|-ERESTARTSYS|-ERESTARTNOINTR => {restart+=1; (*regs).ARM_r0=(*regs).ARM_ORIG_r0; (*regs).ARM_pc=restart_addr;}, _=>{} } }
    if get_signal(&mut ksig) { if restart != 0 && (*regs).ARM_pc==restart_addr && (retval as c_int==-ERESTARTNOHAND || retval as c_int==-ERESTART_RESTARTBLOCK || (retval as c_int==-ERESTARTSYS && (*ksig).ka.sa.sa_flags&SA_RESTART==0)) {(*regs).ARM_r0=-EINTR as _; (*regs).ARM_pc=cont;} handle_signal(&mut ksig, regs); } else { restore_saved_sigmask(); if restart!=0 && (*regs).ARM_pc==restart_addr {(*regs).ARM_pc=cont; return restart;} } 0
}

pub unsafe extern "C" fn do_work_pending(regs:*mut pt_regs, mut thread_flags:c_uint, mut syscall:c_int)->c_int { trace_hardirqs_off(); loop { if thread_flags&_TIF_NEED_RESCHED!=0 {schedule();} else {if !user_mode(regs){return 0;} local_irq_enable(); if thread_flags&(_TIF_SIGPENDING|_TIF_NOTIFY_SIGNAL)!=0 {let r=do_signal(regs,syscall); if r!=0{return r;} syscall=0;} else if thread_flags&_TIF_UPROBE!=0 {uprobe_notify_resume(regs);} else {resume_user_mode_work(regs);}} local_irq_disable(); thread_flags=read_thread_flags(); if thread_flags&_TIF_WORK_MASK==0{break;} } 0 }

pub unsafe extern "C" fn get_signal_page() -> *mut page { let page=alloc_pages(GFP_KERNEL,0); if page.is_null(){return core::ptr::null_mut();} let addr=page_address(page); memset32(addr,__opcode_to_mem_arm(0xe7fddef1),PAGE_SIZE/core::mem::size_of::<u32>()); let offset=0x200+(get_random_u16()&0x7fc) as c_ulong; signal_return_offset=offset; memcpy((addr as *mut u8).add(offset as usize) as *mut _, sigreturn_codes.as_ptr() as *const _, core::mem::size_of_val(&sigreturn_codes)); flush_icache_range(addr as c_ulong,addr as c_ulong+PAGE_SIZE); page }

#[cfg(CONFIG_DEBUG_RSEQ)] pub unsafe extern "C" fn do_rseq_syscall(regs:*mut pt_regs){rseq_syscall(regs);}

// Compile-time siginfo_t layout assertions from the C implementation.
const _: () = {
    assert!(NSIGILL == 11); assert!(NSIGFPE == 15); assert!(NSIGSEGV == 10);
    assert!(NSIGBUS == 5); assert!(NSIGTRAP == 6); assert!(NSIGCHLD == 6);
    assert!(NSIGSYS == 2); assert!(core::mem::size_of::<siginfo_t>() == 128);
    assert!(core::mem::align_of::<siginfo_t>() == 4);
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
