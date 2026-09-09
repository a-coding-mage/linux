// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Anton Ivanov (aivanov@{brocade.com,kot-begemot.co.uk})
 * Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
 * Copyright (C) 2004 PathScale, Inc
 * Copyright (C) 2004 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

/* C dependencies: stdlib, stdarg, stdbool, errno, signal, string, strings,
 * as-layout, kern_util, os, skas, sysdep/mcontext, um_malloc, sys/ucontext,
 * timetravel, and internal.h. */

use core::ffi::{c_int, c_void};

extern "C" {
    static mut signals_enabled: c_int;
    static mut signals_pending: u32;
    static mut signals_active: u32;
    static mut using_seccomp: c_int;
    static mut time_travel_mode: c_int;

    fn relay_signal(sig: c_int, si: *mut siginfo, regs: *mut uml_pt_regs, mc: *mut c_void);
    fn winch(sig: c_int, si: *mut siginfo, regs: *mut uml_pt_regs, mc: *mut c_void);
    fn segv_handler(sig: c_int, si: *mut siginfo, regs: *mut uml_pt_regs, mc: *mut c_void);
    fn sigio_handler(sig: c_int, si: *mut siginfo, regs: *mut uml_pt_regs, mc: *mut c_void);
    fn sigchld_handler(sig: c_int, si: *mut siginfo, regs: *mut uml_pt_regs, mc: *mut c_void);
    fn get_regs_from_mc(regs: *mut uml_pt_regs, mc: *mut mcontext_t);
    fn get_faultinfo_from_mc(faultinfo: *mut c_void, mc: *mut mcontext_t);
    fn unblock_signals_trace();
    fn block_signals_trace();
    fn um_set_signals_trace(enable: c_int) -> c_int;
    fn timer_handler(sig: c_int, si: *mut c_void, regs: *mut uml_pt_regs);
    fn panic(fmt: *const u8, ...);
    fn uml_pm_wake();
    fn os_getpid() -> c_int;
    fn os_local_ipi_disable();
    fn os_local_ipi_enable();
    fn barrier();
    fn um_trace_signals_off();
    fn um_trace_signals_on();
    fn deliver_time_travel_irqs();
    fn sigio_run_timetravel_handlers();
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, sig: c_int) -> c_int;
    fn sigismember(set: *const sigset_t, sig: c_int) -> c_int;
    fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int;
    fn sigaction(sig: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
}

#[no_mangle]
pub static mut sig_info: [Option<unsafe extern "C" fn(c_int, *mut siginfo, *mut uml_pt_regs, *mut c_void)>; NSIG] = [None; NSIG];

#[allow(non_camel_case_types)]
type siginfo = c_void;
#[allow(non_camel_case_types)]
type mcontext_t = c_void;
#[allow(non_camel_case_types)]
type ucontext_t = c_void;
#[allow(non_camel_case_types)]
type sigset_t = c_void;
#[allow(non_camel_case_types)]
type stack_t = c_void;
#[allow(non_camel_case_types)]
type sigaction = c_void;
#[repr(C)]
pub struct uml_pt_regs {
    pub is_user: c_int,
    pub faultinfo: [u8; 0],
}

const NSIG: usize = 65;
const SIGTRAP: c_int = 5;
const SIGFPE: c_int = 8;
const SIGILL: c_int = 4;
const SIGWINCH: c_int = 28;
const SIGBUS: c_int = 7;
const SIGSEGV: c_int = 11;
const SIGIO: c_int = 29;
const SIGCHLD: c_int = 17;
const SIGALRM: c_int = 14;
const SIGUSR1: c_int = 10;
const SIG_UNBLOCK: c_int = 1;
const SIG_BLOCK: c_int = 0;
const SA_SIGINFO: c_int = 4;
const SA_ONSTACK: c_int = 0x08000000;
const SA_NODEFER: c_int = 0x40000000;
const SA_RESTART: c_int = 0x10000000;
const TT_MODE_EXTERNAL: c_int = 1;
const SIGIO_MASK: u32 = 1 << 0;
const SIGALRM_MASK: u32 = 1 << 1;
const SIGCHLD_MASK: u32 = 1 << 2;

