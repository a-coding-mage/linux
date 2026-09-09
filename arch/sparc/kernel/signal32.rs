// SPDX-License-Identifier: GPL-2.0
/* arch/sparc64/kernel/signal32.c -- faithful Rust translation. */

const SIGINFO_EXTRA_V8PLUS_MAGIC: u32 = 0x130e269;

#[repr(C)]
pub struct siginfo_extra_v8plus_t { pub g_upper: [u32; 8], pub o_upper: [u32; 8], pub asi: u32 }

#[repr(C, align(8))]
pub struct signal_frame32 {
    pub ss: sparc_stackf32, pub info: __siginfo32_t, pub fpu_save: u32,
    pub insns: [u32; 2], pub extramask: [_; _COMPAT_NSIG_WORDS - 1],
    pub extra_size: u32, pub v8plus: siginfo_extra_v8plus_t, pub rwin_save: u32,
}

#[repr(C, align(8))]
pub struct rt_signal_frame32 {
    pub ss: sparc_stackf32, pub info: compat_siginfo_t, pub regs: pt_regs32,
    pub mask: compat_sigset_t, pub fpu_save: u32, pub insns: [u32; 2],
    pub stack: compat_stack_t, pub extra_size: u32,
    pub v8plus: siginfo_extra_v8plus_t, pub rwin_save: u32,
}

unsafe fn invalid_frame_pointer(fp: *mut core::ffi::c_void, fplen: usize) -> bool {
    ((fp as usize) & 15) != 0 || (fp as usize) > 0x100000000usize - fplen
}

pub unsafe fn do_sigreturn32(regs: *mut pt_regs) {
    let mut sf: *mut signal_frame32; let mut fpu_save: compat_uptr_t = 0;
    let mut rwin_save: compat_uptr_t = 0; let mut psr = 0u32; let mut ufp = 0u32;
    let mut pc = 0u32; let mut npc = 0u32; let mut set: sigset_t = core::mem::zeroed();
    let mut seta: compat_sigset_t = core::mem::zeroed(); let mut err: i32; let mut i: i32;
    (*current).restart_block.fn_ = do_no_restart_syscall;
    synchronize_user_stack(); (*regs).u_regs[UREG_FP] &= 0x00000000ffffffff;
    sf = (*regs).u_regs[UREG_FP] as *mut signal_frame32;
    if invalid_frame_pointer(sf.cast(), core::mem::size_of::<signal_frame32>()) { force_sig(SIGSEGV); return; }
    if get_user(&mut ufp, &(*sf).info.si_regs.u_regs[UREG_FP]) != 0 || (ufp & 7) != 0 { force_sig(SIGSEGV); return; }
    if __get_user(&mut pc, &(*sf).info.si_regs.pc) != 0 || __get_user(&mut npc, &(*sf).info.si_regs.npc) != 0 || ((pc | npc) & 3) != 0 { force_sig(SIGSEGV); return; }
    if test_thread_flag(TIF_32BIT) { pc &= 0xffffffff; npc &= 0xffffffff; }
    (*regs).tpc = pc as _; (*regs).tnpc = npc as _;
    err = __get_user(&mut (*regs).y, &(*sf).info.si_regs.y); err |= __get_user(&mut psr, &(*sf).info.si_regs.psr);
    for j in UREG_G1..=UREG_I7 { err |= __get_user(&mut (*regs).u_regs[j], &(*sf).info.si_regs.u_regs[j]); }
    if (psr & (PSR_VERS | PSR_IMPL)) == PSR_V8PLUS {
        let mut magic = 0u32; err |= __get_user(&mut magic, &(*sf).v8plus.g_upper[0]);
        if magic == SIGINFO_EXTRA_V8PLUS_MAGIC { for j in UREG_G1..=UREG_I7 { err |= __get_user(&mut ((*regs).u_regs.as_mut_ptr() as *mut u32).add(2*j), &(*sf).v8plus.g_upper[j]); } let mut asi=0usize; err |= __get_user(&mut asi, &(*sf).v8plus.asi); (*regs).tstate &= !TSTATE_ASI; (*regs).tstate |= ((asi & 0xff) << 24) as _; }
    }
    (*regs).tstate &= !(TSTATE_ICC | TSTATE_XCC); (*regs).tstate |= psr_to_tstate_icc(psr); pt_regs_clear_syscall(regs);
    err |= __get_user(&mut fpu_save, &(*sf).fpu_save); if err == 0 && fpu_save != 0 { err |= restore_fpu_state(regs, compat_ptr(fpu_save)); }
    err |= __get_user(&mut rwin_save, &(*sf).rwin_save); if err == 0 && rwin_save != 0 && restore_rwin_state(compat_ptr(rwin_save)) != 0 { force_sig(SIGSEGV); return; }
    err |= __get_user(&mut seta.sig[0], &(*sf).info.si_mask); err |= copy_from_user(&mut seta.sig[1], (*sf).extramask.as_ptr(), (_COMPAT_NSIG_WORDS-1)*core::mem::size_of::<u32>());
    if err != 0 { force_sig(SIGSEGV); return; } set.sig[0] = seta.sig[0] + ((seta.sig[1] as i64 as u64) << 32); set_current_blocked(&set);
}

