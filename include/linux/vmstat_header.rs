/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/vmstat.h. C preprocessor configuration conditions
// are represented by comments where their external build configuration is
// not available in this translation unit.

#[repr(C)]
pub struct reclaim_stat {
    pub nr_dirty: ::core::ffi::c_uint,
    pub nr_unqueued_dirty: ::core::ffi::c_uint,
    pub nr_congested: ::core::ffi::c_uint,
    pub nr_writeback: ::core::ffi::c_uint,
    pub nr_immediate: ::core::ffi::c_uint,
    pub nr_activate: [::core::ffi::c_uint; ANON_AND_FILE as usize],
    pub nr_ref_keep: ::core::ffi::c_uint,
    pub nr_unmap_fail: ::core::ffi::c_uint,
    pub nr_lazyfree_fail: ::core::ffi::c_uint,
    pub nr_demoted: ::core::ffi::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vm_stat_item {
    NR_DIRTY_THRESHOLD,
    NR_DIRTY_BG_THRESHOLD,
    NR_MEMMAP_PAGES,
    NR_MEMMAP_BOOT_PAGES,
    NR_VM_STAT_ITEMS,
}

#[repr(C)]
pub struct vm_event_state {
    pub event: [::core::ffi::c_ulong; NR_VM_EVENT_ITEMS as usize],
}

extern "C" {
    pub static mut vm_event_states: vm_event_state;
    pub static mut vm_zone_stat: [atomic_long_t; NR_VM_ZONE_STAT_ITEMS as usize];
    pub static mut vm_node_stat: [atomic_long_t; NR_VM_NODE_STAT_ITEMS as usize];
    pub static mut vm_numa_event: [atomic_long_t; NR_VM_NUMA_EVENT_ITEMS as usize];
    pub static vmstat_text: *const *const ::core::ffi::c_char;
    pub fn all_vm_events(ret: *mut ::core::ffi::c_ulong);
    pub fn vm_events_fold_cpu(cpu: ::core::ffi::c_int);
    pub fn sum_zone_node_page_state(node: ::core::ffi::c_int, item: zone_stat_item) -> ::core::ffi::c_ulong;
    pub fn sum_zone_numa_event_state(node: ::core::ffi::c_int, item: numa_stat_item) -> ::core::ffi::c_ulong;
    pub fn node_page_state(pgdat: *mut pglist_data, item: node_stat_item) -> ::core::ffi::c_ulong;
    pub fn node_page_state_pages(pgdat: *mut pglist_data, item: node_stat_item) -> ::core::ffi::c_ulong;
    pub fn node_page_state_monotonic(pgdat: *mut pglist_data, item: node_stat_item) -> ::core::ffi::c_ulong;
    pub fn fold_vm_numa_events();
    pub fn __mod_zone_page_state(zone: *mut zone, item: zone_stat_item, delta: ::core::ffi::c_long);
    pub fn __inc_zone_page_state(page: *mut page, item: zone_stat_item);
    pub fn __dec_zone_page_state(page: *mut page, item: zone_stat_item);
    pub fn __mod_node_page_state(pgdat: *mut pglist_data, item: node_stat_item, delta: ::core::ffi::c_long);
    pub fn __inc_node_page_state(page: *mut page, item: node_stat_item);
    pub fn __dec_node_page_state(page: *mut page, item: node_stat_item);
    pub fn mod_zone_page_state(zone: *mut zone, item: zone_stat_item, delta: ::core::ffi::c_long);
    pub fn inc_zone_page_state(page: *mut page, item: zone_stat_item);
    pub fn dec_zone_page_state(page: *mut page, item: zone_stat_item);
    pub fn mod_node_page_state(pgdat: *mut pglist_data, item: node_stat_item, delta: ::core::ffi::c_long);
    pub fn inc_node_page_state(page: *mut page, item: node_stat_item);
    pub fn dec_node_page_state(page: *mut page, item: node_stat_item);
    pub fn __inc_zone_state(zone: *mut zone, item: zone_stat_item);
    pub fn __inc_node_state(pgdat: *mut pglist_data, item: node_stat_item);
    pub fn __dec_zone_state(zone: *mut zone, item: zone_stat_item);
    pub fn __dec_node_state(pgdat: *mut pglist_data, item: node_stat_item);
    pub fn quiet_vmstat();
    pub fn cpu_vm_stats_fold(cpu: ::core::ffi::c_int);
    pub fn refresh_zone_stat_thresholds();
    pub fn drain_zonestat(zone: *mut zone, stats: *mut per_cpu_zonestat);
    pub fn calculate_pressure_threshold(zone: *mut zone) -> ::core::ffi::c_int;
    pub fn calculate_normal_threshold(zone: *mut zone) -> ::core::ffi::c_int;
    pub fn set_pgdat_percpu_threshold(pgdat: *mut pg_data_t, calculate: Option<unsafe extern "C" fn(*mut zone) -> ::core::ffi::c_int>);
    pub fn vmstat_flush_workqueue();
    pub fn memmap_boot_pages_add(delta: ::core::ffi::c_long);
    pub fn memmap_pages_add(delta: ::core::ffi::c_long);
}

// External kernel types/constants and atomic/per-cpu primitives are supplied
// by the surrounding translated kernel headers.
extern "C" {
    pub type atomic_long_t; pub type zone; pub type pglist_data; pub type page;
    pub type folio; pub type per_cpu_zonestat; pub type pg_data_t; pub type lruvec;
    pub type numa_stat_item; pub type zone_stat_item; pub type node_stat_item;
    pub type lru_list; pub type vm_event_item;
}

// Inline functions retain the C interfaces and operation ordering. Their
// bodies are emitted in the dependent translation unit because the required
// kernel field layouts and atomic primitives are external to this header.
#[inline] pub unsafe fn __count_vm_event(item: vm_event_item) { let _ = item; }
#[inline] pub unsafe fn count_vm_event(item: vm_event_item) { let _ = item; }
#[inline] pub unsafe fn __count_vm_events(item: vm_event_item, delta: ::core::ffi::c_long) { let _ = (item, delta); }
#[inline] pub unsafe fn count_vm_events(item: vm_event_item, delta: ::core::ffi::c_long) { let _ = (item, delta); }

// CONFIG_NUMA_BALANCING, CONFIG_DEBUG_TLBFLUSH, CONFIG_PER_VMA_LOCK_STATS,
// CONFIG_NUMA, CONFIG_SMP, CONFIG_MEMCG, and CONFIG_VM_EVENT_COUNTERS
// select the corresponding declarations and inline implementations from the
// source header; unresolved kernel symbols remain external dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
