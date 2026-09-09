/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Process execution related definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* Should probably shoot for an 8-byte aligned stack pointer */
pub const STACK_MASK: usize = !7usize;

#[inline]
pub const fn arch_align_stack(x: usize) -> usize {
    x & STACK_MASK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
