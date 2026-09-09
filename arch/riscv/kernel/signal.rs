// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Sunplus Core Technology Co., Ltd.
 *  Chen Liqin <liqin.chen@sunplusct.com>
 *  Lennox Wu <lennox.wu@sunplusct.com>
 * Copyright (C) 2012 Regents of the University of California
 */

// Linux and architecture headers provide the external types, constants, macros,
// functions, and configuration symbols referenced below.

pub static mut signal_minsigstksz: c_ulong = 0;

extern "C" {
    pub static mut __user_rt_sigreturn: [u32; 2];
}

static mut riscv_v_sc_size: usize = 0;
static mut riscv_zicfiss_sc_size: usize = 0;

#[repr(C)]
struct rt_sigframe {
    info: siginfo,
    uc: ucontext,
    // Present only when CONFIG_MMU is disabled.
    sigreturn_code: [u32; 2],
}

#[cfg(feature = "CONFIG_FPU")]
unsafe fn restore_fp_state(regs: *mut pt_regs, sc_fpregs: *mut __riscv_fp_state) -> c_long {
    let state: *mut __riscv_d_ext_state = &mut (*sc_fpregs).d;
    let err = __copy_from_user(&mut (*current).thread.fstate as *mut _, state as *const _, core::mem::size_of::<__riscv_d_ext_state>());
    if unlikely(err != 0) { return err; }
    fstate_restore(current, regs);
    0
}

#[cfg(feature = "CONFIG_FPU")]
unsafe fn save_fp_state(regs: *mut pt_regs, sc_fpregs: *mut __riscv_fp_state) -> c_long {
    let state: *mut __riscv_d_ext_state = &mut (*sc_fpregs).d;
    fstate_save(current, regs);
    __copy_to_user(state as *mut _, &(*current).thread.fstate as *const _, core::mem::size_of::<__riscv_d_ext_state>())
}

#[cfg(not(feature = "CONFIG_FPU"))]
unsafe fn save_fp_state(_: *mut pt_regs, _: *mut __riscv_fp_state) -> c_long { 0 }
#[cfg(not(feature = "CONFIG_FPU"))]
unsafe fn restore_fp_state(_: *mut pt_regs, _: *mut __riscv_fp_state) -> c_long { 0 }

unsafe fn save_v_state(regs: *mut pt_regs, sc_vec: *mut c_void) -> c_long {
    if !IS_ENABLED(CONFIG_RISCV_ISA_V) || !((has_vector() || has_xtheadvector()) && riscv_v_vstate_query(regs)) { return 0; }
    let state = sc_vec as *mut __sc_riscv_v_state;
    let datap = state.add(1) as *mut c_void;
    WARN_ON(!IS_ALIGNED(datap as c_ulong, 16));
    get_cpu_vector_context();
    riscv_v_vstate_save(&mut (*current).thread.vstate, regs);
    put_cpu_vector_context();
    let mut err = __copy_to_user(&mut (*state).v_state as *mut _, &(*current).thread.vstate as *const _, offset_of!(__riscv_v_ext_state, datap));
    err |= __put_user(datap, &mut (*state).v_state.datap);
    err |= __copy_to_user(datap, (*current).thread.vstate.datap, riscv_v_vsize);
    if unlikely(err != 0) { return -EFAULT; }
    riscv_v_sc_size as c_long
}

unsafe fn __restore_v_state(regs: *mut pt_regs, sc_vec: *mut c_void) -> c_long {
    let state = sc_vec as *mut __sc_riscv_v_state;
    riscv_v_vstate_set_restore(current, regs);
    let mut err = __copy_from_user(&mut (*current).thread.vstate as *mut _, &(*state).v_state as *const _, offset_of!(__riscv_v_ext_state, datap));
    if unlikely(err != 0) { return err; }
    let mut datap: *mut c_void = core::ptr::null_mut();
    err = __get_user(&mut datap, &(*state).v_state.datap);
    if unlikely(err != 0) { return err; }
    copy_from_user((*current).thread.vstate.datap, datap, riscv_v_vsize)
}

unsafe fn save_cfiss_state(_: *mut pt_regs, sc_cfi: *mut c_void) -> c_long {
    let state = sc_cfi as *mut __sc_riscv_cfi_state;
    if !is_shstk_enabled(current) { return 0; }
    let mut ss_ptr: c_ulong = 0;
    let mut err = save_user_shstk(current, &mut ss_ptr);
    err |= __put_user(ss_ptr, &mut (*state).ss_ptr);
    if unlikely(err != 0) { return -EFAULT; }
    riscv_zicfiss_sc_size as c_long
}

unsafe fn __restore_cfiss_state(_: *mut pt_regs, sc_cfi: *mut c_void) -> c_long {
    let state = sc_cfi as *mut __sc_riscv_cfi_state;
    let mut ss_ptr: c_ulong = 0;
    let err = __copy_from_user(&mut ss_ptr as *mut _, &(*state).ss_ptr as *const _, core::mem::size_of::<c_ulong>());
    if unlikely(err != 0) { return err; }
    restore_user_shstk(current, ss_ptr)
}

