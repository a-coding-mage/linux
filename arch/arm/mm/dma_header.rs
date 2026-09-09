/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: asm/glue-cache.h supplies the cache glue and, for MULTI_CACHE,
// the cpu_cache object.

use core::ffi::c_void;

// When MULTI_CACHE is not defined, dmac_map_area and dmac_unmap_area resolve
// through the cache glue to the selected cache implementation.
//
// These are private to the dma-mapping API. Do not use directly.
// Their sole purpose is to ensure that data held in the cache
// is visible to DMA, or data written by DMA to system memory is
// visible to the CPU.
#[cfg(not(feature = "MULTI_CACHE"))]
unsafe extern "C" {
    pub fn dmac_map_area(addr: *const c_void, size: usize, dir: i32);
    pub fn dmac_unmap_area(addr: *const c_void, size: usize, dir: i32);
}

// When MULTI_CACHE is defined, the C macros resolve to
// cpu_cache.dma_map_area and cpu_cache.dma_unmap_area. The corresponding
// externally supplied cpu_cache dependency is intentionally not defined here.
#[cfg(feature = "MULTI_CACHE")]
macro_rules! dmac_map_area {
    ($($arg:tt)*) => { cpu_cache.dma_map_area($($arg)*) };
}

#[cfg(feature = "MULTI_CACHE")]
macro_rules! dmac_unmap_area {
    ($($arg:tt)*) => { cpu_cache.dma_unmap_area($($arg)*) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
