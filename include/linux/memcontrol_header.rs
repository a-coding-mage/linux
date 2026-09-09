/* SPDX-License-Identifier: GPL-2.0-or-later */
// Rust translation of linux/memcontrol.h.  Types supplied by included kernel
// headers remain external dependencies of this header translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)] pub struct mem_cgroup { pub css: cgroup_subsys_state, pub id: mem_cgroup_private_id, pub memory: page_counter, pub swap_or_memsw: page_counter, pub memory_peaks: list_head, pub swap_peaks: list_head, pub peaks_lock: spinlock_t, pub high_work: work_struct, pub zswap_max: c_ulong, pub zswap_writeback: bool, pub vmpressure: vmpressure, pub oom_group: bool, pub events_file: cgroup_file, pub events_local_file: cgroup_file, pub swap_events_file: cgroup_file, pub vmstats: *mut memcg_vmstats, pub memory_events: [atomic_long_t; MEMCG_NR_MEMORY_EVENTS as usize], pub memory_events_local: [atomic_long_t; MEMCG_NR_MEMORY_EVENTS as usize], pub socket_pressure: u64, pub socket_pressure_seqlock: seqlock_t, pub kmemcg_id: c_int, pub nodeinfo: [*mut mem_cgroup_per_node; 0] }
#[repr(C)] pub struct obj_cgroup { pub refcnt: percpu_ref, pub memcg: *mut mem_cgroup, pub nr_charged_bytes: atomic_t, pub list_or_rcu: obj_cgroup_list_rcu, pub is_root: bool }
#[repr(C)] pub union obj_cgroup_list_rcu { pub list: list_head, pub rcu: rcu_head }
#[repr(C)] pub struct mem_cgroup_private_id { pub id: c_int, pub ref_: refcount_t }
#[repr(C)] pub struct mem_cgroup_reclaim_cookie { pub pgdat: *mut pg_data_t, pub generation: c_int }
#[repr(C)] pub struct mem_cgroup_per_node { pub memcg: *mut mem_cgroup, pub lruvec_stats_percpu: *mut lruvec_stats_percpu, pub lruvec_stats: *mut lruvec_stats, pub shrinker_info: *mut shrinker_info, pub lruvec: lruvec, pub lru_zone_size: [[c_ulong; NR_LRU_LISTS as usize]; MAX_NR_ZONES as usize], pub iter: mem_cgroup_reclaim_iter, pub objcg: *mut obj_cgroup, pub orig_objcg: *mut obj_cgroup, pub objcg_list: list_head }
#[repr(C)] pub struct mem_cgroup_reclaim_iter { pub position: *mut mem_cgroup, pub generation: atomic_t }
#[repr(C)] pub struct mem_cgroup_threshold { pub eventfd: *mut eventfd_ctx, pub threshold: c_ulong }
#[repr(C)] pub struct mem_cgroup_threshold_ary { pub current_threshold: c_int, pub size: c_uint, pub entries: [mem_cgroup_threshold; 0] }
#[repr(C)] pub struct mem_cgroup_thresholds { pub primary: *mut mem_cgroup_threshold_ary, pub spare: *mut mem_cgroup_threshold_ary }
#[repr(C)] pub struct memcg_cgwb_frn { pub bdi_id: u64, pub memcg_id: c_int, pub at: u64, pub done: wb_completion }

pub type c_ulong = usize;
#[repr(C)] pub enum memcg_stat_item { MEMCG_SWAP = NR_VM_NODE_STAT_ITEMS, MEMCG_SOCK, MEMCG_PERCPU_B, MEMCG_KMEM, MEMCG_ZSWAP_B, MEMCG_ZSWAPPED, MEMCG_ZSWAP_INCOMP, MEMCG_NR_STAT }
#[repr(C)] pub enum memcg_memory_event { MEMCG_LOW, MEMCG_HIGH, MEMCG_MAX, MEMCG_OOM, MEMCG_OOM_KILL, MEMCG_OOM_GROUP_KILL, MEMCG_SWAP_HIGH, MEMCG_SWAP_MAX, MEMCG_SWAP_FAIL, MEMCG_SOCK_THROTTLED, MEMCG_NR_MEMORY_EVENTS }
#[repr(C)] pub enum page_memcg_data_flags { MEMCG_DATA_OBJEXTS = 1, MEMCG_DATA_KMEM = 2, __NR_MEMCG_DATA_FLAGS = 4 }
#[repr(C)] pub enum objext_flags { OBJEXTS_ALLOC_FAIL = 1, __OBJEXTS_FLAG_UNUSED = 4, __NR_OBJEXTS_FLAGS = 8 }

pub const MEM_CGROUP_ID_SHIFT: u32 = 16;
pub const MEMCG_CGWB_FRN_CNT: usize = 4;
pub const MEMCG_CHARGE_BATCH: u32 = 64;
pub const OBJEXTS_FLAGS_MASK: usize = 7;

extern "C" {
    pub static mut root_mem_cgroup: *mut mem_cgroup;
    pub fn mem_cgroup_calculate_protection(root: *mut mem_cgroup, memcg: *mut mem_cgroup);
    pub fn __mem_cgroup_charge(folio: *mut folio, mm: *mut mm_struct, gfp: gfp_t) -> c_int;
    pub fn __mem_cgroup_uncharge(folio: *mut folio);
    pub fn mem_cgroup_replace_folio(old: *mut folio, new: *mut folio);
    pub fn mem_cgroup_migrate(old: *mut folio, new: *mut folio);
    pub fn mem_cgroup_from_task(p: *mut task_struct) -> *mut mem_cgroup;
    pub fn get_mem_cgroup_from_mm(mm: *mut mm_struct) -> *mut mem_cgroup;
    pub fn get_mem_cgroup_from_current() -> *mut mem_cgroup;
    pub fn get_mem_cgroup_from_folio(folio: *mut folio) -> *mut mem_cgroup;
    pub fn mem_cgroup_init() -> c_int;
}

extern "C" { pub fn mem_cgroup_disabled() -> bool; }

#[inline] pub unsafe fn mem_cgroup_is_root(memcg: *const mem_cgroup) -> bool { memcg == root_mem_cgroup }
#[inline] pub unsafe fn mem_cgroup_charge(folio: *mut folio, mm: *mut mm_struct, gfp: gfp_t) -> c_int { if mem_cgroup_disabled() { 0 } else { __mem_cgroup_charge(folio, mm, gfp) } }
#[inline] pub unsafe fn mem_cgroup_uncharge(folio: *mut folio) { if !mem_cgroup_disabled() { __mem_cgroup_uncharge(folio) } }
#[inline] pub unsafe fn mem_cgroup_tryget(memcg: *mut mem_cgroup) -> bool { memcg.is_null() || css_tryget(&(*memcg).css) }
#[inline] pub unsafe fn mem_cgroup_put(memcg: *mut mem_cgroup) { if !memcg.is_null() { css_put(&mut (*memcg).css) } }
#[inline] pub unsafe fn mem_cgroup_unprotected(target: *mut mem_cgroup, memcg: *mut mem_cgroup) -> bool { mem_cgroup_disabled() || mem_cgroup_is_root(memcg) || target == memcg }

// Remaining declaration-only interfaces and configuration-specific inline
// fallbacks are intentionally represented by their C ABI names; dependent
// kernel headers provide the referenced opaque types and operations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
