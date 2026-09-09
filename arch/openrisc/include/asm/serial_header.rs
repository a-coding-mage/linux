/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

/* The original header guard is omitted from executable Rust. */

/* The original contents are conditional on __KERNEL__.  Preserve that
 * build-time intent here; this translation assumes the kernel context when
 * the macro below is used.
 */

/* There's a generic version of this file, but it assumes a 1.8MHz UART clk...
 * this, on the other hand, assumes the UART clock is tied to the system
 * clock... 8250_early.c (early 8250 serial console) actually uses this, so
 * it needs to be correct to get the early console working.
 */

/// Equivalent of the C object-like `BASE_BAUD` macro.
#[macro_export]
macro_rules! BASE_BAUD {
    () => {
        cpuinfo_or1k[smp_processor_id()].clock_frequency / 16
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
