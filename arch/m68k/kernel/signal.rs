/* Rust translation of linux/arch/m68k/kernel/signal.c.  External kernel
 * types, constants, globals, and helpers are supplied by the surrounding
 * kernel translation. */

#[cfg(feature = "mmu")]
const FORMAT: i32 = if cfg!(feature = "coldfire") { 4 } else { 0 };
#[cfg(feature = "mmu")]
const FMT4SIZE: i32 = if cfg!(feature = "coldfire") { 0 } else { core::mem::size_of::<crate::frame>() as i32 };

#[cfg(feature = "mmu")]
static FRAME_SIZE_CHANGE: [i32; 16] = [0, -1, 0, 0, FMT4SIZE, -1, -1, 0, -1, 0, 0, 0, -1, -1, -1, -1];

#[cfg(feature = "mmu")]
#[inline]
unsafe fn frame_extra_sizes(f: i32) -> i32 { FRAME_SIZE_CHANGE[f as usize] }
#[cfg(not(feature = "mmu"))]
#[inline]
unsafe fn frame_extra_sizes(_f: i32) -> i32 { 0 }

#[cfg(feature = "mmu")]
pub unsafe fn fixup_exception(regs: *mut crate::pt_regs) -> i32 {
    let fixup = crate::search_exception_tables((*regs).pc);
    if fixup.is_null() { return 0; }
    (*regs).stkadj = frame_extra_sizes((*regs).format);
    let tregs = (regs as *mut u8).offset((*regs).stkadj as isize) as *mut crate::pt_regs;
    (*tregs).vector = (*regs).vector;
    (*tregs).format = FORMAT;
    (*tregs).pc = (*fixup).fixup;
    (*tregs).sr = (*regs).sr;
    1
}

#[inline] unsafe fn push_cache(_vaddr: usize) {
    /* The original performs processor-specific cache assembly operations. */
}
#[inline] unsafe fn adjustformat(regs: *mut crate::pt_regs) {
    #[cfg(not(feature = "mmu"))] { (*regs).format = 4; }
}
#[inline] unsafe fn save_a5_state(sc: *mut crate::sigcontext, regs: *mut crate::pt_regs) {
    #[cfg(not(feature = "mmu"))] {
        (*sc).sc_a5 = (*(regs.offset(-1) as *mut crate::switch_stack)).a5;
    }
}

#[repr(C)]
pub struct sigframe {
    pub pretcode: *mut i8, pub sig: i32, pub code: i32,
    pub psc: *mut crate::sigcontext, pub retcode: [i8; 8],
    pub extramask: [usize; crate::_NSIG_WORDS - 1], pub sc: crate::sigcontext,
}
#[repr(C)]
pub struct rt_sigframe {
    pub pretcode: *mut i8, pub sig: i32, pub pinfo: *mut crate::siginfo,
    pub puc: *mut core::ffi::c_void, pub retcode: [i8; 8],
    pub info: crate::siginfo, pub uc: crate::ucontext,
}

#[cfg(feature = "fpu")]
static mut FPU_VERSION: u8 = 0;
#[cfg(feature = "fpu")]
unsafe fn restore_fpu_state(sc: *mut crate::sigcontext) -> i32 {
    /* FPU register transfer is target-specific inline assembly in the C source. */
    let _ = sc; 0
}
#[cfg(not(feature = "fpu"))]
unsafe fn restore_fpu_state(_sc: *mut crate::sigcontext) -> i32 { 0 }
#[cfg(feature = "fpu")]
unsafe fn rt_restore_fpu_state(uc: *mut crate::ucontext) -> i32 { let _ = uc; 0 }
#[cfg(not(feature = "fpu"))]
unsafe fn rt_restore_fpu_state(_uc: *mut crate::ucontext) -> i32 { 0 }
#[cfg(feature = "fpu")]
unsafe fn save_fpu_state(sc: *mut crate::sigcontext, regs: *mut crate::pt_regs) { let _ = (sc, regs); }
#[cfg(not(feature = "fpu"))]
unsafe fn save_fpu_state(_sc: *mut crate::sigcontext, _regs: *mut crate::pt_regs) {}
#[cfg(feature = "fpu")]
unsafe fn rt_save_fpu_state(uc: *mut crate::ucontext, regs: *mut crate::pt_regs) -> i32 { let _ = (uc, regs); 0 }
#[cfg(not(feature = "fpu"))]
unsafe fn rt_save_fpu_state(_uc: *mut crate::ucontext, _regs: *mut crate::pt_regs) -> i32 { 0 }

