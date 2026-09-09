/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding page and cache headers.

#[inline]
pub const fn cpu_dcache_is_aliasing() -> bool {
    NIOS2_DCACHE_SIZE > PAGE_SIZE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
