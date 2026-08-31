// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Data structures shared between BPF and tools. */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum syscall_aggr_mode {
    SYSCALL_AGGR_THREAD,
    SYSCALL_AGGR_CPU,
    SYSCALL_AGGR_CGROUP,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct syscall_key {
    pub cgroup: u64,
    pub cpu_or_tid: i32,
    pub nr: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct syscall_stats {
    pub total_time: u64,
    pub squared_sum: u64,
    pub max_time: u64,
    pub min_time: u64,
    pub count: u32,
    pub error: u32,
}
