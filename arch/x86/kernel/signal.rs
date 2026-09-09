// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  Copyright (C) 2000, 2001, 2002 Andi Kleen SuSE Labs
 *
 *  1997-11-28  Modified for POSIX.1b signals by Richard Henderson
 *  2000-06-20  Pentium III FXSR, SSE support by Gareth Hughes
 *  2000-2002   x86-64 support by Andi Kleen
 */

// Kernel headers and build-time configuration are supplied by the surrounding crate.

#[inline]
unsafe fn is_ia32_compat_frame(ksig: *mut ksignal) -> i32 {
    (cfg!(feature = "ia32_emulation") && ((*ksig).ka.sa.sa_flags & SA_IA32_ABI) != 0) as i32
}

#[inline]
unsafe fn is_ia32_frame(ksig: *mut ksignal) -> i32 {
    (cfg!(feature = "x86_32") || is_ia32_compat_frame(ksig) != 0) as i32
}

#[inline]
unsafe fn is_x32_frame(ksig: *mut ksignal) -> i32 {
    (cfg!(feature = "x86_x32_abi") && ((*ksig).ka.sa.sa_flags & SA_X32_ABI) != 0) as i32
}

#[inline]
unsafe fn sig_prepare_pkru() -> u32 {
    let orig_pkru = read_pkru();
    write_pkru(0);
    orig_pkru
}

const FRAME_ALIGNMENT: usize = 16;
const MAX_FRAME_PADDING: usize = FRAME_ALIGNMENT - 1;

pub unsafe fn get_sigframe(
    ksig: *mut ksignal,
    regs: *mut pt_regs,
    frame_size: usize,
    fpstate: *mut *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let ka = &mut (*ksig).ka;
    let ia32_frame = is_ia32_frame(ksig) != 0;
    let nested_altstack = on_sig_stack((*regs).sp);
    let mut entering_altstack = false;
    let mut math_size: usize = 0;
    let mut sp = (*regs).sp;
    let mut buf_fx: usize = 0;

    if !ia32_frame { sp = sp.wrapping_sub(128); }
    if (ka.sa.sa_flags & SA_ONSTACK) != 0 {
        if sas_ss_flags(sp) == 0 {
            sp = current().sas_ss_sp.wrapping_add(current().sas_ss_size);
            entering_altstack = true;
        }
    } else if ia32_frame && !nested_altstack && (*regs).ss != __USER_DS
        && (ka.sa.sa_flags & SA_RESTORER) == 0 && !ka.sa.sa_restorer.is_null() {
        sp = ka.sa.sa_restorer as usize;
        entering_altstack = true;
    }

    sp = fpu__alloc_mathframe(sp, ia32_frame, &mut buf_fx, &mut math_size);
    *fpstate = sp as *mut core::ffi::c_void;
    sp = sp.wrapping_sub(frame_size);
    if ia32_frame {
        sp = (sp.wrapping_add(4) & (!FRAME_ALIGNMENT + 1)).wrapping_sub(4);
    } else {
        sp = round_down(sp, FRAME_ALIGNMENT) - 8;
    }
    if (nested_altstack || entering_altstack) && !__on_sig_stack(sp) {
        if show_unhandled_signals != 0 && printk_ratelimit() != 0 {
            pr_info!("{}[{}] overflowed sigaltstack\n", current().comm, task_pid_nr(current()));
        }
        return (-1isize) as *mut core::ffi::c_void;
    }
    let pkru = sig_prepare_pkru();
    if !copy_fpstate_to_sigframe(*fpstate, buf_fx as *mut core::ffi::c_void, math_size, pkru) {
        write_pkru(pkru);
        return (-1isize) as *mut core::ffi::c_void;
    }
    sp as *mut core::ffi::c_void
}

#[cfg(any(feature = "x86_32", feature = "ia32_emulation"))]
const MAX_FRAME_SIGINFO_UCTXT_SIZE: usize = core::mem::size_of::<sigframe_ia32>();
#[cfg(not(any(feature = "x86_32", feature = "ia32_emulation")))]
const MAX_FRAME_SIGINFO_UCTXT_SIZE: usize = core::mem::size_of::<rt_sigframe>();
const MAX_XSAVE_PADDING: usize = 63;

static mut max_frame_size: usize = 0;
static mut fpu_default_state_size: u32 = 0;

unsafe fn init_sigframe_size() -> i32 {
    fpu_default_state_size = fpu__get_fpstate_size();
    max_frame_size = MAX_FRAME_SIGINFO_UCTXT_SIZE + MAX_FRAME_PADDING;
    max_frame_size += fpu_default_state_size as usize + MAX_XSAVE_PADDING;
    max_frame_size = round_up(max_frame_size, FRAME_ALIGNMENT);
    pr_info!("max sigframe size: {}\n", max_frame_size);
    0
}