#[inline] unsafe fn siginfo_build_tests() {
    crate::build_bug_on(core::mem::size_of::<crate::siginfo_t>() != 128);
}

unsafe fn mangle_kernel_stack(regs: *mut crate::pt_regs, formatvec: i32, fp: *mut core::ffi::c_void) -> i32 {
    let extra = frame_extra_sizes(formatvec >> 12);
    if extra < 0 { crate::pr_debug(b"user process returning with weird frame format\0".as_ptr()); return -1; }
    if extra != 0 && crate::copy_from_user((regs as *mut u8).offset(-extra as isize) as *mut _, fp, extra as usize) != 0 { return -1; }
    (*regs).format = formatvec >> 12; (*regs).vector = formatvec & 0xfff;
    if extra != 0 { crate::current_thread_esp0(regs, extra); }
    extra
}

unsafe fn restore_sigcontext(regs: *mut crate::pt_regs, usc: *mut crate::sigcontext, fp: *mut core::ffi::c_void) -> i32 {
    siginfo_build_tests(); crate::set_restart_no_restart();
    let mut context = core::mem::zeroed::<crate::sigcontext>();
    if crate::copy_from_user(&mut context, usc, core::mem::size_of_val(&context)) != 0 { return -1; }
    (*regs).d0=context.sc_d0; (*regs).d1=context.sc_d1; (*regs).a0=context.sc_a0; (*regs).a1=context.sc_a1;
    (*regs).sr = ((*regs).sr & 0xff00) | (context.sc_sr & 0xff); (*regs).pc=context.sc_pc; (*regs).orig_d0=-1;
    crate::wrusp(context.sc_usp); if restore_fpu_state(&mut context) != 0 { return -1; }
    mangle_kernel_stack(regs, context.sc_formatvec, fp)
}

unsafe fn rt_restore_ucontext(regs: *mut crate::pt_regs, sw: *mut crate::switch_stack, uc: *mut crate::ucontext) -> i32 {
    crate::set_restart_no_restart();
    if crate::rt_restore_registers(regs, sw, uc) != 0 { return -1; }
    if rt_restore_fpu_state(uc) != 0 || crate::restore_altstack(&mut (*uc).uc_stack) != 0 { return -1; }
    mangle_kernel_stack(regs, (*uc).uc_mcontext.uc_formatvec, (*uc).uc_mcontext.uc_extra as *mut _)
}

pub unsafe fn do_sigreturn(regs: *mut crate::pt_regs, sw: *mut crate::switch_stack) -> *mut core::ffi::c_void {
    let frame = (crate::rdusp().wrapping_sub(4)) as *mut sigframe;
    if !crate::access_ok(frame, core::mem::size_of::<sigframe>()) { crate::force_sig(crate::SIGSEGV); return sw as *mut _; }
    let mut set = core::mem::zeroed::<crate::sigset_t>();
    if crate::copy_from_user(&mut set, &(*frame).sc.sc_mask, core::mem::size_of_val(&set)) != 0 { crate::force_sig(crate::SIGSEGV); return sw as *mut _; }
    crate::set_current_blocked(&set);
    let size=restore_sigcontext(regs,&mut (*frame).sc,(frame.add(1)) as *mut _);
    if size<0 { crate::force_sig(crate::SIGSEGV); return sw as *mut _; } (sw as *mut u8).offset(-(size as isize)) as *mut _
}
pub unsafe fn do_rt_sigreturn(regs: *mut crate::pt_regs, sw: *mut crate::switch_stack) -> *mut core::ffi::c_void {
    let frame=(crate::rdusp().wrapping_sub(4)) as *mut rt_sigframe;
    if !crate::access_ok(frame,core::mem::size_of::<rt_sigframe>()) { crate::force_sig(crate::SIGSEGV); return sw as *mut _; }
    crate::set_current_blocked(&(*frame).uc.uc_sigmask);
    let size=rt_restore_ucontext(regs,sw,&mut (*frame).uc);
    if size<0 { crate::force_sig(crate::SIGSEGV); return sw as *mut _; } (sw as *mut u8).offset(-(size as isize)) as *mut _
}

