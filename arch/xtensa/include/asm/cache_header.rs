/*
 * include/asm-xtensa/cache.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * (C) 2001 - 2005 Tensilica Inc.
 */

// Dependency supplied by the corresponding asm/core translation.

pub const L1_CACHE_SHIFT: _ = XCHAL_DCACHE_LINEWIDTH;
pub const L1_CACHE_BYTES: _ = XCHAL_DCACHE_LINESIZE;
pub const SMP_CACHE_BYTES: _ = L1_CACHE_BYTES;

pub const DCACHE_WAY_SIZE: _ = XCHAL_DCACHE_SIZE / XCHAL_DCACHE_WAYS;
pub const ICACHE_WAY_SIZE: _ = XCHAL_ICACHE_SIZE / XCHAL_ICACHE_WAYS;
pub const DCACHE_WAY_SHIFT: _ = XCHAL_DCACHE_SETWIDTH + XCHAL_DCACHE_LINEWIDTH;
pub const ICACHE_WAY_SHIFT: _ = XCHAL_ICACHE_SETWIDTH + XCHAL_ICACHE_LINEWIDTH;

/* Maximum cache size per way. */
pub const CACHE_WAY_SIZE: _ = if DCACHE_WAY_SIZE >= ICACHE_WAY_SIZE {
    DCACHE_WAY_SIZE
} else {
    ICACHE_WAY_SIZE
};

pub const ARCH_DMA_MINALIGN: _ = L1_CACHE_BYTES;

/*
 * R/O after init is actually writable, it cannot go to .rodata
 * according to vmlinux linker script.
 *
 * The C __ro_after_init qualifier expands to __read_mostly; Rust has no
 * file-local qualifier with equivalent linker-section semantics.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