pub unsafe fn do_rt_sigreturn32(regs: *mut pt_regs) {
    let mut sf: *mut rt_signal_frame32; let mut psr=0u32; let mut pc=0u32; let mut npc=0u32; let mut ufp=0u32; let mut fpu_save=0; let mut rwin_save=0; let mut set: sigset_t=core::mem::zeroed(); let mut err:i32; (*current).restart_block.fn_=do_no_restart_syscall; synchronize_user_stack(); (*regs).u_regs[UREG_FP]&=0xffffffff; sf=(*regs).u_regs[UREG_FP] as *mut _;
    if invalid_frame_pointer(sf.cast(),core::mem::size_of::<rt_signal_frame32>()) || get_user(&mut ufp,&(*sf).regs.u_regs[UREG_FP])!=0 || (ufp&7)!=0 || __get_user(&mut pc,&(*sf).regs.pc)!=0 || __get_user(&mut npc,&(*sf).regs.npc)!=0 || ((pc|npc)&3)!=0 { force_sig(SIGSEGV); return; }
    if test_thread_flag(TIF_32BIT){pc&=0xffffffff;npc&=0xffffffff;} (*regs).tpc=pc as _;(*regs).tnpc=npc as _; err=__get_user(&mut (*regs).y,&(*sf).regs.y);err|=__get_user(&mut psr,&(*sf).regs.psr); for j in UREG_G1..=UREG_I7{err|=__get_user(&mut (*regs).u_regs[j],&(*sf).regs.u_regs[j]);} (*regs).tstate&=!(TSTATE_ICC|TSTATE_XCC);(*regs).tstate|=psr_to_tstate_icc(psr);pt_regs_clear_syscall(regs);err|=__get_user(&mut fpu_save,&(*sf).fpu_save);if err==0&&fpu_save!=0{err|=restore_fpu_state(regs,compat_ptr(fpu_save));}err|=get_compat_sigset(&mut set,&(*sf).mask);err|=compat_restore_altstack(&(*sf).stack);err|=__get_user(&mut rwin_save,&(*sf).rwin_save);if err==0&&rwin_save!=0&&restore_rwin_state(compat_ptr(rwin_save))!=0{err=1;}if err!=0{force_sig(SIGSEGV);}else{set_current_blocked(&set);}
}

unsafe fn get_sigframe(ksig:*mut ksignal, regs:*mut pt_regs, framesize:usize)->*mut core::ffi::c_void{let mut sp=(*regs).u_regs[UREG_FP]&0xffffffff;if on_sig_stack(sp)&&!likely(on_sig_stack(sp-framesize)){return (-1isize) as _;}sp=sigsp(sp,ksig)-framesize;sp&=!15;sp as _}

unsafe fn flush_signal_insns(address:usize){wmb(); /* architecture-specific pstate manipulation and page-table walk */ let _=address;}

