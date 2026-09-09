/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. Definitions from asm/cachetype.h and
// asm/fixmap.h are supplied by other dependencies.

pub const PKMAP_BASE: usize = PAGE_OFFSET - PMD_SIZE;
pub const LAST_PKMAP: usize = PTRS_PER_PTE;
pub const LAST_PKMAP_MASK: usize = LAST_PKMAP - 1;

#[inline]
pub const fn PKMAP_NR(virt: usize) -> usize {
    (virt - PKMAP_BASE) >> PAGE_SHIFT
}

#[inline]
pub const fn PKMAP_ADDR(nr: usize) -> usize {
    PKMAP_BASE + (nr << PAGE_SHIFT)
}

#[inline]
pub unsafe fn flush_cache_kmaps() {
    if cache_is_vivt() {
        flush_cache_all();
    }
}

pub static mut pkmap_page_table: *mut pte_t;

// ARCH_NEEDS_KMAP_HIGH_GET is enabled by default. The C preprocessor
// conditionals below depend on build-time kernel configuration:
// CONFIG_SMP && CONFIG_CPU_TLB_V6 disables it, and CONFIG_HIGHMEM &&
// CONFIG_CPU_CACHE_VIVT is rejected in that configuration. CONFIG_ARM_ERRATA_798181
// also disables it.

#[cfg(not(any(
    all!(feature = "CONFIG_SMP", feature = "CONFIG_CPU_TLB_V6"),
    feature = "CONFIG_ARM_ERRATA_798181"
)))]
pub const ARCH_NEEDS_KMAP_HIGH_GET: bool = true;

#[cfg(not(any(
    all!(feature = "CONFIG_SMP", feature = "CONFIG_CPU_TLB_V6"),
    feature = "CONFIG_ARM_ERRATA_798181"
)))]
unsafe extern "C" {
    pub fn kmap_high_get(page: *const page) -> *mut core::ffi::c_void;
}

#[cfg(not(any(
    all!(feature = "CONFIG_SMP", feature = "CONFIG_CPU_TLB_V6"),
    feature = "CONFIG_ARM_ERRATA_798181"
)))]
#[inline]
pub unsafe fn arch_kmap_local_high_get(page: *const page) -> *mut core::ffi::c_void {
    // IS_ENABLED(CONFIG_DEBUG_HIGHMEM) && !cache_is_vivt()
    if !cache_is_vivt() {
        return core::ptr::null_mut();
    }
    kmap_high_get(page)
}

#[cfg(any(
    all!(feature = "CONFIG_SMP", feature = "CONFIG_CPU_TLB_V6"),
    feature = "CONFIG_ARM_ERRATA_798181"
))]
#[inline]
pub unsafe fn kmap_high_get(_page: *const page) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn arch_kmap_local_post_map(vaddr: usize, _pteval: usize) {
    local_flush_tlb_kernel_page(vaddr);
}

#[inline]
pub unsafe fn arch_kmap_local_pre_unmap(vaddr: usize) {
    if cache_is_vivt() {
        __cpuc_flush_dcache_area(vaddr as *mut core::ffi::c_void, PAGE_SIZE);
    }
}

#[inline]
pub unsafe fn arch_kmap_local_post_unmap(vaddr: usize) {
    local_flush_tlb_kernel_page(vaddr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
