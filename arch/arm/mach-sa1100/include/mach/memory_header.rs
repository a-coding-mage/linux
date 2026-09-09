/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/mach-sa1100/include/mach/memory.h
 *
 * Copyright (C) 1999-2000 Nicolas Pitre <nico@fluxnic.net>
 */

// C header guard: __ASM_ARCH_MEMORY_H
// C dependency: <linux/sizes.h>

/*
 * Because of the wide memory address space between physical RAM banks on the
 * SA1100, it's much convenient to use Linux's SparseMEM support to implement
 * our memory map representation.  Assuming all memory nodes have equal access
 * characteristics, we then have generic discontiguous memory support.
 *
 * The sparsemem banks are matched with the physical memory bank addresses
 * which are incidentally the same as virtual addresses.
 * 
 * 	node 0:  0xc0000000 - 0xc7ffffff
 * 	node 1:  0xc8000000 - 0xcfffffff
 * 	node 2:  0xd0000000 - 0xd7ffffff
 * 	node 3:  0xd8000000 - 0xdfffffff
 */
pub const MAX_PHYSMEM_BITS: u32 = 32;
pub const SECTION_SIZE_BITS: u32 = 27;

/*
 * Cache flushing area - SA1100 zero bank
 */
pub const FLUSH_BASE_PHYS: u32 = 0xe000_0000;
pub const FLUSH_BASE: u32 = 0xf500_0000;
pub const FLUSH_BASE_MINICACHE: u32 = 0xf510_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
