/* SPDX-License-Identifier: GPL-2.0 */
// Translation of trace/events/huge_memory.h.
// The Linux tracepoint and format-string machinery is supplied externally.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScanStatus {
    ScanFail,
    ScanSucceed,
    ScanNoPteTable,
    ScanPmdMapped,
    ScanExceedNonePte,
    ScanExceedSwapPte,
    ScanExceedSharedPte,
    ScanPteNonPresent,
    ScanPteUffd,
    ScanPteMappedHugepage,
    ScanLackReferencedPage,
    ScanPageNull,
    ScanScanAbort,
    ScanPageCount,
    ScanPageLru,
    ScanPageLock,
    ScanPageAnon,
    ScanPageLazyfree,
    ScanPageCompound,
    ScanAnyProcess,
    ScanVmaNull,
    ScanVmaCheck,
    ScanAddressRange,
    ScanDelPageLru,
    ScanAllocHugePageFail,
    ScanCgroupChargeFail,
    ScanTruncated,
    ScanPageHasPrivate,
    ScanStoreFailed,
    ScanCopyMc,
    ScanPageFilled,
    ScanPageDirtyOrWriteback,
}

pub const SCAN_STATUS: &[(ScanStatus, &str)] = &[
    (ScanStatus::ScanFail, "failed"),
    (ScanStatus::ScanSucceed, "succeeded"),
    (ScanStatus::ScanNoPteTable, "no_pte_table"),
    (ScanStatus::ScanPmdMapped, "page_pmd_mapped"),
    (ScanStatus::ScanExceedNonePte, "exceed_none_pte"),
    (ScanStatus::ScanExceedSwapPte, "exceed_swap_pte"),
    (ScanStatus::ScanExceedSharedPte, "exceed_shared_pte"),
    (ScanStatus::ScanPteNonPresent, "pte_non_present"),
    (ScanStatus::ScanPteUffd, "pte_uffd_wp"),
    (ScanStatus::ScanPteMappedHugepage, "pte_mapped_hugepage"),
    (ScanStatus::ScanLackReferencedPage, "lack_referenced_page"),
    (ScanStatus::ScanPageNull, "page_null"),
    (ScanStatus::ScanScanAbort, "scan_aborted"),
    (ScanStatus::ScanPageCount, "not_suitable_page_count"),
    (ScanStatus::ScanPageLru, "page_not_in_lru"),
    (ScanStatus::ScanPageLock, "page_locked"),
    (ScanStatus::ScanPageAnon, "page_not_anon"),
    (ScanStatus::ScanPageLazyfree, "page_lazyfree"),
    (ScanStatus::ScanPageCompound, "page_compound"),
    (ScanStatus::ScanAnyProcess, "no_process_for_page"),
    (ScanStatus::ScanVmaNull, "vma_null"),
    (ScanStatus::ScanVmaCheck, "vma_check_failed"),
    (ScanStatus::ScanAddressRange, "not_suitable_address_range"),
    (ScanStatus::ScanDelPageLru, "could_not_delete_page_from_lru"),
    (ScanStatus::ScanAllocHugePageFail, "alloc_huge_page_failed"),
    (ScanStatus::ScanCgroupChargeFail, "ccgroup_charge_failed"),
    (ScanStatus::ScanTruncated, "truncated"),
    (ScanStatus::ScanPageHasPrivate, "page_has_private"),
    (ScanStatus::ScanStoreFailed, "store_failed"),
    (ScanStatus::ScanCopyMc, "copy_poisoned_page"),
    (ScanStatus::ScanPageFilled, "page_filled"),
    (ScanStatus::ScanPageDirtyOrWriteback, "page_dirty_or_writeback"),
];

// struct mm_struct, struct folio, struct file, and pgoff_t are external Linux types.
#[repr(C)]
pub struct MmKhugepagedScanPmd {
    pub mm: *mut core::ffi::c_void,
    pub pfn: u64,
    pub referenced: i32,
    pub none_or_zero: i32,
    pub status: i32,
    pub unmapped: i32,
}

#[repr(C)]
pub struct MmCollapseHugePage { pub mm: *mut core::ffi::c_void, pub isolated: i32, pub status: i32, pub order: u32 }
#[repr(C)]
pub struct MmCollapseHugePageIsolate { pub pfn: u64, pub none_or_zero: i32, pub referenced: i32, pub status: i32, pub order: u32 }
#[repr(C)]
pub struct MmCollapseHugePageSwapin { pub mm: *mut core::ffi::c_void, pub swapped_in: i32, pub referenced: i32, pub ret: i32, pub order: u32 }
#[repr(C)]
pub struct MmKhugepagedScanFile { pub mm: *mut core::ffi::c_void, pub pfn: u64, pub filename: *const core::ffi::c_char, pub present: i32, pub swap: i32, pub result: i32 }
#[repr(C)]
pub struct MmKhugepagedCollapseFile { pub mm: *mut core::ffi::c_void, pub hpfn: u64, pub index: isize, pub addr: u64, pub is_shmem: bool, pub filename: *const core::ffi::c_char, pub nr: i32, pub result: i32 }
#[repr(C)]
pub struct MmKhugepagedScan { pub mm: *mut core::ffi::c_void, pub progress: u32, pub full_scan_finished: bool }

// TRACE_EVENT declarations:
// mm_khugepaged_scan_pmd(mm, folio, referenced, none_or_zero, status, unmapped)
// mm_collapse_huge_page(mm, isolated, status, order)
// mm_collapse_huge_page_isolate(folio, none_or_zero, referenced, status, order)
// mm_collapse_huge_page_swapin(mm, swapped_in, referenced, ret, order)
// mm_khugepaged_scan_file(mm, folio, file, present, swap, result)
// mm_khugepaged_collapse_file(mm, new_folio, index, addr, is_shmem, file, nr, result)
// mm_khugepaged_scan(mm, progress, full_scan_finished)


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
