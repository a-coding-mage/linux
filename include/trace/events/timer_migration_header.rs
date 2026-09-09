/* SPDX-License-Identifier: GPL-2.0-only */

//! Rust translation of the timer migration trace-event header.
//!
//! The C trace-event macros below describe generated tracepoints.  Their
//! payload layouts and format strings are retained here as Rust declarations;
//! the tracepoint implementation is supplied by the surrounding kernel.

use core::ffi::c_void;

#[repr(C)]
pub struct TmigrGroupAndCpu {
    pub group: *mut c_void,
    pub parent: *mut c_void,
    pub lvl: u32,
    pub numa_node: u32,
    pub childmask: u32,
    pub active: u8,
    pub migrator: u8,
}

#[repr(C)]
pub struct TmigrCpugroup {
    pub wakeup: u64,
    pub parent: *mut c_void,
    pub cpu: u32,
}

#[repr(C)]
pub struct TmigrIdle {
    pub nextevt: u64,
    pub wakeup: u64,
    pub parent: *mut c_void,
    pub cpu: u32,
}

#[repr(C)]
pub struct TmigrUpdateEvents {
    pub child: *mut c_void,
    pub group: *mut c_void,
    pub nextevt: u64,
    pub group_next_expiry: u64,
    pub child_evt_expiry: u64,
    pub group_lvl: u32,
    pub child_evtcpu: u32,
    pub child_active: u8,
    pub group_active: u8,
}

pub const TMIGR_GROUP_SET_FORMAT: &str = "group=%p lvl=%d numa=%d";
pub const TMIGR_CONNECT_CHILD_PARENT_FORMAT: &str =
    "group=%p groupmask=%0x parent=%p lvl=%d numa=%d capacity=%d num_children=%d";
pub const TMIGR_CONNECT_CPU_PARENT_FORMAT: &str =
    "cpu=%d groupmask=%0x parent=%p lvl=%d numa=%d capacity=%d num_children=%d";
pub const TMIGR_GROUP_AND_CPU_FORMAT: &str =
    "group=%p lvl=%d numa=%d active=%0x migrator=%0x parent=%p childmask=%0x";
pub const TMIGR_CPUGROUP_FORMAT: &str = "cpu=%d parent=%p wakeup=%llu";
pub const TMIGR_IDLE_FORMAT: &str = "cpu=%d parent=%p nextevt=%llu wakeup=%llu";
pub const TMIGR_UPDATE_EVENTS_FORMAT: &str =
    "child=%p group=%p group_lvl=%d child_active=%0x group_active=%0x nextevt=%llu next_expiry=%llu child_evt_expiry=%llu child_evtcpu=%d";
pub const TMIGR_HANDLE_REMOTE_FORMAT: &str = "group=%p lvl=%d";

// Declaration-only tracepoint interfaces corresponding to DEFINE_EVENT and
// TRACE_EVENT entries.  The concrete tracepoint machinery is external.
extern "C" {
    pub fn tmigr_group_set(group: *mut c_void);
    pub fn tmigr_connect_child_parent(hier: *mut c_void, child: *mut c_void);
    pub fn tmigr_connect_cpu_parent(hier: *mut c_void, tmc: *mut c_void);
    pub fn tmigr_group_set_cpu_inactive(group: *mut c_void, state: *const c_void, childmask: u32);
    pub fn tmigr_group_set_cpu_active(group: *mut c_void, state: *const c_void, childmask: u32);
    pub fn tmigr_cpu_new_timer(tmc: *mut c_void);
    pub fn tmigr_cpu_active(tmc: *mut c_void);
    pub fn tmigr_cpu_available(tmc: *mut c_void);
    pub fn tmigr_cpu_unavailable(tmc: *mut c_void);
    pub fn tmigr_handle_remote_cpu(tmc: *mut c_void);
    pub fn tmigr_cpu_idle(tmc: *mut c_void, nextevt: u64);
    pub fn tmigr_cpu_new_timer_idle(tmc: *mut c_void, nextevt: u64);
    pub fn tmigr_update_events(
        child: *mut c_void,
        group: *mut c_void,
        childstate: *const c_void,
        groupstate: *const c_void,
        nextevt: u64,
    );
    pub fn tmigr_handle_remote(group: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
