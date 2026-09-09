/* SPDX-License-Identifier: GPL-2.0 */
//! Source-level Rust translation of Linux `mmzone.h`.
//!
//! The included kernel types, constants, atomics, locks, and helper functions
//! are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

pub const MAX_PAGE_ORDER: usize = 10;
pub const MAX_ORDER_NR_PAGES: usize = 1usize << MAX_PAGE_ORDER;
pub const NR_PAGE_ORDERS: usize = MAX_PAGE_ORDER + 1;
pub const PAGE_ALLOC_COSTLY_ORDER: usize = 3;
pub const MAX_FOLIO_ORDER: usize = MAX_PAGE_ORDER;
pub const MAX_FOLIO_NR_PAGES: usize = 1usize << MAX_FOLIO_ORDER;
pub const VMEMMAP_TAIL_MIN_ORDER: usize = 0;
pub const __NR_VMEMMAP_TAILS: usize = MAX_FOLIO_ORDER + 1;
pub const NR_VMEMMAP_TAILS: usize = __NR_VMEMMAP_TAILS;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum migratetype { MIGRATE_UNMOVABLE, MIGRATE_MOVABLE, MIGRATE_RECLAIMABLE,
    MIGRATE_PCPTYPES, MIGRATE_HIGHATOMIC = 3, MIGRATE_CMA, __MIGRATE_TYPE_END,
    MIGRATE_ISOLATE, MIGRATE_TYPES }

extern "C" {
    pub static migratetype_names: [*const core::ffi::c_char; MIGRATE_TYPES as usize];
    pub static mut page_group_by_mobility_disabled: core::ffi::c_int;
}

pub const fn is_migrate_movable(mt: migratetype) -> bool { mt == migratetype::MIGRATE_MOVABLE }
pub const fn migratetype_is_mergeable(mt: migratetype) -> bool { (mt as i32) < migratetype::MIGRATE_PCPTYPES as i32 }

#[repr(C)] pub struct free_area { pub free_list: [list_head; migratetype::MIGRATE_TYPES as usize], pub nr_free: c_ulong }
#[repr(C)] pub struct pglist_data;

#[repr(C)] pub enum zone_stat_item {
    NR_FREE_PAGES, NR_FREE_PAGES_BLOCKS, NR_ZONE_LRU_BASE, NR_ZONE_INACTIVE_ANON = 2,
    NR_ZONE_ACTIVE_ANON, NR_ZONE_INACTIVE_FILE, NR_ZONE_ACTIVE_FILE, NR_ZONE_UNEVICTABLE,
    NR_ZONE_WRITE_PENDING, NR_MLOCK, NR_FREE_CMA_PAGES, NR_VM_ZONE_STAT_ITEMS,
}
#[repr(C)] pub enum node_stat_item {
    NR_LRU_BASE, NR_INACTIVE_ANON, NR_ACTIVE_ANON, NR_INACTIVE_FILE, NR_ACTIVE_FILE,
    NR_UNEVICTABLE, NR_SLAB_RECLAIMABLE_B, NR_SLAB_UNRECLAIMABLE_B, NR_ISOLATED_ANON,
    NR_ISOLATED_FILE, WORKINGSET_NODES, WORKINGSET_REFAULT_BASE, WORKINGSET_REFAULT_ANON,
    WORKINGSET_REFAULT_FILE, WORKINGSET_ACTIVATE_BASE, WORKINGSET_ACTIVATE_ANON,
    WORKINGSET_ACTIVATE_FILE, WORKINGSET_RESTORE_BASE, WORKINGSET_RESTORE_ANON,
    WORKINGSET_RESTORE_FILE, WORKINGSET_NODERECLAIM, NR_ANON_MAPPED, NR_FILE_MAPPED,
    NR_FILE_PAGES, NR_FILE_DIRTY, NR_WRITEBACK, NR_SHMEM, NR_SHMEM_THPS, NR_SHMEM_PMDMAPPED,
    NR_FILE_THPS, NR_FILE_PMDMAPPED, NR_ANON_THPS, NR_VMSCAN_WRITE, NR_VMSCAN_IMMEDIATE,
    NR_DIRTIED, NR_WRITTEN, NR_THROTTLED_WRITTEN, NR_KERNEL_MISC_RECLAIMABLE,
    NR_FOLL_PIN_ACQUIRED, NR_FOLL_PIN_RELEASED, NR_VMALLOC, NR_KERNEL_STACK_KB,
    NR_PAGETABLE, NR_SECONDARY_PAGETABLE, PGDEMOTE_KSWAPD, PGDEMOTE_DIRECT,
    PGDEMOTE_KHUGEPAGED, PGDEMOTE_PROACTIVE, PGSTEAL_KSWAPD, PGSTEAL_DIRECT,
    PGSTEAL_KHUGEPAGED, PGSTEAL_PROACTIVE, PGSTEAL_ANON, PGSTEAL_FILE, PGSCAN_KSWAPD,
    PGSCAN_DIRECT, PGSCAN_KHUGEPAGED, PGSCAN_PROACTIVE, PGSCAN_ANON, PGSCAN_FILE,
    PGROTATE_ANON, PGROTATE_FILE, PGREFILL, NR_BALLOON_PAGES, NR_KERNEL_FILE_PAGES,
    NR_GPU_ACTIVE, NR_GPU_RECLAIM, NR_VM_NODE_STAT_ITEMS,
}
pub const LRU_BASE: usize = 0; pub const LRU_ACTIVE: usize = 1; pub const LRU_FILE: usize = 2;
#[repr(C)] pub enum lru_list { LRU_INACTIVE_ANON=0, LRU_ACTIVE_ANON=1, LRU_INACTIVE_FILE=2, LRU_ACTIVE_FILE=3, LRU_UNEVICTABLE, NR_LRU_LISTS }
#[repr(C)] pub enum vmscan_throttle_state { VMSCAN_THROTTLE_WRITEBACK, VMSCAN_THROTTLE_ISOLATED, VMSCAN_THROTTLE_NOPROGRESS, VMSCAN_THROTTLE_CONGESTED, NR_VMSCAN_THROTTLE }
#[repr(C)] pub enum lruvec_flags { LRUVEC_CGROUP_CONGESTED, LRUVEC_NODE_CONGESTED }

