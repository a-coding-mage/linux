/* Translated from signal.c. External kernel types, constants, macros and
 * functions are intentionally referenced as supplied by the surrounding
 * kernel translation. */

use core::mem::{size_of, offset_of};

static mut SAVE_FP_CONTEXT: Option<unsafe fn(*mut core::ffi::c_void) -> i32> = None;
static mut RESTORE_FP_CONTEXT: Option<unsafe fn(*mut core::ffi::c_void) -> i32> = None;

#[repr(C)]
struct Sigframe {
    sf_ass: [u32; 4],
    sf_pad: [u32; 2],
    sf_sc: sigcontext,
    sf_mask: sigset_t,
    sf_extcontext: [u64; 0],
}

#[repr(C)]
struct RtSigframe {
    rs_ass: [u32; 4],
    rs_pad: [u32; 2],
    rs_info: siginfo,
    rs_uc: ucontext,
}

#[cfg(feature = "CONFIG_MIPS_FP_SUPPORT")]
unsafe fn copy_fp_to_sigcontext(sc: *mut core::ffi::c_void) -> i32 {
    let abi = (*current).thread.abi;
    let fpregs = (sc as *mut u8).add((*abi).off_sc_fpregs) as *mut u64;
    let csr = (sc as *mut u8).add((*abi).off_sc_fpc_csr) as *mut u32;
    let mut err = 0;
    let inc = if test_thread_flag(TIF_32BIT_FPREGS) { 2 } else { 1 };
    let mut i = 0;
    while i < NUM_FPU_REGS {
        err |= __put_user(get_fpr64(&(*current).thread.fpu.fpr[i], 0), fpregs.add(i));
        i += inc;
    }
    err |= __put_user((*current).thread.fpu.fcr31, csr);
    err
}

#[cfg(not(feature = "CONFIG_MIPS_FP_SUPPORT"))]
unsafe fn copy_fp_to_sigcontext(_sc: *mut core::ffi::c_void) -> i32 { 0 }

#[cfg(feature = "CONFIG_MIPS_FP_SUPPORT")]
unsafe fn copy_fp_from_sigcontext(sc: *mut core::ffi::c_void) -> i32 {
    let abi = (*current).thread.abi;
    let fpregs = (sc as *mut u8).add((*abi).off_sc_fpregs) as *mut u64;
    let csr = (sc as *mut u8).add((*abi).off_sc_fpc_csr) as *mut u32;
    let mut err = 0;
    let inc = if test_thread_flag(TIF_32BIT_FPREGS) { 2 } else { 1 };
    let mut i = 0;
    while i < NUM_FPU_REGS {
        let mut v = 0u64;
        err |= __get_user(&mut v, fpregs.add(i));
        set_fpr64(&mut (*current).thread.fpu.fpr[i], 0, v);
        i += inc;
    }
    err |= __get_user(&mut (*current).thread.fpu.fcr31, csr);
    err
}

#[cfg(not(feature = "CONFIG_MIPS_FP_SUPPORT"))]
unsafe fn copy_fp_from_sigcontext(_sc: *mut core::ffi::c_void) -> i32 { 0 }

unsafe fn save_hw_fp_context(sc: *mut core::ffi::c_void) -> i32 {
    let abi = (*current).thread.abi;
    let fpregs = (sc as *mut u8).add((*abi).off_sc_fpregs) as *mut u64;
    let csr = (sc as *mut u8).add((*abi).off_sc_fpc_csr) as *mut u32;
    _save_fp_context(fpregs, csr)
}
unsafe fn restore_hw_fp_context(sc: *mut core::ffi::c_void) -> i32 {
    let abi = (*current).thread.abi;
    let fpregs = (sc as *mut u8).add((*abi).off_sc_fpregs) as *mut u64;
    let csr = (sc as *mut u8).add((*abi).off_sc_fpc_csr) as *mut u32;
    _restore_fp_context(fpregs, csr)
}

unsafe fn sc_to_extcontext(sc: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let uc = container_of!(sc, ucontext, uc_mcontext);
    &mut (*uc).uc_extcontext as *mut _ as *mut core::ffi::c_void
}

