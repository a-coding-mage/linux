/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent: <asm/cacheflush.h>

/* Flush D-cache on persistent ring buffer */
macro_rules! arch_ring_buffer_flush_range {
    ($start:expr, $end:expr) => {
        dcache_clean_pop($start, $end)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
