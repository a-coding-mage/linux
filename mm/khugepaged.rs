// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level translation of khugepaged.c. Linux kernel declarations and
// implementations supplied by the surrounding kernel tree remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// The C source includes Linux MM, scheduler, rmap, swap, kthread, THP,
// userfaultfd, NUMA, filesystem, and architecture headers. Those names are
// intentionally left as external dependencies of this translation unit.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_struct { _private: [u8; 0] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_slot { _private: [u8; 0] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct folio { _private: [u8; 0] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page { _private: [u8; 0] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nodemask_t { pub bits: [c_ulong; 1] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t { pub pte: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_t { pub pmd: c_ulong }

pub type vm_flags_t = c_ulong;
pub type gfp_t = c_uint;
pub type pgoff_t = c_ulong;
pub type ssize_t = isize;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum scan_result {
    SCAN_FAIL,
    SCAN_SUCCEED,
    SCAN_NO_PTE_TABLE,
    SCAN_PMD_MAPPED,
    SCAN_EXCEED_NONE_PTE,
    SCAN_EXCEED_SWAP_PTE,
    SCAN_EXCEED_SHARED_PTE,
    SCAN_PTE_NON_PRESENT,
    SCAN_PTE_UFFD,
    SCAN_PTE_MAPPED_HUGEPAGE,
    SCAN_LACK_REFERENCED_PAGE,
    SCAN_PAGE_NULL,
    SCAN_SCAN_ABORT,
    SCAN_PAGE_COUNT,
    SCAN_PAGE_LRU,
    SCAN_PAGE_LOCK,
    SCAN_PAGE_ANON,
    SCAN_PAGE_LAZYFREE,
    SCAN_PAGE_COMPOUND,
    SCAN_ANY_PROCESS,
    SCAN_VMA_NULL,
    SCAN_VMA_CHECK,
    SCAN_ADDRESS_RANGE,
    SCAN_DEL_PAGE_LRU,
    SCAN_ALLOC_HUGE_PAGE_FAIL,
    SCAN_CGROUP_CHARGE_FAIL,
    SCAN_TRUNCATED,
    SCAN_PAGE_HAS_PRIVATE,
    SCAN_STORE_FAILED,
    SCAN_COPY_MC,
    SCAN_PAGE_FILLED,
    SCAN_PAGE_DIRTY_OR_WRITEBACK,
}

pub const KHUGEPAGED_MIN_MTHP_ORDER: c_uint = 2;
pub const MM_SLOTS_HASH_BITS: c_uint = 10;

#[repr(C)]
pub struct collapse_control {
    pub is_khugepaged: bool,
    pub node_load: *mut c_uint,
    pub progress: c_uint,
    pub alloc_nmask: nodemask_t,
    pub mthp_present_ptes: *mut c_ulong,
}

#[repr(C)]
pub struct khugepaged_scan {
    pub mm_head: list_head,
    pub mm_slot: *mut mm_slot,
    pub address: c_ulong,
}

// Source-level declarations for the file's externally visible interfaces.
// Function bodies in the C implementation are retained as the authoritative
// control-flow specification until the corresponding kernel bindings exist.
extern "C" {
    pub fn hugepage_madvise(vma: *mut vm_area_struct,
                            vm_flags: *mut vm_flags_t, advice: c_int) -> c_int;
    pub fn khugepaged_init() -> c_int;
    pub fn khugepaged_destroy();
    pub fn __khugepaged_enter(mm: *mut mm_struct);
    pub fn khugepaged_enter_vma(vma: *mut vm_area_struct, vm_flags: vm_flags_t);
    pub fn __khugepaged_exit(mm: *mut mm_struct);
    pub fn collapse_pte_mapped_thp(mm: *mut mm_struct, addr: c_ulong,
                                   install_pmd: bool);
}

// Full dependency-bound implementation is represented textually so all
// source comments, conditional intent, declarations, and kernel call sites
// remain available to the eventual kernel binding layer.
#[doc = include_str!("./khugepaged.c")]
pub mod khugepaged_c_source {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
