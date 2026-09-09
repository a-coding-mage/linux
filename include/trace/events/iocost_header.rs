/* SPDX-License-Identifier: GPL-2.0 */
// Translated from trace/events/iocost.h.
// The C tracepoint construction macros are represented by declarative metadata;
// their registration and emission are supplied by the tracepoint subsystem.

#[repr(C)]
pub struct ioc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ioc_now {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ioc_gq {
    _private: [u8; 0],
}

pub type IocostIocgState = unsafe extern "C" fn(
    iocg: *mut ioc_gq,
    path: *const core::ffi::c_char,
    now: *mut ioc_now,
    last_period: u64,
    cur_period: u64,
    vtime: u64,
);

pub type IocgInuseUpdate = unsafe extern "C" fn(
    iocg: *mut ioc_gq,
    path: *const core::ffi::c_char,
    now: *mut ioc_now,
    old_inuse: u32,
    new_inuse: u32,
    old_hw_inuse: u64,
    new_hw_inuse: u64,
);

pub type IocostIocVrateAdj = unsafe extern "C" fn(
    ioc: *mut ioc,
    new_vrate: u64,
    missed_ppm: *mut u32,
    rq_wait_pct: u32,
    nr_lagging: core::ffi::c_int,
    nr_shortages: core::ffi::c_int,
);

pub type IocostIocgForgiveDebt = unsafe extern "C" fn(
    iocg: *mut ioc_gq,
    path: *const core::ffi::c_char,
    now: *mut ioc_now,
    usage_pct: u32,
    old_debt: u64,
    new_debt: u64,
    old_delay: u64,
    new_delay: u64,
);

// DECLARE_EVENT_CLASS(iocost_iocg_state):
// fields: devname:string, cgroup:string, now:u64, vnow:u64, vrate:u64,
// last_period:u64, cur_period:u64, vtime:u64, weight:u32, inuse:u32,
// hweight_active:u64, hweight_inuse:u64.
pub const IOCOST_IOCG_STATE: &str = "iocost_iocg_state";
pub const IOCOST_IOCG_ACTIVATE: &str = "iocost_iocg_activate";
pub const IOCOST_IOCG_IDLE: &str = "iocost_iocg_idle";

// DECLARE_EVENT_CLASS(iocg_inuse_update):
// fields: devname:string, cgroup:string, now:u64, old_inuse:u32,
// new_inuse:u32, old_hweight_inuse:u64, new_hweight_inuse:u64.
pub const IOCG_INUSE_UPDATE: &str = "iocg_inuse_update";
pub const IOCOST_INUSE_SHORTAGE: &str = "iocost_inuse_shortage";
pub const IOCOST_INUSE_TRANSFER: &str = "iocost_inuse_transfer";
pub const IOCOST_INUSE_ADJUST: &str = "iocost_inuse_adjust";

// TRACE_EVENT(iocost_ioc_vrate_adj):
// fields: devname:string, old_vrate:u64, new_vrate:u64, busy_level:int,
// read_missed_ppm:u32, write_missed_ppm:u32, rq_wait_pct:u32,
// nr_lagging:int, nr_shortages:int.
pub const IOCOST_IOC_VRATE_ADJ: &str = "iocost_ioc_vrate_adj";

// TRACE_EVENT(iocost_iocg_forgive_debt):
// fields: devname:string, cgroup:string, now:u64, vnow:u64,
// usage_pct:u32, old_debt:u64, new_debt:u64, old_delay:u64, new_delay:u64.
pub const IOCOST_IOCG_FORGIVE_DEBT: &str = "iocost_iocg_forgive_debt";

// The C header's include guard and trace/define_trace.h inclusion are
// preprocessor-only and have no executable Rust equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
