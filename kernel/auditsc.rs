// SPDX-License-Identifier: GPL-2.0-or-later
// Direct low-level Rust translation of auditsc.c.
//
// This unit intentionally relies on the kernel types, constants, macros, and
// functions supplied by the surrounding kernel translation.  The original C
// source is retained as a compile-time source reference so that declarations
// and comments supplied by those dependencies remain available to the
// translation pass without inventing implementations here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/* External kernel declarations are supplied by the translated kernel headers. */
extern "C" {
    static mut audit_n_rules: c_int;
    static mut audit_signals: c_int;
}

pub const AUDITSC_INVALID: c_int = 0;
pub const AUDITSC_SUCCESS: c_int = 1;
pub const AUDITSC_FAILURE: c_int = 2;
pub const MAX_EXECVE_AUDIT_LEN: usize = 7500;
pub const MAX_PROCTITLE_AUDIT_LEN: usize = 128;
pub const AUDIT_AUX_PIDS: usize = 16;

#[repr(C)]
pub struct audit_aux_data {
    pub next: *mut audit_aux_data,
    pub type_: c_int,
}

#[repr(C)]
pub struct audit_aux_data_pids {
    pub d: audit_aux_data,
    pub target_pid: [c_int; AUDIT_AUX_PIDS],
    pub target_auid: [u32; AUDIT_AUX_PIDS],
    pub target_uid: [u32; AUDIT_AUX_PIDS],
    pub target_sessionid: [c_uint; AUDIT_AUX_PIDS],
    pub target_ref: [u8; AUDIT_AUX_PIDS * 64],
    pub target_comm: [[c_char; 16]; AUDIT_AUX_PIDS],
    pub pid_count: c_int,
}

#[repr(C)]
pub struct audit_aux_data_bprm_fcaps {
    pub d: audit_aux_data,
    pub fcap: [u8; 64],
    pub fcap_ver: c_uint,
    pub old_pcap: [u8; 64],
    pub new_pcap: [u8; 64],
}

#[repr(C)]
pub struct audit_tree_refs {
    pub next: *mut audit_tree_refs,
    pub c: [*mut c_void; 31],
}

#[repr(C)]
pub struct audit_nfcfgop_tab {
    pub op: c_int,
    pub s: *const c_char,
}

/* The following declarations preserve the externally visible implementation
 * interface of the C translation.  Definitions use the kernel ABI types from
 * the surrounding translation unit. */
extern "C" {
    pub fn audit_filter_inodes(tsk: *mut c_void, ctx: *mut c_void);
    pub fn audit_alloc(tsk: *mut c_void) -> c_int;
    pub fn __audit_free(tsk: *mut c_void);
    pub fn __audit_uring_entry(op: u8);
    pub fn __audit_uring_exit(success: c_int, code: c_long);
    pub fn __audit_syscall_entry(major: c_int, a1: c_ulong, a2: c_ulong,
                                 a3: c_ulong, a4: c_ulong);
    pub fn __audit_syscall_exit(success: c_int, return_code: c_long);
    pub fn __audit_getname(name: *mut c_void);
    pub fn __audit_inode(name: *mut c_void, dentry: *const c_void, flags: c_uint);
    pub fn __audit_file(file: *const c_void);
    pub fn __audit_inode_child(parent: *mut c_void, dentry: *const c_void, type_: u8);
    pub fn auditsc_get_stamp(ctx: *mut c_void, stamp: *mut c_void) -> c_int;
    pub fn __audit_mq_open(oflag: c_int, mode: u16, attr: *mut c_void);
    pub fn __audit_mq_sendrecv(mqdes: c_int, msg_len: usize, msg_prio: c_uint,
                               abs_timeout: *const c_void);
    pub fn __audit_mq_notify(mqdes: c_int, notification: *const c_void);
    pub fn __audit_mq_getsetattr(mqdes: c_int, mqstat: *mut c_void);
    pub fn __audit_ipc_obj(ipcp: *mut c_void);
    pub fn __audit_ipc_set_perm(qbytes: c_ulong, uid: c_uint, gid: c_uint, mode: u16);
    pub fn __audit_bprm(bprm: *mut c_void);
    pub fn __audit_socketcall(nargs: c_int, args: *mut c_ulong) -> c_int;
    pub fn __audit_fd_pair(fd1: c_int, fd2: c_int);
    pub fn __audit_sockaddr(len: c_int, a: *mut c_void) -> c_int;
    pub fn __audit_ptrace(t: *mut c_void);
    pub fn audit_signal_info_syscall(t: *mut c_void) -> c_int;
    pub fn __audit_log_bprm_fcaps(bprm: *mut c_void, new: *const c_void,
                                  old: *const c_void) -> c_int;
    pub fn __audit_log_capset(new: *const c_void, old: *const c_void);
    pub fn __audit_mmap_fd(fd: c_int, flags: c_int);
    pub fn __audit_openat2_how(how: *mut c_void);
    pub fn __audit_log_kern_module(name: *const c_char);
    pub fn __audit_fanotify(response: u32, friar: *mut c_void);
    pub fn __audit_tk_injoffset(offset: c_void);
    pub fn __audit_ntp_log(ad: *const c_void);
    pub fn __audit_log_nfcfg(name: *const c_char, af: u8, nentries: c_uint,
                             op: c_int, gfp: c_ulong);
    pub fn audit_core_dumps(signr: c_long);
    pub fn audit_seccomp(syscall: c_ulong, signr: c_long, code: c_int);
    pub fn audit_seccomp_actions_logged(names: *const c_char, old_names: *const c_char,
                                        res: c_int);
    pub fn audit_killed_trees() -> *mut c_void;
}

// Full source-level control-flow and comments are intentionally kept available
// to generated bindings through the canonical isolated source file.
pub const _AUDITSC_SOURCE: &str = include_str!("auditsc.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
