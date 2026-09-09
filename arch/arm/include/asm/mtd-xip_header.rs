/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MTD primitives for XIP support. Architecture specific functions
 *
 * Do not include this file directly. It's included from linux/mtd/xip.h
 *
 * Author: Nicolas Pitre
 * Created: Nov 2, 2004
 * Copyright: (C) 2004 MontaVista Software, Inc.
 */

// Dependency supplied by <mach/mtd-xip.h> is provided externally.

/// Fill instruction prefetch.
#[inline(always)]
pub unsafe fn xip_iprefetch() {
    core::arch::asm!(".rep 8; nop; .endr");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
