/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-rpc/include/mach/memory.h
 *
 *  Copyright (C) 1996,1997,1998 Russell King.
 *
 *  Changelog:
 *   20-Oct-1996 RMK	Created
 *   31-Dec-1997 RMK	Fixed definitions to reduce warnings
 *   11-Jan-1998 RMK	Uninlined to reduce hits on cache
 *   08-Feb-1998 RMK	Added __virt_to_bus and __bus_to_virt
 *   21-Mar-1999 RMK	Renamed to memory.h
 *			 RMK	Added TASK_SIZE and PAGE_OFFSET
 */

/*
 * Cache flushing area - ROM
 */
pub const FLUSH_BASE_PHYS: u32 = 0x0000_0000;
pub const FLUSH_BASE: u32 = 0xdf00_0000;

/*
 * Sparsemem support.  Each section is a maximum of 64MB.  The sections
 * are offset by 128MB and can cover 128MB, so that gives us a maximum
 * of 29 physmem bits.
 */
pub const MAX_PHYSMEM_BITS: u32 = 29;
pub const SECTION_SIZE_BITS: u32 = 26;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
