/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2015 Naveen N. Rao, IBM Corporation
 */

// <linux/compiler.h>
// <linux/types.h>

/// External PowerPC time-base trace clock function (`notrace`).
unsafe extern "C" {
    pub fn trace_clock_ppc_tb() -> u64;
}

/// Equivalent of `ARCH_TRACE_CLOCKS`.
#[macro_export]
macro_rules! ARCH_TRACE_CLOCKS {
    () => {
        (trace_clock_ppc_tb, "ppc-tb", 0)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