#[repr(C)]
struct arch_ext_priv {
    magic: u32,
    save: Option<unsafe extern "C" fn(*mut pt_regs, *mut c_void) -> c_long>,
}

static mut arch_ext_list: [arch_ext_priv; 2] = [
    arch_ext_priv { magic: RISCV_V_MAGIC, save: Some(save_v_state) },
    arch_ext_priv { magic: RISCV_ZICFISS_MAGIC, save: Some(save_cfiss_state) },
];
static nr_arch_exts: usize = 2;

unsafe fn restore_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext) -> c_long {
    let mut sc_ext_ptr = &mut (*sc).sc_extdesc.hdr as *mut __riscv_ctx_hdr as *mut c_void;
    let mut rsvd: u32 = 0;
    let mut err = __copy_from_user(regs as *mut _, &(*sc).sc_regs as *const _, core::mem::size_of_val(&(*sc).sc_regs));
    if unlikely(err != 0) { return err; }
    if has_fpu() { err = restore_fp_state(regs, &mut (*sc).sc_fpregs); if unlikely(err != 0) { return err; } }
    err = __get_user(&mut rsvd, &(*sc).sc_extdesc.reserved);
    if unlikely(err != 0) { return err; }
    if unlikely(rsvd != 0) { return -EINVAL; }
    while err == 0 {
        let mut magic: u32 = 0; let mut size: u32 = 0;
        let head = sc_ext_ptr as *mut __riscv_ctx_hdr;
        err |= __get_user(&mut magic, &(*head).magic); err |= __get_user(&mut size, &(*head).size);
        if unlikely(err != 0) { return err; }
        sc_ext_ptr = sc_ext_ptr.add(core::mem::size_of::<__riscv_ctx_hdr>());
        match magic {
            END_MAGIC => { if size != END_HDR_SIZE { return -EINVAL; } return 0; }
            RISCV_V_MAGIC => { if !(has_vector() || has_xtheadvector()) || !riscv_v_vstate_query(regs) || size as usize != riscv_v_sc_size { return -EINVAL; } err = __restore_v_state(regs, sc_ext_ptr); }
            RISCV_ZICFISS_MAGIC => { if !is_shstk_enabled(current) || size as usize != riscv_zicfiss_sc_size { return -EINVAL; } err = __restore_cfiss_state(regs, sc_ext_ptr); }
            _ => return -EINVAL,
        }
        sc_ext_ptr = (head as *mut u8).add(size as usize) as *mut c_void;
    }
    err
}

unsafe fn get_rt_frame_size(cal_all: bool) -> usize {
    let mut total_context_size = 0;
    if has_vector() || has_xtheadvector() { if cal_all || riscv_v_vstate_query(task_pt_regs(current)) { total_context_size += riscv_v_sc_size; } }
    if is_shstk_enabled(current) { total_context_size += riscv_zicfiss_sc_size; }
    if total_context_size != 0 { total_context_size += core::mem::size_of::<__riscv_ctx_hdr>(); }
    (core::mem::size_of::<rt_sigframe>() + total_context_size + 15) & !15
}

#[no_mangle]
pub unsafe extern "C" fn rt_sigreturn() -> c_long {
    let regs = current_pt_regs();
    let frame_size = get_rt_frame_size(false);
    (*current).restart_block.fn_ = do_no_restart_syscall;
    let frame = (*regs).sp as *mut rt_sigframe;
    if !access_ok(frame, frame_size) { force_sig(SIGSEGV); return 0; }
    let mut set = core::mem::MaybeUninit::<sigset_t>::uninit();
    if __copy_from_user(set.as_mut_ptr() as *mut _, &(*frame).uc.uc_sigmask as *const _, core::mem::size_of::<sigset_t>()) != 0 { force_sig(SIGSEGV); return 0; }
    set_current_blocked(set.assume_init_ref());
    if restore_sigcontext(regs, &(*frame).uc.uc_mcontext as *const _ as *mut _) != 0 { force_sig(SIGSEGV); return 0; }
    if restore_altstack(&(*frame).uc.uc_stack as *const _ as *mut _) != 0 { force_sig(SIGSEGV); return 0; }
    (*regs).cause = !0;
    (*regs).a0 as c_long
}

