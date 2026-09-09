/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic arch dependent ring_buffer macros.
 */

// Dependency intent: the C header includes <linux/cacheflush.h>.

/* Flush cache on ring buffer range if needed. Do nothing by default. */
macro_rules! arch_ring_buffer_flush_range {
    ($start:expr, $end:expr) => {{
        // The default implementation intentionally does nothing.
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
