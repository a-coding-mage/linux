/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/types.h supplies the C integer and boolean types used by
// this header. The task_struct definition is supplied by another dependency.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

/*
 * task->jobctl flags
 */
pub const JOBCTL_STOP_SIGMASK: usize = 0xffff; /* signr of the last group stop */

pub const JOBCTL_STOP_DEQUEUED_BIT: usize = 16; /* stop signal dequeued */
pub const JOBCTL_STOP_PENDING_BIT: usize = 17; /* task should stop for group stop */
pub const JOBCTL_STOP_CONSUME_BIT: usize = 18; /* consume group stop count */
pub const JOBCTL_TRAP_STOP_BIT: usize = 19; /* trap for STOP */
pub const JOBCTL_TRAP_NOTIFY_BIT: usize = 20; /* trap for NOTIFY */
pub const JOBCTL_TRAPPING_BIT: usize = 21; /* switching to TRACED */
pub const JOBCTL_LISTENING_BIT: usize = 22; /* ptracer is listening for events */
pub const JOBCTL_TRAP_FREEZE_BIT: usize = 23; /* trap for cgroup freezer */
pub const JOBCTL_PTRACE_FROZEN_BIT: usize = 24; /* frozen for ptrace */

pub const JOBCTL_STOPPED_BIT: usize = 26; /* do_signal_stop() */
pub const JOBCTL_TRACED_BIT: usize = 27; /* ptrace_stop() */

pub const JOBCTL_STOP_DEQUEUED: usize = 1usize << JOBCTL_STOP_DEQUEUED_BIT;
pub const JOBCTL_STOP_PENDING: usize = 1usize << JOBCTL_STOP_PENDING_BIT;
pub const JOBCTL_STOP_CONSUME: usize = 1usize << JOBCTL_STOP_CONSUME_BIT;
pub const JOBCTL_TRAP_STOP: usize = 1usize << JOBCTL_TRAP_STOP_BIT;
pub const JOBCTL_TRAP_NOTIFY: usize = 1usize << JOBCTL_TRAP_NOTIFY_BIT;
pub const JOBCTL_TRAPPING: usize = 1usize << JOBCTL_TRAPPING_BIT;
pub const JOBCTL_LISTENING: usize = 1usize << JOBCTL_LISTENING_BIT;
pub const JOBCTL_TRAP_FREEZE: usize = 1usize << JOBCTL_TRAP_FREEZE_BIT;
pub const JOBCTL_PTRACE_FROZEN: usize = 1usize << JOBCTL_PTRACE_FROZEN_BIT;

pub const JOBCTL_STOPPED: usize = 1usize << JOBCTL_STOPPED_BIT;
pub const JOBCTL_TRACED: usize = 1usize << JOBCTL_TRACED_BIT;

pub const JOBCTL_TRAP_MASK: usize = JOBCTL_TRAP_STOP | JOBCTL_TRAP_NOTIFY;
pub const JOBCTL_PENDING_MASK: usize = JOBCTL_STOP_PENDING | JOBCTL_TRAP_MASK;

extern "C" {
    pub fn task_set_jobctl_pending(task: *mut task_struct, mask: usize) -> bool;
    pub fn task_clear_jobctl_trapping(task: *mut task_struct);
    pub fn task_clear_jobctl_pending(task: *mut task_struct, mask: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
