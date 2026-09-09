/* SPDX-License-Identifier: GPL-2.0 */
/* hardirq.h: 64-bit Sparc hard IRQ support.
 *
 * Copyright (C) 1997, 1998, 2005 David S. Miller (davem@davemloft.net)
 */

// Dependency equivalent of: #include <asm/cpudata.h>

/* Marker macro: #define __ARCH_IRQ_STAT */
#[allow(non_upper_case_globals)]
pub const __ARCH_IRQ_STAT: () = ();

/*
 * Equivalent of:
 * #define local_softirq_pending_ref __cpu_data.__softirq_pending
 *
 * `__cpu_data` is supplied by the corresponding CPU-data dependency.
 */
#[macro_export]
macro_rules! local_softirq_pending_ref {
    () => {
        __cpu_data.__softirq_pending
    };
}

extern "C" {
    pub fn ack_bad_irq(irq: core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
