/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm/timex.h
 *
 * sparc64 architecture timex specifications
 */

// Dependency: <asm/timer.h>

/* Getting on the cycle counter on sparc64. */
pub type cycles_t = u64;

/* Equivalent to: tick_ops->get_tick() */
#[macro_export]
macro_rules! get_cycles {
    () => {
        unsafe { tick_ops.get_tick() }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
