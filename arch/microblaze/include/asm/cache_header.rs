/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Cache operations
 *
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2003 John Williams <jwilliams@itee.uq.edu.au>
 */

// Dependency supplied by the surrounding MicroBlaze environment:
// #include <asm/registers.h>

pub const L1_CACHE_SHIFT: i32 = 5;
/* word-granular cache in microblaze */
pub const L1_CACHE_BYTES: i32 = 1 << L1_CACHE_SHIFT;

pub const SMP_CACHE_BYTES: i32 = L1_CACHE_BYTES;

/* MS be sure that SLAB allocates aligned objects */
pub const ARCH_DMA_MINALIGN: i32 = L1_CACHE_BYTES;

pub const ARCH_SLAB_MINALIGN: i32 = L1_CACHE_BYTES;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
