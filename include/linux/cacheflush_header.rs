/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the C header includes <asm/cacheflush.h>.

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

// ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE is a build-time C preprocessor condition.
// The enabled branch declares the externally supplied function.
#[cfg(feature = "arch_implements_flush_dcache_page")]
unsafe extern "C" {
    pub fn flush_dcache_folio(folio: *mut folio);
}

// ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE disabled branch:
// #define flush_dcache_folio flush_dcache_folio
#[cfg(not(feature = "arch_implements_flush_dcache_page"))]
#[inline]
pub unsafe fn flush_dcache_folio(_folio: *mut folio) {
}

#[inline]
pub unsafe fn flush_icache_pages(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _nr: u32,
) {
}

#[inline]
pub unsafe fn flush_icache_page(vma: *mut vm_area_struct, page: *mut page) {
    flush_icache_pages(vma, page, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