#[cfg(feature = "CONFIG_CPU_HAS_MSA")]
unsafe fn save_msa_extcontext(buf: *mut core::ffi::c_void) -> i32 {
    let msa = buf as *mut msa_extcontext;
    if !thread_msa_context_live() { return 0; }
    preempt_disable();
    let mut err;
    if is_msa_enabled() {
        BUG_ON!(IS_ENABLED!(CONFIG_EVA));
        err = __put_user(read_msa_csr(), &mut (*msa).csr);
        err |= _save_msa_all_upper(&mut (*msa).wr);
        preempt_enable();
    } else {
        preempt_enable();
        err = __put_user((*current).thread.fpu.msacsr, &mut (*msa).csr);
        for i in 0..NUM_FPU_REGS {
            err |= __put_user(get_fpr64(&(*current).thread.fpu.fpr[i], 1), &mut (*msa).wr[i]);
        }
    }
    err |= __put_user(MSA_EXTCONTEXT_MAGIC, &mut (*msa).ext.magic);
    err |= __put_user(size_of::<msa_extcontext>(), &mut (*msa).ext.size);
    if err != 0 { -EFAULT } else { size_of::<msa_extcontext>() as i32 }
}

#[cfg(not(feature = "CONFIG_CPU_HAS_MSA"))]
unsafe fn save_msa_extcontext(_buf: *mut core::ffi::c_void) -> i32 { 0 }

#[cfg(feature = "CONFIG_CPU_HAS_MSA")]
unsafe fn restore_msa_extcontext(buf: *mut core::ffi::c_void, size: u32) -> i32 {
    let msa = buf as *mut msa_extcontext;
    if size as usize != size_of::<msa_extcontext>() { return -EINVAL; }
    let mut csr = 0u32;
    let mut err = __get_user(&mut csr, &(*msa).csr);
    if err != 0 { return err; }
    preempt_disable();
    if is_msa_enabled() {
        BUG_ON!(IS_ENABLED!(CONFIG_EVA));
        write_msa_csr(csr);
        err |= _restore_msa_all_upper(&(*msa).wr);
        preempt_enable();
    } else {
        preempt_enable();
        (*current).thread.fpu.msacsr = csr;
        for i in 0..NUM_FPU_REGS {
            let mut v = 0u64;
            err |= __get_user(&mut v, &(*msa).wr[i]);
            set_fpr64(&mut (*current).thread.fpu.fpr[i], 1, v);
        }
    }
    err
}

#[cfg(not(feature = "CONFIG_CPU_HAS_MSA"))]
unsafe fn restore_msa_extcontext(_buf: *mut core::ffi::c_void, _size: u32) -> i32 { SIGSYS }

unsafe fn save_extcontext(mut buf: *mut core::ffi::c_void) -> i32 {
    let mut sz = save_msa_extcontext(buf);
    if sz < 0 { return sz; }
    buf = (buf as *mut u8).add(sz as usize) as *mut _;
    if sz == 0 { return 0; }
    if __put_user(END_EXTCONTEXT_MAGIC, buf as *mut u32) != 0 { return -EFAULT; }
    sz + size_of::<u32>() as i32
}

unsafe fn restore_extcontext(mut buf: *mut core::ffi::c_void) -> i32 {
    let mut ext = extcontext { magic: 0, size: 0 };
    loop {
        let mut err = __get_user(&mut ext.magic, buf as *const u32);
        if err != 0 { return err; }
        if ext.magic == END_EXTCONTEXT_MAGIC { return 0; }
        err = __get_user(&mut ext.size, (buf as *mut u8).add(offset_of!(extcontext, size)) as *const u32);
        if err != 0 { return err; }
        err = match ext.magic { MSA_EXTCONTEXT_MAGIC => restore_msa_extcontext(buf, ext.size), _ => -EINVAL };
        if err != 0 { return err; }
        buf = (buf as *mut u8).add(ext.size as usize) as *mut _;
    }
}

