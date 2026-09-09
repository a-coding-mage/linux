/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mm.h, asm/string.h, and asm/cache.h.

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

extern "C" {
    pub fn flush_dcache_page(page: *mut page);
    pub fn flush_dcache_folio(folio: *mut folio);
}

// C macro: flush_dcache_folio flush_dcache_folio

#[inline]
pub unsafe fn flush_cache_mm(_mm: *mut core::ffi::c_void) {
    dcache_wbinv_all();
}

#[inline]
pub unsafe fn flush_cache_page(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _pfn: core::ffi::c_ulong,
) {
    cache_wbinv_all();
}

#[inline]
pub unsafe fn flush_cache_dup_mm(_mm: *mut core::ffi::c_void) {
    cache_wbinv_all();
}

#[inline]
pub unsafe fn flush_dcache_mmap_lock(mapping: *mut mapping) {
    xa_lock_irq(&mut (*mapping).i_pages);
}

#[inline]
pub unsafe fn flush_dcache_mmap_unlock(mapping: *mut mapping) {
    xa_unlock_irq(&mut (*mapping).i_pages);
}

pub const ARCH_IMPLEMENTS_FLUSH_KERNEL_VMAP_RANGE: i32 = 1;

#[inline]
pub unsafe fn flush_kernel_vmap_range(_addr: *mut core::ffi::c_void, _size: i32) {
    dcache_wbinv_all();
}

#[inline]
pub unsafe fn invalidate_kernel_vmap_range(_addr: *mut core::ffi::c_void, _size: i32) {
    dcache_wbinv_all();
}

// #define ARCH_HAS_FLUSH_ANON_PAGE

#[inline]
pub unsafe fn flush_anon_page(
    _vma: *mut vm_area_struct,
    page: *mut page,
    _vmaddr: core::ffi::c_ulong,
) {
    if PageAnon(page) {
        cache_wbinv_all();
    }
}

/*
 * if (current_mm != vma->mm) cache_wbinv_range(start, end) will be broken.
 * Use cache_wbinv_all() here and need to be improved in future.
 */
extern "C" {
    pub fn flush_cache_range(
        vma: *mut vm_area_struct,
        start: core::ffi::c_ulong,
        end: core::ffi::c_ulong,
    );
}

#[inline]
pub unsafe fn flush_cache_vmap(start: core::ffi::c_ulong, end: core::ffi::c_ulong) {
    let _ = (start, end);
    cache_wbinv_all();
}

#[inline]
pub unsafe fn flush_cache_vmap_early(_start: core::ffi::c_ulong, _end: core::ffi::c_ulong) {}

#[inline]
pub unsafe fn flush_cache_vunmap(_start: core::ffi::c_ulong, _end: core::ffi::c_ulong) {
    cache_wbinv_all();
}

#[inline]
pub unsafe fn flush_icache_range(start: core::ffi::c_ulong, end: core::ffi::c_ulong) {
    cache_wbinv_range(start, end);
}

#[inline]
pub unsafe fn flush_icache_mm_range(
    _mm: *mut core::ffi::c_void,
    start: core::ffi::c_ulong,
    end: core::ffi::c_ulong,
) {
    cache_wbinv_range(start, end);
}

#[inline]
pub unsafe fn flush_icache_deferred(_mm: *mut core::ffi::c_void) {}

#[inline]
pub unsafe fn copy_from_user_page(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _vaddr: core::ffi::c_ulong,
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: usize,
) {
    memcpy(dst, src, len);
}

#[inline]
pub unsafe fn copy_to_user_page(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _vaddr: core::ffi::c_ulong,
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: usize,
) {
    memcpy(dst, src, len);
    cache_wbinv_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
