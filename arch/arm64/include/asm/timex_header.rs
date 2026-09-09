/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency supplied by <asm/arch_timer.h>.
// Use the current timer as a cycle counter since this is what we use for
// the delay loop.
#[inline]
pub unsafe fn get_cycles() -> u64 {
    arch_timer_read_counter()
}

// Declarations supplied by <asm/arch_timer.h> and <asm-generic/timex.h>.
unsafe extern "C" {
    fn arch_timer_read_counter() -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
