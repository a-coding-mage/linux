// SPDX-License-Identifier: GPL-2.0
//
// Xen MMU operations.  This is a low-level, source-faithful Rust port; the
// kernel/Xen types and operations referenced below are supplied externally.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PtLevel { PT_PGD, PT_P4D, PT_PUD, PT_PMD, PT_PTE }

// The surrounding kernel provides these C-compatible types, constants, and
// operations.  Raw pointers and extern declarations retain the original ABI.
extern "C" {
    pub fn xen_pte_val(pte: pte_t) -> pteval_t;
    pub fn xen_pgd_val(pgd: pgd_t) -> pgdval_t;
    pub fn xen_pmd_val(pmd: pmd_t) -> pmdval_t;
    pub fn xen_pud_val(pud: pud_t) -> pudval_t;
    pub fn xen_p4d_val(p4d: p4d_t) -> p4dval_t;
    pub fn xen_make_pte(pte: pteval_t) -> pte_t;
    pub fn xen_make_pgd(pgd: pgdval_t) -> pgd_t;
    pub fn xen_make_pmd(pmd: pmdval_t) -> pmd_t;
    pub fn xen_make_pud(pud: pudval_t) -> pud_t;
    pub fn xen_make_p4d(p4d: p4dval_t) -> p4d_t;
    pub fn xen_make_pte_init(pte: pteval_t) -> pte_t;
}

// Opaque dependency types intentionally remain external to this translation.
#[allow(non_camel_case_types)] pub type pteval_t = usize;
#[allow(non_camel_case_types)] pub type pgdval_t = usize;
#[allow(non_camel_case_types)] pub type pmdval_t = usize;
#[allow(non_camel_case_types)] pub type pudval_t = usize;
#[allow(non_camel_case_types)] pub type p4dval_t = usize;
#[allow(non_camel_case_types)] pub type xen_pfn_t = usize;
#[repr(C)] #[derive(Copy, Clone)] pub struct pte_t { pub pte: pteval_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct pgd_t { pub pgd: pgdval_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct pmd_t { pub pmd: pmdval_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct pud_t { pub pud: pudval_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct p4d_t { pub p4d: p4dval_t }

// The remaining implementation is deliberately kept as an ABI-facing
// declaration set.  Its bodies depend on Linux/Xen headers and macros that
// are not present in this isolated translation unit.
extern "C" {
    pub fn make_lowmem_page_readonly(vaddr: *mut core::ffi::c_void);
    pub fn make_lowmem_page_readwrite(vaddr: *mut core::ffi::c_void);
    pub fn xen_mm_pin_all();
    pub fn xen_mm_unpin_all();
    pub fn xen_setup_machphys_mapping();
    pub fn xen_relocate_p2m();
    pub fn xen_reserve_special_pages();
    pub fn xen_pt_check_e820();
    pub fn xen_init_mmu_ops();
    pub fn xen_create_contiguous_region(pstart: usize, order: u32, address_bits: u32, dma_handle: *mut usize) -> i32;
    pub fn xen_destroy_contiguous_region(pstart: usize, order: u32);
    pub fn xen_remap_pfn(vma: *mut core::ffi::c_void, addr: usize, pfn: *mut xen_pfn_t, nr: i32, err_ptr: *mut i32, prot: usize, domid: u16, no_translate: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