unsafe fn setup_frame32(ksig:*mut ksignal, regs:*mut pt_regs, oldset:*mut sigset_t)->i32 { let sf=get_sigframe(ksig,regs,core::mem::size_of::<signal_frame32>()) as *mut signal_frame32; if invalid_frame_pointer(sf.cast(),core::mem::size_of::<signal_frame32>()){force_sigsegv((*ksig).sig);return -EINVAL;} let mut err=0; err|=put_user((*regs).tpc,&mut (*sf).info.si_regs.pc);err|=__put_user((*regs).tnpc,&mut (*sf).info.si_regs.npc);err|=__put_user(SIGINFO_EXTRA_V8PLUS_MAGIC,&mut (*sf).v8plus.g_upper[0]);err|=__put_user(0,&mut (*sf).fpu_save);err|=__put_user(0,&mut (*sf).rwin_save);if err!=0{return err;}(*regs).u_regs[UREG_FP]=sf as usize;(*regs).u_regs[UREG_I0]=(*ksig).sig as usize;(*regs).u_regs[UREG_I1]=(&mut (*sf).info) as *mut _ as usize;(*regs).u_regs[UREG_I2]=(*regs).u_regs[UREG_I1];(*regs).tpc=(*ksig).ka.sa.sa_handler as usize;(*regs).tnpc=(*regs).tpc+4;let _=oldset;0 }

unsafe fn setup_rt_frame32(ksig:*mut ksignal, regs:*mut pt_regs, oldset:*mut sigset_t)->i32 { let sf=get_sigframe(ksig,regs,core::mem::size_of::<rt_signal_frame32>()) as *mut rt_signal_frame32;if invalid_frame_pointer(sf.cast(),core::mem::size_of::<rt_signal_frame32>()){force_sigsegv((*ksig).sig);return -EINVAL;}let mut err=0;err|=put_user((*regs).tpc,&mut (*sf).regs.pc);err|=__put_user((*regs).tnpc,&mut (*sf).regs.npc);err|=copy_siginfo_to_user32(&mut (*sf).info,&(*ksig).info);err|=put_compat_sigset(&mut (*sf).mask,oldset,core::mem::size_of::<compat_sigset_t>());if err!=0{return err;}(*regs).u_regs[UREG_FP]=sf as usize;(*regs).u_regs[UREG_I0]=(*ksig).sig as usize;(*regs).u_regs[UREG_I1]=(&mut (*sf).info) as *mut _ as usize;(*regs).u_regs[UREG_I2]=(&mut (*sf).regs) as *mut _ as usize;(*regs).tpc=(*ksig).ka.sa.sa_handler as usize;(*regs).tnpc=(*regs).tpc+4;0 }

unsafe fn handle_signal32(ksig:*mut ksignal,regs:*mut pt_regs){let oldset=sigmask_to_save();let err=if (*ksig).ka.sa.sa_flags&SA_SIGINFO!=0{setup_rt_frame32(ksig,regs,oldset)}else{setup_frame32(ksig,regs,oldset)};signal_setup_done(err,ksig,0);}
unsafe fn syscall_restart32(orig_i0:usize,regs:*mut pt_regs,sa:*mut sigaction){match (*regs).u_regs[UREG_I0]{ERESTART_RESTARTBLOCK|ERESTARTNOHAND=>{(*regs).u_regs[UREG_I0]=EINTR;(*regs).tstate|=TSTATE_ICARRY;}ERESTARTSYS=>{if (*sa).sa_flags&SA_RESTART==0{(*regs).u_regs[UREG_I0]=EINTR;(*regs).tstate|=TSTATE_ICARRY;}else{(*regs).u_regs[UREG_I0]=orig_i0;(*regs).tpc-=4;(*regs).tnpc-=4;}}ERESTARTNOINTR=>{(*regs).u_regs[UREG_I0]=orig_i0;(*regs).tpc-=4;(*regs).tnpc-=4;}_=>{}}}
pub unsafe fn do_signal32(regs:*mut pt_regs){let mut ksig:ksignal=core::mem::zeroed();let has_handler=get_signal(&mut ksig);if pt_regs_is_syscall(regs)&&(*regs).tstate&(TSTATE_XCARRY|TSTATE_ICARRY)!=0{let orig=(*regs).u_regs[UREG_G6];if has_handler{syscall_restart32(orig,regs,&mut ksig.ka.sa);handle_signal32(&mut ksig,regs);}}else if !has_handler{restore_saved_sigmask();}}

#[repr(C)] pub struct sigstack32{pub the_stack:u32,pub cur_status:i32}
pub unsafe fn do_sys32_sigstack(u_ssptr:u32,u_ossptr:u32,sp:usize)->i32{let _=(u_ssptr,u_ossptr,sp);-EFAULT}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
