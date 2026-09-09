/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * (C) 2001 - 2013 Tensilica Inc.
 */

// Translated from xtensa/include/asm/cacheflush.h.
// C preprocessor conditions involving XCHAL_DCACHE_IS_WRITEBACK, CONFIG_MMU,
// CONFIG_SMP, DCACHE_WAY_SIZE, ICACHE_WAY_SIZE, and PAGE_SIZE are preserved
// as source-level intent below.

extern "C" {
    pub fn __invalidate_dcache_all();
    pub fn __invalidate_icache_all();
    pub fn __invalidate_dcache_page(address: usize);
    pub fn __invalidate_icache_page(address: usize);
    pub fn __invalidate_icache_range(start: usize, size: usize);
    pub fn __invalidate_dcache_range(start: usize, size: usize);

    // Present when XCHAL_DCACHE_IS_WRITEBACK is enabled.
    pub fn __flush_invalidate_dcache_all();
    pub fn __flush_dcache_page(address: usize);
    pub fn __flush_dcache_range(start: usize, size: usize);
    pub fn __flush_invalidate_dcache_page(address: usize);
    pub fn __flush_invalidate_dcache_range(start: usize, size: usize);

    // Present when CONFIG_MMU && (DCACHE_WAY_SIZE > PAGE_SIZE).
    pub fn __flush_invalidate_dcache_page_alias(virt: usize, phys: usize);
    pub fn __invalidate_dcache_page_alias(virt: usize, phys: usize);

    // Present when CONFIG_MMU && (ICACHE_WAY_SIZE > PAGE_SIZE).
    pub fn __invalidate_icache_page_alias(virt: usize, phys: usize);
}

// Non-writeback-cache inline fallbacks.
#[inline]
pub unsafe fn flush_dcache_page_no_writeback(_va: usize) {}

#[inline]
pub unsafe fn flush_dcache_range_no_writeback(_va: usize, _sz: usize) {}

// Alias-operation fallbacks when the MMU/cache-way-size condition is false.
#[inline]
pub unsafe fn flush_invalidate_dcache_page_alias_noop(_virt: usize, _phys: usize) {}

#[inline]
pub unsafe fn invalidate_dcache_page_alias_noop(_virt: usize, _phys: usize) {}

#[inline]
pub unsafe fn invalidate_icache_page_alias_noop(_virt: usize, _phys: usize) {}

// External kernel types and functions supplied by other translated files.
#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

extern "C" {
    pub fn flush_cache_all();
    pub fn flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn flush_icache_range(start: usize, end: usize);
    pub fn flush_cache_page(vma: *mut vm_area_struct, address: usize, pfn: usize);
    pub fn flush_dcache_folio(folio: *mut folio);
    pub fn local_flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn local_flush_cache_page(vma: *mut vm_area_struct, address: usize, pfn: usize);
    pub fn copy_to_user_page(
        vma: *mut vm_area_struct,
        page: *mut page,
        vaddr: usize,
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: usize,
    );
    pub fn copy_from_user_page(
        vma: *mut vm_area_struct,
        page: *mut page,
        vaddr: usize,
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: usize,
    );
}

#[inline]
pub unsafe fn local_flush_cache_all() {
    __flush_invalidate_dcache_all();
    __invalidate_icache_all();
}

#[inline]
pub unsafe fn flush_cache_mm(_mm: *mut core::ffi::c_void) {
    flush_cache_all();
}

#[inline]
pub unsafe fn flush_cache_dup_mm(mm: *mut core::ffi::c_void) {
    flush_cache_mm(mm);
}

#[inline]
pub unsafe fn flush_cache_vmap(_start: usize, _end: usize) {
    flush_cache_all();
}

#[inline]
pub unsafe fn flush_cache_vmap_early(_start: usize, _end: usize) {}

#[inline]
pub unsafe fn flush_cache_vunmap(_start: usize, _end: usize) {
    flush_cache_all();
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[inline]
pub unsafe fn flush_dcache_page(page: *mut page) {
    // Equivalent to flush_dcache_folio(page_folio(page)); page_folio is an
    // external dependency supplied by another translated file.
    extern "C" {
        fn page_folio(page: *mut page) -> *mut folio;
    }
    flush_dcache_folio(page_folio(page));
}

#[inline]
pub unsafe fn local_flush_icache_range(start: usize, end: usize) {
    __flush_dcache_range(start, end.wrapping_sub(start));
    __invalidate_icache_range(start, end.wrapping_sub(start));
}

#[inline]
pub unsafe fn flush_icache_user_range(start: usize, end: usize) {
    local_flush_icache_range(start, end);
}

#[inline]
pub unsafe fn copy_to_user_page_fallback(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _vaddr: usize,
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: usize,
) {
    // Equivalent to memcpy(dst, src, len), followed by cache maintenance.
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    __flush_dcache_range(dst as usize, len);
    __invalidate_icache_range(dst as usize, len);
}

#[inline]
pub unsafe fn copy_from_user_page_fallback(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _vaddr: usize,
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: usize,
) {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
