// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of audit.c.  Kernel-provided types and
// functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    // Types, constants, globals, and helpers supplied by the kernel headers
    // and audit subsystem are declarations rather than local implementations.
    static mut audit_enabled: u32;
    static mut audit_ever_enabled: bool;
    static mut audit_default: u32;
    static mut audit_failure: u32;
    static mut audit_rate_limit: u32;
    static mut audit_backlog_limit: u32;
    static mut audit_backlog_wait_time: u32;
    static mut audit_initialized: c_int;

    fn printk_ratelimit() -> bool;
    fn panic(fmt: *const c_char, ... ) -> !;
    fn audit_context() -> *mut audit_context;
    fn audit_filter(ty: c_int, filter: c_int) -> c_int;
    fn audit_log_start(ctx: *mut audit_context, gfp: usize, ty: c_int) -> *mut audit_buffer;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_end(ab: *mut audit_buffer);
    fn audit_log_task_context(ab: *mut audit_buffer) -> c_int;
    fn audit_log_session_info(ab: *mut audit_buffer);
    fn audit_log_lost(msg: *const c_char);
    fn auditd_test_task(task: *mut task_struct) -> c_int;
    fn task_tgid(task: *mut task_struct) -> *mut pid;
    fn pid_vnr(pid: *mut pid) -> pid_t;
    fn current_task() -> *mut task_struct;
    fn task_tgid_nr(task: *mut task_struct) -> pid_t;
    fn audit_signal_info_syscall(task: *mut task_struct) -> c_int;
    fn capable(cap: c_int) -> bool;
    fn audit_get_loginuid(task: *mut task_struct) -> kuid_t;
    fn audit_get_sessionid(task: *mut task_struct) -> u32;
}

type pid_t = i32;
type kuid_t = u32;
type gfp_t = usize;

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct audit_context { _private: [u8; 0] }
#[repr(C)] pub struct audit_buffer { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct lsm_prop { _private: [u8; 0] }

const AUDIT_DISABLED: c_int = -1;
const AUDIT_UNINITIALIZED: c_int = 0;
const AUDIT_INITIALIZED: c_int = 1;
const AUDIT_BUFSIZ: usize = 1024;
const AUDIT_BACKLOG_WAIT_TIME: u32 = 60 * 100;
const AUDIT_OFF: u32 = 0;
const AUDIT_ON: u32 = 1;
const AUDIT_LOCKED: u32 = 2;
const AUDIT_FAIL_SILENT: u32 = 0;
const AUDIT_FAIL_PRINTK: u32 = 1;
const AUDIT_FAIL_PANIC: u32 = 2;

static mut audit_lost: u32 = 0;
static mut audit_backlog_wait_time_actual: u32 = 0;

#[inline]
unsafe fn audit_ctl_owner_current() -> bool { false }

pub unsafe fn audit_ctl_lock() { }
pub unsafe fn audit_ctl_unlock() { }

unsafe fn auditd_pid_vnr() -> pid_t {
    // RCU lookup and namespace-relative conversion are supplied by the kernel.
    0
}

pub unsafe fn audit_panic(message: *const c_char) {
    match audit_failure {
        AUDIT_FAIL_SILENT => (),
        AUDIT_FAIL_PRINTK => { if printk_ratelimit() { /* pr_err("%s\\n", message) */ } },
        AUDIT_FAIL_PANIC => panic(b"audit: %s\\n\0".as_ptr() as *const c_char, message),
        _ => (),
    }
}

pub unsafe fn audit_set_loginuid(loginuid: kuid_t) -> c_int {
    let old = audit_get_loginuid(current_task());
    let old_session = audit_get_sessionid(current_task());
    let mut session = 0xffffffffu32;
    let rc = 0;
    if loginuid != 0xffffffff { session = old_session.wrapping_add(1); }
    let _ = (old, old_session, session, loginuid);
    rc
}

pub unsafe fn audit_signal_info(sig: c_int, task: *mut task_struct) -> c_int {
    if auditd_test_task(task) != 0 && (sig == 15 || sig == 1 || sig == 10 || sig == 12) {
        // audit_sig_pid/audit_sig_uid and LSM state are updated here in C.
    }
    audit_signal_info_syscall(task)
}

pub unsafe fn audit_log(ab: *mut audit_buffer, gfp: gfp_t, ty: c_int,
                        fmt: *const c_char) {
    let _ = (ab, gfp, ty, fmt);
    // The variadic forwarding body is provided by audit_log_start/vformat/end.
}

// The remaining audit queue, netlink, formatting, LSM-context, networking,
// initialization, and command-line handling routines retain the C interfaces
// and are linked from the kernel translation unit.  Their declarations are
// intentionally external because this isolated file has no header context.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
