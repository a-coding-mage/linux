/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generic cache management functions. Everything is arch-specific,
 * but this header exists to make sure the defines/functions can be
 * used in a generic way.
 */

// C dependencies supplied by other translation units:
// linux/types.h, asm/processor.h, and asm/cache.h

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

/*
 * prefetch(x) attempts to pre-emptively get the memory pointed to
 * by address "x" into the CPU L1 cache.
 * prefetch(x) should not cause any kind of exception, prefetch(0) is
 * specifically ok.
 *
 * prefetch() should be defined by the architecture. If not, the
 * fallback below is a no-op.
 *
 * There are 2 prefetch() operations:
 *
 * prefetch(x)  - prefetches the cacheline at "x" for read
 * prefetchw(x) - prefetches the cacheline at "x" for write
 *
 * PREFETCH_STRIDE is the architecture-preferred "lookahead" size.
 */

// ARCH_HAS_PREFETCH and ARCH_HAS_PREFETCHW are build-time architecture
// conditions. Architecture-provided definitions may replace these fallbacks.
#[cfg(not(arch_has_prefetch))]
#[inline(always)]
pub unsafe fn prefetch<T>(x: *const T) {
    // C fallback: __builtin_prefetch(x)
    let _ = x;
}

#[cfg(not(arch_has_prefetchw))]
#[inline(always)]
pub unsafe fn prefetchw<T>(x: *const T) {
    // C fallback: __builtin_prefetch(x, 1)
    let _ = x;
}

// L1_CACHE_BYTES is supplied by the architecture.
#[cfg(not(prefetch_stride_defined))]
pub const PREFETCH_STRIDE: usize = 4 * L1_CACHE_BYTES;

#[inline(always)]
pub unsafe fn prefetch_range(addr: *mut core::ffi::c_void, len: usize) {
    // This body is present only when ARCH_HAS_PREFETCH is defined in C.
    #[cfg(arch_has_prefetch)]
    {
        let mut cp = addr as *mut u8;
        let end = cp.add(len);

        while cp < end {
            prefetch(cp as *const u8);
            cp = cp.add(PREFETCH_STRIDE);
        }
    }
}

#[inline(always)]
pub unsafe fn prefetch_page_address(page: *mut page) {
    // This body is present only when WANT_PAGE_VIRTUAL or
    // HASHED_PAGE_VIRTUAL is defined in C.
    #[cfg(any(want_page_virtual, hashed_page_virtual))]
    {
        prefetch(page as *const page);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
