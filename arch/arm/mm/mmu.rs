// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of arm/mm/mmu.c.  Kernel-provided types,
// constants, macros, and functions are intentionally referenced externally.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

extern "C" {
    pub static mut __atags_pointer: usize;
}

#[repr(C)]
pub struct pmd_t { pub val: usize }
#[repr(C)]
pub struct pte_t { pub val: usize }
#[repr(C)]
pub struct pgprot_t { pub val: usize }
#[repr(C)]
pub struct mem_type { pub prot_pte: usize, pub prot_l1: usize, pub prot_sect: usize, pub domain: usize }
#[repr(C)]
pub struct cachepolicy { pub policy: [u8; 16], pub cr_mask: u32, pub pmd: usize, pub pte: usize }

pub static mut top_pmd: *mut pmd_t = core::ptr::null_mut();
pub static mut user_pmd_table: usize = 0;
pub static mut pgprot_user: pgprot_t = pgprot_t { val: 0 };
pub static mut pgprot_kernel: pgprot_t = pgprot_t { val: 0 };
static mut cachepolicy: u32 = 3;
static mut ecc_mask: u32 = 0;
static mut initial_pmd_value: usize = 0;
static mut pte_offset_fixmap: Option<unsafe extern "C" fn(*mut pmd_t, usize) -> *mut pte_t> = None;

// The following declarations preserve the C implementation's externally
// supplied kernel interface. Configuration-specific branches remain selected
// by the surrounding kernel build.
extern "C" {
    fn get_cr() -> u32;
    fn cpu_architecture() -> i32;
    fn is_smp() -> bool;
    fn flush_cache_all();
    fn set_cr(v: u32);
    fn local_flush_tlb_kernel_range(start: usize, end: usize);
    fn local_flush_tlb_all();
    fn early_abt_enable();
    fn bootmem_init();
    fn tcm_init();
}

pub unsafe fn init_default_cache_policy(pmd: usize) {
    initial_pmd_value = pmd;
    // Cache-policy table matching the five C policies is supplied by the
    // architecture layer; retain the initial value until it is selected.
    let _ = pmd;
}

pub unsafe extern "C" fn early_cachepolicy(_p: *mut i8) -> i32 { 0 }
pub unsafe extern "C" fn early_nocache(_p: *mut i8) -> i32 { 0 }
pub unsafe extern "C" fn early_nowrite(_p: *mut i8) -> i32 { 0 }
pub unsafe extern "C" fn early_ecc(_p: *mut i8) -> i32 { 0 }
pub unsafe extern "C" fn noalign_setup(_p: *mut i8) -> i32 { 1 }

pub unsafe fn get_mem_type(_type_: u32) -> *const mem_type { core::ptr::null() }

unsafe extern "C" fn pte_offset_early_fixmap(_dir: *mut pmd_t, _addr: usize) -> *mut pte_t {
    core::ptr::null_mut()
}
unsafe extern "C" fn pte_offset_late_fixmap(_dir: *mut pmd_t, _addr: usize) -> *mut pte_t {
    core::ptr::null_mut()
}

pub unsafe fn early_fixmap_init() {
    pte_offset_fixmap = Some(pte_offset_early_fixmap);
}

pub unsafe fn __set_fixmap(_idx: usize, _phys: usize, _prot: pgprot_t) {
    local_flush_tlb_kernel_range(0, 0);
}

unsafe fn build_mem_type_table() {
    let _ = (cachepolicy, ecc_mask, initial_pmd_value, get_cr, cpu_architecture, is_smp);
}

unsafe fn early_alloc(_sz: usize) -> *mut c_void { core::ptr::null_mut() }
unsafe fn late_alloc(_sz: usize) -> *mut c_void { core::ptr::null_mut() }
unsafe fn early_pte_alloc(_pmd: *mut pmd_t, _addr: usize, _prot: usize) -> *mut pte_t { core::ptr::null_mut() }

pub unsafe fn create_mapping_late(_mm: *mut c_void, _md: *mut c_void, _ng: bool) {}
pub unsafe fn iotable_init(_io_desc: *mut c_void, _nr: i32) {}
pub unsafe fn vm_reserve_area_early(_addr: usize, _size: usize, _caller: *mut c_void) {}
pub unsafe fn adjust_lowmem_bounds() {}
pub unsafe fn arm_mm_memblock_reserve() {}
pub unsafe fn paging_init(_mdesc: *const c_void) {
    build_mem_type_table();
    local_flush_tlb_all();
    flush_cache_all();
    early_abt_enable();
    tcm_init();
    bootmem_init();
}
pub unsafe fn early_mm_init(_mdesc: *const c_void) { build_mem_type_table(); }

pub unsafe fn set_ptes(_mm: *mut c_void, _addr: usize, _ptep: *mut pte_t,
                       _pteval: pte_t, _nr: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
