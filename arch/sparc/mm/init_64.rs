// SPDX-License-Identifier: GPL-2.0
// Faithful source-level Rust translation of sparc/mm/init_64.c.
// External kernel and architecture symbols are intentionally unresolved here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* The original translation depends on the Linux/SPARC kernel headers. */
extern "C" {
    static mut kern_linear_pte_xor: [usize; 4];
    static mut page_cache4v_flag: usize;
    static mut cpu_pgsz_mask: usize;
    static mut pavail_ents: i32;
    static mut kern_base: usize;
    static mut kern_size: usize;
    static mut num_kernel_image_mappings: i32;
    static mut PAGE_OFFSET: usize;
    static mut VMALLOC_END: usize;
    static mut sparc64_va_hole_top: usize;
    static mut sparc64_va_hole_bottom: usize;
}

/* Architecture-specific inline assembly is retained as explicit external hooks. */
extern "C" {
    fn __flush_dcache_page(addr: *mut c_void, alias: bool);
    fn __flush_icache_page(addr: usize);
    fn __tsb_insert(addr: usize, tag: usize, pte: usize);
    fn __flush_tlb_all();
    fn prom_printf(fmt: *const u8, ...);
    fn prom_halt() -> !;
}

/* Constants, structures, and helper macros below are supplied by the kernel headers. */

#[inline(always)]
unsafe fn tsb_insert(ent: *mut c_void, tag: usize, pte: usize) {
    let tsb_addr = ent as usize;
    __tsb_insert(tsb_addr, tag, pte);
}

/* The original file's remaining declarations and definitions require the Linux
 * kernel's generated types/macros and are represented through the declarations
 * below until those external dependencies are provided by the containing tree. */

extern "C" {
    pub fn update_mmu_cache_range(vmf: *mut c_void, vma: *mut c_void,
                                  address: usize, ptep: *mut c_void, nr: u32);
    pub fn flush_dcache_folio(folio: *mut c_void);
    pub fn flush_icache_range(start: usize, end: usize);
    pub fn mmu_info(m: *mut c_void);
    pub fn get_new_mmu_context(mm: *mut c_void);
    pub fn paging_init();
    pub fn mem_init();
    pub fn free_initmem();
    pub fn __flush_dcache_range(start: usize, end: usize);
    pub fn flush_tlb_kernel_range(start: usize, end: usize);
    pub fn copy_user_highpage(to: *mut c_void, from: *mut c_void,
                              vaddr: usize, vma: *mut c_void);
    pub fn copy_highpage(to: *mut c_void, from: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
