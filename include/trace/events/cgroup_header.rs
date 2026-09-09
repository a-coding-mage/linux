/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/cgroup.h.
// The Linux cgroup and tracepoint definitions referenced by this header are
// supplied by other translation units.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong};

// struct cgroup and struct task_struct are external C types.
#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CgroupRootEvent {
    pub root: c_int,
    pub ss_mask: c_uint,
    pub name: *const c_char,
}

#[repr(C)]
pub struct CgroupEvent {
    pub root: c_int,
    pub level: c_int,
    pub id: c_ulonglong,
    pub path: *const c_char,
}

#[repr(C)]
pub struct CgroupMigrateEvent {
    pub dst_root: c_int,
    pub dst_level: c_int,
    pub dst_id: c_ulonglong,
    pub pid: c_int,
    pub dst_path: *const c_char,
    pub comm: *const c_char,
}

#[repr(C)]
pub struct CgroupEventEvent {
    pub root: c_int,
    pub level: c_int,
    pub id: c_ulonglong,
    pub path: *const c_char,
    pub val: c_int,
}

#[repr(C)]
pub struct CgroupRstatEvent {
    pub root: c_int,
    pub level: c_int,
    pub id: c_ulonglong,
    pub cpu: c_int,
    pub contended: bool,
}

// DECLARE_EVENT_CLASS/DEFINE_EVENT declarations from the C tracepoint API.
// The event payload layouts above preserve each TP_STRUCT__entry definition;
// event registration and trace formatting are provided by the tracepoint
// dependency.

pub const CGROUP_ROOT_EVENTS: [&str; 3] = [
    "cgroup_setup_root",
    "cgroup_destroy_root",
    "cgroup_remount",
];

pub const CGROUP_EVENTS: [&str; 6] = [
    "cgroup_mkdir",
    "cgroup_rmdir",
    "cgroup_release",
    "cgroup_rename",
    "cgroup_freeze",
    "cgroup_unfreeze",
];

pub const CGROUP_MIGRATE_EVENTS: [&str; 2] = [
    "cgroup_attach_task",
    "cgroup_transfer_tasks",
];

pub const CGROUP_EVENT_EVENTS: [&str; 2] = [
    "cgroup_notify_populated",
    "cgroup_notify_frozen",
];

/*
 * Related to locks:
 * global rstat_base_lock for base stats
 * cgroup_subsys::rstat_ss_lock for subsystem stats
 */
pub const CGROUP_RSTAT_EVENTS: [&str; 3] = [
    "cgroup_rstat_lock_contended",
    "cgroup_rstat_locked",
    "cgroup_rstat_unlock",
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
