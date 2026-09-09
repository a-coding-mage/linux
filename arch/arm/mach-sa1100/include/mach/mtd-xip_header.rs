/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MTD primitives for XIP support. Architecture specific functions
 *
 * Do not include this file directly. It's included from linux/mtd/xip.h
 *
 * Author:\tNicolas Pitre
 * Created:\tNov 2, 2004
 * Copyright:\t(C) 2004 MontaVista Software, Inc.
 */

// Dependency supplied by mach/hardware.h in the original C header.

/*
 * xip_irqpending()\t(ICIP & ICMR)
 */
#[macro_export]
macro_rules! xip_irqpending {
    () => {
        ICIP & ICMR
    };
}

/* we sample OSCR and convert desired delta to usec (1/4 ~= 1000000/3686400) */
#[macro_export]
macro_rules! xip_currtime {
    () => {
        readl_relaxed(OSCR)
    };
}

#[macro_export]
macro_rules! xip_elapsed_since {
    ($x:expr) => {
        ((readl_relaxed(OSCR).wrapping_sub($x)) / 4) as i32
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
