/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <asm/spitfire.h>: PAGE_SIZE and L1DCACHE_SIZE are
// supplied by the surrounding translation unit.

pub const __ARCH_FORCE_SHMLBA: i32 = 1;

// attach addr a multiple of this
pub const SHMLBA: usize = if PAGE_SIZE > L1DCACHE_SIZE {
    PAGE_SIZE
} else {
    L1DCACHE_SIZE
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
