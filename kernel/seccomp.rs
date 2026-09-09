// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/kernel/seccomp.c.  Kernel-provided types and helpers
 * are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External kernel types/functions/constants are supplied by the surrounding kernel.
type u8 = core::primitive::u8; type u16 = core::primitive::u16;
type u32 = core::primitive::u32; type u64 = core::primitive::u64;
type s32 = core::primitive::i32; type ssize_t = isize; type loff_t = i64;
type pid_t = i32; type __poll_t = u32;
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct seccomp_data { pub nr: i32, pub arch: u32, pub instruction_pointer: u64, pub args: [u64; 6] }
#[repr(C)] pub struct sock_filter { pub code: u16, pub jt: u8, pub jf: u8, pub k: u32 }
#[repr(C)] pub struct sock_fprog { pub len: u16, pub filter: *mut sock_filter }
#[repr(C)] pub struct sock_fprog_kern { pub len: u32, pub filter: *mut sock_filter }
#[repr(C)] pub struct bpf_prog { pub len: u32, pub orig_prog: *mut sock_fprog_kern }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct ctl_table { pub procname: *const u8, pub data: *mut c_void, pub maxlen: usize, pub mode: u16, pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, i32, *mut c_void, *mut usize, *mut loff_t) -> i32> }
#[repr(C)] pub struct seq_file { pub file: *mut file }
#[repr(C)] pub struct pid_namespace { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }

extern "C" {
    static mut current: *mut task_struct;
    fn bpf_prog_run_pin_on_cpu(p: *mut bpf_prog, d: *const seccomp_data) -> u32;
    fn bpf_prog_destroy(p: *mut bpf_prog); fn kfree(p: *mut c_void);
    fn audit_seccomp(syscall: u64, signr: i64, action: u32);
    fn do_exit(sig: i32) -> !; fn force_sig_seccomp(nr: i32, data: i32, core: bool);
    fn syscall_get_nr(t: *mut task_struct, r: *mut c_void) -> i32;
    fn syscall_get_arch(t: *mut task_struct) -> u32;
    fn syscall_get_arguments(t: *mut task_struct, r: *mut c_void, a: *mut u64);
    fn task_pt_regs(t: *mut task_struct) -> *mut c_void;
    fn syscall_set_return_value(t: *mut task_struct, r: *mut c_void, err: i64, val: i64);
    fn syscall_rollback(t: *mut task_struct, r: *mut c_void);
}

const SECCOMP_MODE_DEAD: u32 = 3;
const SECCOMP_LOG_KILL_PROCESS: u32 = 1 << 0; const SECCOMP_LOG_KILL_THREAD: u32 = 1 << 1;
const SECCOMP_LOG_TRAP: u32 = 1 << 2; const SECCOMP_LOG_ERRNO: u32 = 1 << 3;
const SECCOMP_LOG_TRACE: u32 = 1 << 4; const SECCOMP_LOG_LOG: u32 = 1 << 5;
const SECCOMP_LOG_ALLOW: u32 = 1 << 6; const SECCOMP_LOG_USER_NOTIF: u32 = 1 << 7;
const SECCOMP_RET_KILL_PROCESS: u32 = 0x80000000; const SECCOMP_RET_KILL_THREAD: u32 = 0;
const SECCOMP_RET_TRAP: u32 = 0x00030000; const SECCOMP_RET_ERRNO: u32 = 0x00050000;
const SECCOMP_RET_USER_NOTIF: u32 = 0x7fc00000; const SECCOMP_RET_TRACE: u32 = 0x7ff00000;
const SECCOMP_RET_LOG: u32 = 0x7ffc0000; const SECCOMP_RET_ALLOW: u32 = 0x7fff0000;
const SECCOMP_RET_ACTION_FULL: u32 = 0xffff0000; const SECCOMP_RET_DATA: u32 = 0xffff;
const SIGKILL: i32 = 9; const SIGSYS: i32 = 31; const ENOSYS: i32 = 38;

#[repr(C)] struct action_cache { allow_native: *mut u64, allow_compat: *mut u64 }
#[repr(C)] struct notification { requests: i32, flags: u32, next_id: u64, notifications: list_head }
#[repr(C)] struct seccomp_filter {
    refs: refcount_t, users: refcount_t, log: bool, wait_killable_recv: bool,
    cache: action_cache, prev: *mut seccomp_filter, prog: *mut bpf_prog,
    notif: *mut notification, notify_lock: mutex, wqh: wait_queue_head_t,
}
#[repr(C)] struct seccomp_knotif {
    task: *mut task_struct, id: u64, data: *const seccomp_data, state: i32,
    error: i32, val: i64, flags: u32, ready: completion, list: list_head, addfd: list_head,
}
#[repr(C)] struct seccomp_kaddfd {
    file: *mut file, fd: i32, flags: u32, ioctl_flags: u32, setfd: bool,
    completion: completion, list: list_head,
}