pub unsafe fn protected_save_fp_context(sc: *mut core::ffi::c_void) -> i32 {
    let abi = (*current).thread.abi;
    let fpregs = (sc as *mut u8).add((*abi).off_sc_fpregs) as *mut u64;
    let csr = (sc as *mut u8).add((*abi).off_sc_fpc_csr) as *mut u32;
    let used_math = (sc as *mut u8).add((*abi).off_sc_used_math) as *mut u32;
    let mut used = if used_math() { USED_FP } else { 0 };
    let mut err = 0;
    if used != 0 {
        if !test_thread_flag(TIF_32BIT_FPREGS) { used |= USED_FR1; }
        if test_thread_flag(TIF_HYBRID_FPREGS) { used |= USED_HYBRID_FPRS; }
        if IS_ENABLED!(CONFIG_EVA) { lose_fpu(1); }
        loop {
            lock_fpu_owner();
            err = if is_fpu_owner() { SAVE_FP_CONTEXT.unwrap()(sc) } else { copy_fp_to_sigcontext(sc) };
            unlock_fpu_owner();
            if likely!(err == 0) { break; }
            err = __put_user(0, fpregs) | __put_user(0, fpregs.add(31)) | __put_user(0, csr);
            if err != 0 { return err; }
        }
    }
    let ext_sz = save_extcontext(sc_to_extcontext(sc));
    if ext_sz < 0 { return ext_sz; }
    if ext_sz != 0 { used |= USED_EXTCONTEXT; }
    __put_user(used, used_math)
}

pub unsafe fn protected_restore_fp_context(sc: *mut core::ffi::c_void) -> i32 {
    let abi = (*current).thread.abi;
    let fpregs = (sc as *mut u8).add((*abi).off_sc_fpregs) as *mut u64;
    let csr = (sc as *mut u8).add((*abi).off_sc_fpc_csr) as *mut u32;
    let used_math = (sc as *mut u8).add((*abi).off_sc_used_math) as *mut u32;
    let mut used = 0u32;
    let err = __get_user(&mut used, used_math);
    conditional_used_math(used & USED_FP);
    if err != 0 || used & USED_FP == 0 { lose_fpu(0); }
    if err != 0 || used & USED_FP == 0 { return err; }
    let sig = fpcsr_pending(csr);
    if sig < 0 { return sig; }
    if IS_ENABLED!(CONFIG_EVA) { lose_fpu(0); }
    let mut e;
    loop {
        lock_fpu_owner();
        e = if is_fpu_owner() { RESTORE_FP_CONTEXT.unwrap()(sc) } else { copy_fp_from_sigcontext(sc) };
        unlock_fpu_owner();
        if likely!(e == 0) { break; }
        let mut tmp = 0u64;
        e = __get_user(&mut tmp, fpregs) | __get_user(&mut tmp, fpregs.add(31)) | __get_user(&mut tmp, csr);
        if e != 0 { break; }
    }
    if e == 0 && used & USED_EXTCONTEXT != 0 { e = restore_extcontext(sc_to_extcontext(sc)); }
    if e != 0 { e } else { sig }
}

pub unsafe fn setup_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext) -> i32 {
    let mut err = __put_user((*regs).cp0_epc, &mut (*sc).sc_pc) | __put_user(0, &mut (*sc).sc_regs[0]);
    for i in 1..32 { err |= __put_user((*regs).regs[i], &mut (*sc).sc_regs[i]); }
    #[cfg(feature = "CONFIG_CPU_HAS_SMARTMIPS")] { err |= __put_user((*regs).acx, &mut (*sc).sc_acx); }
    err |= __put_user((*regs).hi, &mut (*sc).sc_mdhi) | __put_user((*regs).lo, &mut (*sc).sc_mdlo);
    if cpu_has_dsp { err |= __put_user(mfhi1(), &mut (*sc).sc_hi1) | __put_user(mflo1(), &mut (*sc).sc_lo1) | __put_user(mfhi2(), &mut (*sc).sc_hi2) | __put_user(mflo2(), &mut (*sc).sc_lo2) | __put_user(mfhi3(), &mut (*sc).sc_hi3) | __put_user(mflo3(), &mut (*sc).sc_lo3) | __put_user(rddsp(DSP_MASK), &mut (*sc).sc_dsp); }
    err | protected_save_fp_context(sc as *mut _)
}

