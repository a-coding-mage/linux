/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <asm/cpu-features.h>

macro_rules! cpu_dcache_is_aliasing {
    () => {
        cpu_has_dc_aliases
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
