/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-parisc/prefetch.h
 *
 * PA 2.0 defines data prefetch instructions on page 6-11 of the Kane book.
 * In addition, many implementations do hardware prefetching of both
 * instructions and data.
 *
 * PA7300LC (page 14-4 of the ERS) also implements prefetching by a load
 * to gr0 but not in a way that Linux can use.  If the load would cause an
 * interruption (eg due to prefetching 0), it is suppressed on PA2.0
 * processors, but not on 7300LC.
 *
 */

/* CONFIG_PREFETCH is a build-time configuration condition. */
#[cfg(CONFIG_PREFETCH)]
pub const ARCH_HAS_PREFETCH: bool = true;

#[cfg(CONFIG_PREFETCH)]
#[inline]
pub fn prefetch(addr: *const core::ffi::c_void) {
    unsafe {
        #[cfg(not(CONFIG_PA20))]
        core::arch::asm!(
            // Need to avoid prefetch of NULL on PA7300LC.
            "extrw,u,= {addr}, 31, 32, r0",
            addr = in(reg) addr,
        );
        core::arch::asm!(
            "ldw 0({addr}), r0",
            addr = in(reg) addr,
        );
    }
}

/* LDD is a PA2.0 addition. */
/* CONFIG_PA20 is a build-time configuration condition. */
#[cfg(all(CONFIG_PREFETCH, CONFIG_PA20))]
pub const ARCH_HAS_PREFETCHW: bool = true;

#[cfg(all(CONFIG_PREFETCH, CONFIG_PA20))]
#[inline]
pub fn prefetchw(addr: *const core::ffi::c_void) {
    unsafe {
        core::arch::asm!(
            "ldd 0({addr}), r0",
            addr = in(reg) addr,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