unsafe fn extcontext_max_size() -> usize {
    let mut sz = 0;
    if thread_msa_context_live() { sz += size_of::<msa_extcontext>(); }
    if sz != 0 { sz += size_of::<u32>(); }
    sz
}

pub unsafe fn fpcsr_pending(fpcsr: *mut u32) -> i32 {
    let mut csr = 0; let mut err = __get_user(&mut csr, fpcsr);
    let enabled = FPU_CSR_UNI_X | ((csr & FPU_CSR_ALL_E) << 5); let mut sig = 0;
    if csr & enabled != 0 { csr &= !enabled; err |= __put_user(csr, fpcsr); sig = SIGFPE; }
    if err != 0 { err } else { sig }
}

pub unsafe fn restore_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext) -> i32 {
    (*current).restart_block.fn_ = do_no_restart_syscall;
    let mut err = __get_user(&mut (*regs).cp0_epc, &(*sc).sc_pc);
    #[cfg(feature = "CONFIG_CPU_HAS_SMARTMIPS")] { err |= __get_user(&mut (*regs).acx, &(*sc).sc_acx); }
    err |= __get_user(&mut (*regs).hi, &(*sc).sc_mdhi) | __get_user(&mut (*regs).lo, &(*sc).sc_mdlo);
    if cpu_has_dsp { let mut t=0; err |= __get_user(&mut t,&(*sc).sc_hi1); mthi1(t); err |= __get_user(&mut t,&(*sc).sc_lo1); mtlo1(t); err |= __get_user(&mut t,&(*sc).sc_hi2); mthi2(t); err |= __get_user(&mut t,&(*sc).sc_lo2); mtlo2(t); err |= __get_user(&mut t,&(*sc).sc_hi3); mthi3(t); err |= __get_user(&mut t,&(*sc).sc_lo3); mtlo3(t); err |= __get_user(&mut t,&(*sc).sc_dsp); wrdsp(t,DSP_MASK); }
    for i in 1..32 { err |= __get_user(&mut (*regs).regs[i], &(*sc).sc_regs[i]); }
    if err != 0 { err } else { protected_restore_fp_context(sc as *mut _) }
}

#[cfg(feature = "CONFIG_WAR_ICACHE_REFILLS")]
const SIGMASK: usize = !(cpu_icache_line_size() - 1);
#[cfg(not(feature = "CONFIG_WAR_ICACHE_REFILLS"))]
const SIGMASK: usize = ALMASK;

pub unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, mut frame_size: usize) -> *mut core::ffi::c_void {
    frame_size += extcontext_max_size(); let mut sp = (*regs).regs[29];
    if on_sig_stack(sp) && !likely!(on_sig_stack(sp - frame_size)) { return usize::MAX as *mut _; }
    sp -= 32; sp = sigsp(sp, ksig); ((sp - frame_size) & SIGMASK) as *mut _
}

#[cfg(feature = "CONFIG_TRAD_SIGNALS")]
pub unsafe fn sys_sigsuspend(uset: *mut sigset_t) -> isize { sys_rt_sigsuspend(uset, size_of::<sigset_t>()) }

#[cfg(feature = "CONFIG_TRAD_SIGNALS")]
pub unsafe fn sys_sigaction(sig: i32, act: *const sigaction, oact: *mut sigaction) -> i32 {
    let mut new_ka = core::mem::zeroed(); let mut old_ka = core::mem::zeroed(); let mut err = 0;
    if !act.is_null() {
        if !access_ok(act, size_of::<sigaction>()) { return -EFAULT; }
        let mut mask = 0; err |= __get_user(&mut new_ka.sa.sa_handler, &(*act).sa_handler); err |= __get_user(&mut new_ka.sa.sa_flags, &(*act).sa_flags); err |= __get_user(&mut mask, &(*act).sa_mask.sig[0]);
        if err != 0 { return -EFAULT; } siginitset(&mut new_ka.sa.sa_mask, mask);
    }
    let ret = do_sigaction(sig, if act.is_null(){core::ptr::null()}else{&mut new_ka}, if oact.is_null(){core::ptr::null_mut()}else{&mut old_ka});
    if ret == 0 && !oact.is_null() { if !access_ok(oact,size_of::<sigaction>()){return -EFAULT;} err |= __put_user(old_ka.sa.sa_flags,&mut (*oact).sa_flags)|__put_user(old_ka.sa.sa_handler,&mut (*oact).sa_handler)|__put_user(old_ka.sa.sa_mask.sig[0],&mut (*oact).sa_mask.sig[0]); for i in 1..4 { err |= __put_user(0,&mut (*oact).sa_mask.sig[i]); } if err != 0{return -EFAULT;} }
    ret
}

