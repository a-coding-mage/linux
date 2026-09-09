/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 Google, Inc.
 *
 * Author:
 *	Colin Cross <ccross@android.com>
 */

/*
 * When a CPU goes to a low power state that turns off power to the CPU's
 * power domain, the contents of some blocks (floating point coprocessors,
 * interrupt controllers, caches, timers) in the same power domain can be lost.
 * The cpm_pm notifiers provide a method for platform idle, suspend, and
 * hotplug implementations to notify the drivers for these blocks that they
 * may be reset.
 *
 * All cpu_pm notifications must be called with interrupts disabled.
 *
 * The notifications are split into two classes: CPU notifications and CPU
 * cluster notifications.
 *
 * CPU notifications apply to a single CPU and must be called on the affected
 * CPU. They are used to save per-cpu context for affected blocks.
 *
 * CPU cluster notifications apply to all CPUs in a single power domain. They
 * are used to save any global context for affected blocks, and must be called
 * after all the CPUs in the power domain have been notified of the low power
 * state.
 */

/* Event codes passed as unsigned long val to notifier calls. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CpuPmEvent {
    /* A single cpu is entering a low power state. */
    CPU_PM_ENTER,

    /* A single cpu failed to enter a low power state. */
    CPU_PM_ENTER_FAILED,

    /* A single cpu is exiting a low power state. */
    CPU_PM_EXIT,

    /* A cpu power domain is entering a low power state. */
    CPU_CLUSTER_PM_ENTER,

    /* A cpu power domain failed to enter a low power state. */
    CPU_CLUSTER_PM_ENTER_FAILED,

    /* A cpu power domain is exiting a low power state. */
    CPU_CLUSTER_PM_EXIT,
}

/* `CONFIG_CPU_PM` is a build-time configuration condition from the C header. */
#[cfg(feature = "CONFIG_CPU_PM")]
extern "C" {
    pub fn cpu_pm_register_notifier(nb: *mut notifier_block) -> i32;
    pub fn cpu_pm_unregister_notifier(nb: *mut notifier_block) -> i32;
    pub fn cpu_pm_enter() -> i32;
    pub fn cpu_pm_exit() -> i32;
    pub fn cpu_cluster_pm_enter() -> i32;
    pub fn cpu_cluster_pm_exit() -> i32;
}

#[cfg(not(feature = "CONFIG_CPU_PM"))]
#[inline]
pub unsafe fn cpu_pm_register_notifier(_nb: *mut notifier_block) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_CPU_PM"))]
#[inline]
pub unsafe fn cpu_pm_unregister_notifier(_nb: *mut notifier_block) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_CPU_PM"))]
#[inline]
pub unsafe fn cpu_pm_enter() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_CPU_PM"))]
#[inline]
pub unsafe fn cpu_pm_exit() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_CPU_PM"))]
#[inline]
pub unsafe fn cpu_cluster_pm_enter() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_CPU_PM"))]
#[inline]
pub unsafe fn cpu_cluster_pm_exit() -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
