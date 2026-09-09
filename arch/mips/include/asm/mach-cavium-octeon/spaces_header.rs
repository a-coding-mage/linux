/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 Cavium, Inc.
 */

// Dependency intent: linux/const.h
// Dependency intent: asm/mach-generic/spaces.h

// #ifdef CONFIG_64BIT
// They are all the same and some OCTEON II cores cannot handle 0xa8..
pub const CAC_BASE: u64 = 0x8000_0000_0000_0000u64;
pub const UNCAC_BASE: u64 = 0x8000_0000_0000_0000u64;
pub const IO_BASE: u64 = 0x8000_0000_0000_0000u64;
// #endif /* CONFIG_64BIT */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
