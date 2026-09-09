/* SPDX-License-Identifier: GPL-2.0 */
// TRACE_SYSTEM pagemap
// C header guards and tracepoint includes are intentionally omitted.

pub const PAGEMAP_MAPPED: u32 = 0x0001;
pub const PAGEMAP_ANONYMOUS: u32 = 0x0002;
pub const PAGEMAP_FILE: u32 = 0x0004;
pub const PAGEMAP_SWAPCACHE: u32 = 0x0008;
pub const PAGEMAP_SWAPBACKED: u32 = 0x0010;
pub const PAGEMAP_MAPPEDDISK: u32 = 0x0020;
pub const PAGEMAP_BUFFERS: u32 = 0x0040;

// Supplied by linux/mm.h in the C source.
#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

pub type lru_list = i32;

unsafe extern "C" {
    pub fn folio_test_anon(folio: *const folio) -> bool;
    pub fn folio_mapped(folio: *const folio) -> bool;
    pub fn folio_test_swapcache(folio: *const folio) -> bool;
    pub fn folio_test_swapbacked(folio: *const folio) -> bool;
    pub fn folio_test_mappedtodisk(folio: *const folio) -> bool;
    pub fn folio_test_private(folio: *const folio) -> bool;
    pub fn folio_pfn(folio: *const folio) -> ::core::ffi::c_ulong;
    pub fn folio_lru_list(folio: *const folio) -> lru_list;
}

#[inline]
pub unsafe fn trace_pagemap_flags(folio: *const folio) -> u32 {
    (if folio_test_anon(folio) {
        PAGEMAP_ANONYMOUS
    } else {
        PAGEMAP_FILE
    }) | (if folio_mapped(folio) { PAGEMAP_MAPPED } else { 0 })
        | (if folio_test_swapcache(folio) {
            PAGEMAP_SWAPCACHE
        } else {
            0
        })
        | (if folio_test_swapbacked(folio) {
            PAGEMAP_SWAPBACKED
        } else {
            0
        })
        | (if folio_test_mappedtodisk(folio) {
            PAGEMAP_MAPPEDDISK
        } else {
            0
        })
        | (if folio_test_private(folio) {
            PAGEMAP_BUFFERS
        } else {
            0
        })
}

#[repr(C)]
pub struct MmLruInsertionEntry {
    pub folio: *mut folio,
    pub pfn: ::core::ffi::c_ulong,
    pub lru: lru_list,
    pub flags: ::core::ffi::c_ulong,
}

#[inline]
pub unsafe fn mm_lru_insertion_entry(folio: *mut folio) -> MmLruInsertionEntry {
    MmLruInsertionEntry {
        folio,
        pfn: folio_pfn(folio),
        lru: folio_lru_list(folio),
        flags: trace_pagemap_flags(folio) as ::core::ffi::c_ulong,
    }
}

#[repr(C)]
pub struct MmLruActivateEntry {
    pub folio: *mut folio,
    pub pfn: ::core::ffi::c_ulong,
}

#[inline]
pub unsafe fn mm_lru_activate_entry(folio: *mut folio) -> MmLruActivateEntry {
    MmLruActivateEntry {
        folio,
        pfn: folio_pfn(folio),
    }
}

// Flag format is based on page-types.c formatting for pagemap.
// TP_printk formats are represented by the event entry layouts above; the
// surrounding TRACE_EVENT machinery is provided by the tracepoint backend.

unsafe extern "C" {
    pub fn mm_lru_add_drain(cpu: i32, nr_folios: u32);
    pub fn mm_lru_add_drain_all(force_all_cpus: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
