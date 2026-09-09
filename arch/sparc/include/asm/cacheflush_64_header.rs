/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of the non-assembler portion of asm/cacheflush_64.h. */

/* Cache flush operations. */
#[inline(always)]
pub unsafe fn flushw_all() {
    core::arch::asm!("flushw", options(nostack, preserves_flags));
}

unsafe extern "C" {
    pub fn __flushw_user();

    pub fn flush_icache_range(start: c_ulong, end: c_ulong);
    pub fn __flush_icache_page(address: c_ulong);

    pub fn __flush_dcache_page(addr: *mut core::ffi::c_void, flush_icache: c_int);
    pub fn flush_dcache_folio_impl(folio: *mut folio);

    #[cfg(feature = "CONFIG_SMP")]
    pub fn smp_flush_dcache_folio_impl(folio: *mut folio, cpu: c_int);
    #[cfg(feature = "CONFIG_SMP")]
    pub fn flush_dcache_folio_all(mm: *mut mm_struct, folio: *mut folio);

    pub fn __flush_dcache_range(start: c_ulong, end: c_ulong);
    pub fn flush_dcache_folio(folio: *mut folio);

    pub fn flush_ptrace_access(
        vma: *mut vm_area_struct,
        page: *mut page,
        uaddr: c_ulong,
        kaddr: *mut core::ffi::c_void,
        len: c_ulong,
        write: c_int,
    );
}

pub type c_ulong = core::ffi::c_ulong;
pub type c_int = core::ffi::c_int;

/* These are the same regardless of whether this is an SMP kernel or not. */
#[inline(always)]
pub unsafe fn flushw_user() {
    __flushw_user();
}

pub use flushw_all as flush_register_windows;
pub use flushw_user as flush_user_windows;

/* The current-mm comparison and no-op macro bodies are preserved by these wrappers. */
#[inline(always)]
pub unsafe fn flush_cache_mm(_mm: *mut mm_struct) {}

#[inline(always)]
pub unsafe fn flush_cache_dup_mm(mm: *mut mm_struct) {
    flush_cache_mm(mm);
}

#[inline(always)]
pub unsafe fn flush_cache_range(vma: *mut vm_area_struct, _start: c_ulong, _end: c_ulong) {
    flush_cache_mm((*vma).vm_mm);
}

#[inline(always)]
pub unsafe fn flush_cache_page(vma: *mut vm_area_struct, _page: c_ulong, _pfn: c_ulong) {
    flush_cache_mm((*vma).vm_mm);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn smp_flush_dcache_folio_impl(folio: *mut folio, _cpu: c_int) {
    flush_dcache_folio_impl(folio);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn flush_dcache_folio_all(_mm: *mut mm_struct, folio: *mut folio) {
    flush_dcache_folio_impl(folio);
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: c_int = 1;

#[inline(always)]
pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

pub type folio = core::ffi::c_void;
pub type page = core::ffi::c_void;
pub type mm_struct = core::ffi::c_void;
pub type vm_area_struct = core::ffi::c_void;

unsafe extern "C" {
    pub fn page_folio(page: *mut page) -> *mut folio;
}

/* copy_to_user_page and copy_from_user_page retain their original operation order. */
#[inline(always)]
pub unsafe fn copy_to_user_page(
    vma: *mut vm_area_struct,
    page: *mut page,
    vaddr: c_ulong,
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: c_ulong,
) {
    flush_cache_page(vma, vaddr, page_to_pfn(page));
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len as usize);
    flush_ptrace_access(vma, page, vaddr, src as *mut core::ffi::c_void, len, 0);
}

#[inline(always)]
pub unsafe fn copy_from_user_page(
    vma: *mut vm_area_struct,
    page: *mut page,
    vaddr: c_ulong,
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: c_ulong,
) {
    flush_cache_page(vma, vaddr, page_to_pfn(page));
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len as usize);
    flush_ptrace_access(vma, page, vaddr, dst, len, 1);
}

/* The following C macros are intentional no-ops. */
#[inline(always)] pub unsafe fn flush_dcache_mmap_lock(_mapping: *mut core::ffi::c_void) {}
#[inline(always)] pub unsafe fn flush_dcache_mmap_unlock(_mapping: *mut core::ffi::c_void) {}
#[inline(always)] pub unsafe fn flush_cache_vmap(_start: c_ulong, _end: c_ulong) {}
#[inline(always)] pub unsafe fn flush_cache_vmap_early(_start: c_ulong, _end: c_ulong) {}
#[inline(always)] pub unsafe fn flush_cache_vunmap(_start: c_ulong, _end: c_ulong) {}

unsafe extern "C" {
    pub fn page_to_pfn(page: *mut page) -> c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
