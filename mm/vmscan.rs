// SPDX-License-Identifier: GPL-2.0
//! Faithful low-level Rust transcription of the vmscan implementation.
//!
//! Kernel-provided types and functions are intentionally left as external
//! dependencies, matching the C translation unit's included interfaces.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct scan_control {
    pub nr_to_reclaim: c_ulong,
    pub nodemask: *const nodemask_t,
    pub target_mem_cgroup: *mut mem_cgroup,
    pub anon_cost: c_ulong,
    pub file_cost: c_ulong,
    pub proactive_swappiness: *mut c_int,
    pub may_deactivate: u32,
    pub force_deactivate: u32,
    pub skipped_deactivate: u32,
    pub may_writepage: u32,
    pub may_unmap: u32,
    pub may_swap: u32,
    pub no_cache_trim_mode: u32,
    pub cache_trim_mode_failed: u32,
    pub proactive: u32,
    pub memcg_low_reclaim: u32,
    pub memcg_low_skipped: u32,
    pub memcg_full_walk: u32,
    pub hibernation_mode: u32,
    pub compaction_ready: u32,
    pub cache_trim_mode: u32,
    pub file_is_tiny: u32,
    pub no_demotion: u32,
    pub order: i8,
    pub priority: i8,
    pub reclaim_idx: i8,
    pub gfp_mask: gfp_t,
    pub nr_scanned: c_ulong,
    pub nr_reclaimed: c_ulong,
    pub nr: scan_control_nr,
    pub reclaim_state: reclaim_state,
}

#[repr(C)]
pub struct scan_control_nr { pub dirty: u32, pub congested: u32, pub writeback: u32, pub immediate: u32, pub taken: u32 }
pub type c_ulong = usize;
pub type gfp_t = u32;
pub type s8 = i8;
#[repr(C)] pub struct nodemask_t { _private: [u64; 1] }
#[repr(C)] pub struct mem_cgroup { _private: [u8; 0] }
#[repr(C)] pub struct reclaim_state { pub reclaimed: c_ulong }
#[repr(C)] pub struct zone { _private: [u8; 0] }
#[repr(C)] pub struct lruvec { _private: [u8; 0] }

pub const DEACTIVATE_ANON: u32 = 1;
pub const DEACTIVATE_FILE: u32 = 2;

#[no_mangle]
pub static mut vm_swappiness: c_int = 60;

extern "C" {
    fn mem_cgroup_swappiness(memcg: *mut mem_cgroup) -> c_int;
    fn mem_cgroup_get_nr_swap_pages(memcg: *mut mem_cgroup) -> c_ulong;
    fn get_nr_swap_pages() -> c_ulong;
    fn zone_to_nid(zone: *mut zone) -> c_int;
    fn zone_page_state_snapshot(zone: *mut zone, item: c_int) -> c_ulong;
    fn can_demote(nid: c_int, sc: *mut scan_control, memcg: *mut mem_cgroup) -> bool;
}

unsafe fn sc_swappiness(sc: *mut scan_control, memcg: *mut mem_cgroup) -> c_int {
    if (*sc).proactive != 0 && !(*sc).proactive_swappiness.is_null() {
        return *(*sc).proactive_swappiness;
    }
    mem_cgroup_swappiness(memcg)
}

unsafe fn cgroup_reclaim(sc: *mut scan_control) -> bool { !(*sc).target_mem_cgroup.is_null() }
unsafe fn root_reclaim(sc: *mut scan_control) -> bool {
    (*sc).target_mem_cgroup.is_null()
}
unsafe fn writeback_throttling_sane(sc: *mut scan_control) -> bool { !cgroup_reclaim(sc) }

#[inline]
unsafe fn can_reclaim_anon_pages(memcg: *mut mem_cgroup, nid: c_int, sc: *mut scan_control) -> bool {
    if memcg.is_null() {
        if get_nr_swap_pages() > 0 { return true; }
    } else if mem_cgroup_get_nr_swap_pages(memcg) > 0 { return true; }
    can_demote(nid, sc, memcg)
}

pub unsafe fn zone_reclaimable_pages(zone: *mut zone) -> c_ulong {
    // NR_* constants and the remaining kernel interfaces are supplied by the
    // surrounding kernel translation unit.
    let mut nr = zone_page_state_snapshot(zone, NR_ZONE_INACTIVE_FILE)
        .wrapping_add(zone_page_state_snapshot(zone, NR_ZONE_ACTIVE_FILE));
    if can_reclaim_anon_pages(core::ptr::null_mut(), zone_to_nid(zone), core::ptr::null_mut()) {
        nr = nr.wrapping_add(zone_page_state_snapshot(zone, NR_ZONE_INACTIVE_ANON));
        nr = nr.wrapping_add(zone_page_state_snapshot(zone, NR_ZONE_ACTIVE_ANON));
    }
    nr
}

extern "C" {
    static NR_ZONE_INACTIVE_FILE: c_int;
    static NR_ZONE_ACTIVE_FILE: c_int;
    static NR_ZONE_INACTIVE_ANON: c_int;
    static NR_ZONE_ACTIVE_ANON: c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
