// SPDX-License-Identifier: GPL-2.0-only
//
// Rust translation of debug_vm_pgtable.c.  Kernel-provided types and helpers
// are intentionally left as external dependencies, as in the original file.

use core::ffi::c_int;

#[repr(C)]
pub struct pgtable_debug_args {
    pub mm: *mut mm_struct,
    pub vma: *mut vm_area_struct,
    pub pgdp: *mut pgd_t,
    pub p4dp: *mut p4d_t,
    pub pudp: *mut pud_t,
    pub pmdp: *mut pmd_t,
    pub ptep: *mut pte_t,
    pub start_p4dp: *mut p4d_t,
    pub start_pudp: *mut pud_t,
    pub start_pmdp: *mut pmd_t,
    pub start_ptep: pgtable_t,
    pub vaddr: usize,
    pub page_prot: pgprot_t,
    pub page_prot_none: pgprot_t,
    pub is_contiguous_page: bool,
    pub pud_pfn: usize,
    pub pmd_pfn: usize,
    pub pte_pfn: usize,
    pub fixed_alignment: usize,
    pub fixed_pgd_pfn: usize,
    pub fixed_p4d_pfn: usize,
    pub fixed_pud_pfn: usize,
    pub fixed_pmd_pfn: usize,
    pub fixed_pte_pfn: usize,
    pub swp_entry: swp_entry_t,
    pub leaf_entry: softleaf_t,
}

// These declarations are supplied by the kernel translation unit.
extern "C" {
    fn init_args(args: *mut pgtable_debug_args) -> c_int;
    fn destroy_args(args: *mut pgtable_debug_args);
    fn pte_basic_tests(args: *mut pgtable_debug_args, idx: c_int);
    fn pmd_basic_tests(args: *mut pgtable_debug_args, idx: c_int);
    fn pud_basic_tests(args: *mut pgtable_debug_args, idx: c_int);
    fn p4d_basic_tests(args: *mut pgtable_debug_args);
    fn pgd_basic_tests(args: *mut pgtable_debug_args);
    fn pmd_leaf_tests(args: *mut pgtable_debug_args);
    fn pud_leaf_tests(args: *mut pgtable_debug_args);
    fn pte_special_tests(args: *mut pgtable_debug_args);
    fn pte_protnone_tests(args: *mut pgtable_debug_args);
    fn pmd_protnone_tests(args: *mut pgtable_debug_args);
    fn pte_soft_dirty_tests(args: *mut pgtable_debug_args);
    fn pmd_soft_dirty_tests(args: *mut pgtable_debug_args);
    fn pte_swap_soft_dirty_tests(args: *mut pgtable_debug_args);
    fn pmd_leaf_soft_dirty_tests(args: *mut pgtable_debug_args);
    fn pte_swap_exclusive_tests(args: *mut pgtable_debug_args);
    fn pte_swap_tests(args: *mut pgtable_debug_args);
    fn pmd_softleaf_tests(args: *mut pgtable_debug_args);
    fn swap_migration_tests(args: *mut pgtable_debug_args);
    fn pmd_thp_tests(args: *mut pgtable_debug_args);
    fn pud_thp_tests(args: *mut pgtable_debug_args);
    fn hugetlb_basic_tests(args: *mut pgtable_debug_args);
    fn pte_clear_tests(args: *mut pgtable_debug_args);
    fn pte_advanced_tests(args: *mut pgtable_debug_args);
    fn pmd_clear_tests(args: *mut pgtable_debug_args);
    fn pmd_advanced_tests(args: *mut pgtable_debug_args);
    fn pmd_huge_tests(args: *mut pgtable_debug_args);
    fn pmd_populate_tests(args: *mut pgtable_debug_args);
    fn pud_clear_tests(args: *mut pgtable_debug_args);
    fn pud_advanced_tests(args: *mut pgtable_debug_args);
    fn pud_huge_tests(args: *mut pgtable_debug_args);
    fn pud_populate_tests(args: *mut pgtable_debug_args);
    fn p4d_clear_tests(args: *mut pgtable_debug_args);
    fn pgd_clear_tests(args: *mut pgtable_debug_args);
    fn p4d_populate_tests(args: *mut pgtable_debug_args);
    fn pgd_populate_tests(args: *mut pgtable_debug_args);
}

// Kernel opaque types and constants are provided by the surrounding build.
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct pgd_t { _private: [u8; 0] }
#[repr(C)] pub struct p4d_t { _private: [u8; 0] }
#[repr(C)] pub struct pud_t { _private: [u8; 0] }
#[repr(C)] pub struct pmd_t { _private: [u8; 0] }
#[repr(C)] pub struct pte_t { _private: [u8; 0] }
#[repr(C)] pub struct pgprot_t { _private: [u8; 0] }
#[repr(C)] pub struct swp_entry_t { pub val: usize }
#[repr(C)] pub struct softleaf_t { pub val: usize }
pub type pgtable_t = *mut core::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn debug_vm_pgtable() -> c_int {
    let mut args: pgtable_debug_args = core::mem::zeroed();
    let ret = init_args(&mut args);
    if ret != 0 { return ret; }

    // VM_FLAGS_START = VM_NONE; VM_FLAGS_END = VM_SHARED|VM_EXEC|VM_WRITE|VM_READ.
    // The kernel supplies these flag values in the target architecture.
    for idx in 0..=0x7fff_i32 {
        pte_basic_tests(&mut args, idx);
        pmd_basic_tests(&mut args, idx);
        pud_basic_tests(&mut args, idx);
    }
    p4d_basic_tests(&mut args);
    pgd_basic_tests(&mut args);
    pmd_leaf_tests(&mut args); pud_leaf_tests(&mut args);
    pte_special_tests(&mut args); pte_protnone_tests(&mut args); pmd_protnone_tests(&mut args);
    pte_soft_dirty_tests(&mut args); pmd_soft_dirty_tests(&mut args);
    pte_swap_soft_dirty_tests(&mut args); pmd_leaf_soft_dirty_tests(&mut args);
    pte_swap_exclusive_tests(&mut args); pte_swap_tests(&mut args); pmd_softleaf_tests(&mut args);
    swap_migration_tests(&mut args); pmd_thp_tests(&mut args); pud_thp_tests(&mut args);
    hugetlb_basic_tests(&mut args);
    pte_clear_tests(&mut args); pte_advanced_tests(&mut args);
    pmd_clear_tests(&mut args); pmd_advanced_tests(&mut args); pmd_huge_tests(&mut args); pmd_populate_tests(&mut args);
    pud_clear_tests(&mut args); pud_advanced_tests(&mut args); pud_huge_tests(&mut args); pud_populate_tests(&mut args);
    p4d_clear_tests(&mut args); pgd_clear_tests(&mut args); p4d_populate_tests(&mut args); pgd_populate_tests(&mut args);
    destroy_args(&mut args);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
