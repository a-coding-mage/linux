/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/types.h>

#[inline]
pub const fn cpu_dcache_is_aliasing() -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
