/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MTD primitives for XIP support
 *
 * Author: Nicolas Pitre
 * Created: Nov 2, 2004
 * Copyright: (C) 2004 MontaVista Software, Inc.
 *
 * This XIP support for MTD has been loosely inspired
 * by an earlier patch authored by David Woodhouse.
 */

/* The declarations below are active when CONFIG_MTD_XIP is enabled. */

/*
 * Function that are modifying the flash state away from array mode must
 * obviously not be running from flash.  The __xipram marker is therefore
 * intended to relocate those functions to RAM when CONFIG_XIP_KERNEL is set.
 */

/*
 * Each architecture has to provide the following macros.  They must access
 * the hardware directly and not rely on any other (XIP) functions since they
 * won't be available when used (flash not in array mode).
 *
 * xip_irqpending(): return non-zero when any hardware interrupt is pending.
 * xip_currtime(): return a platform-specific time reference.
 * xip_elapsed_since(x): return elapsed microseconds since the reference.
 * xip_iprefetch(): fill the instruction prefetch buffer.
 */

/* Architecture-provided <asm/mtd-xip.h> definitions are external dependencies. */

/* Fallbacks used when the architecture does not provide XIP primitives. */
#[inline(always)]
pub const fn xip_irqpending() -> i32 {
    0
}

#[inline(always)]
pub const fn xip_currtime() -> i32 {
    0
}

#[inline(always)]
pub const fn xip_elapsed_since<T>(_x: T) -> i32 {
    0
}

#[inline(always)]
pub fn xip_iprefetch() {}

/*
 * xip_cpu_idle() is used when waiting for a delay equal or larger than the
 * system timer tick period.  This should put the CPU into idle mode to save
 * power and to be woken up only when some interrupts are pending.  It should
 * not rely upon standard kernel code.
 */
#[inline(always)]
pub fn xip_cpu_idle() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
