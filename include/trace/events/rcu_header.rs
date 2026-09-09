/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of `trace/events/rcu.h`.
//!
//! The C tracepoint registration macros are represented by their payload
//! layouts.  Configuration conditions remain build-time conditions supplied by
//! the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct RcuUtilizationEvent {
    pub s: *const c_char,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuGracePeriodEvent {
    pub rcuname: *const c_char,
    pub gp_seq: c_long,
    pub gpevent: *const c_char,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuFutureGracePeriodEvent {
    pub rcuname: *const c_char,
    pub gp_seq: c_long,
    pub gp_seq_req: c_long,
    pub level: u8,
    pub grplo: c_int,
    pub grphi: c_int,
    pub gpevent: *const c_char,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuGracePeriodInitEvent {
    pub rcuname: *const c_char,
    pub gp_seq: c_long,
    pub level: u8,
    pub grplo: c_int,
    pub grphi: c_int,
    pub qsmask: c_ulong,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuExpGracePeriodEvent {
    pub rcuname: *const c_char,
    pub gpseq: c_long,
    pub gpevent: *const c_char,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuExpFunnelLockEvent {
    pub rcuname: *const c_char,
    pub level: u8,
    pub grplo: c_int,
    pub grphi: c_int,
    pub gpevent: *const c_char,
}

#[cfg(all(feature = "config_tree_rcu", feature = "config_rcu_nocb_cpu"))]
#[repr(C)]
pub struct RcuNocbWakeEvent {
    pub rcuname: *const c_char,
    pub cpu: c_int,
    pub reason: *const c_char,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuPreemptTaskEvent {
    pub rcuname: *const c_char,
    pub gp_seq: c_long,
    pub pid: c_int,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuUnlockPreemptedTaskEvent {
    pub rcuname: *const c_char,
    pub gp_seq: c_long,
    pub pid: c_int,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuQuiescentStateReportEvent {
    pub rcuname: *const c_char,
    pub gp_seq: c_long,
    pub mask: c_ulong,
    pub qsmask: c_ulong,
    pub level: u8,
    pub grplo: c_int,
    pub grphi: c_int,
    pub gp_tasks: u8,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuFqsEvent {
    pub rcuname: *const c_char,
    pub gp_seq: c_long,
    pub cpu: c_int,
    pub qsevent: *const c_char,
}

#[cfg(feature = "config_tree_rcu")]
#[repr(C)]
pub struct RcuStallWarningEvent {
    pub rcuname: *const c_char,
    pub msg: *const c_char,
}

#[repr(C)]
pub struct RcuWatchingEvent {
    pub polarity: *const c_char,
    pub oldnesting: c_long,
    pub newnesting: c_long,
    pub counter: c_int,
}

#[repr(C)]
pub struct RcuCallbackEvent {
    pub rcuname: *const c_char,
    pub rhp: *mut c_void,
    pub func: *mut c_void,
    pub qlen: c_long,
}

/// Number of callback-list segments, supplied by the RCU implementation.
pub const RCUTORTURENAME_LEN: usize = 8;

#[repr(C)]
pub struct RcuSegcbStatsEvent {
    pub ctx: *const c_char,
    pub gp_seq: *mut c_ulong,
    pub seglen: *mut c_long,
}

#[repr(C)]
pub struct RcuBatchStartEvent {
    pub rcuname: *const c_char,
    pub qlen: c_long,
    pub blimit: c_long,
}

#[repr(C)]
pub struct RcuInvokeCallbackEvent {
    pub rcuname: *const c_char,
    pub rhp: *mut c_void,
    pub func: *mut c_void,
}

#[repr(C)]
pub struct RcuInvokeKvfreeCallbackEvent {
    pub rcuname: *const c_char,
    pub rhp: *mut c_void,
    pub offset: c_ulong,
}

#[repr(C)]
pub struct RcuInvokeKfreeBulkCallbackEvent {
    pub rcuname: *const c_char,
    pub nr_records: c_ulong,
    pub p: *mut *mut c_void,
}

#[repr(C)]
pub struct RcuSrNormalEvent {
    pub rcuname: *const c_char,
    pub rhp: *mut c_void,
    pub srevent: *const c_char,
}

#[repr(C)]
pub struct RcuBatchEndEvent {
    pub rcuname: *const c_char,
    pub callbacks_invoked: c_int,
    pub cb: c_char,
    pub nr: c_char,
    pub iit: c_char,
    pub risk: c_char,
}

#[repr(C)]
pub struct RcuTortureReadEvent {
    pub rcutorturename: [c_char; RCUTORTURENAME_LEN],
    pub rhp: *mut c_void,
    pub secs: c_ulong,
    pub c_old: c_ulong,
    pub c: c_ulong,
}

#[repr(C)]
pub struct RcuBarrierEvent {
    pub rcuname: *const c_char,
    pub s: *const c_char,
    pub cpu: c_int,
    pub cnt: c_int,
    pub done: c_ulong,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
