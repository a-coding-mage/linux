/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of m68k/include/asm/cacheflush_no.h. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not implemented here.

unsafe extern "C" {
    pub fn mcf_cache_push();
}

// C macros: flush_cache_all() expands to __flush_cache_all(),
// flush_dcache_range(start, len) expands to __flush_dcache_all(), and
// flush_icache_range(start, len) expands to __flush_icache_all().
#[inline(always)]
pub unsafe fn flush_cache_all() {
    __flush_cache_all();
}

#[inline(always)]
pub unsafe fn flush_dcache_range(_start: usize, _len: usize) {
    __flush_dcache_all();
}

#[inline(always)]
pub unsafe fn flush_icache_range(_start: usize, _len: usize) {
    __flush_icache_all();
}

#[inline(always)]
pub unsafe fn __clear_cache_all() {
    // Conditional on the C build-time CACHE_INVALIDATE definition.
    #[cfg(feature = "CACHE_INVALIDATE")]
    {
        // The original emits: movec CACHE_INVALIDATE, CACR; nop.
        core::arch::asm!(
            "movec {value}, %CACR",
            "nop",
            value = in(reg) CACHE_INVALIDATE,
            options(nostack)
        );
    }
}

#[inline(always)]
pub unsafe fn __flush_cache_all() {
    // Conditional on the C build-time CACHE_PUSH definition.
    #[cfg(feature = "CACHE_PUSH")]
    {
        mcf_cache_push();
    }
    __clear_cache_all();
}

/*
 * Some ColdFire parts implement separate instruction and data caches,
 * on those we should just flush the appropriate cache. If we don't need
 * to do any specific flushing then this will be optimized away.
 */
#[inline(always)]
pub unsafe fn __flush_icache_all() {
    // Conditional on the C build-time CACHE_INVALIDATEI definition.
    #[cfg(feature = "CACHE_INVALIDATEI")]
    {
        // The original emits: movec CACHE_INVALIDATEI, CACR; nop.
        core::arch::asm!(
            "movec {value}, %CACR",
            "nop",
            value = in(reg) CACHE_INVALIDATEI,
            options(nostack)
        );
    }
}

#[inline(always)]
pub unsafe fn __flush_dcache_all() {
    // Conditional on the C build-time CACHE_PUSH definition.
    #[cfg(feature = "CACHE_PUSH")]
    {
        mcf_cache_push();
    }
    // Conditional on the C build-time CACHE_INVALIDATED definition.
    #[cfg(feature = "CACHE_INVALIDATED")]
    {
        // The original emits: movec CACHE_INVALIDATED, CACR; nop.
        core::arch::asm!(
            "movec {value}, %CACR",
            "nop",
            value = in(reg) CACHE_INVALIDATED,
            options(nostack)
        );
    }
    #[cfg(not(feature = "CACHE_INVALIDATED"))]
    {
        // Flush the write buffer.
        core::arch::asm!("nop", options(nostack));
    }
}

/*
 * Push cache entries at supplied address. We want to write back any dirty
 * data and then invalidate the cache lines associated with this address.
 */
#[inline(always)]
pub unsafe fn cache_push(_paddr: u64, _len: i32) {
    __flush_cache_all();
}

/*
 * Clear cache entries at supplied address (that is don't write back any
 * dirty data).
 */
#[inline(always)]
pub unsafe fn cache_clear(_paddr: u64, _len: i32) {
    __clear_cache_all();
}

// #include <asm-generic/cacheflush.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
