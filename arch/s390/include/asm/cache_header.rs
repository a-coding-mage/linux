/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999
 *
 *  Derived from "include/asm-i386/cache.h"
 *    Copyright (C) 1992, Linus Torvalds
 */

// Original C header guard: __ARCH_S390_CACHE_H

pub const L1_CACHE_BYTES: usize = 256;
pub const L1_CACHE_SHIFT: u32 = 8;
pub const NET_SKB_PAD: usize = 32;

// Original C macro: __read_mostly __section(".data..read_mostly")
// The section attribute is supplied by the surrounding build environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
