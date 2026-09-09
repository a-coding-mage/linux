/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard `___ASM_SPARC_CACHEFLUSH_H` has no direct Rust equivalent.

/* flush addr - to allow use of self-modifying code */
#[inline(always)]
pub unsafe fn flushi<T>(addr: *const T) {
    core::arch::asm!("flush {0}", in(reg) addr, options(nostack, preserves_flags));
}

// On 64-bit SPARC, this header includes <asm/cacheflush_64.h>; otherwise it
// includes <asm/cacheflush_32.h>. Those external declarations are supplied by
// the corresponding translated dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