unsafe fn setup_sigcontext(frame: *mut rt_sigframe, regs: *mut pt_regs) -> c_long {
    let sc = &mut (*frame).uc.uc_mcontext as *mut sigcontext;
    let mut err = __copy_to_user(&mut (*sc).sc_regs as *mut _, regs as *const _, core::mem::size_of_val(&(*sc).sc_regs));
    if has_fpu() { err |= save_fp_state(regs, &mut (*sc).sc_fpregs); }
    let mut sc_ext_ptr = &mut (*sc).sc_extdesc.hdr as *mut __riscv_ctx_hdr;
    let mut i = 0;
    while i < nr_arch_exts { let ext = &arch_ext_list[i]; if let Some(save) = ext.save { let ext_size = save(regs, sc_ext_ptr.add(1) as *mut c_void); if ext_size <= 0 { err |= ext_size; } else { err |= __put_user(ext.magic, &mut (*sc_ext_ptr).magic); err |= __put_user(ext_size as usize, &mut (*sc_ext_ptr).size); sc_ext_ptr = (sc_ext_ptr as *mut u8).add(ext_size as usize) as *mut __riscv_ctx_hdr; } } i += 1; }
    err |= __put_user(0, &mut (*sc).sc_extdesc.reserved);
    err |= __put_user(END_MAGIC, &mut (*sc_ext_ptr).magic); err |= __put_user(END_HDR_SIZE, &mut (*sc_ext_ptr).size);
    err
}

unsafe fn get_sigframe(ksig: *mut ksignal, regs: *mut pt_regs, framesize: usize) -> *mut c_void {
    let mut sp = (*regs).sp;
    if on_sig_stack(sp) && !likely(on_sig_stack(sp.wrapping_sub(framesize as c_ulong))) { return (-1isize) as *mut c_void; }
    sp = sigsp(sp, ksig).wrapping_sub(framesize as c_ulong); sp &= !0xf;
    sp as *mut c_void
}

unsafe fn setup_rt_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> c_int {
    let frame_size = get_rt_frame_size(false); let frame = get_sigframe(ksig, regs, frame_size) as *mut rt_sigframe;
    if !access_ok(frame, frame_size) { return -EFAULT; }
    let mut err = copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info);
    err |= __put_user(0, &mut (*frame).uc.uc_flags); err |= __put_user(core::ptr::null_mut(), &mut (*frame).uc.uc_link); err |= __save_altstack(&mut (*frame).uc.uc_stack, (*regs).sp); err |= setup_sigcontext(frame, regs); err |= __copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>());
    if err != 0 { return -EFAULT; }
    // CONFIG_MMU uses the VDSO rt_sigreturn address; CONFIG_MMU-disabled builds
    // copy __user_rt_sigreturn into frame.sigreturn_code and flush the icache.
    (*regs).ra = VDSO_SYMBOL((*current).mm.context.vdso, rt_sigreturn);
    if is_shstk_enabled(current) { (*regs).t0 = (*regs).ra; }
    (*regs).epc = (*ksig).ka.sa.sa_handler as c_ulong; (*regs).sp = frame as c_ulong; (*regs).a0 = (*ksig).sig as c_ulong; (*regs).a1 = &(*frame).info as *const _ as c_ulong; (*regs).a2 = &(*frame).uc as *const _ as c_ulong;
    0
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    let oldset = sigmask_to_save(); rseq_signal_deliver(ksig, regs);
    let ret = if is_compat_task() { compat_setup_rt_frame(ksig, oldset, regs) } else { setup_rt_frame(ksig, oldset, regs) };
    signal_setup_done(ret, ksig, 0);
}

#[no_mangle]
pub unsafe extern "C" fn arch_do_signal_or_restart(regs: *mut pt_regs) {
    let mut continue_addr = 0; let mut restart_addr = 0; let mut retval = 0; let mut ksig = core::mem::MaybeUninit::<ksignal>::uninit(); let syscall = (*regs).cause == EXC_SYSCALL;
    if syscall { continue_addr = (*regs).epc; restart_addr = continue_addr - 4; retval = (*regs).a0 as c_long; (*regs).cause = !0; match retval { -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR | -ERESTART_RESTARTBLOCK => { (*regs).a0 = (*regs).orig_a0; (*regs).epc = restart_addr; }, _ => {} } }
    if get_signal(ksig.as_mut_ptr()) { let ksig = ksig.as_mut_ptr(); if (*regs).epc == restart_addr && (retval == -ERESTARTNOHAND || retval == -ERESTART_RESTARTBLOCK || (retval == -ERESTARTSYS && ((*ksig).ka.sa.sa_flags & SA_RESTART) == 0)) { (*regs).a0 = -EINTR as c_ulong; (*regs).epc = continue_addr; } handle_signal(ksig, regs); return; }
    if syscall && (*regs).epc == restart_addr && retval == -ERESTART_RESTARTBLOCK { (*regs).a7 = __NR_restart_syscall; }
    restore_saved_sigmask();
}

pub unsafe fn init_rt_signal_env() {
    riscv_v_sc_size = core::mem::size_of::<__riscv_ctx_hdr>() + core::mem::size_of::<__sc_riscv_v_state>() + riscv_v_vsize;
    riscv_zicfiss_sc_size = core::mem::size_of::<__riscv_ctx_hdr>() + core::mem::size_of::<__sc_riscv_cfi_state>();
    signal_minsigstksz = get_rt_frame_size(true) as c_ulong;
}

#[cfg(feature = "CONFIG_DYNAMIC_SIGFRAME")]
pub unsafe fn sigaltstack_size_valid(ss_size: usize) -> bool { ss_size > get_rt_frame_size(false) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
