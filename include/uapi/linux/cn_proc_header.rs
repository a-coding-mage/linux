/* SPDX-License-Identifier: LGPL-2.1 WITH Linux-syscall-note */
/* Translated from cn_proc.h. */

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum proc_cn_mcast_op {
    PROC_CN_MCAST_LISTEN = 1,
    PROC_CN_MCAST_IGNORE = 2,
}

pub const PROC_EVENT_ALL: u32 = PROC_EVENT_FORK
    | PROC_EVENT_EXEC
    | PROC_EVENT_UID
    | PROC_EVENT_GID
    | PROC_EVENT_SID
    | PROC_EVENT_PTRACE
    | PROC_EVENT_COMM
    | PROC_EVENT_NONZERO_EXIT
    | PROC_EVENT_COREDUMP
    | PROC_EVENT_EXIT;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum proc_cn_event {
    PROC_EVENT_NONE = 0x00000000,
    PROC_EVENT_FORK = 0x00000001,
    PROC_EVENT_EXEC = 0x00000002,
    PROC_EVENT_UID = 0x00000004,
    PROC_EVENT_GID = 0x00000040,
    PROC_EVENT_SID = 0x00000080,
    PROC_EVENT_PTRACE = 0x00000100,
    PROC_EVENT_COMM = 0x00000200,
    PROC_EVENT_NONZERO_EXIT = 0x20000000,
    PROC_EVENT_COREDUMP = 0x40000000,
    PROC_EVENT_EXIT = 0x80000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_input {
    pub mcast_op: proc_cn_mcast_op,
    pub event_type: proc_cn_event,
}

#[inline]
pub fn valid_event(ev_type: proc_cn_event) -> proc_cn_event {
    // C enum bitwise operations are represented through their underlying u32.
    unsafe { core::mem::transmute((ev_type as u32) & PROC_EVENT_ALL) }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_event {
    pub what: proc_cn_event,
    pub cpu: __u32,
    pub timestamp_ns: __u64,
    pub event_data: proc_event_event_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_event_ack {
    pub err: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fork_proc_event {
    pub parent_pid: __kernel_pid_t,
    pub parent_tgid: __kernel_pid_t,
    pub child_pid: __kernel_pid_t,
    pub child_tgid: __kernel_pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exec_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union id_proc_event_r {
    pub ruid: __u32,
    pub rgid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union id_proc_event_e {
    pub euid: __u32,
    pub egid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct id_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub r: id_proc_event_r,
    pub e: id_proc_event_e,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sid_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub tracer_pid: __kernel_pid_t,
    pub tracer_tgid: __kernel_pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub comm: [i8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coredump_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub parent_pid: __kernel_pid_t,
    pub parent_tgid: __kernel_pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exit_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub exit_code: __u32,
    pub exit_signal: __u32,
    pub parent_pid: __kernel_pid_t,
    pub parent_tgid: __kernel_pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union proc_event_event_data {
    pub ack: proc_event_ack,
    pub fork: fork_proc_event,
    pub exec: exec_proc_event,
    pub id: id_proc_event,
    pub sid: sid_proc_event,
    pub ptrace: ptrace_proc_event,
    pub comm: comm_proc_event,
    pub coredump: coredump_proc_event,
    pub exit: exit_proc_event,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
