/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/cma.h, linux/debugfs.h, and linux/kobject.h.

#[repr(C)]
pub struct cma_kobject {
    pub kobj: kobject,
    pub cma: *mut cma,
}

/*
 * Multi-range support. This can be useful if the size of the allocation
 * is not expected to be larger than the alignment (like with hugetlb_cma),
 * and the total amount of memory requested, while smaller than the total
 * amount of memory available, is large enough that it doesn't fit in a
 * single physical memory range because of memory holes.
 *
 * Fields:
 *   @base_pfn: physical address of range
 *   @early_pfn: first PFN not reserved through cma_reserve_early
 *   @count: size of range
 *   @bitmap: bitmap of allocated (1 << order_per_bit)-sized chunks.
 */
#[repr(C)]
pub struct cma_memrange {
    pub base_pfn: ::core::ffi::c_ulong,
    pub count: ::core::ffi::c_ulong,
    pub _bindgen_anon_1: cma_memrange__bindgen_ty_1,
    #[cfg(CONFIG_CMA_DEBUGFS)]
    pub dfs_bitmap: debugfs_u32_array,
}

#[repr(C)]
pub union cma_memrange__bindgen_ty_1 {
    pub early_pfn: ::core::ffi::c_ulong,
    pub bitmap: *mut ::core::ffi::c_ulong,
}

pub const CMA_MAX_RANGES: usize = 8;

#[repr(C)]
pub struct cma {
    pub count: ::core::ffi::c_ulong,
    pub available_count: ::core::ffi::c_ulong,
    pub order_per_bit: ::core::ffi::c_uint, /* Order of pages represented by one bit */
    pub lock: spinlock_t,
    pub alloc_mutex: mutex,
    #[cfg(CONFIG_CMA_DEBUGFS)]
    pub mem_head: hlist_head,
    #[cfg(CONFIG_CMA_DEBUGFS)]
    pub mem_head_lock: spinlock_t,
    pub name: [::core::ffi::c_char; CMA_MAX_NAME],
    pub nranges: ::core::ffi::c_int,
    pub ranges: [cma_memrange; CMA_MAX_RANGES],
    #[cfg(CONFIG_CMA_SYSFS)]
    /* the number of CMA page successful allocations */
    pub nr_pages_succeeded: atomic64_t,
    #[cfg(CONFIG_CMA_SYSFS)]
    /* the number of CMA page allocation failures */
    pub nr_pages_failed: atomic64_t,
    #[cfg(CONFIG_CMA_SYSFS)]
    /* the number of CMA page released */
    pub nr_pages_released: atomic64_t,
    #[cfg(CONFIG_CMA_SYSFS)]
    /* kobject requires dynamic object */
    pub cma_kobj: *mut cma_kobject,
    pub flags: ::core::ffi::c_ulong,
    /* NUMA node (NUMA_NO_NODE if unspecified) */
    pub nid: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cma_flags {
    CMA_RESERVE_PAGES_ON_ERROR,
    CMA_ZONES_VALID,
    CMA_ZONES_INVALID,
    CMA_ACTIVATED,
}

extern "C" {
    pub static mut cma_areas: [cma; MAX_CMA_AREAS];
    pub static mut cma_area_count: ::core::ffi::c_uint;
}

#[inline]
pub unsafe fn cma_bitmap_maxno(cma: *mut cma, cmr: *mut cma_memrange) -> ::core::ffi::c_ulong {
    (*cmr).count >> (*cma).order_per_bit
}

#[cfg(CONFIG_CMA_SYSFS)]
extern "C" {
    pub fn cma_sysfs_account_success_pages(cma: *mut cma, nr_pages: ::core::ffi::c_ulong);
    pub fn cma_sysfs_account_fail_pages(cma: *mut cma, nr_pages: ::core::ffi::c_ulong);
    pub fn cma_sysfs_account_release_pages(cma: *mut cma, nr_pages: ::core::ffi::c_ulong);
}

#[cfg(not(CONFIG_CMA_SYSFS))]
#[inline]
pub unsafe fn cma_sysfs_account_success_pages(_cma: *mut cma, _nr_pages: ::core::ffi::c_ulong) {}

#[cfg(not(CONFIG_CMA_SYSFS))]
#[inline]
pub unsafe fn cma_sysfs_account_fail_pages(_cma: *mut cma, _nr_pages: ::core::ffi::c_ulong) {}

#[cfg(not(CONFIG_CMA_SYSFS))]
#[inline]
pub unsafe fn cma_sysfs_account_release_pages(_cma: *mut cma, _nr_pages: ::core::ffi::c_ulong) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
