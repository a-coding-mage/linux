/*
 * include/linux/dmapool.h
 *
 * Allocation pools for DMAable (coherent) memory.
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// Dependencies supplied by the surrounding kernel translation:
// nodemask types, scatterlist/allocator types, and I/O definitions.

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_pool {
    _private: [u8; 0],
}

// CONFIG_HAS_DMA is a build-time condition preserved as a Rust cfg.
#[cfg(CONFIG_HAS_DMA)]
extern "C" {
    pub fn dma_pool_create_node(
        name: *const core::ffi::c_char,
        dev: *mut device,
        size: usize,
        align: usize,
        boundary: usize,
        node: core::ffi::c_int,
    ) -> *mut dma_pool;

    pub fn dma_pool_destroy(pool: *mut dma_pool);

    pub fn dma_pool_alloc(
        pool: *mut dma_pool,
        mem_flags: gfp_t,
        handle: *mut dma_addr_t,
    ) -> *mut c_void;

    pub fn dma_pool_free(pool: *mut dma_pool, vaddr: *mut c_void, addr: dma_addr_t);

    /* Managed DMA pool */
    pub fn dmam_pool_create(
        name: *const core::ffi::c_char,
        dev: *mut device,
        size: usize,
        align: usize,
        allocation: usize,
    ) -> *mut dma_pool;

    pub fn dmam_pool_destroy(pool: *mut dma_pool);
}

// !CONFIG_HAS_DMA: the following inline functions are the no-DMA fallbacks.
#[cfg(not(CONFIG_HAS_DMA))]
#[inline]
pub unsafe fn dma_pool_create_node(
    _name: *const core::ffi::c_char,
    _dev: *mut device,
    _size: usize,
    _align: usize,
    _boundary: usize,
    _node: core::ffi::c_int,
) -> *mut dma_pool {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_HAS_DMA))]
#[inline]
pub unsafe fn dma_pool_destroy(_pool: *mut dma_pool) {}

#[cfg(not(CONFIG_HAS_DMA))]
#[inline]
pub unsafe fn dma_pool_alloc(
    _pool: *mut dma_pool,
    _mem_flags: gfp_t,
    _handle: *mut dma_addr_t,
) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_HAS_DMA))]
#[inline]
pub unsafe fn dma_pool_free(_pool: *mut dma_pool, _vaddr: *mut c_void, _addr: dma_addr_t) {}

#[cfg(not(CONFIG_HAS_DMA))]
#[inline]
pub unsafe fn dmam_pool_create(
    _name: *const core::ffi::c_char,
    _dev: *mut device,
    _size: usize,
    _align: usize,
    _allocation: usize,
) -> *mut dma_pool {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_HAS_DMA))]
#[inline]
pub unsafe fn dmam_pool_destroy(_pool: *mut dma_pool) {}

#[inline]
pub unsafe fn dma_pool_create(
    name: *const core::ffi::c_char,
    dev: *mut device,
    size: usize,
    align: usize,
    boundary: usize,
) -> *mut dma_pool {
    dma_pool_create_node(name, dev, size, align, boundary, NUMA_NO_NODE)
}

/**
 * dma_pool_zalloc - Get a zero-initialized block of DMA coherent memory.
 * @pool: dma pool that will produce the block
 * @mem_flags: GFP_* bitmask
 * @handle: pointer to dma address of block
 *
 * Same as dma_pool_alloc(), but the returned memory is zeroed.
 */
#[inline]
pub unsafe fn dma_pool_zalloc(
    pool: *mut dma_pool,
    mem_flags: gfp_t,
    handle: *mut dma_addr_t,
) -> *mut c_void {
    dma_pool_alloc(pool, mem_flags | __GFP_ZERO, handle)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
