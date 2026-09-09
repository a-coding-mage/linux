/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding cache and page headers:
// #include <asm/cache.h>
// #include <asm/page.h>

/// Returns whether the data cache is aliasing.
#[inline]
pub const fn cpu_dcache_is_aliasing() -> bool {
    DCACHE_WAY_SIZE > PAGE_SIZE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
