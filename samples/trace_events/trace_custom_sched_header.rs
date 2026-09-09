/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of the C trace_custom_sched.h header.  The C include
 * guard and TRACE_CUSTOM_EVENT machinery are represented by ordinary Rust
 * declarations; their registration is supplied by the surrounding tracing
 * implementation.
 */

use core::ffi::c_char;

/* Supplied by the kernel/task_struct dependency. */
#[repr(C)]
pub struct task_struct {
    pub prio: u16,
    pub pid: i32,
}

/* TRACE_CUSTOM_EVENT(sched_switch, ...). */
#[repr(C)]
pub struct SchedSwitchEntry {
    pub prev_prio: u16,
    pub next_prio: u16,
    pub next_pid: i32,
}

pub type SchedSwitchProto = unsafe extern "C" fn(
    preempt: bool,
    prev: *mut task_struct,
    next: *mut task_struct,
    prev_state: u32,
);

pub const SCHED_SWITCH_PRINTK: &[u8] =
    b"prev_prio=%d next_pid=%d next_prio=%d\0";

#[inline]
pub unsafe fn sched_switch_fast_assign(
    entry: *mut SchedSwitchEntry,
    prev: *const task_struct,
    next: *const task_struct,
) {
    (*entry).prev_prio = (*prev).prio;
    (*entry).next_pid = (*next).pid;
    (*entry).next_prio = (*next).prio;
}

/* TRACE_CUSTOM_EVENT(sched_waking, ...). */
#[repr(C)]
pub struct SchedWakingEntry {
    pub pid: i32,
    pub prio: u16,
}

pub type SchedWakingProto = unsafe extern "C" fn(p: *mut task_struct);

pub const SCHED_WAKING_PRINTK: &[u8] = b"pid=%d prio=%d\0";

#[inline]
pub unsafe fn sched_waking_fast_assign(entry: *mut SchedWakingEntry, p: *const task_struct) {
    (*entry).pid = (*p).pid;
    (*entry).prio = (*p).prio;
}

/*
 * The following C preprocessor directives configure trace generation outside
 * the include guard.  Rust has no direct equivalent; the generated tracing
 * registration is provided by the dependent trace implementation.
 */
pub const TRACE_INCLUDE_PATH: &str = ".";
pub const TRACE_INCLUDE_FILE: &str = "trace_custom_sched";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
