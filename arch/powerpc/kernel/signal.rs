// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Common signal handling code for both 32 and 64 bits
 *
 *    Copyright (c) 2007 Benjamin Herrenschmidt, IBM Corporation
 *    Extracted from signal_32.c and signal_64.c
 */

// Dependencies are supplied by the surrounding kernel translation unit.

#[cfg(feature = "CONFIG_VSX")]
pub unsafe fn copy_fpr_to_user(to: *mut core::ffi::c_void, task: *mut task_struct) -> c_ulong {
    let mut buf: [u64; ELF_NFPREG] = [0; ELF_NFPREG];
    let mut i: c_int = 0;
    while i < (ELF_NFPREG as c_int - 1) { buf[i as usize] = (*task).thread.TS_FPR(i); i += 1; }
    buf[i as usize] = (*task).thread.fp_state.fpscr;
    __copy_to_user(to, buf.as_ptr() as *const core::ffi::c_void, ELF_NFPREG * core::mem::size_of::<f64>())
}

#[cfg(feature = "CONFIG_VSX")]
pub unsafe fn copy_fpr_from_user(task: *mut task_struct, from: *mut core::ffi::c_void) -> c_ulong {
    let mut buf: [u64; ELF_NFPREG] = [0; ELF_NFPREG];
    if __copy_from_user(buf.as_mut_ptr() as *mut core::ffi::c_void, from, ELF_NFPREG * core::mem::size_of::<f64>()) != 0 { return 1; }
    let mut i: c_int = 0;
    while i < (ELF_NFPREG as c_int - 1) { (*task).thread.TS_FPR(i) = buf[i as usize]; i += 1; }
    (*task).thread.fp_state.fpscr = buf[i as usize];
    0
}

#[cfg(feature = "CONFIG_VSX")]
pub unsafe fn copy_vsx_to_user(to: *mut core::ffi::c_void, task: *mut task_struct) -> c_ulong {
    let mut buf: [u64; ELF_NVSRHALFREG] = [0; ELF_NVSRHALFREG];
    for i in 0..ELF_NVSRHALFREG { buf[i] = (*task).thread.fp_state.fpr[i][TS_VSRLOWOFFSET]; }
    __copy_to_user(to, buf.as_ptr() as *const core::ffi::c_void, ELF_NVSRHALFREG * core::mem::size_of::<f64>())
}

#[cfg(feature = "CONFIG_VSX")]
pub unsafe fn copy_vsx_from_user(task: *mut task_struct, from: *mut core::ffi::c_void) -> c_ulong {
    let mut buf: [u64; ELF_NVSRHALFREG] = [0; ELF_NVSRHALFREG];
    if __copy_from_user(buf.as_mut_ptr() as *mut core::ffi::c_void, from, ELF_NVSRHALFREG * core::mem::size_of::<f64>()) != 0 { return 1; }
    for i in 0..ELF_NVSRHALFREG { (*task).thread.fp_state.fpr[i][TS_VSRLOWOFFSET] = buf[i]; }
    0
}

pub static mut show_unhandled_signals: c_int = 1;

