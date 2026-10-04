// SPDX-License-Identifier: GPL-2.0
// Rust owner of mm/compaction.c; native ABI is imported from configured headers.
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/compaction_native_generated.rs"
    ));
}
use b::*;
use core::cmp::{max, min};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_long, c_uint, c_ulong, c_void};

#[cfg(CONFIG_COMPACTION)]
const HPAGE_FRAG_CHECK_INTERVAL_MSEC: c_uint = 500;
#[cfg(CONFIG_COMPACTION)]
const COMPACT_MAX_DEFER_SHIFT: c_uint = 6;

// Macros preserve the C disabled-config non-evaluation rule.
#[cfg(CONFIG_COMPACTION)]
macro_rules! count_compact_event {
    ($item:expr) => {
        rust_compaction_count_vm_event($item)
    };
}
#[cfg(not(CONFIG_COMPACTION))]
macro_rules! count_compact_event {
    ($item:expr) => {{}};
}
#[cfg(CONFIG_COMPACTION)]
macro_rules! count_compact_events {
    ($item:expr, $delta:expr) => {
        rust_compaction_count_vm_events($item, $delta as c_long)
    };
}
#[cfg(not(CONFIG_COMPACTION))]
macro_rules! count_compact_events {
    ($item:expr, $delta:expr) => {{}};
}
#[inline]
fn is_via_compact_memory(order: c_int) -> bool {
    #[cfg(CONFIG_COMPACTION)]
    {
        order == -1
    }
    #[cfg(not(CONFIG_COMPACTION))]
    {
        false
    }
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
fn block_start_pfn(pfn: c_ulong, order: c_uint) -> c_ulong {
    pfn & !(1 as c_ulong).wrapping_shl(order).wrapping_sub(1)
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
fn block_end_pfn(pfn: c_ulong, order: c_uint) -> c_ulong {
    let mask = (1 as c_ulong).wrapping_shl(order).wrapping_sub(1);
    pfn.wrapping_add(1).wrapping_add(mask) & !mask
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
unsafe fn pageblock_start_pfn(pfn: c_ulong) -> c_ulong {
    block_start_pfn(pfn, rust_compaction_pageblock_order())
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
unsafe fn pageblock_end_pfn(pfn: c_ulong) -> c_ulong {
    block_end_pfn(pfn, rust_compaction_pageblock_order())
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
unsafe fn pageblock_nr_pages() -> c_ulong {
    (1 as c_ulong).wrapping_shl(rust_compaction_pageblock_order())
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
unsafe fn list_init(head: *mut list_head) {
    rust_compaction_init_list_head(head);
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
unsafe fn list_empty(head: *const list_head) -> bool {
    rust_compaction_list_empty(head)
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
unsafe fn list_del(entry: *mut list_head) {
    rust_compaction_list_del(entry);
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
unsafe fn list_add(entry: *mut list_head, head: *mut list_head) {
    rust_compaction_list_add(entry, head);
}
#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
#[inline]
unsafe fn list_add_tail(entry: *mut list_head, head: *mut list_head) {
    rust_compaction_list_add_tail(entry, head);
}

#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
include!("compaction_isolation.rs");
#[cfg(CONFIG_COMPACTION)]
include!("compaction_scanners.rs");
#[cfg(CONFIG_COMPACTION)]
include!("compaction_policy.rs");
#[cfg(CONFIG_COMPACTION)]
include!("compaction_daemon.rs");

// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
