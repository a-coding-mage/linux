/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of trace/events/vmscan.h.  Tracepoint registration is
 * supplied by the surrounding kernel tracing implementation. */

pub const RECLAIM_WB_ANON: u32 = 0x0001;
pub const RECLAIM_WB_FILE: u32 = 0x0002;
pub const RECLAIM_WB_MIXED: u32 = 0x0010;
pub const RECLAIM_WB_SYNC: u32 = 0x0004; // Unused, all reclaim async.
pub const RECLAIM_WB_ASYNC: u32 = 0x0008;
pub const RECLAIM_WB_LRU: u32 = RECLAIM_WB_ANON | RECLAIM_WB_FILE;

pub const _VMSCAN_THROTTLE_WRITEBACK: u32 = 1 << VMSCAN_THROTTLE_WRITEBACK;
pub const _VMSCAN_THROTTLE_ISOLATED: u32 = 1 << VMSCAN_THROTTLE_ISOLATED;
pub const _VMSCAN_THROTTLE_NOPROGRESS: u32 = 1 << VMSCAN_THROTTLE_NOPROGRESS;
pub const _VMSCAN_THROTTLE_CONGESTED: u32 = 1 << VMSCAN_THROTTLE_CONGESTED;

pub const KSWAPD_CLEAR_HOPELESS_OTHER: i32 = 0;
pub const KSWAPD_CLEAR_HOPELESS_KSWAPD: i32 = 1;
pub const KSWAPD_CLEAR_HOPELESS_DIRECT: i32 = 2;
pub const KSWAPD_CLEAR_HOPELESS_PCP: i32 = 3;

pub fn show_reclaim_flags(flags: u32) -> &'static str {
    match flags {
        0 => "RECLAIM_WB_NONE",
        RECLAIM_WB_ANON => "RECLAIM_WB_ANON",
        RECLAIM_WB_FILE => "RECLAIM_WB_FILE",
        RECLAIM_WB_MIXED => "RECLAIM_WB_MIXED",
        RECLAIM_WB_SYNC => "RECLAIM_WB_SYNC",
        RECLAIM_WB_ASYNC => "RECLAIM_WB_ASYNC",
        _ => "RECLAIM_WB_ANON|RECLAIM_WB_FILE|RECLAIM_WB_MIXED|RECLAIM_WB_SYNC|RECLAIM_WB_ASYNC",
    }
}

pub fn show_throttle_flags(flags: u32) -> &'static str {
    match flags {
        0 => "VMSCAN_THROTTLE_NONE",
        _ => "VMSCAN_THROTTLE_WRITEBACK|VMSCAN_THROTTLE_ISOLATED|VMSCAN_THROTTLE_NOPROGRESS|VMSCAN_THROTTLE_CONGESTED",
    }
}

pub const fn trace_reclaim_flags(file: bool) -> u32 {
    (if file { RECLAIM_WB_FILE } else { RECLAIM_WB_ANON }) | RECLAIM_WB_ASYNC
}

#[repr(C)]
pub struct MmVmscanKswapdSleep { pub nid: i32 }
#[repr(C)]
pub struct MmVmscanKswapdWake { pub nid: i32, pub zid: i32, pub order: i32 }
#[repr(C)]
pub struct MmVmscanBalancePgdatBegin { pub nid: i32, pub order: i32, pub highest_zoneidx: i32 }
#[repr(C)]
pub struct MmVmscanBalancePgdatEnd { pub nid: i32, pub order: i32, pub highest_zoneidx: i32, pub nr_reclaimed: usize }
#[repr(C)]
pub struct MmVmscanWakeupKswapd { pub nid: i32, pub zid: i32, pub order: i32, pub gfp_flags: usize }
#[repr(C)]
pub struct MmVmscanDirectReclaimBegin { pub gfp_flags: usize, pub memcg_id: u64, pub order: i32 }
#[repr(C)]
pub struct MmVmscanDirectReclaimEnd { pub nr_reclaimed: usize, pub memcg_id: u64 }
#[repr(C)]
pub struct MmShrinkSlabStart {
    pub shr: *mut core::ffi::c_void, pub shrink: *mut core::ffi::c_void,
    pub nr_objects_to_shrink: isize, pub gfp_flags: usize, pub cache_items: usize,
    pub delta: u64, pub total_scan: usize, pub priority: i32, pub nid: i32, pub memcg_id: u64,
}
#[repr(C)]
pub struct MmShrinkSlabEnd {
    pub shr: *mut core::ffi::c_void, pub shrink: *mut core::ffi::c_void,
    pub unused_scan: isize, pub new_scan: isize, pub total_scan: isize,
    pub nid: i32, pub retval: i32, pub memcg_id: u64,
}
#[repr(C)]
pub struct MmVmscanLruIsolate { pub highest_zoneidx: i32, pub order: i32, pub nr_requested: usize, pub nr_scanned: usize, pub nr_skipped: usize, pub nr_taken: usize, pub lru: i32 }
#[repr(C)]
pub struct MmVmscanWriteFolio { pub pfn: usize, pub reclaim_flags: i32 }
#[repr(C)]
pub struct MmVmscanReclaimPages {
    pub nid: i32, pub nr_scanned: usize, pub nr_reclaimed: usize, pub nr_dirty: usize,
    pub nr_writeback: usize, pub nr_congested: usize, pub nr_immediate: usize,
    pub nr_activate0: u32, pub nr_activate1: u32, pub nr_ref_keep: usize, pub nr_unmap_fail: usize,
}
#[repr(C)]
pub struct MmVmscanLruShrinkInactive {
    pub nid: i32, pub nr_scanned: usize, pub nr_reclaimed: usize, pub nr_dirty: usize,
    pub nr_writeback: usize, pub nr_congested: usize, pub nr_immediate: usize,
    pub nr_activate0: u32, pub nr_activate1: u32, pub nr_ref_keep: usize, pub nr_unmap_fail: usize,
    pub priority: i32, pub reclaim_flags: i32,
}
#[repr(C)]
pub struct MmVmscanLruShrinkActive { pub nid: i32, pub nr_taken: usize, pub nr_active: usize, pub nr_deactivated: usize, pub nr_referenced: usize, pub priority: i32, pub reclaim_flags: i32 }
#[repr(C)]
pub struct MmVmscanNodeReclaimBegin { pub nid: i32, pub order: i32, pub gfp_flags: usize }
#[repr(C)]
pub struct MmVmscanThrottled { pub nid: i32, pub usec_timeout: i32, pub usec_delayed: i32, pub reason: i32 }
#[repr(C)]
pub struct MmVmscanKswapdReclaimFail { pub nid: i32, pub failures: i32 }
#[repr(C)]
pub struct MmVmscanKswapdClearHopeless { pub nid: i32, pub reason: i32 }

/* TRACE_EVENT/DECLARE_EVENT_CLASS/DEFINE_EVENT declarations.  The event
 * names and their C-compatible payload layouts are retained above; the
 * external tracing backend supplies registration, formatting, and emission. */
pub const KSWAPD_CLEAR_HOPELESS_REASON_OPS: &[(i32, &str)] = &[
    (KSWAPD_CLEAR_HOPELESS_KSWAPD, "KSWAPD"),
    (KSWAPD_CLEAR_HOPELESS_DIRECT, "DIRECT"),
    (KSWAPD_CLEAR_HOPELESS_PCP, "PCP"),
    (KSWAPD_CLEAR_HOPELESS_OTHER, "OTHER"),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