pub const MIN_NR_GENS: u8 = 2; pub const MAX_NR_GENS: u8 = 4; pub const MAX_NR_TIERS: u8 = 4;
pub const ANON_AND_FILE: usize = 2; pub const MEMCG_NR_GENS: usize = 3; pub const MEMCG_NR_BINS: usize = 8;

#[repr(C)] pub struct lru_gen_folio {
    pub max_seq: c_ulong, pub min_seq: [c_ulong; 2], pub timestamps: [c_ulong; 4],
    pub folios: [[[list_head; MAX_NR_ZONES]; 2]; 4], pub nr_pages: [[[c_long; MAX_NR_ZONES]; 2]; 4],
    pub avg_refaulted: [[c_ulong; 4]; 2], pub avg_total: [[c_ulong; 4]; 2],
    pub protected: [[[c_ulong; 4]; 2]; 1], pub evicted: [[[atomic_long_t; 4]; 2]; 1],
    pub refaulted: [[[atomic_long_t; 4]; 2]; 1], pub enabled: bool, pub gen: u8, pub seg: u8,
    pub list: hlist_nulls_node,
}
#[repr(C)] pub struct lru_gen_mm_state { pub seq: c_ulong, pub head: *mut list_head, pub tail: *mut list_head, pub filters: [*mut c_ulong; 2], pub stats: [[c_ulong; 4]; 4] }
#[repr(C)] pub struct lru_gen_mm_walk { pub lruvec: *mut lruvec, pub seq: c_ulong, pub next_addr: c_ulong, pub nr_pages: [[[c_int; MAX_NR_ZONES]; 2]; 4], pub mm_stats: [c_int; 4], pub batched: c_int, pub swappiness: c_int, pub force_scan: bool }
#[repr(C)] pub struct lru_gen_memcg { pub seq: c_ulong, pub nr_memcgs: [c_ulong; 3], pub fifo: [[hlist_nulls_head; 8]; 3], pub lock: spinlock_t }
#[repr(C)] pub struct lru_cost { pub count: c_ulong, pub last_rotated: c_ulong, pub last_io: c_ulong }
#[repr(C)] pub struct lruvec { pub lists: [list_head; 5], pub lru_lock: spinlock_t, pub cost: [lru_cost; 2], pub cost_lock: spinlock_t, pub nonresident_age: atomic_long_t, pub refaults: [c_ulong; 2], pub flags: c_ulong, pub lrugen: lru_gen_folio, pub pgdat: *mut pglist_data, pub zswap_lruvec_state: zswap_lruvec_state }

#[repr(C)] pub enum zone_watermarks { WMARK_MIN, WMARK_LOW, WMARK_HIGH, WMARK_PROMO, NR_WMARK }
#[repr(C)] pub enum zone_type { ZONE_NORMAL, ZONE_MOVABLE, ZONE_DEVICE, __MAX_NR_ZONES }
#[repr(C)] pub struct per_cpu_pages { pub lock: spinlock_t, pub count: c_int, pub high: c_int, pub high_min: c_int, pub high_max: c_int, pub batch: c_int, pub flags: u8, pub alloc_factor: u8, pub expire: u8, pub free_count: i16, pub lists: [list_head; 8] }
#[repr(C)] pub struct per_cpu_zonestat { pub vm_stat_diff: [i8; 1], pub stat_threshold: i8, pub vm_numa_event: [c_ulong; 1] }
#[repr(C)] pub struct per_cpu_nodestat { pub stat_threshold: i8, pub vm_node_stat_diff: [i8; 1] }
#[repr(C)] pub struct zone { pub _watermark: [c_ulong; 4], pub watermark_boost: c_ulong, pub nr_reserved_highatomic: c_ulong, pub nr_free_highatomic: c_ulong, pub lowmem_reserve: [c_long; 1], pub node: c_int, pub zone_pgdat: *mut pglist_data, pub per_cpu_pageset: *mut per_cpu_pages, pub per_cpu_zonestats: *mut per_cpu_zonestat, pub zone_start_pfn: c_ulong, pub managed_pages: atomic_long_t, pub spanned_pages: c_ulong, pub present_pages: c_ulong, pub name: *const c_char, pub initialized: c_int, pub free_area: [free_area; NR_PAGE_ORDERS], pub flags: c_ulong, pub lock: spinlock_t, pub percpu_drift_mark: c_ulong, pub contiguous: bool, pub vm_stat: [atomic_long_t; 1], pub vm_numa_event: [atomic_long_t; 1] }
#[repr(C)] pub struct zoneref { pub zone: *mut zone, pub zone_idx: c_int }
#[repr(C)] pub struct zonelist { pub _zonerefs: [zoneref; 1] }

