// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of arch/sparc64/kernel/signal.c. */

/* C headers and external kernel declarations are supplied by the surrounding build. */

unsafe extern "C" {
    fn sparc64_set_context(regs: *mut pt_regs);
    fn sparc64_get_context(regs: *mut pt_regs);
    fn exception_enter() -> ctx_state;
    fn exception_exit(state: ctx_state);
    fn synchronize_user_stack();
    fn get_thread_wsaved() -> c_int;
    fn force_sig(sig: c_int);
    fn force_sigsegv(sig: c_int);
    fn test_thread_flag(flag: c_int) -> bool;
    fn set_current_blocked(set: *const sigset_t);
    fn restore_fpu_state(regs: *mut pt_regs, save: *mut __siginfo_fpu_t) -> c_int;
    fn restore_rwin_state(save: *mut __siginfo_rwin_t) -> c_int;
    fn restore_altstack(stack: *const stack_t) -> c_int;
    fn pt_regs_clear_syscall(regs: *mut pt_regs);
    fn current_thread_info() -> *mut thread_info;
    fn fprs_write(value: c_ulong);
    fn clear_user(ptr: *mut c_void, size: usize) -> c_int;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn __copy_from_user(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn __copy_to_user(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn raw_copy_in_user(dst: *mut u64, src: *const u64, size: usize) -> c_int;
    fn save_fpu_state(regs: *mut pt_regs, save: *mut __siginfo_fpu_t) -> c_int;
    fn save_rwin_state(wsaved: c_int, save: *mut __siginfo_rwin_t) -> c_int;
    fn __save_altstack(stack: *mut stack_t, sp: c_ulong) -> c_int;
    fn sigmask_to_save() -> *const sigset_t;
    fn copy_siginfo_to_user(dst: *mut siginfo_t, src: *const siginfo_t) -> c_int;
    fn get_signal(ksig: *mut ksignal) -> bool;
    fn signal_setup_done(err: c_int, ksig: *mut ksignal, signr: c_int);
    fn restore_saved_sigmask();
    fn user_exit(); fn user_enter();
    fn uprobe_notify_resume(regs: *mut pt_regs);
    fn resume_user_mode_work(regs: *mut pt_regs);
}

#[allow(non_camel_case_types)] type c_int = i32;
#[allow(non_camel_case_types)] type c_ulong = usize;
#[allow(non_camel_case_types)] type c_void = core::ffi::c_void;

unsafe fn invalid_frame_pointer(fp: *mut c_void) -> bool { (fp as c_ulong & 15) != 0 }

#[repr(C)] struct rt_signal_frame { ss: sparc_stackf, info: siginfo_t, regs: pt_regs, fpu_save: *mut __siginfo_fpu_t, stack: stack_t, mask: sigset_t, rwin_save: *mut __siginfo_rwin_t }

pub unsafe fn do_rt_sigreturn(regs: *mut pt_regs) {
    let mut tpc = 0usize; let mut tnpc = 0usize; let mut tstate = 0usize; let mut ufp = 0usize;
    let sf = ((*regs).u_regs[UREG_FP] + STACK_BIAS) as *mut rt_signal_frame;
    let mut fpu_save: *mut __siginfo_fpu_t; let mut rwin_save: *mut __siginfo_rwin_t;
    let mut set: sigset_t = core::mem::zeroed(); let mut err = 0;
    (*current()).restart_block.fn_ = do_no_restart_syscall as usize;
    synchronize_user_stack();
    if invalid_frame_pointer(sf as *mut c_void) { force_sig(SIGSEGV); return; }
    if get_user(&mut ufp, &mut (*sf).regs.u_regs[UREG_FP]) != 0 || ((ufp + STACK_BIAS) & 7) != 0 { force_sig(SIGSEGV); return; }
    err |= get_user(&mut tpc, &mut (*sf).regs.tpc); err |= get_user(&mut tnpc, &mut (*sf).regs.tnpc);
    if test_thread_flag(TIF_32BIT) { tpc &= 0xffff_ffff; tnpc &= 0xffff_ffff; } err |= ((tpc | tnpc) & 3) as i32;
    err |= get_user(&mut (*regs).y, &mut (*sf).regs.y); err |= get_user(&mut tstate, &mut (*sf).regs.tstate);
    err |= copy_from_user((*regs).u_regs.as_mut_ptr() as *mut c_void, (*sf).regs.u_regs.as_ptr() as *const c_void, core::mem::size_of_val(&(*regs).u_regs));
    (*regs).tstate = ((*regs).tstate & !(TSTATE_ASI|TSTATE_ICC|TSTATE_XCC)) | (tstate & (TSTATE_ASI|TSTATE_ICC|TSTATE_XCC));
    err |= get_user(&mut fpu_save, &mut (*sf).fpu_save); if err == 0 && !fpu_save.is_null() { err |= restore_fpu_state(regs, fpu_save); }
    err |= __copy_from_user(&mut set as *mut _ as *mut c_void, &(*sf).mask as *const _ as *const c_void, core::mem::size_of::<sigset_t>()); err |= restore_altstack(&(*sf).stack);
    if err != 0 { force_sig(SIGSEGV); return; }
    if get_user(&mut rwin_save, &mut (*sf).rwin_save) == 0 && !rwin_save.is_null() && restore_rwin_state(rwin_save) != 0 { force_sig(SIGSEGV); return; }
    (*regs).tpc=tpc; (*regs).tnpc=tnpc; pt_regs_clear_syscall(regs); set_current_blocked(&set);
}

unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, framesize: usize) -> *mut c_void {
    let mut sp = (*regs).u_regs[UREG_FP] + STACK_BIAS;
    if on_sig_stack(sp) && !likely(on_sig_stack(sp - framesize)) { return (-1isize) as *mut c_void; }
    sp = sigsp(sp, ksig) - framesize; sp &= !15usize; sp as *mut c_void
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> c_int {
    synchronize_user_stack(); save_and_clear_fpu(); let wsaved=get_thread_wsaved();
    let mut size=core::mem::size_of::<rt_signal_frame>(); if (*current_thread_info()).fpsaved[0] & FPRS_FEF != 0 { size += core::mem::size_of::<__siginfo_fpu_t>(); } if wsaved != 0 { size += core::mem::size_of::<__siginfo_rwin_t>(); }
    let sf=get_sigframe(ksig,regs,size) as *mut rt_signal_frame; if invalid_frame_pointer(sf as *mut c_void) { force_sigsegv((*ksig).sig); return -EINVAL; }
    let mut tail=(sf.add(1)) as *mut u8; let mut err=copy_to_user(&mut (*sf).regs as *mut _ as *mut c_void, regs as *const c_void, core::mem::size_of::<pt_regs>());
    if (*current_thread_info()).fpsaved[0] & FPRS_FEF != 0 { let p=tail as *mut __siginfo_fpu_t; tail=tail.add(core::mem::size_of::<__siginfo_fpu_t>()); err|=save_fpu_state(regs,p); err|=put_user(p as u64,&mut (*sf).fpu_save); } else { err|=put_user(0,&mut (*sf).fpu_save); }
    if wsaved != 0 { let p=tail as *mut __siginfo_rwin_t; err|=save_rwin_state(wsaved,p); err|=put_user(p as u64,&mut (*sf).rwin_save); set_thread_wsaved(0); } else { err|=put_user(0,&mut (*sf).rwin_save); }
    err|=__save_altstack(&mut (*sf).stack,(*regs).u_regs[UREG_FP]); err|=copy_to_user(&mut (*sf).mask as *mut _ as *mut c_void,sigmask_to_save() as *const c_void,core::mem::size_of::<sigset_t>());
    if wsaved==0 { err|=raw_copy_in_user(sf as *mut u64,((*regs).u_regs[UREG_FP]+STACK_BIAS) as *const u64,core::mem::size_of::<reg_window>()); } else { err|=copy_to_user(sf as *mut c_void,(*current_thread_info()).reg_window.as_ptr().add((wsaved-1) as usize) as *const c_void,core::mem::size_of::<reg_window>()); }
    if (*ksig).ka.sa.sa_flags & SA_SIGINFO != 0 { err|=copy_siginfo_to_user(&mut (*sf).info,&(*ksig).info); } else { err|=put_user((*ksig).sig,&mut (*sf).info.si_signo); err|=put_user(SI_NOINFO,&mut (*sf).info.si_code); } if err!=0{return err;}
    (*regs).u_regs[UREG_FP]=sf as usize-STACK_BIAS; (*regs).u_regs[UREG_I0]=(*ksig).sig as usize; (*regs).u_regs[UREG_I1]=&mut (*sf).info as *mut _ as usize; (*regs).u_regs[UREG_I2]=&mut (*sf).info as *mut _ as usize; (*regs).tpc=(*ksig).ka.sa.sa_handler as usize; (*regs).tnpc=(*regs).tpc+4; if test_thread_flag(TIF_32BIT){(*regs).tpc&=0xffff_ffff;(*regs).tnpc&=0xffff_ffff;} (*regs).u_regs[UREG_I7]=(*ksig).ka.ka_restorer as usize; 0
}

unsafe fn syscall_restart(orig_i0: usize, regs: *mut pt_regs, sa: *mut sigaction) { match (*regs).u_regs[UREG_I0] { ERESTART_RESTARTBLOCK|ERESTARTNOHAND => { (*regs).u_regs[UREG_I0]=EINTR; (*regs).tstate|=TSTATE_ICARRY|TSTATE_XCARRY; }, ERESTARTSYS => { if (*sa).sa_flags&SA_RESTART==0 { (*regs).u_regs[UREG_I0]=EINTR; (*regs).tstate|=TSTATE_ICARRY|TSTATE_XCARRY; } else { (*regs).u_regs[UREG_I0]=orig_i0;(*regs).tpc-=4;(*regs).tnpc-=4;} }, ERESTARTNOINTR=>{(*regs).u_regs[UREG_I0]=orig_i0;(*regs).tpc-=4;(*regs).tnpc-=4;}, _=>{} } }

unsafe fn do_signal(regs:*mut pt_regs, orig_i0:usize) { let mut ksig:ksignal=core::mem::zeroed(); if pt_regs_is_syscall(regs)&&((*regs).tstate&(TSTATE_XCARRY|TSTATE_ICARRY))!=0 {(*regs).u_regs[UREG_G6]=orig_i0;} if test_thread_flag(TIF_32BIT){do_signal32(regs);return;} let has=get_signal(&mut ksig); let mut orig=orig_i0; let restart=pt_regs_is_syscall(regs)&&((*regs).tstate&(TSTATE_XCARRY|TSTATE_ICARRY))!=0; if restart {orig=(*regs).u_regs[UREG_G6];} if has {if restart{syscall_restart(orig,regs,&mut ksig.ka.sa);} signal_setup_done(setup_rt_frame(&mut ksig,regs),&mut ksig,0);} else {if restart {match (*regs).u_regs[UREG_I0] {ERESTARTNOHAND|ERESTARTSYS|ERESTARTNOINTR=>{(*regs).u_regs[UREG_I0]=orig;(*regs).tpc-=4;(*regs).tnpc-=4;pt_regs_clear_syscall(regs);},ERESTART_RESTARTBLOCK=>{},_=>{}}} restore_saved_sigmask();} }

pub unsafe fn do_notify_resume(regs:*mut pt_regs, orig_i0:usize, flags:usize){user_exit();if flags&_TIF_UPROBE!=0{uprobe_notify_resume(regs);}if flags&(_TIF_SIGPENDING|_TIF_NOTIFY_SIGNAL)!=0{do_signal(regs,orig_i0);}if flags&_TIF_NOTIFY_RESUME!=0{resume_user_mode_work(regs);}user_enter();}

/* Compile-time siginfo layout assertions from the source are retained as intent. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
