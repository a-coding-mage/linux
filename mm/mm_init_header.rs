/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * mm_init.h:
 *
 * mm/ internal mm_init and memblock declarations
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static check_pages_enabled: static_key;
}

/* Forward declarations supplied by other kernel headers. */
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vmem_altmap {
    _private: [u8; 0],
}

extern "C" {
    pub fn set_zone_contiguous(zone: *mut zone);
    pub fn pfn_range_intersects_zones(
        nid: c_int,
        start_pfn: c_ulong,
        nr_pages: c_ulong,
    ) -> bool;

    pub fn memblock_free_pages(pfn: c_ulong, order: c_uint);

    pub fn memmap_alloc(
        size: phys_addr_t,
        align: phys_addr_t,
        min_addr: phys_addr_t,
        nid: c_int,
        exact_nid: bool,
    ) -> *mut c_void;

    pub fn memmap_init_range(
        size: c_ulong,
        nid: c_int,
        zone: c_ulong,
        start_pfn: c_ulong,
        zone_end_pfn: c_ulong,
        context: mminit_context,
        altmap: *mut vmem_altmap,
        migratetype: c_int,
        isolate_pageblock: bool,
    );

    pub fn init_deferred_page(pfn: c_ulong, nid: c_int);

    pub fn memblock_has_mirror() -> bool;
    pub fn memblock_free_all();

    pub fn __init_single_page(page: *mut page, pfn: c_ulong, zone: c_ulong, nid: c_int);
}

#[repr(C)]
pub struct zone {
    pub contiguous: bool,
}

pub type phys_addr_t = c_ulong;

#[repr(C)]
pub struct static_key {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mminit_context {
    MMINIT_UNDEFINED = 0,
}

#[inline]
pub unsafe fn clear_zone_contiguous(zone: *mut zone) {
    (*zone).contiguous = false;
}

#[cfg(any(CONFIG_COMPACTION, CONFIG_CMA))]
extern "C" {
    pub fn init_cma_reserved_pageblock(page: *mut page);
}

/* CONFIG_CMA selects the external implementation; otherwise the C inline is empty. */
#[cfg(CONFIG_CMA)]
extern "C" {
    pub fn init_cma_pageblock(page: *mut page);
}

/* Keep the declaration-only debug logging interface from the C header. */
#[cfg(CONFIG_DEBUG_MEMORY_INIT)]
#[macro_export]
macro_rules! mminit_dprintk {
    ($($arg:tt)*) => {{
        /* Implemented by the kernel printk layer. */
    }};
}

/* The C inline is empty when CONFIG_CMA is disabled. */
#[cfg(not(CONFIG_CMA))]
#[inline]
pub unsafe fn init_cma_pageblock(_page: *mut page) {}

/*
 * The following declaration is retained only for source-level compatibility
 * with the C static-key helper used by deferred_pages_enabled.
 */
#[cfg(CONFIG_DEFERRED_STRUCT_PAGE_INIT)]
extern "C" {
    fn static_branch_unlikely(key: *const static_key) -> bool;
}

#[cfg(CONFIG_DEFERRED_STRUCT_PAGE_INIT)]
extern "C" {
    static deferred_pages: static_key;
    pub fn deferred_grow_zone(zone: *mut zone, order: c_uint) -> bool;
}

#[cfg(CONFIG_DEFERRED_STRUCT_PAGE_INIT)]
#[inline]
pub unsafe fn deferred_pages_enabled() -> bool {
    static_branch_unlikely(&deferred_pages)
}

#[cfg(not(CONFIG_DEFERRED_STRUCT_PAGE_INIT))]
#[inline]
pub fn deferred_pages_enabled() -> bool {
    false
}

pub const MMINIT_WARNING: mminit_level = mminit_level::MMINIT_WARNING;
pub const MMINIT_VERIFY: mminit_level = mminit_level::MMINIT_VERIFY;
pub const MMINIT_TRACE: mminit_level = mminit_level::MMINIT_TRACE;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mminit_level {
    MMINIT_WARNING,
    MMINIT_VERIFY,
    MMINIT_TRACE,
}

#[cfg(CONFIG_DEBUG_MEMORY_INIT)]
extern "C" {
    pub static mut mminit_loglevel: c_int;
    pub fn mminit_verify_pageflags_layout();
    pub fn mminit_verify_zonelist();
}

#[cfg(not(CONFIG_DEBUG_MEMORY_INIT))]
#[inline]
pub unsafe fn mminit_dprintk(
    _level: mminit_level,
    _prefix: *const c_char,
    _fmt: *const c_char,
) {
}

#[cfg(not(CONFIG_DEBUG_MEMORY_INIT))]
#[inline]
pub unsafe fn mminit_verify_pageflags_layout() {}

#[cfg(not(CONFIG_DEBUG_MEMORY_INIT))]
#[inline]
pub unsafe fn mminit_verify_zonelist() {}

extern "C" {
    pub static mut mirrored_kernelcore: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
