/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2006, Intel Corporation.
 *
 * Copyright (C) 2006-2008 Intel Corporation
 * Author: Anil S Keshavamurthy <anil.s.keshavamurthy@intel.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct iova {
    pub node: rb_node,
    pub pfn_hi: c_ulong, /* Highest allocated pfn */
    pub pfn_lo: c_ulong, /* Lowest allocated pfn */
}

#[repr(C)]
pub struct iova_rcache;

#[repr(C)]
pub struct iova_domain {
    pub iova_rbtree_lock: spinlock_t, /* Lock to protect update of rbtree */
    pub rbroot: rb_root, /* iova domain rbtree root */
    pub cached_node: *mut rb_node, /* Save last alloced node */
    pub cached32_node: *mut rb_node, /* Save last 32-bit alloced node */
    pub granule: c_ulong, /* pfn granularity for this domain */
    pub start_pfn: c_ulong, /* Lower limit for this domain */
    pub dma_32bit_pfn: c_ulong,
    pub max32_alloc_size: c_ulong, /* Size of last failed allocation */
    pub anchor: iova, /* rbtree lookup anchor */
    pub rcaches: *mut iova_rcache,
    pub cpuhp_dead: hlist_node,
}

#[inline]
pub unsafe fn iova_size(iova: *mut iova) -> c_ulong {
    (*iova).pfn_hi.wrapping_sub((*iova).pfn_lo).wrapping_add(1)
}

#[inline]
pub unsafe fn iova_shift(iovad: *mut iova_domain) -> c_ulong {
    (*iovad).granule.trailing_zeros() as c_ulong
}

#[inline]
pub unsafe fn iova_mask(iovad: *mut iova_domain) -> c_ulong {
    (*iovad).granule.wrapping_sub(1)
}

#[inline]
pub unsafe fn iova_offset(iovad: *mut iova_domain, iova_addr: dma_addr_t) -> usize {
    (iova_addr & iova_mask(iovad) as dma_addr_t) as usize
}

#[inline]
pub unsafe fn iova_align(iovad: *mut iova_domain, size: usize) -> usize {
    size.wrapping_add((*iovad).granule as usize).wrapping_sub(1)
        & !((*iovad).granule as usize).wrapping_sub(1)
}

#[inline]
pub unsafe fn iova_align_down(iovad: *mut iova_domain, size: usize) -> usize {
    size & !((*iovad).granule as usize).wrapping_sub(1)
}

#[inline]
pub unsafe fn iova_dma_addr(iovad: *mut iova_domain, iova_obj: *mut iova) -> dma_addr_t {
    ((*iova_obj).pfn_lo as dma_addr_t) << iova_shift(iovad)
}

#[inline]
pub unsafe fn iova_pfn(iovad: *mut iova_domain, iova_addr: dma_addr_t) -> c_ulong {
    (iova_addr >> iova_shift(iovad)) as c_ulong
}

/* IS_REACHABLE(CONFIG_IOMMU_IOVA), represented here by a build feature. */
#[cfg(feature = "CONFIG_IOMMU_IOVA")]
extern "C" {
    pub fn iova_cache_get() -> c_int;
    pub fn iova_cache_put();
    pub fn iova_rcache_range() -> c_ulong;
    pub fn free_iova(iovad: *mut iova_domain, pfn: c_ulong);
    pub fn __free_iova(iovad: *mut iova_domain, iova_obj: *mut iova);
    pub fn alloc_iova(iovad: *mut iova_domain, size: c_ulong, limit_pfn: c_ulong, size_aligned: bool) -> *mut iova;
    pub fn free_iova_fast(iovad: *mut iova_domain, pfn: c_ulong, size: c_ulong);
    pub fn alloc_iova_fast(iovad: *mut iova_domain, size: c_ulong, limit_pfn: c_ulong, flush_rcache: bool) -> c_ulong;
    pub fn reserve_iova(iovad: *mut iova_domain, pfn_lo: c_ulong, pfn_hi: c_ulong) -> *mut iova;
    pub fn init_iova_domain(iovad: *mut iova_domain, granule: c_ulong, start_pfn: c_ulong);
    pub fn iova_domain_init_rcaches(iovad: *mut iova_domain) -> c_int;
    pub fn find_iova(iovad: *mut iova_domain, pfn: c_ulong) -> *mut iova;
    pub fn put_iova_domain(iovad: *mut iova_domain);
}

#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn iova_cache_get() -> c_int { -ENOTSUPP as c_int }
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn iova_cache_put() {}
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn free_iova(_: *mut iova_domain, _: c_ulong) {}
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn __free_iova(_: *mut iova_domain, _: *mut iova) {}
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn alloc_iova(_: *mut iova_domain, _: c_ulong, _: c_ulong, _: bool) -> *mut iova { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn free_iova_fast(_: *mut iova_domain, _: c_ulong, _: c_ulong) {}
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn alloc_iova_fast(_: *mut iova_domain, _: c_ulong, _: c_ulong, _: bool) -> c_ulong { 0 }
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn reserve_iova(_: *mut iova_domain, _: c_ulong, _: c_ulong) -> *mut iova { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn init_iova_domain(_: *mut iova_domain, _: c_ulong, _: c_ulong) {}
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn find_iova(_: *mut iova_domain, _: c_ulong) -> *mut iova { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_IOMMU_IOVA"))]
#[inline]
pub unsafe fn put_iova_domain(_: *mut iova_domain) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
