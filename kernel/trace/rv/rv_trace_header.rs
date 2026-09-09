/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/rv/rv_trace.h.
// The Linux tracepoint framework and monitor-specific trace declarations are
// external dependencies and are intentionally not implemented here.

// TRACE_SYSTEM: rv
// Original includes: <linux/rv.h>, <linux/tracepoint.h>

#[cfg(feature = "config_da_mon_events_implicit")]
#[repr(C)]
pub struct EventDaMonitor {
    pub state: *mut core::ffi::c_char,
    pub event: *mut core::ffi::c_char,
    pub next_state: *mut core::ffi::c_char,
    pub final_state: bool,
}

#[cfg(feature = "config_da_mon_events_implicit")]
#[repr(C)]
pub struct ErrorDaMonitor {
    pub state: *mut core::ffi::c_char,
    pub event: *mut core::ffi::c_char,
}

// Original monitor declarations:
// <monitors/wip/wip_trace.h>, <monitors/sco/sco_trace.h>,
// <monitors/scpd/scpd_trace.h>, <monitors/snep/snep_trace.h>,
// <monitors/sts/sts_trace.h>

#[cfg(feature = "config_ha_mon_events_implicit")]
#[repr(C)]
pub struct ErrorEnvDaMonitor {
    pub state: *mut core::ffi::c_char,
    pub event: *mut core::ffi::c_char,
    pub env: *mut core::ffi::c_char,
}

// Original monitor declaration: <monitors/opid/opid_trace.h>

#[cfg(feature = "config_da_mon_events_id")]
#[repr(C)]
pub struct EventDaMonitorId {
    pub id: i32,
    pub state: *mut core::ffi::c_char,
    pub event: *mut core::ffi::c_char,
    pub next_state: *mut core::ffi::c_char,
    pub final_state: bool,
}

#[cfg(feature = "config_da_mon_events_id")]
#[repr(C)]
pub struct ErrorDaMonitorId {
    pub id: i32,
    pub state: *mut core::ffi::c_char,
    pub event: *mut core::ffi::c_char,
}

// Original monitor declarations:
// <monitors/wwnr/wwnr_trace.h>, <monitors/snroc/snroc_trace.h>,
// <monitors/nrp/nrp_trace.h>, <monitors/sssw/sssw_trace.h>

#[cfg(feature = "config_ha_mon_events_id")]
#[repr(C)]
pub struct ErrorEnvDaMonitorId {
    pub id: i32,
    pub state: *mut core::ffi::c_char,
    pub event: *mut core::ffi::c_char,
    pub env: *mut core::ffi::c_char,
}

// Original monitor declarations:
// <monitors/stall/stall_trace.h>, <monitors/nomiss/nomiss_trace.h>

#[cfg(feature = "config_ltl_mon_events_id")]
#[repr(C)]
pub struct EventLtlMonitorId {
    // task->comm and task->pid are copied by the original TP_fast_assign.
    pub comm: *mut core::ffi::c_char,
    pub pid: i32,
    pub states: *mut core::ffi::c_char,
    pub atoms: *mut core::ffi::c_char,
    pub next: *mut core::ffi::c_char,
}

#[cfg(feature = "config_ltl_mon_events_id")]
#[repr(C)]
pub struct ErrorLtlMonitorId {
    // task->comm and task->pid are copied by the original TP_fast_assign.
    pub comm: *mut core::ffi::c_char,
    pub pid: i32,
}

// Original monitor declarations:
// <monitors/pagefault/pagefault_trace.h>, <monitors/sleep/sleep_trace.h>,
// <monitors/wakeup/wakeup_trace.h>

#[cfg(feature = "config_rv_monitor_maintenance_events")]
#[repr(C)]
pub struct RvRetriesError {
    pub name: *mut core::ffi::c_char,
    pub event: *mut core::ffi::c_char,
}

#[cfg(feature = "config_rv_monitor_maintenance_events")]
extern "C" {
    /// Tracepoint: MAX_DA_RETRY_RACING_EVENTS retries reached for an event,
    /// resetting monitor. The tracepoint implementation is external.
    pub fn trace_rv_retries_error(
        name: *mut core::ffi::c_char,
        event: *mut core::ffi::c_char,
    );
}

// Original trace formatting is retained here for the generated tracepoint:
// "%s x %s -> %s%s"; "%s x %s -> %s%s"; "event %s not expected in the
// state %s"; "%s[%d]: (%s) x (%s) -> (%s)"; "%s[%d]: violation detected".

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
