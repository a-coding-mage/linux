/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/sched/sysctl.h. */

/* CONFIG_DETECT_HUNG_TASK selects the external variable; otherwise the
 * header supplies a zero constant to avoid conditionals elsewhere. */
#[cfg(feature = "CONFIG_DETECT_HUNG_TASK")]
extern "C" {
    pub static mut sysctl_hung_task_timeout_secs: usize;
}

#[cfg(not(feature = "CONFIG_DETECT_HUNG_TASK"))]
pub const sysctl_hung_task_timeout_secs: usize = 0;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum sched_tunable_scaling {
    SCHED_TUNABLESCALING_NONE = 0,
    SCHED_TUNABLESCALING_LOG = 1,
    SCHED_TUNABLESCALING_LINEAR = 2,
    SCHED_TUNABLESCALING_END = 3,
}

pub const NUMA_BALANCING_DISABLED: u32 = 0x0;
pub const NUMA_BALANCING_NORMAL: u32 = 0x1;
pub const NUMA_BALANCING_MEMORY_TIERING: u32 = 0x2;

#[cfg(feature = "CONFIG_NUMA_BALANCING")]
extern "C" {
    pub static mut sysctl_numa_balancing_mode: i32;
}

#[cfg(not(feature = "CONFIG_NUMA_BALANCING"))]
pub const sysctl_numa_balancing_mode: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