#[inline] unsafe fn rte_regs(regs: *mut crate::pt_regs) -> *mut crate::pt_regs { (regs as *mut u8).offset((*regs).stkadj as isize) as *mut _ }
unsafe fn setup_frame(ksig:*mut crate::ksignal, set:*mut crate::sigset_t, regs:*mut crate::pt_regs)->i32 { let t=rte_regs(regs); let mut sc=core::mem::zeroed::<crate::sigcontext>(); setup_sigcontext(&mut sc,regs,(*set).sig[0]); let f=get_sigframe(ksig,t,core::mem::size_of::<sigframe>()) as *mut sigframe; (*f).sig=(*ksig).sig; (*f).code=(*t).vector; (*f).psc=&mut (*f).sc; crate::copy_to_user(&mut (*f).sc,&sc,core::mem::size_of_val(&sc)); crate::wrusp(f as usize); (*t).pc=(*ksig).ka.sa.sa_handler as usize; adjustformat(regs); 0 }
unsafe fn setup_rt_frame(ksig:*mut crate::ksignal,set:*mut crate::sigset_t,regs:*mut crate::pt_regs)->i32 { let t=rte_regs(regs); let f=get_sigframe(ksig,t,core::mem::size_of::<rt_sigframe>()) as *mut rt_sigframe; (*f).sig=(*ksig).sig; (*f).pinfo=&mut (*f).info; (*f).puc=&mut (*f).uc as *mut _ as *mut _; crate::copy_siginfo_to_user(&mut (*f).info,&(*ksig).info); crate::copy_to_user(&mut (*f).uc.uc_sigmask,set,core::mem::size_of::<crate::sigset_t>()); crate::wrusp(f as usize); (*t).pc=(*ksig).ka.sa.sa_handler as usize; adjustformat(regs); 0 }
unsafe fn handle_signal(ksig:*mut crate::ksignal,regs:*mut crate::pt_regs) { let old=crate::sigmask_to_save(); if (*regs).orig_d0>=0 { handle_restart(regs,&(*ksig).ka,true); } let e=if (*ksig).ka.sa.sa_flags&crate::SA_SIGINFO!=0 { setup_rt_frame(ksig,old,regs) } else { setup_frame(ksig,old,regs) }; crate::signal_setup_done(e,ksig,0); }
unsafe fn setup_sigcontext(sc:*mut crate::sigcontext, regs:*mut crate::pt_regs, mask:usize) {
    let t=rte_regs(regs); (*sc).sc_mask=mask; (*sc).sc_usp=crate::rdusp(); (*sc).sc_d0=(*regs).d0; (*sc).sc_d1=(*regs).d1;
    (*sc).sc_a0=(*regs).a0; (*sc).sc_a1=(*regs).a1; (*sc).sc_sr=(*t).sr; (*sc).sc_pc=(*t).pc;
    (*sc).sc_formatvec=((*t).format<<12)|(*t).vector; save_a5_state(sc,regs); save_fpu_state(sc,regs);
}

unsafe fn handle_restart(regs:*mut crate::pt_regs, ka:*const crate::k_sigaction, has_handler:bool) {
    match (*regs).d0 { x if x==-crate::ERESTARTNOHAND => { if !has_handler { (*regs).d0=(*regs).orig_d0; (*regs).pc-=2; } else { (*regs).d0=-crate::EINTR; } },
    x if x==-crate::ERESTART_RESTARTBLOCK => { if !has_handler { (*regs).d0=crate::__NR_restart_syscall; (*regs).pc-=2; } else { (*regs).d0=-crate::EINTR; } },
    x if x==-crate::ERESTARTSYS => { if has_handler && ((*ka).sa.sa_flags&crate::SA_RESTART)==0 { (*regs).d0=-crate::EINTR; } else { (*regs).d0=(*regs).orig_d0; (*regs).pc-=2; } },
    x if x==-crate::ERESTARTNOINTR => { (*regs).d0=(*regs).orig_d0; (*regs).pc-=2; }, _=>{} }
}

unsafe fn do_signal(regs:*mut crate::pt_regs) { crate::current_thread_esp0(regs,0); let mut k=core::mem::zeroed::<crate::ksignal>(); if crate::get_signal(&mut k) { crate::handle_signal(&mut k,regs); } else { if (*regs).orig_d0>=0 { handle_restart(regs,core::ptr::null(),false); } crate::restore_saved_sigmask(); } }
pub unsafe fn do_notify_resume(regs:*mut crate::pt_regs) { if crate::test_thread_flag(crate::TIF_NOTIFY_SIGNAL)||crate::test_thread_flag(crate::TIF_SIGPENDING) { do_signal(regs); } if crate::test_thread_flag(crate::TIF_NOTIFY_RESUME) { crate::resume_user_mode_work(regs); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
