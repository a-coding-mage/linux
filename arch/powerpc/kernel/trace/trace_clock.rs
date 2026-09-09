// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2015 Naveen N. Rao, IBM Corporation
 */

// External dependency corresponding to <asm/time.h>.
unsafe extern "C" {
    fn get_tb() -> u64;
}

/// Equivalent of the C `notrace` function `trace_clock_ppc_tb`.
pub unsafe fn trace_clock_ppc_tb() -> u64 {
    unsafe { get_tb() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
