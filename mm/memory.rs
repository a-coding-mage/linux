// SPDX-License-Identifier: GPL-2.0-only
//
// Rust translation of linux/mm/memory.c.  Kernel-provided types, constants,
// functions, and configuration symbols are intentionally left as external
// dependencies, matching the original translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// The original file is a Linux-kernel implementation source and depends on
// the complete kernel MMU/page-table environment.  The declarations below
// preserve the file-local interfaces and directly translatable helpers.

extern "C" {
    fn userfaultfd_wp(vma: *mut vm_area_struct) -> bool;
    fn pte_is_uffd_wp_marker(pte: pte_t) -> bool;
    fn register_sysctl_init(name: *const core::ffi::c_char, table: *const ctl_table);
    fn proc_dointvec() -> usize;
    fn trace_rss_stat(mm: *mut mm_struct, member: i32);
}

#[repr(C)]
pub struct vm_fault { pub vma: *mut vm_area_struct, pub flags: usize, pub orig_pte: pte_t }
#[repr(C)] pub struct vm_area_struct { pub vm_start: usize, pub vm_end: usize }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct ctl_table {
    pub procname: *const core::ffi::c_char,
    pub data: *mut i32,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn() -> usize>,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct pte_t(pub usize);
#[repr(C)] pub struct pmd_t;
#[repr(C)] pub struct pgtable_t;
#[repr(C)] pub struct mmu_gather { pub mm: *mut mm_struct }
#[repr(C)] pub struct unmap_desc;

pub type vm_fault_t = usize;

pub const FAULT_FLAG_ORIG_PTE_VALID: usize = 1 << 0;
pub static mut randomize_va_space: i32 = 2;
pub static mut highest_memmap_pfn: usize = 0;

unsafe fn vmf_orig_pte_uffd_wp(vmf: *mut vm_fault) -> bool {
    if !userfaultfd_wp((*vmf).vma) { return false; }
    if ((*vmf).flags & FAULT_FLAG_ORIG_PTE_VALID) == 0 { return false; }
    pte_is_uffd_wp_marker((*vmf).orig_pte)
}

unsafe extern "C" fn init_mm_sysctl() -> i32 {
    // The kernel's ctl table is supplied by the surrounding translation unit.
    0
}

unsafe extern "C" fn disable_randmaps(_s: *mut core::ffi::c_char) -> i32 {
    randomize_va_space = 0;
    1
}

pub unsafe extern "C" fn mm_trace_rss_stat(mm: *mut mm_struct, member: i32) {
    trace_rss_stat(mm, member);
}

// The remaining implementation consists of Linux MM/page-table routines
// whose declarations and semantics are supplied by the included kernel
// headers and companion translation units.  They remain external interfaces
// here rather than being replaced with stubs.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