#[thread_local]
static mut SIGNALS_ENABLED: c_int = 0;
#[thread_local]
static mut SIGNALS_PENDING: u32 = 0;
#[thread_local]
static mut SIGNALS_ACTIVE: u32 = 0;

unsafe fn sig_handler_common(sig: c_int, si: *mut siginfo, mc: *mut mcontext_t) {
    let mut r = uml_pt_regs { is_user: 0, faultinfo: [] };
    if sig == SIGSEGV {
        get_regs_from_mc(&mut r, mc);
        get_faultinfo_from_mc(r.faultinfo.as_mut_ptr() as *mut c_void, mc);
    }
    if sig != SIGIO && sig != SIGWINCH && sig != SIGCHLD { unblock_signals_trace(); }
    if let Some(handler) = sig_info[sig as usize] { handler(sig, si, &mut r, mc as *mut c_void); }
}

unsafe fn sig_handler(sig: c_int, si: *mut siginfo, mc: *mut mcontext_t) {
    let enabled = SIGNALS_ENABLED;
    if !enabled && sig == SIGIO {
        if time_travel_mode == TT_MODE_EXTERNAL { sigio_run_timetravel_handlers(); }
        else { SIGNALS_PENDING |= SIGIO_MASK; }
        return;
    }
    if !enabled && sig == SIGCHLD { SIGNALS_PENDING |= SIGCHLD_MASK; return; }
    block_signals_trace();
    sig_handler_common(sig, si, mc);
    um_set_signals_trace(enabled);
}

unsafe fn timer_real_alarm_handler(mc: *mut mcontext_t) {
    let mut regs = uml_pt_regs { is_user: 0, faultinfo: [] };
    if !mc.is_null() { get_regs_from_mc(&mut regs, mc); }
    timer_handler(SIGALRM, core::ptr::null_mut(), &mut regs);
}

unsafe fn timer_alarm_handler(_sig: c_int, _si: *mut siginfo, mc: *mut mcontext_t) {
    let enabled = SIGNALS_ENABLED;
    if !enabled { SIGNALS_PENDING |= SIGALRM_MASK; return; }
    block_signals_trace();
    SIGNALS_ACTIVE |= SIGALRM_MASK;
    timer_real_alarm_handler(mc);
    SIGNALS_ACTIVE &= !SIGALRM_MASK;
    um_set_signals_trace(enabled);
}

pub unsafe fn deliver_alarm() { timer_alarm_handler(SIGALRM, core::ptr::null_mut(), core::ptr::null_mut()); }
pub unsafe fn timer_set_signal_handler() { set_handler(SIGALRM); }
pub unsafe fn timer_alarm_pending() -> c_int { ((SIGNALS_PENDING & SIGALRM_MASK) != 0) as c_int }

pub unsafe fn set_sigstack(sig_stack: *mut c_void, size: usize) {
    let stack: stack_t = core::mem::zeroed();
    let _ = (sig_stack, size, stack);
    if sigaltstack(&stack, core::ptr::null_mut()) != 0 { panic(b"enabling signal stack failed, errno = %d\0".as_ptr(), 0); }
}

unsafe fn sigusr1_handler(_sig: c_int, _si: *mut siginfo, _mc: *mut mcontext_t) { uml_pm_wake(); }
pub unsafe fn register_pm_wake_signal() { set_handler(SIGUSR1); }

unsafe fn hard_handler(sig: c_int, si: *mut siginfo, p: *mut c_void) {
    let mc = p;
    let _save_errno: c_int = 0;
    sig_handler(sig, si, mc as *mut mcontext_t);
}

pub unsafe fn set_handler(sig: c_int) {
    let mut action: sigaction = core::mem::zeroed();
    let mut mask: sigset_t = core::mem::zeroed();
    let mut flags = SA_SIGINFO | SA_ONSTACK;
    let _ = hard_handler as unsafe fn(c_int, *mut siginfo, *mut c_void);
    sigemptyset(&mut mask); sigaddset(&mut mask, SIGIO); sigaddset(&mut mask, SIGWINCH); sigaddset(&mut mask, SIGALRM);
    if using_seccomp != 0 { sigaddset(&mut mask, SIGCHLD); }
    if sig == SIGSEGV { flags |= SA_NODEFER; }
    if sigismember(&mask, sig) != 0 { flags |= SA_RESTART; }
    let _ = (action, flags);
    if sigaction(sig, &action, core::ptr::null_mut()) < 0 { panic(b"sigaction failed - errno = %d\0".as_ptr(), 0); }
    sigemptyset(&mut mask); sigaddset(&mut mask, sig);
    if sigprocmask(SIG_UNBLOCK, &mask, core::ptr::null_mut()) < 0 { panic(b"sigprocmask failed - errno = %d\0".as_ptr(), 0); }
}

