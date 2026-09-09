/* SPDX-License-Identifier: GPL-2.0 */

// C source-level translation of the resctrl monitor trace header.
// The original TRACE_EVENT declaration is expanded by the Linux tracepoint
// infrastructure; that external machinery is intentionally not implemented
// here.

/// Payload stored by the `mon_llc_occupancy_limbo` trace event.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MonLlcOccupancyLimboEntry {
    pub ctrl_hw_id: u32,
    pub mon_hw_id: u32,
    pub domain_id: ::core::ffi::c_int,
    pub llc_occupancy_bytes: u64,
}

/// External tracepoint generated from the C `TRACE_EVENT` declaration.
///
/// C prototype:
/// `void trace_mon_llc_occupancy_limbo(u32, u32, int, u64);`
extern "C" {
    pub fn trace_mon_llc_occupancy_limbo(
        ctrl_hw_id: u32,
        mon_hw_id: u32,
        domain_id: ::core::ffi::c_int,
        llc_occupancy_bytes: u64,
    );
}

// TP_ARGS(ctrl_hw_id, mon_hw_id, domain_id, llc_occupancy_bytes)
// TP_fast_assign:
//   __entry->ctrl_hw_id = ctrl_hw_id;
//   __entry->mon_hw_id = mon_hw_id;
//   __entry->domain_id = domain_id;
//   __entry->llc_occupancy_bytes = llc_occupancy_bytes;
// TP_printk("ctrl_hw_id=%u mon_hw_id=%u domain_id=%d llc_occupancy_bytes=%llu",
//           __entry->ctrl_hw_id, __entry->mon_hw_id, __entry->domain_id,
//           __entry->llc_occupancy_bytes)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