pub unsafe fn sys_rt_sigreturn() { let regs=current_pt_regs(); let frame=(*regs).regs[29] as *mut RtSigframe; let mut set=core::mem::zeroed(); if !access_ok(frame,size_of::<RtSigframe>()) || __copy_from_user(&mut set,&(*frame).rs_uc.uc_sigmask,size_of::<sigset_t>()) != 0 { force_sig(SIGSEGV); return; } set_current_blocked(&set); let sig=restore_sigcontext(regs,&mut (*frame).rs_uc.uc_mcontext); if sig<0 {force_sig(SIGSEGV);return;} if sig!=0 {force_sig(sig);} if restore_altstack(&mut (*frame).rs_uc.uc_stack)!=0 {force_sig(SIGSEGV);return;} }

unsafe fn setup_rt_frame(sig_return:*mut core::ffi::c_void, ksig:*mut ksignal, regs:*mut pt_regs, set:*mut sigset_t)->i32 { let frame=get_sigframe(ksig,regs,size_of::<RtSigframe>()) as *mut RtSigframe; if !access_ok(frame,size_of::<RtSigframe>()){return -EFAULT;} if copy_siginfo_to_user(&mut (*frame).rs_info,&(*ksig).info)!=0{return -EFAULT;} if __put_user(0,&mut (*frame).rs_uc.uc_flags)!=0{return -EFAULT;} if __put_user(core::ptr::null_mut(),&mut (*frame).rs_uc.uc_link)!=0{return -EFAULT;} if __save_altstack(&mut (*frame).rs_uc.uc_stack,(*regs).regs[29])!=0{return -EFAULT;} if setup_sigcontext(regs,&mut (*frame).rs_uc.uc_mcontext)!=0{return -EFAULT;} if __copy_to_user(&mut (*frame).rs_uc.uc_sigmask,set,size_of::<sigset_t>())!=0{return -EFAULT;} (*regs).regs[4]=(*ksig).sig;(*regs).regs[5]=&mut (*frame).rs_info as usize;(*regs).regs[6]=&mut (*frame).rs_uc as usize;(*regs).regs[29]=frame as usize;(*regs).regs[31]=sig_return as usize;(*regs).cp0_epc=(*regs).regs[25]=(*ksig).ka.sa.sa_handler as usize;0 }

pub unsafe fn do_notify_resume(regs:*mut pt_regs,_unused:*mut core::ffi::c_void,flags:u32){local_irq_enable();user_exit();if flags&_TIF_UPROBE!=0{uprobe_notify_resume(regs);}if flags&(_TIF_SIGPENDING|_TIF_NOTIFY_SIGNAL)!=0{let mut k=core::mem::zeroed();if get_signal(&mut k){handle_signal(&mut k,regs);}}if flags&_TIF_NOTIFY_RESUME!=0{resume_user_mode_work(regs);}user_enter();}

unsafe fn handle_signal(_ksig:*mut ksignal,_regs:*mut pt_regs){}
unsafe fn signal_setup()->i32{if cpu_has_fpu{SAVE_FP_CONTEXT=Some(save_hw_fp_context);RESTORE_FP_CONTEXT=Some(restore_hw_fp_context);}else{SAVE_FP_CONTEXT=Some(copy_fp_to_sigcontext);RESTORE_FP_CONTEXT=Some(copy_fp_from_sigcontext);}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