pub unsafe fn send_sigio_to_self() { kill(os_getpid(), SIGIO); }
pub unsafe fn change_sig(signal: c_int, on: c_int) -> c_int {
    let mut set: sigset_t = core::mem::zeroed(); sigemptyset(&mut set); sigaddset(&mut set, signal);
    if sigprocmask(if on != 0 { SIG_UNBLOCK } else { SIG_BLOCK }, &set, core::ptr::null_mut()) < 0 { return -1; } 0
}

unsafe fn __block_signals() { if SIGNALS_ENABLED == 0 { return; } os_local_ipi_disable(); barrier(); SIGNALS_ENABLED = 0; }
unsafe fn __unblock_signals() { if SIGNALS_ENABLED != 0 { return; } SIGNALS_ENABLED = 1; barrier(); os_local_ipi_enable(); }
pub unsafe fn block_signals() { __block_signals(); barrier(); }

pub unsafe fn unblock_signals() {
    if SIGNALS_ENABLED == 1 { return; }
    __unblock_signals();
    loop {
        barrier(); let save_pending = SIGNALS_PENDING; if save_pending == 0 { return; } SIGNALS_PENDING = 0;
        __block_signals(); um_trace_signals_off();
        if save_pending & SIGIO_MASK != 0 { sig_handler_common(SIGIO, core::ptr::null_mut(), core::ptr::null_mut()); }
        if save_pending & SIGCHLD_MASK != 0 { let mut regs = uml_pt_regs { is_user: 0, faultinfo: [] }; sigchld_handler(SIGCHLD, core::ptr::null_mut(), &mut regs, core::ptr::null_mut()); }
        if save_pending & SIGALRM_MASK != 0 && SIGNALS_ACTIVE & SIGALRM_MASK == 0 { timer_real_alarm_handler(core::ptr::null_mut()); }
        if SIGNALS_PENDING & SIGIO_MASK == 0 && SIGNALS_ACTIVE & SIGALRM_MASK != 0 { return; }
        um_trace_signals_on(); __unblock_signals();
    }
}
pub unsafe fn um_get_signals() -> c_int { SIGNALS_ENABLED }
pub unsafe fn um_set_signals(enable: c_int) -> c_int { if SIGNALS_ENABLED == enable { return enable; } let ret = SIGNALS_ENABLED; if enable != 0 { unblock_signals(); } else { block_signals(); } ret }
pub unsafe fn um_set_signals_trace_local(enable: c_int) -> c_int { um_set_signals(enable) }

/* IS_ENABLED(CONFIG_UML_TIME_TRAVEL_SUPPORT) conditional declarations and
 * definitions are retained below; the configuration is supplied by the build. */
#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
static mut signals_blocked: c_int = 0;
#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
static mut signals_blocked_pending: c_int = 0;

#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
pub unsafe fn mark_sigio_pending() { SIGNALS_PENDING |= SIGIO_MASK; }

#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
pub unsafe fn block_signals_hard() { signals_blocked += 1; barrier(); }

#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
pub unsafe fn unblock_signals_hard() {
    static mut unblocking: bool = false;
    if signals_blocked == 0 { panic(b"unblocking signals while not blocked\0".as_ptr()); }
    signals_blocked -= 1;
    if signals_blocked != 0 || unblocking { return; }
    barrier();
    unblocking = true;
    while signals_blocked_pending != 0 {
        if SIGNALS_ENABLED != 0 {
            SIGNALS_PENDING |= SIGIO_MASK;
            block_signals();
            unblock_signals();
        } else {
            sigio_run_timetravel_handlers();
        }
        signals_blocked_pending -= 1;
    }
    unblocking = false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
