/* SPDX-License-Identifier: GPL-2.0 */
// TRACE_SYSTEM: task
// The tracepoint definitions below correspond to the C TRACE_EVENT declarations
// in this header.  `TASK_COMM_LEN` is supplied by the kernel dependencies.

use core::ffi::{c_char, c_int, c_ulong, c_short, c_void};

#[allow(non_camel_case_types)]
pub type pid_t = c_int;

extern "C" {
    pub static TASK_COMM_LEN: usize;
}

#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
    pub comm: *const c_char,
    pub signal: *mut signal_struct,
}

#[repr(C)]
pub struct signal_struct {
    pub oom_score_adj: c_short,
}

#[repr(C)]
pub struct task_newtask_entry {
    pub pid: pid_t,
    pub comm: [c_char; 16], // TASK_COMM_LEN
    pub clone_flags: u64,
    pub oom_score_adj: c_short,
}

#[repr(C)]
pub struct task_rename_entry {
    pub pid: pid_t,
    pub oldcomm: [c_char; 16], // TASK_COMM_LEN
    pub newcomm: [c_char; 16], // TASK_COMM_LEN
    pub oom_score_adj: c_short,
}

#[repr(C)]
pub struct task_prctl_unknown_entry {
    pub option: c_int,
    pub arg2: c_ulong,
    pub arg3: c_ulong,
    pub arg4: c_ulong,
    pub arg5: c_ulong,
}

extern "C" {
    pub fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
}

/// task_prctl_unknown - called on unknown prctl() option
/// @option: option passed
/// @arg2:   arg2 passed
/// @arg3:   arg3 passed
/// @arg4:   arg4 passed
/// @arg5:   arg5 passed
///
/// Called on an unknown prctl() option.

#[inline]
pub unsafe fn task_newtask_fast_assign(
    entry: *mut task_newtask_entry,
    task: *mut task_struct,
    clone_flags: u64,
) {
    (*entry).pid = (*task).pid;
    memcpy((*entry).comm.as_mut_ptr().cast(), (*task).comm.cast(), 16);
    (*entry).clone_flags = clone_flags;
    (*entry).oom_score_adj = (*(*task).signal).oom_score_adj;
}

#[inline]
pub unsafe fn task_rename_fast_assign(
    entry: *mut task_rename_entry,
    task: *mut task_struct,
    comm: *const c_char,
) {
    (*entry).pid = (*task).pid;
    memcpy((*entry).oldcomm.as_mut_ptr().cast(), (*task).comm.cast(), 16);
    strscpy((*entry).newcomm.as_mut_ptr(), comm, 16);
    (*entry).oom_score_adj = (*(*task).signal).oom_score_adj;
}

#[inline]
pub unsafe fn task_prctl_unknown_fast_assign(
    entry: *mut task_prctl_unknown_entry,
    option: c_int,
    arg2: c_ulong,
    arg3: c_ulong,
    arg4: c_ulong,
    arg5: c_ulong,
) {
    (*entry).option = option;
    (*entry).arg2 = arg2;
    (*entry).arg3 = arg3;
    (*entry).arg4 = arg4;
    (*entry).arg5 = arg5;
}

// TP_printk formats:
// task_newtask: "pid=%d comm=%s clone_flags=%llx oom_score_adj=%hd"
// task_rename: "pid=%d oldcomm=%s newcomm=%s oom_score_adj=%hd"
// task_prctl_unknown: "option=%d arg2=%ld arg3=%ld arg4=%ld arg5=%ld"


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