pub unsafe fn get_sigframe_size() -> usize { max_frame_size }

unsafe fn setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> i32 {
    rseq_signal_deliver(ksig, regs);
    if is_ia32_frame(ksig) != 0 {
        if ((*ksig).ka.sa.sa_flags & SA_SIGINFO) != 0 { ia32_setup_rt_frame(ksig, regs) }
        else { ia32_setup_frame(ksig, regs) }
    } else if is_x32_frame(ksig) != 0 { x32_setup_rt_frame(ksig, regs) }
    else { x64_setup_rt_frame(ksig, regs) }
}

unsafe fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    let mut stepping = false;
    let fpu = x86_task_fpu(current());
    if v8086_mode(regs) { save_v86_state(regs as *mut kernel_vm86_regs, VM86_SIGNAL); }
    if syscall_get_nr(current(), regs) != -1 {
        match syscall_get_error(current(), regs) {
            -ERESTART_RESTARTBLOCK | -ERESTARTNOHAND => (*regs).ax = -EINTR,
            -ERESTARTSYS => { if ((*ksig).ka.sa.sa_flags & SA_RESTART) == 0 { (*regs).ax = -EINTR; } else { (*regs).ax = (*regs).orig_ax; (*regs).ip = (*regs).ip.wrapping_sub(2); } },
            -ERESTARTNOINTR => { (*regs).ax = (*regs).orig_ax; (*regs).ip = (*regs).ip.wrapping_sub(2); },
            _ => {}
        }
    }
    stepping = test_thread_flag(TIF_SINGLESTEP) != 0;
    if stepping { user_disable_single_step(current()); }
    let failed = setup_rt_frame(ksig, regs) < 0;
    if !failed { (*regs).flags &= !(X86_EFLAGS_DF | X86_EFLAGS_RF | X86_EFLAGS_TF); fpu__clear_user_states(fpu); }
    signal_setup_done(failed, ksig, stepping);
}

#[inline]
unsafe fn get_nr_restart_syscall(regs: *const pt_regs) -> usize {
    #[cfg(feature = "ia32_emulation")]
    if (current().restart_block.arch_data & TS_COMPAT) != 0 { return __NR_ia32_restart_syscall; }
    #[cfg(feature = "x86_x32_abi")]
    { return __NR_restart_syscall | ((*regs).orig_ax & __X32_SYSCALL_BIT); }
    __NR_restart_syscall
}

pub unsafe fn arch_do_signal_or_restart(regs: *mut pt_regs) {
    let mut ksig: ksignal = core::mem::zeroed();
    if get_signal(&mut ksig) { handle_signal(&mut ksig, regs); return; }
    if syscall_get_nr(current(), regs) != -1 {
        match syscall_get_error(current(), regs) {
            -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => { (*regs).ax = (*regs).orig_ax; (*regs).ip = (*regs).ip.wrapping_sub(2); },
            -ERESTART_RESTARTBLOCK => { (*regs).ax = get_nr_restart_syscall(regs); (*regs).ip = (*regs).ip.wrapping_sub(2); },
            _ => {}
        }
    }
    restore_saved_sigmask();
}

pub unsafe fn signal_fault(regs: *mut pt_regs, frame: *mut core::ffi::c_void, where_: *mut i8) {
    let me = current();
    if show_unhandled_signals != 0 && printk_ratelimit() != 0 {
        printk!("{}[{}] bad frame in {} frame:{:p} ip:{:x} sp:{:x} orax:{:x}", task_pid_nr(current()) > 1, me.comm, me.pid, where_, frame, (*regs).ip, (*regs).sp, (*regs).orig_ax);
        print_vma_addr(" in ", (*regs).ip); pr_cont!("\n");
    }
    force_sig(SIGSEGV);
}

#[cfg(feature = "dynamic_sigframe")]
static mut strict_sigaltstack_size: bool = cfg!(feature = "strict_sigaltstack_size");

#[cfg(feature = "dynamic_sigframe")]
pub unsafe fn sigaltstack_size_valid(ss_size: usize) -> bool {
    let mut fsize = max_frame_size - fpu_default_state_size as usize;
    if !fpu_state_size_dynamic() && !strict_sigaltstack_size { return true; }
    fsize += (*x86_task_fpu(current().group_leader)).perm.__user_state_size as usize;
    if ss_size > fsize { return true; }
    if strict_sigaltstack_size { return ss_size > fsize; }
    let mask = (*x86_task_fpu(current().group_leader)).perm.__state_perm;
    if (mask & XFEATURE_MASK_USER_DYNAMIC) != 0 { return ss_size > fsize; }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
