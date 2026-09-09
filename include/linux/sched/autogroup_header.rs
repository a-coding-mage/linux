/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations from the surrounding kernel translation.
#[repr(C)]
pub struct signal_struct;
#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct task_group;
#[repr(C)]
pub struct seq_file;

#[cfg(feature = "CONFIG_SCHED_AUTOGROUP")]
extern "C" {
    pub fn sched_autogroup_create_attach(p: *mut task_struct);
    pub fn sched_autogroup_detach(p: *mut task_struct);
    pub fn sched_autogroup_fork(sig: *mut signal_struct);
    pub fn sched_autogroup_exit(sig: *mut signal_struct);
    pub fn sched_autogroup_exit_task(p: *mut task_struct);

    #[cfg(feature = "CONFIG_PROC_FS")]
    pub fn proc_sched_autogroup_show_task(p: *mut task_struct, m: *mut seq_file);
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub fn proc_sched_autogroup_set_nice(p: *mut task_struct, nice: core::ffi::c_int) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_SCHED_AUTOGROUP"))]
#[inline]
pub unsafe fn sched_autogroup_create_attach(_p: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_SCHED_AUTOGROUP"))]
#[inline]
pub unsafe fn sched_autogroup_detach(_p: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_SCHED_AUTOGROUP"))]
#[inline]
pub unsafe fn sched_autogroup_fork(_sig: *mut signal_struct) {}

#[cfg(not(feature = "CONFIG_SCHED_AUTOGROUP"))]
#[inline]
pub unsafe fn sched_autogroup_exit(_sig: *mut signal_struct) {}

#[cfg(not(feature = "CONFIG_SCHED_AUTOGROUP"))]
#[inline]
pub unsafe fn sched_autogroup_exit_task(_p: *mut task_struct) {}

#[cfg(feature = "CONFIG_CGROUP_SCHED")]
extern "C" {
    pub static mut root_task_group: task_group;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
