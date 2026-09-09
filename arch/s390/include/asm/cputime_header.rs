/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright IBM Corp. 2004
 *
 *  Author: Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Dependency intent: the C header includes linux/types.h and asm/timex.h.

/*
 * Convert cputime to nanoseconds.
 */
#[macro_export]
macro_rules! cputime_to_nsecs {
    ($cputime:expr) => {
        tod_to_ns($cputime)
    };
}

unsafe extern "C" {
    pub fn account_idle_time_irq();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