pub unsafe fn get_min_sigframe_size() -> c_ulong {
    if IS_ENABLED(CONFIG_PPC64) { get_min_sigframe_size_64() } else { get_min_sigframe_size_32() }
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn get_min_sigframe_size_compat() -> c_ulong { get_min_sigframe_size_32() }

pub unsafe fn get_sigframe(ksig: *mut ksignal, tsk: *mut task_struct, frame_size: usize, is_32: c_int) -> *mut core::ffi::c_void {
    let sp = get_tm_stackpointer(tsk);
    let mut oldsp = if is_32 != 0 { sp & 0x0ffffffff } else { sp };
    oldsp = sigsp(oldsp, ksig);
    ((oldsp.wrapping_sub(frame_size as c_ulong)) & !0xf) as *mut core::ffi::c_void
}

unsafe fn check_syscall_restart(regs: *mut pt_regs, ka: *mut k_sigaction, has_handler: c_int) {
    let mut ret = (*regs).gpr[3];
    let mut restart = 1;
    if !trap_is_syscall(regs) || trap_norestart(regs) { return; }
    if trap_is_scv(regs) { if !IS_ERR_VALUE(ret) { return; } ret = ret.wrapping_neg(); }
    else if (*regs).ccr & 0x10000000 == 0 { return; }
    match ret {
        ERESTART_RESTARTBLOCK | ERESTARTNOHAND => restart = if has_handler != 0 { 0 } else { 1 },
        ERESTARTSYS => restart = if has_handler == 0 || ((*ka).sa.sa_flags & SA_RESTART) != 0 { 1 } else { 0 },
        ERESTARTNOINTR => {},
        _ => return,
    }
    if restart != 0 {
        if ret == ERESTART_RESTARTBLOCK { (*regs).gpr[0] = __NR_restart_syscall; } else { (*regs).gpr[3] = (*regs).orig_gpr3; }
        regs_add_return_ip(regs, -4); (*regs).result = 0;
    } else {
        (*regs).result = -EINTR;
        if trap_is_scv(regs) { (*regs).gpr[3] = -EINTR; }
        else { (*regs).gpr[3] = EINTR; (*regs).ccr |= 0x10000000; }
    }
}

unsafe fn do_signal(tsk: *mut task_struct) {
    let oldset = sigmask_to_save();
    let mut ksig = ksignal { sig: 0, ..core::mem::zeroed() };
    BUG_ON(tsk != current);
    get_signal(&mut ksig);
    check_syscall_restart((*tsk).thread.regs, &mut ksig.ka, (ksig.sig > 0) as c_int);
    if ksig.sig <= 0 { restore_saved_sigmask(); set_trap_norestart((*tsk).thread.regs); return; }
    thread_change_pc(tsk, (*tsk).thread.regs);
    rseq_signal_deliver(&mut ksig, (*tsk).thread.regs);
    let ret = if is_32bit_task() != 0 {
        if ksig.ka.sa.sa_flags & SA_SIGINFO != 0 { handle_rt_signal32(&mut ksig, oldset, tsk) } else { handle_signal32(&mut ksig, oldset, tsk) }
    } else { handle_rt_signal64(&mut ksig, oldset, tsk) };
    set_trap_norestart((*tsk).thread.regs);
    signal_setup_done(ret, &mut ksig, test_thread_flag(TIF_SINGLESTEP));
}

unsafe fn get_tm_stackpointer(tsk: *mut task_struct) -> c_ulong {
    let regs = (*tsk).thread.regs;
    let mut ret = (*regs).gpr[1];
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")]
    { BUG_ON(tsk != current); if MSR_TM_ACTIVE((*regs).msr) { preempt_disable(); tm_reclaim_current(TM_CAUSE_SIGNAL); if MSR_TM_TRANSACTIONAL((*regs).msr) { ret = (*tsk).thread.ckpt_regs.gpr[1]; } regs_set_return_msr(regs, (*regs).msr & !MSR_TS_MASK); preempt_enable(); } }
    ret
}

static FM32: &str = "bad frame in %s[%d]: %s";
static FM64: &str = "bad frame in %s[%d]: %s";

pub unsafe fn signal_fault(tsk: *mut task_struct, regs: *mut pt_regs, where_: *const c_char, ptr: *mut core::ffi::c_void) {
    if show_unhandled_signals != 0 { printk_ratelimited(if (*regs).msr & MSR_64BIT != 0 { FM64 } else { FM32 }, (*tsk).comm, task_pid_nr(tsk), where_, ptr, (*regs).nip, (*regs).link); }
}

pub unsafe fn arch_do_signal_or_restart(regs: *mut pt_regs) {
    BUG_ON(regs != (*current).thread.regs);
    (*current_thread_info()).exit_flags |= _TIF_RESTOREALL;
    do_signal(current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
