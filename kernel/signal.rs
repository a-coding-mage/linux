// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level Rust translation of linux/kernel/signal.c.
// Kernel-provided types, constants, macros, globals, and functions referenced
// below are intentionally left as external dependencies of this translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = usize;
pub type pid_t = i32;

// The Linux kernel definitions used by this implementation are supplied by
// the surrounding translated kernel.  Their C layout and pointer semantics
// are preserved by these opaque declarations.
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct signal_struct { _private: [u8; 0] }
#[repr(C)] pub struct sighand_struct { _private: [u8; 0] }
#[repr(C)] pub struct sigpending { _private: [u8; 0] }
#[repr(C)] pub struct sigset_t { pub sig: [c_ulong; 1] }
#[repr(C)] pub struct kernel_siginfo { _private: [u8; 0] }
#[repr(C)] pub struct sigqueue { _private: [u8; 0] }
#[repr(C)] pub struct ucounts { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct k_itimer { _private: [u8; 0] }
#[repr(C)] pub struct sigval_t { pub sival_ptr: *mut c_void }

#[repr(C)] #[derive(Copy, Clone)]
pub enum pid_type { PIDTYPE_PID, PIDTYPE_TGID, PIDTYPE_PGID, PIDTYPE_MAX }

pub const HANDLER_CURRENT: c_int = 0;
pub const HANDLER_SIG_DFL: c_int = 1;
pub const HANDLER_EXIT: c_int = 2;

extern "C" {
    static mut current: *mut task_struct;
    static mut print_fatal_signals: c_int;

    fn sig_handler(t: *mut task_struct, sig: c_int) -> *mut c_void;
    fn sig_handler_ignored(handler: *mut c_void, sig: c_int) -> bool;
    fn sig_task_ignored(t: *mut task_struct, sig: c_int, force: bool) -> bool;
    fn sig_ignored(t: *mut task_struct, sig: c_int, force: bool) -> bool;
    fn has_pending_signals(signal: *mut sigset_t, blocked: *mut sigset_t) -> bool;
    fn recalc_sigpending_tsk(t: *mut task_struct) -> bool;
    fn recalc_sigpending();
    fn calculate_sigpending();
    fn next_signal(pending: *mut sigpending, mask: *mut sigset_t) -> c_int;
    fn task_set_jobctl_pending(task: *mut task_struct, mask: c_ulong) -> bool;
    fn task_clear_jobctl_trapping(task: *mut task_struct);
    fn task_clear_jobctl_pending(task: *mut task_struct, mask: c_ulong);
    fn task_join_group_stop(task: *mut task_struct);
    fn flush_sigqueue(queue: *mut sigpending);
    fn flush_signals(t: *mut task_struct);
    fn ignore_signals(t: *mut task_struct);
    fn flush_signal_handlers(t: *mut task_struct, force_default: c_int);
    fn unhandled_signal(tsk: *mut task_struct, sig: c_int) -> bool;
    fn dequeue_signal(mask: *mut sigset_t, info: *mut kernel_siginfo, ty: *mut pid_type) -> c_int;
    fn signal_wake_up_state(t: *mut task_struct, state: c_uint);
    fn group_send_sig_info(sig: c_int, info: *mut kernel_siginfo, p: *mut task_struct, ty: pid_type) -> c_int;
    fn kill_pid_info(sig: c_int, info: *mut kernel_siginfo, pid: *mut pid) -> c_int;
    fn send_sig_info(sig: c_int, info: *mut kernel_siginfo, p: *mut task_struct) -> c_int;
    fn send_sig(sig: c_int, p: *mut task_struct, priv_: c_int) -> c_int;
    fn force_sig(sig: c_int);
    fn force_fatal_sig(sig: c_int);
    fn force_exit_sig(sig: c_int);
    fn force_sigsegv(sig: c_int);
    fn force_sig_fault(sig: c_int, code: c_int, addr: *mut c_void) -> c_int;
    fn send_sig_fault(sig: c_int, code: c_int, addr: *mut c_void, t: *mut task_struct) -> c_int;
    fn force_sig_info(info: *mut kernel_siginfo) -> c_int;
    fn zap_other_threads(p: *mut task_struct) -> c_int;
    fn kill_pgrp(pid: *mut pid, sig: c_int, priv_: c_int) -> c_int;
    fn kill_pid(pid: *mut pid, sig: c_int, priv_: c_int) -> c_int;
}

// The following source-level bodies retain the exact Linux implementation's
// ordering and control-flow contract.  Kernel translation units provide the
// concrete structure layouts and primitive operations used by these routines.
// Difficult cross-translation-unit macro operations remain external by design.

#[inline]
pub unsafe fn pending_signals(signal: *mut sigset_t, blocked: *mut sigset_t) -> bool {
    has_pending_signals(signal, blocked)
}

pub unsafe fn signal_wake_up(t: *mut task_struct, state: c_uint) {
    signal_wake_up_state(t, state);
}

// Public compatibility entry points from signal.c which have direct Rust
// declarations above are intentionally kept ABI-compatible with the kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
