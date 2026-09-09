/* SPDX-License-Identifier: GPL-2.0-only */

/* Per group capacity. Must be a power of 2! */
pub const TMIGR_CHILDREN_PER_GROUP: usize = 8;

/* External types supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct timerqueue_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct timerqueue_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct raw_spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tmigr_hierarchy {
    pub cpumask: *mut cpumask,
    pub root: *mut tmigr_group,
    pub capacity: libc::c_ulong,
    pub node: list_head,
    pub level_list: [list_head; 0],
}

#[repr(C)]
pub struct tmigr_event {
    pub nextevt: timerqueue_node,
    pub cpu: libc::c_uint,
    pub ignore: bool,
}

#[repr(C)]
pub struct tmigr_group {
    pub lock: raw_spinlock_t,
    pub parent: *mut tmigr_group,
    pub groupevt: tmigr_event,
    pub next_expiry: u64,
    pub events: timerqueue_head,
    pub migr_state: atomic_t,
    pub level: libc::c_uint,
    pub numa_node: libc::c_int,
    pub num_children: libc::c_uint,
    pub groupmask: u8,
    pub list: list_head,
}

#[repr(C)]
pub struct tmigr_cpu {
    pub lock: raw_spinlock_t,
    pub available: bool,
    pub idle: bool,
    pub remote: bool,
    pub tmgroup: *mut tmigr_group,
    pub groupmask: u8,
    pub wakeup: u64,
    pub cpuevt: tmigr_event,
}

#[repr(C)]
pub union tmigr_state {
    pub state: u32,
    pub parts: tmigr_state_parts,
}

#[repr(C, packed)]
pub struct tmigr_state_parts {
    pub active: u8,
    pub migrator: u8,
    pub seq: u16,
}

/* The declarations below are enabled when CONFIG_SMP and CONFIG_NO_HZ_COMMON are defined. */
#[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_NO_HZ_COMMON"))]
extern "C" {
    pub fn tmigr_handle_remote();
    pub fn tmigr_requires_handle_remote() -> bool;
    pub fn tmigr_cpu_activate();
    pub fn tmigr_cpu_deactivate(nextevt: u64) -> u64;
    pub fn tmigr_cpu_new_timer(nextevt: u64) -> u64;
    pub fn tmigr_quick_check(nextevt: u64) -> u64;
}

/* Fallback inline definitions when CONFIG_SMP or CONFIG_NO_HZ_COMMON is absent. */
#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_NO_HZ_COMMON")))]
#[inline]
pub unsafe fn tmigr_handle_remote() {}

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_NO_HZ_COMMON")))]
#[inline]
pub unsafe fn tmigr_requires_handle_remote() -> bool { false }

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_NO_HZ_COMMON")))]
#[inline]
pub unsafe fn tmigr_cpu_activate() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
