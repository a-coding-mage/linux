/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of arch/arm/include/asm/cacheflush.h. */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

pub const CACHE_COLOUR: unsafe fn(usize) -> usize = |vaddr| {
    (vaddr & (SHMLBA - 1)) >> PAGE_SHIFT
};

pub const PG_dcache_clean: u32 = PG_arch_1;

#[repr(C)]
pub struct cpu_cache_fns {
    pub flush_icache_all: Option<unsafe extern "C" fn()>,
    pub flush_kern_all: Option<unsafe extern "C" fn()>,
    pub flush_kern_louis: Option<unsafe extern "C" fn()>,
    pub flush_user_all: Option<unsafe extern "C" fn()>,
    pub flush_user_range: Option<unsafe extern "C" fn(usize, usize, u32)>,
    pub coherent_kern_range: Option<unsafe extern "C" fn(usize, usize)>,
    pub coherent_user_range: Option<unsafe extern "C" fn(usize, usize) -> i32>,
    pub flush_kern_dcache_area: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize)>,
    pub dma_map_area: Option<unsafe extern "C" fn(*const core::ffi::c_void, usize, i32)>,
    pub dma_unmap_area: Option<unsafe extern "C" fn(*const core::ffi::c_void, usize, i32)>,
    pub dma_flush_range: Option<unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void)>,
}

// Under MULTI_CACHE these names designate fields of the external cpu_cache object.
// Otherwise they designate the following external functions.
extern "C" {
    pub static mut cpu_cache: cpu_cache_fns;
    pub fn __cpuc_flush_icache_all();
    pub fn __cpuc_flush_kern_all();
    pub fn __cpuc_flush_kern_louis();
    pub fn __cpuc_flush_user_all();
    pub fn __cpuc_flush_user_range(start: usize, end: usize, flags: u32);
    pub fn __cpuc_coherent_kern_range(start: usize, end: usize);
    pub fn __cpuc_coherent_user_range(start: usize, end: usize) -> i32;
    pub fn __cpuc_flush_dcache_area(addr: *mut core::ffi::c_void, size: usize);
    pub fn dmac_flush_range(start: *const core::ffi::c_void, end: *const core::ffi::c_void);
}

pub unsafe fn __flush_icache_all_generic() {
    // C inline assembly: mcr p15, 0, r0, c7, c5, 0
}

pub unsafe fn __flush_icache_all_v7_smp() {
    // C inline assembly: mcr p15, 0, r0, c7, c1, 0
}

// The preprocessor selects __flush_icache_preferred according to the ARM build configuration.
pub unsafe fn __flush_icache_all() {
    __cpuc_flush_icache_all();
    dsb(ishst);
}

pub unsafe fn flush_cache_louis() { __cpuc_flush_kern_louis(); }
pub unsafe fn flush_cache_all() { __cpuc_flush_kern_all(); }

pub unsafe fn vivt_flush_cache_mm(mm: *mut mm_struct) {
    if cpumask_test_cpu(smp_processor_id(), mm_cpumask(mm)) {
        __cpuc_flush_user_all();
    }
}

pub unsafe fn vivt_flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize) {
    let mm = (*vma).vm_mm;
    if mm.is_null() || cpumask_test_cpu(smp_processor_id(), mm_cpumask(mm)) {
        __cpuc_flush_user_range(start & PAGE_MASK, PAGE_ALIGN(end), (*vma).vm_flags);
    }
}

pub unsafe fn vivt_flush_cache_pages(vma: *mut vm_area_struct, user_addr: usize, _pfn: usize, nr: u32) {
    let mm = (*vma).vm_mm;
    if mm.is_null() || cpumask_test_cpu(smp_processor_id(), mm_cpumask(mm)) {
        let addr = user_addr & PAGE_MASK;
        __cpuc_flush_user_range(addr, addr.wrapping_add((nr as usize).wrapping_mul(PAGE_SIZE)), (*vma).vm_flags);
    }
}

// If CONFIG_CPU_CACHE_VIPT is absent, these are aliases to the VIVT helpers.
extern "C" {
    pub fn flush_cache_mm(mm: *mut mm_struct);
    pub fn flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn flush_cache_pages(vma: *mut vm_area_struct, user_addr: usize, pfn: usize, nr: u32);
    pub fn copy_to_user_page(mm: *mut vm_area_struct, page: *mut page, vaddr: usize, dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize);
    pub fn flush_dcache_page(page: *mut page);
    pub fn flush_dcache_folio(folio: *mut folio);
    pub fn __flush_anon_page(vma: *mut vm_area_struct, page: *mut page, vmaddr: usize);
    pub fn flush_uprobe_xol_access(page: *mut page, uaddr: usize, kaddr: *mut core::ffi::c_void, len: usize);
}

pub unsafe fn copy_from_user_page(_vma: *mut vm_area_struct, _page: *mut page, _vaddr: usize, dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;
pub const ARCH_IMPLEMENTS_FLUSH_KERNEL_VMAP_RANGE: i32 = 1;

pub unsafe fn flush_kernel_vmap_range(addr: *mut core::ffi::c_void, size: i32) {
    if cache_is_vivt() || cache_is_vipt_aliasing() { __cpuc_flush_dcache_area(addr, size as usize); }
}
pub unsafe fn invalidate_kernel_vmap_range(addr: *mut core::ffi::c_void, size: i32) {
    if cache_is_vivt() || cache_is_vipt_aliasing() { __cpuc_flush_dcache_area(addr, size as usize); }
}

pub unsafe fn flush_anon_page(vma: *mut vm_area_struct, page: *mut page, vmaddr: usize) {
    if PageAnon(page) { __flush_anon_page(vma, page, vmaddr); }
}

pub unsafe fn flush_cache_vmap(_start: usize, _end: usize) {
    if !cache_is_vipt_nonaliasing() { flush_cache_all(); } else { dsb(ishst); }
}
pub unsafe fn flush_cache_vmap_early(_start: usize, _end: usize) {}
pub unsafe fn flush_cache_vunmap(_start: usize, _end: usize) {
    if !cache_is_vipt_nonaliasing() { flush_cache_all(); }
}

pub const __CACHE_WRITEBACK_ORDER: u32 = 6;
pub const __CACHE_WRITEBACK_GRANULE: u32 = 1 << __CACHE_WRITEBACK_ORDER;

pub unsafe fn __sync_cache_range_w(p: *mut core::ffi::c_void, size: usize) {
    __cpuc_flush_dcache_area(p, size);
    outer_clean_range(__pa(p), __pa((p as *mut u8).add(size) as *mut core::ffi::c_void));
}

pub unsafe fn __sync_cache_range_r(p: *mut core::ffi::c_void, size: usize) {
    // CONFIG_OUTER_CACHE conditionally performs the outer flush here.
    __cpuc_flush_dcache_area(p, size);
}

pub unsafe fn v7_exit_coherency_flush(_level: i32) {
    // C inline assembly performs the SCTLR/ACTLR and cache-flush sequence.
}

// CONFIG_CPU_ICACHE_MISMATCH_WORKAROUND controls whether this is external or empty.
extern "C" { pub fn check_cpu_icache_size(cpuid: i32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
