/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Scheduler interfaces for hotplug CPU support:
 */

unsafe extern "C" {
    pub fn sched_cpu_starting(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn sched_cpu_activate(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn sched_cpu_deactivate(cpu: core::ffi::c_uint) -> core::ffi::c_int;
}

/* CONFIG_HOTPLUG_CPU */
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" {
    pub fn sched_cpu_wait_empty(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn sched_cpu_dying(cpu: core::ffi::c_uint) -> core::ffi::c_int;
}

/* When CONFIG_HOTPLUG_CPU is disabled, these C macros expand to NULL. */
#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
pub const sched_cpu_wait_empty: Option<unsafe extern "C" fn(core::ffi::c_uint) -> core::ffi::c_int> = None;
#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
pub const sched_cpu_dying: Option<unsafe extern "C" fn(core::ffi::c_uint) -> core::ffi::c_int> = None;

#[inline]
pub const fn idle_task_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
