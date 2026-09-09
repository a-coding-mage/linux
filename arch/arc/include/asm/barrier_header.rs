/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014-15 Synopsys, Inc. (www.synopsys.com)
 */

/*
 * CONFIG_ISA_ARCV2 is a build-time condition from the original header.
 * It is represented here by the corresponding Rust feature.
 */
#[cfg(feature = "CONFIG_ISA_ARCV2")]
mod arc_v2_barriers {
    /*
     * ARCv2 based HS38 cores are in-order issue, but still weakly ordered
     * due to micro-arch buffering/queuing of load/store, cache hit vs. miss ...
     *
     * Explicit barrier provided by DMB instruction
     *  - Operand supports fine grained load/store/load+store semantics
     *  - Ensures that selected memory operation issued before it will complete
     *    before any subsequent memory operation of same type
     *  - DMB guarantees SMP as well as local barrier semantics
     *    (asm-generic/barrier.h ensures sane smp_*mb if not defined here, i.e.
     *    UP: barrier(), SMP: smp_*mb == *mb)
     *  - DSYNC provides DMB+completion_of_cache_bpu_maintenance_ops hence not needed
     *    in the general case. Plus it only provides full barrier.
     */
    #[macro_export]
    macro_rules! mb {
        () => {{
            unsafe { core::arch::asm!("dmb 3", options(nostack)) }
        }};
    }

    #[macro_export]
    macro_rules! rmb {
        () => {{
            unsafe { core::arch::asm!("dmb 1", options(nostack)) }
        }};
    }

    #[macro_export]
    macro_rules! wmb {
        () => {{
            unsafe { core::arch::asm!("dmb 2", options(nostack)) }
        }};
    }
}

#[cfg(not(feature = "CONFIG_ISA_ARCV2"))]
mod arc_compact_barriers {
    /*
     * ARCompact based cores (ARC700) only have SYNC instruction which is super
     * heavy weight as it flushes the pipeline as well.
     * There are no real SMP implementations of such cores.
     */
    #[macro_export]
    macro_rules! mb {
        () => {{
            unsafe { core::arch::asm!("sync", options(nostack)) }
        }};
    }
}

/* The original header includes <asm-generic/barrier.h>. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
