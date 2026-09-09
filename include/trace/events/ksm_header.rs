/* SPDX-License-Identifier: GPL-2.0 */
// Translated from trace/events/ksm.h.  The C tracepoint registration macros
// are represented here by their event payload layouts and assignment helpers.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KsmScanTemplate {
    pub seq: core::ffi::c_int,
    pub rmap_entries: u32,
}

#[inline]
pub const fn ksm_scan_template(seq: core::ffi::c_int, rmap_entries: u32) -> KsmScanTemplate {
    KsmScanTemplate { seq, rmap_entries }
}

pub type KsmStartScan = KsmScanTemplate;
pub type KsmStopScan = KsmScanTemplate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KsmEnterExitTemplate {
    pub mm: *mut core::ffi::c_void,
}

#[inline]
pub const fn ksm_enter_exit_template(mm: *mut core::ffi::c_void) -> KsmEnterExitTemplate {
    KsmEnterExitTemplate { mm }
}

pub type KsmEnter = KsmEnterExitTemplate;
pub type KsmExit = KsmEnterExitTemplate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KsmMergeOnePage {
    pub pfn: core::ffi::c_ulong,
    pub rmap_item: *mut core::ffi::c_void,
    pub mm: *mut core::ffi::c_void,
    pub err: core::ffi::c_int,
}

#[inline]
pub const fn ksm_merge_one_page(
    pfn: core::ffi::c_ulong,
    rmap_item: *mut core::ffi::c_void,
    mm: *mut core::ffi::c_void,
    err: core::ffi::c_int,
) -> KsmMergeOnePage {
    KsmMergeOnePage { pfn, rmap_item, mm, err }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KsmMergeWithKsmPage {
    pub ksm_page: *mut core::ffi::c_void,
    pub pfn: core::ffi::c_ulong,
    pub rmap_item: *mut core::ffi::c_void,
    pub mm: *mut core::ffi::c_void,
    pub err: core::ffi::c_int,
}

#[inline]
pub const fn ksm_merge_with_ksm_page(
    ksm_page: *mut core::ffi::c_void,
    pfn: core::ffi::c_ulong,
    rmap_item: *mut core::ffi::c_void,
    mm: *mut core::ffi::c_void,
    err: core::ffi::c_int,
) -> KsmMergeWithKsmPage {
    KsmMergeWithKsmPage { ksm_page, pfn, rmap_item, mm, err }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KsmRemoveKsmPage {
    pub pfn: core::ffi::c_ulong,
}

#[inline]
pub const fn ksm_remove_ksm_page(pfn: core::ffi::c_ulong) -> KsmRemoveKsmPage {
    KsmRemoveKsmPage { pfn }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KsmRemoveRmapItem {
    pub pfn: core::ffi::c_ulong,
    pub rmap_item: *mut core::ffi::c_void,
    pub mm: *mut core::ffi::c_void,
}

#[inline]
pub const fn ksm_remove_rmap_item(
    pfn: core::ffi::c_ulong,
    rmap_item: *mut core::ffi::c_void,
    mm: *mut core::ffi::c_void,
) -> KsmRemoveRmapItem {
    KsmRemoveRmapItem { pfn, rmap_item, mm }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KsmAdvisor {
    pub scan_time: i64,
    pub pages_to_scan: core::ffi::c_ulong,
    pub cpu_percent: core::ffi::c_uint,
}

#[inline]
pub const fn ksm_advisor(
    scan_time: i64,
    pages_to_scan: core::ffi::c_ulong,
    cpu_percent: core::ffi::c_uint,
) -> KsmAdvisor {
    KsmAdvisor { scan_time, pages_to_scan, cpu_percent }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