#[repr(C)] pub struct pglist_data { pub node_zones: [zone; 1], pub node_zonelists: [zonelist; 1], pub nr_zones: c_int, pub node_start_pfn: c_ulong, pub node_present_pages: c_ulong, pub node_spanned_pages: c_ulong, pub node_id: c_int, pub flags: c_ulong, pub __lruvec: lruvec, pub per_cpu_nodestats: *mut per_cpu_nodestat, pub vm_stat: [atomic_long_t; 1] }
pub type pg_data_t = pglist_data;

pub const ASYNC_AND_SYNC: usize = 2; pub const DEF_PRIORITY: usize = 12;
pub const MAX_ZONES_PER_ZONELIST: usize = MAX_NUMNODES * MAX_NR_ZONES;
pub const ZONELIST_FALLBACK: usize = 0; pub const MAX_ZONELISTS: usize = 1;

extern "C" {
    pub static mut mem_map: *mut page;
    pub fn build_all_zonelists(pgdat: *mut pg_data_t);
    pub fn lruvec_init(lruvec: *mut lruvec);
    pub fn first_online_pgdat() -> *mut pg_data_t;
    pub fn next_online_pgdat(pgdat: *mut pg_data_t) -> *mut pg_data_t;
    pub fn next_zone(zone: *mut zone) -> *mut zone;
    pub fn __next_zones_zonelist(z: *mut zoneref, highest_zoneidx: zone_type, nodes: *const nodemask_t) -> *mut zoneref;
    pub fn pfn_valid(pfn: c_ulong) -> c_int;
}

#[inline] pub unsafe fn wmark_pages(z: *const zone, w: zone_watermarks) -> c_ulong { (*z)._watermark[w as usize] + (*z).watermark_boost }
#[inline] pub unsafe fn min_wmark_pages(z: *const zone) -> c_ulong { wmark_pages(z, zone_watermarks::WMARK_MIN) }
#[inline] pub unsafe fn low_wmark_pages(z: *const zone) -> c_ulong { wmark_pages(z, zone_watermarks::WMARK_LOW) }
#[inline] pub unsafe fn high_wmark_pages(z: *const zone) -> c_ulong { wmark_pages(z, zone_watermarks::WMARK_HIGH) }
#[inline] pub unsafe fn promo_wmark_pages(z: *const zone) -> c_ulong { wmark_pages(z, zone_watermarks::WMARK_PROMO) }
#[inline] pub unsafe fn zone_end_pfn(z: *const zone) -> c_ulong { (*z).zone_start_pfn + (*z).spanned_pages }
#[inline] pub unsafe fn zone_spans_pfn(z: *const zone, pfn: c_ulong) -> bool { (*z).zone_start_pfn <= pfn && pfn < zone_end_pfn(z) }
#[inline] pub unsafe fn zone_is_empty(z: *const zone) -> bool { (*z).spanned_pages == 0 }
#[inline] pub unsafe fn zone_intersects(z: *const zone, start_pfn: c_ulong, nr_pages: c_ulong) -> bool { !zone_is_empty(z) && start_pfn < zone_end_pfn(z) && start_pfn + nr_pages > (*z).zone_start_pfn }
#[inline] pub unsafe fn zonelist_zone(z: *const zoneref) -> *mut zone { (*z).zone }
#[inline] pub unsafe fn zonelist_zone_idx(z: *const zoneref) -> c_int { (*z).zone_idx }
#[inline] pub unsafe fn zonelist_node_idx(z: *const zoneref) -> c_int { 0 }

/* External kernel types referenced by this header. */
pub type c_ulong = core::ffi::c_ulong; pub type c_long = core::ffi::c_long; pub type c_int = core::ffi::c_int; pub type c_char = core::ffi::c_char;
pub enum list_head {} pub enum hlist_nulls_node {} pub enum hlist_nulls_head {} pub enum spinlock_t {} pub enum atomic_long_t {} pub enum atomic_t {} pub enum zswap_lruvec_state {} pub enum page {} pub enum nodemask_t {} pub enum mem_cgroup {} pub enum page_ext {} pub enum rcu_head {} pub enum dev_pagemap {} pub type memdesc_flags_t = c_ulong;
pub const MAX_NR_ZONES: usize = 1; pub const MAX_NUMNODES: usize = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
