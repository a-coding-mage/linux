/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/types.h, linux/errno.h, linux/range.h, and linux/bug.h.

#[repr(C)]
pub struct memregion_info {
    pub target_node: i32,
    pub range: range,
}

#[cfg(CONFIG_MEMREGION)]
extern "C" {
    pub fn memregion_alloc(gfp: gfp_t) -> i32;
    pub fn memregion_free(id: i32);
}

#[cfg(not(CONFIG_MEMREGION))]
#[inline]
pub fn memregion_alloc(_gfp: gfp_t) -> i32 {
    -(ENOMEM as i32)
}

#[cfg(not(CONFIG_MEMREGION))]
#[inline]
pub fn memregion_free(_id: i32) {}

/**
 * cpu_cache_invalidate_memregion - drop any CPU cached data for
 *     memregion
 * @start: start physical address of the target memory region.
 * @len: length of the target memory region. -1 for all the regions of
 *       the target type.
 *
 * Perform cache maintenance after a memory event / operation that
 * changes the contents of physical memory in a cache-incoherent manner.
 * For example, device memory technologies like NVDIMM and CXL have
 * device secure erase, and dynamic region provision that can replace
 * the memory mapped to a given physical address.
 *
 * Limit the functionality to architectures that have an efficient way
 * to writeback and invalidate potentially terabytes of address space at
 * once.  Note that this routine may or may not write back any dirty
 * contents while performing the invalidation. It is only exported for
 * the explicit usage of the NVDIMM and CXL modules in the 'DEVMEM'
 * symbol namespace on bare platforms.
 *
 * Returns 0 on success or negative error code on a failure to perform
 * the cache maintenance.
 */
#[cfg(CONFIG_ARCH_HAS_CPU_CACHE_INVALIDATE_MEMREGION)]
extern "C" {
    pub fn cpu_cache_invalidate_memregion(start: phys_addr_t, len: size_t) -> i32;
    pub fn cpu_cache_has_invalidate_memregion() -> bool;
}

#[cfg(not(CONFIG_ARCH_HAS_CPU_CACHE_INVALIDATE_MEMREGION))]
#[inline]
pub fn cpu_cache_has_invalidate_memregion() -> bool {
    false
}

#[cfg(not(CONFIG_ARCH_HAS_CPU_CACHE_INVALIDATE_MEMREGION))]
#[inline]
pub fn cpu_cache_invalidate_memregion(_start: phys_addr_t, _len: size_t) -> i32 {
    // Equivalent to WARN_ON_ONCE("CPU cache invalidation required").
    -(ENXIO as i32)
}

#[inline]
pub fn cpu_cache_invalidate_all() -> i32 {
    cpu_cache_invalidate_memregion(0, (-1i32) as size_t)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
