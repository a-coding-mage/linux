/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Dependency provided by <asm-generic/mman-common.h>.

pub const PROT_SAO: i32 = 0x10; /* Strong Access Ordering */

pub const MAP_RENAME: i32 = MAP_ANONYMOUS; /* In SunOS terminology */
pub const MAP_NORESERVE: i32 = 0x40; /* don't reserve swap pages */
pub const MAP_LOCKED: i32 = 0x80;

pub const MAP_GROWSDOWN: i32 = 0x0100; /* stack-like segment */
pub const MAP_DENYWRITE: i32 = 0x0800; /* ETXTBSY */
pub const MAP_EXECUTABLE: i32 = 0x1000; /* mark it as an executable */

pub const MCL_CURRENT: i32 = 0x2000; /* lock all currently mapped pages */
pub const MCL_FUTURE: i32 = 0x4000; /* lock all additions to address space */
pub const MCL_ONFAULT: i32 = 0x8000; /* lock all pages that are faulted in */

/* Override any generic PKEY permission defines. */
pub const PKEY_DISABLE_EXECUTE: i32 = 0x4;
// #undef PKEY_ACCESS_MASK
pub const PKEY_ACCESS_MASK: i32 =
    PKEY_DISABLE_ACCESS | PKEY_DISABLE_WRITE | PKEY_DISABLE_EXECUTE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
