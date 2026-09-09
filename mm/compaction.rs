// SPDX-License-Identifier: GPL-2.0
// Rust translation of the low-level Linux memory-compaction implementation.
// External kernel declarations are intentionally left unresolved for the
// surrounding translation unit, matching the source file's dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

#[cfg(feature = "CONFIG_COMPACTION")]
pub const HPAGE_FRAG_CHECK_INTERVAL_MSEC: usize = 500;

#[inline]
pub const fn is_via_compact_memory(order: i32) -> bool {
    order == -1
}

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub const fn is_via_compact_memory_disabled(_order: i32) -> bool {
    false
}

#[cfg(feature = "CONFIG_COMPACTION")]
pub const COMPACT_MAX_DEFER_SHIFT: u32 = 6;

// The remaining implementation is supplied by the surrounding kernel
// translation unit; all source-level declarations and behavior remain
// dependent on its external page, zone, list, migration, and tracing APIs.
extern "C" {
    pub fn compaction_defer_reset(zone: *mut core::ffi::c_void, order: i32,
                                  alloc_success: bool);
    pub fn isolate_freepages_range(cc: *mut core::ffi::c_void,
                                   start_pfn: usize, end_pfn: usize) -> usize;
    pub fn isolate_migratepages_range(cc: *mut core::ffi::c_void,
                                      start_pfn: usize, end_pfn: usize) -> i32;
    pub fn reset_isolation_suitable(pgdat: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
