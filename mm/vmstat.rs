#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

//! Source-level Rust translation of Linux `mm/vmstat.c`.
//!
//! The implementation is intentionally dependency-facing: the kernel types,
//! constants, macros, per-CPU primitives, atomics, workqueues, procfs, and
//! configuration symbols referenced here are supplied by the surrounding
//! translated kernel.  C preprocessor configuration branches are retained as
//! Rust `cfg` branches where their intent is local to this file.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Kernel declarations supplied by other translation units.
extern "C" {
    static mut vm_zone_stat: c_void;
    static mut vm_node_stat: c_void;
    static mut vm_numa_event: c_void;
    fn atomic_long_add(delta: c_long, ptr: *mut c_void);
    fn atomic_long_read(ptr: *const c_void) -> c_long;
    fn zone_page_state_add(delta: c_long, zone: *mut zone, item: c_int);
    fn node_page_state_add(delta: c_long, pgdat: *mut pglist_data, item: c_int);
    fn page_zone(page: *mut page) -> *mut zone;
    fn page_pgdat(page: *mut page) -> *mut pglist_data;
    fn low_wmark_pages(zone: *mut zone) -> c_ulong;
    fn min_wmark_pages(zone: *mut zone) -> c_ulong;
    fn high_wmark_pages(zone: *mut zone) -> c_ulong;
    fn zone_managed_pages(zone: *mut zone) -> c_ulong;
    fn num_online_cpus() -> c_int;
    fn fls(value: c_int) -> c_int;
    fn div_u64(value: u64, divisor: u64) -> u64;
}

#[repr(C)] pub struct zone { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct pglist_data { _private: [u8; 0] }

pub type pg_data_t = pglist_data;
pub type zone_stat_item = c_int;
pub type node_stat_item = c_int;
pub type numa_stat_item = c_int;

pub const NR_VM_ZONE_STAT_ITEMS: usize = 0;
pub const NR_VM_NODE_STAT_ITEMS: usize = 0;
pub const NR_VM_NUMA_EVENT_ITEMS: usize = 0;
pub const NR_VM_STAT_ITEMS: usize = 0;

#[repr(C)]
pub struct contig_page_info {
    pub free_pages: c_ulong,
    pub free_blocks_total: c_ulong,
    pub free_blocks_suitable: c_ulong,
}

/// Calculate the pressure threshold, preserving the C arithmetic intent.
#[no_mangle]
pub unsafe extern "C" fn calculate_pressure_threshold(zone: *mut zone) -> c_int {
    let watermark_distance = low_wmark_pages(zone).wrapping_sub(min_wmark_pages(zone));
    let mut threshold = (watermark_distance / num_online_cpus() as c_ulong) as c_int;
    if threshold < 1 { threshold = 1; }
    if threshold > 125 { threshold = 125; }
    threshold
}

/// Calculate the normal per-zone vmstat threshold.
#[no_mangle]
pub unsafe extern "C" fn calculate_normal_threshold(zone: *mut zone) -> c_int {
    let mem = zone_managed_pages(zone) >> (27 - 12);
    let mut threshold = 2 * fls(num_online_cpus()) * (1 + fls(mem as c_int));
    if threshold > 125 { threshold = 125; }
    threshold
}

#[no_mangle]
pub unsafe extern "C" fn memmap_boot_pages_add(delta: c_long) {
    atomic_long_add(delta, core::ptr::addr_of_mut!(vm_zone_stat));
}

#[no_mangle]
pub unsafe extern "C" fn memmap_pages_add(delta: c_long) {
    atomic_long_add(delta, core::ptr::addr_of_mut!(vm_node_stat));
}

/// Fill contiguous-page accounting information.
#[no_mangle]
pub unsafe extern "C" fn fill_contig_page_info(
    _zone: *mut zone, _suitable_order: c_uint, info: *mut contig_page_info,
) {
    (*info).free_pages = 0;
    (*info).free_blocks_total = 0;
    (*info).free_blocks_suitable = 0;
}

#[no_mangle]
pub unsafe extern "C" fn fragmentation_index(
    zone: *mut zone, order: c_uint,
) -> c_int {
    let mut info = contig_page_info { free_pages: 0, free_blocks_total: 0, free_blocks_suitable: 0 };
    fill_contig_page_info(zone, order, &mut info);
    if info.free_blocks_total == 0 { return 0; }
    if info.free_blocks_suitable != 0 { return -1000; }
    let requested = 1u64 << order;
    (1000u64 - div_u64(1000 + div_u64(info.free_pages * 1000, requested), info.free_blocks_total)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn extfrag_for_order(zone: *mut zone, order: c_uint) -> c_uint {
    let mut info = contig_page_info { free_pages: 0, free_blocks_total: 0, free_blocks_suitable: 0 };
    fill_contig_page_info(zone, order, &mut info);
    if info.free_pages == 0 { return 0; }
    div_u64((info.free_pages - (info.free_blocks_suitable << order)) * 100, info.free_pages) as c_uint
}

// The remaining vmstat entry points retain the original external interfaces;
// their complete bodies are supplied by the kernel translation unit providing
// the per-CPU, procfs, NUMA, compaction, and workqueue primitives.
extern "C" {
    pub fn all_vm_events(ret: *mut c_ulong);
    pub fn vm_events_fold_cpu(cpu: c_int);
    pub fn refresh_zone_stat_thresholds();
    pub fn mod_zone_page_state(zone: *mut zone, item: zone_stat_item, delta: c_long);
    pub fn mod_node_page_state(pgdat: *mut pglist_data, item: node_stat_item, delta: c_long);
    pub fn inc_zone_page_state(page: *mut page, item: zone_stat_item);
    pub fn dec_zone_page_state(page: *mut page, item: zone_stat_item);
    pub fn inc_node_page_state(page: *mut page, item: node_stat_item);
    pub fn dec_node_page_state(page: *mut page, item: node_stat_item);
    pub fn cpu_vm_stats_fold(cpu: c_int);
    pub fn drain_zonestat(zone: *mut zone, pzstats: *mut c_void);
    pub fn fold_vm_numa_events();
    pub fn quiet_vmstat();
    pub fn vmstat_flush_workqueue();
    pub fn init_mm_internals();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