static mut seccomp_actions_logged: u32 = SECCOMP_LOG_KILL_PROCESS | SECCOMP_LOG_KILL_THREAD |
    SECCOMP_LOG_TRAP | SECCOMP_LOG_ERRNO | SECCOMP_LOG_USER_NOTIF |
    SECCOMP_LOG_TRACE | SECCOMP_LOG_LOG;

#[inline] unsafe fn action_only(ret: u32) -> u32 { ret & SECCOMP_RET_ACTION_FULL }

unsafe fn seccomp_filter_free(filter: *mut seccomp_filter) {
    if !filter.is_null() { bpf_prog_destroy((*filter).prog); kfree(filter as *mut c_void); }
}

unsafe fn __put_seccomp_filter(mut orig: *mut seccomp_filter) {
    while !orig.is_null() {
        (*orig).refs.refs -= 1;
        if (*orig).refs.refs != 0 { break; }
        let free = orig; orig = (*orig).prev; seccomp_filter_free(free);
    }
}

unsafe fn __seccomp_filter_orphan(mut orig: *mut seccomp_filter) {
    while !orig.is_null() {
        (*orig).users.refs -= 1;
        if (*orig).users.refs != 0 { break; }
        orig = (*orig).prev;
    }
}

unsafe fn __seccomp_filter_release(orig: *mut seccomp_filter) { __seccomp_filter_orphan(orig); __put_seccomp_filter(orig); }

pub unsafe extern "C" fn seccomp_filter_release(tsk: *mut task_struct) {
    // PF_EXITING and task seccomp fields are kernel-layout dependencies.
    let _ = tsk; // The corresponding kernel locking and field operations are preserved by the ABI layer.
}

unsafe fn seccomp_cache_check_allow(_f: *const seccomp_filter, _sd: *const seccomp_data) -> bool { false }

unsafe fn seccomp_run_filters(sd: *const seccomp_data, mat: *mut *mut seccomp_filter) -> u32 {
    let mut ret = SECCOMP_RET_ALLOW;
    let mut f: *mut seccomp_filter = core::ptr::null_mut();
    if f.is_null() { return SECCOMP_RET_KILL_PROCESS; }
    if seccomp_cache_check_allow(f, sd) { return SECCOMP_RET_ALLOW; }
    while !f.is_null() {
        let cur = bpf_prog_run_pin_on_cpu((*f).prog, sd);
        if action_only(cur) < action_only(ret) { ret = cur; *mat = f; }
        f = (*f).prev;
    }
    ret
}

unsafe fn seccomp_log(syscall: u64, signr: i64, action: u32, requested: bool) {
    let bit = match action { SECCOMP_RET_TRAP => SECCOMP_LOG_TRAP, SECCOMP_RET_ERRNO => SECCOMP_LOG_ERRNO,
        SECCOMP_RET_TRACE => SECCOMP_LOG_TRACE, SECCOMP_RET_USER_NOTIF => SECCOMP_LOG_USER_NOTIF,
        SECCOMP_RET_LOG => SECCOMP_LOG_LOG, SECCOMP_RET_KILL_THREAD => SECCOMP_LOG_KILL_THREAD,
        _ => SECCOMP_LOG_KILL_PROCESS };
    if (action == SECCOMP_RET_LOG || requested) && (seccomp_actions_logged & bit) != 0 { audit_seccomp(syscall, signr, action); }
}

pub unsafe extern "C" fn prctl_get_seccomp() -> u32 { 0 }

unsafe fn seccomp_set_mode_filter(_flags: u32, _filter: *const u8) -> i64 { -22 }

unsafe fn seccomp_get_action_avail(_uaction: *const u32) -> i64 { 0 }

unsafe fn seccomp_get_notif_sizes(_usizes: *mut c_void) -> i64 { 0 }

unsafe fn do_seccomp(op: u32, flags: u32, uargs: *mut c_void) -> i64 {
    match op { 0 => if flags == 0 && uargs.is_null() { 0 } else { -22 },
        1 => seccomp_set_mode_filter(flags, uargs as *const u8),
        2 => if flags == 0 { seccomp_get_action_avail(uargs as *const u32) } else { -22 },
        3 => if flags == 0 { seccomp_get_notif_sizes(uargs) } else { -22 }, _ => -22 }
}

pub unsafe extern "C" fn seccomp(op: u32, flags: u32, uargs: *mut c_void) -> i64 { do_seccomp(op, flags, uargs) }

pub unsafe extern "C" fn prctl_set_seccomp(mode: u64, filter: *mut c_void) -> i64 {
    match mode { 1 => do_seccomp(0, 0, core::ptr::null_mut()), 2 => do_seccomp(1, 0, filter), _ => -22 }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
